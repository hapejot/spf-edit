# Chapter 2. Controlling ISPF sessions

Source file: f54dg00_v3r1.md
Start page: 35
Page span: 35-64

## Page 35

Chapter 2. Controlling ISPF sessions
This topic is intended to help you understand how to control ISPF sessions. It describes how to start and
stop an ISPF session and how to use many of the ISPF facilities.
Dialog control and data flow
Figure 5 on page 7 illustrates dialog control and data flow. At the start of an ISPF session, you can use
the ISPSTART command either to request a selection panel from which to choose the first task or to call a
dialog function. The figure also illustrates how the ISPF services interact with the various dialog elements.
Figure 5. Control and data flow
Processing a dialog
Figure 6 on page 8 shows a dialog being processed under ISPF. The figure shows that ISPF dialog
services are available only to command procedures or programs running under ISPF. During dialog
processing, the dialog requests specific ISPF services and identifies the panel and message definitions,
skeletons, and tables to use. The figure also shows that entries in the log and list data sets, as well as the
file-tailoring output data sets, can be generated during dialog processing.
© Copyright IBM Corp. 1980, 2025 7

## Page 36

Figure 6. Application dialog running under ISPF
Dialog processing begins either with the display of a selection panel or with a function. In either case, you
can invoke a dialog from a terminal running under control of TSO.
Starting a dialog
You can use the ISPF, PDF, or ISPSTART command, with the CMD, PGM, or PANEL keyword, to start ISPF
or other dialogs. ISPF is a command procedure that runs under TSO. For example, it can be run from a
terminal running under TSO, or from a CLIST or REXX command procedure.
Before a dialog starts, data sets referred to by that dialog must be defined to ISPF.
Syntax for issuing the ISPSTART command
You invoke ISPF by using the ISPSTART command. ISPSTART command parameters specify the first menu
to be displayed or the first function to receive control before the display of a menu.
If no parameters are specified, the ISPSTART command displays the default primary panel specified in
the DEFAULT_PRIMARY_PANEL keyword in the ISPF configuration table. This keyword is typically set to
ISP@MSTR.
An initial primary menu option or a command stack variable name can be specified with the ISPSTART
command. If a command stack variable name is specified, ISPF attempts to retrieve and validate
the specified profile variable. If neither an initial menu option nor a command stack variable name
is specified, ISPF attempts to retrieve and validate the default command stack profile variable
ZSTART. If the specified or default command stack profile variable is valid (of the format ISPF||
command_delimiter|| command_stack), ISPF executes the command stack contained in the variable
as though it were entered from the primary panel. If ISPF cannot retrieve the variable or the variable
is not valid then ISPF displays the primary panel. Refer to the description for the ISPSTART command
parameter cmd_stack_var_name for details. For information on ISPF variables related to the processing
8  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 37

of an initial command stack, refer to the descriptions of ZSTARTPR and ZINICMD in the table of general
variables in Appendix E.
The PDF and ISPF commands are aliases for ISPSTART that can be used to start ISPF. If you enter ISPF
or PDF with no parameters, the command ISPSTART PANEL(panel) NEWAPPL(ISR) is run, where panel is
determined by these rules:
• If the default primary panel is ISP@MSTR or is not set, panel=ISR@PRIM
• If the default primary panel is set to any other panel, panel=DEFAULT_PRIMARY_PANEL
Parameters
All the parameters described here apply to the PDF and ISPF commands as well as ISPSTART. 
Chapter 2. Controlling ISPF sessions  9

## Page 38

ISPSTART
PANEL( panel_name)
OPT(ZSTART)
OPT( panopt
BASIC
cmd_stack_var_name
) ADDPOP
CMD( commandparm1parm2)
LANG( APL
CREX
)
PGM( program_name)
PARM( parameters)
ZSTART
option
BASIC
cmd_stack_var_name
CODEPAGE( codepage) CHARSET( character_set)
NEWAPPL
( application_id)
SHRPROF
EXCLPROF
SCRNAME( screen_name) TEST
TESTX
TRACE
TRACEX
NOLOGO
LOGO( logo_panel_name)
BATSCRW( screen_width) BATSCRD( screen_depth)
BDISPMAX( max_number_of_displays)
BREDIMAX( max_number_of_redisplays) BDBCS DANISH
ENGLISH
GERMAN
JAPANESE
PORTUGUE
SPANISH
KOREAN
FRENCH
ITALIAN
CHINESET
CHINESES
SGERMAN
UPPERENG
NESTMACS
where:
panel_name
Specifies the name of the first panel to be displayed. This panel is referred to in this section as the
primary option menu.
10  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 39

OPT
Specifies an initial option or an initial command stack variable to be processed by the primary option
menu. If you specify an option or an initial command stack variable that is not valid, the primary
option menu displays an appropriate error message.
panopt
Specifies an initial option, which should be a valid option on the primary option menu. This causes
direct entry to that option without displaying the primary option menu. The primary option menu is
processed in nondisplay mode, as though the user had entered the option.
A NULL panopt value, OPT(), can be used to bypass processing of the ZSTART variable. When this
is used, the primary option menu is displayed in display mode with no option selected and without
executing any options listed in variable ZSTART.
BASIC
When the default initial command stack variable ZSTART is defined, BASIC can be used to bypass
processing of the ZSTART variable. When this is used, the primary option menu is displayed in
display mode with no option selected and without executing any options listed in variable ZSTART.
cmd_stack_var_name
The name of an ISPF profile variable that contains an initial command stack to be processed by
ISPF. The specified command stack is processed by ISPF as though it had been entered on the
initial display of the primary option menu. The first four characters of the variable value must be
"ISPF" followed by the command delimiter character followed by the initial command stack.
Note: The default command delimiter is a semi-colon (;). You can change it in the ISPF settings.
The following example specifies a command stack to have three logical screens created when
ISPF starts:
Screen 1 - Data Set List Utility (ISPF option 3.4) 
Screen 2 - z/OS UNIX Directory List Utility (ISPF option 3.17)
Screen 3 - SCLM (ISPF option 10)
The Data Set List Utility is the initial logical screen displayed:
ISPF;3.4;START 3.17;START 10;SWAP 1
When the PANEL parameter is specified and the OPT parameter is not specified, the default
cmd_stack_var_name value ZSTART is used. If the ZSTART variable is defined and valid, ISPF
executes the command stack contained in the ZSTART variable as though entered on the initial
display of the primary option menu. If ISPF cannot retrieve the ZSTART variable or the variable
definition is not valid, ISPF displays the primary option menu.
To support an initial command stack being provided in an ISPF variable, ISPF puts the variable
name (or "ZSTART DEFAULT" when the default cmd_stack_var_name value ZSTART is used) into
the ZCMD variable. Then, the )PROC section of the first primary option menu displayed is executed
before the initial display of the panel. The primary option menu must not perform verification
of the ZCMD variable in the )PROC section unless the verification allows for the initial command
stack variable name (for example, ZSTART) to be stored in ZCMD.
ADDPOP
Specifies that the panel displayed from a SELECT service appears in a pop-up window. An explicit
REMPOP is performed when the SELECT PANEL has ended.
command
Specifies a command procedure (CLIST or REXX), an APL2 command, or a TSO command processor
that is to be invoked as the first dialog function. For more information about invoking APL2 dialogs,
refer to the z/OS ISPF Services Guide.
Chapter 2. Controlling ISPF sessions  11

## Page 40

CLIST or REXX command parameters can be included within the parentheses. For example, the call
format would be:
   ISPSTART CMD(MYCLIST parm1 parm2 …)
These parameters are passed to the command procedure. For information about specifying CLIST
parameters, see z/OS TSO/E CLISTs. For information about specifying REXX parameters, see z/OS
TSO/E REXX User's Guide.
You can type a percent sign (%) preceding the CLIST or REXX procedure name to:
• Improve performance
• Prevent ISPF from entering line-display mode when the procedure is started.
Note: When starting a CLIST or REXX procedure or a program through the SELECT service, a
MODE(LINE|FSCR) parameter is available for specifying either line mode or full-screen mode. If you
do not specify the mode parameter or do not use the % prefix, ISPF enters line-display mode.
• Ensure that the command procedure is invoked if ISPF has access to a program function that has
the same name as the procedure. If you use the percent sign prefix, ISPF searches only for a
procedure with the specified name. However, without the percent sign prefix, ISPF searches first for
a program, then for a CLIST or REXX procedure.
On extended data stream terminals, using the percent sign causes the keyboard to remain in a locked
condition. To avoid this condition, the CLIST or REXX procedure can issue output line I/O before
issuing a READ.
LANG(APL|CREX)
Specifies special language invocations. LANG(APL) specifies to start the command specified by the
CMD keyword, and to start an APL2 environment. LANG(CREX) specifies that the command specified
by the CMD keyword is a REXX exec that has been compiled and link-edited into a LOAD module and
that a CLIST/REXX function pool is to be used. LANG(CREX) is optional if the compiled REXX has been
link-edited to include any of the stubs EAGSTCE, EAGSTCPP, or EAGSTMP.
program_name
Specifies the name of a program that is to be invoked as the first dialog function. In PL/I, it must be a
MAIN procedure. This parameter must specify the name of a load module that is accessible by use of
the LINK macro.
However, if the program dialog consists of multiple tasks and if any of the subtasks use ISPF services,
the CMD keyword, not the PGM keyword, must be used. Dialog developers should avoid using prefixes
ISP and ISR, the ISPF component codes, in naming dialog functions. Special linkage conventions,
intended only for internal ISPF use, are used to invoke programs named ISPxxxxx and ISRxxxxx.
parameters
Specifies input parameters to be passed to the program. The program should not attempt to modify
these parameters.
The parameters within the parentheses are passed as a single character string, preceded by a half-
word containing the length of the character string, in binary. (The length value does not include itself.)
This convention is the same as that for passing parameters by use of the PARM= keyword on a JCL
EXEC statement.
Parameters on the ISPSTART command to be passed to a PL/I program are coded in the standard
way:
XXX:  PROC (PARM) OPTIONS(MAIN);
      DCL PARM CHAR (nnn) VAR;
 
If the value of the PARM field is to be used as an ISPF dialog variable, it must be assigned to a fixed
character string because the VDEFINE service cannot handle varying length PL/I strings. In PL/I the
first character of the PARM field must be a slash (/), as PL/I assumes that any value before the slash is
a runtime option.
12  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 41

option|BASIC|cmd_stack_var_name
Specifies an initial option or an initial command stack variable to be processed by the default primary
option menu. If you specify an option or an initial command stack variable that is not valid, the default
primary option menu displays an appropriate error message.
option
Specifies an initial option, which should be a valid option on the default primary option menu. This
causes direct entry to that option without displaying the default primary option menu. The default
primary option menu is processed in nondisplay mode, as though the user had entered the option.
BASIC
When the default initial command stack variable ZSTART is defined, BASIC can be used to
bypass processing of the ZSTART variable. When this is used, the default primary option menu
is displayed in display mode with no option selected and without executing any options listed in
variable ZSTART.
cmd_stack_var_name
The name of an ISPF profile variable that contains an initial command stack to be processed by
ISPF. The specified command stack is processed by ISPF as though it had been entered on the
initial display of the default primary option menu. The first four characters of the variable value
must be "ISPF" followed by the command delimiter character followed by the initial command
stack.
Note: The default command delimiter is a semi-colon (;). You can change it in the ISPF settings.
The following example specifies a command stack to have three logical screens created when
ISPF starts:
Screen 1 - Data Set List Utility (ISPF option 3.4) 
Screen 2 - z/OS UNIX Directory List Utility (ISPF option 3.17)
Screen 3 - SCLM (ISPF option 10)
The Data Set List Utility is the initial logical screen displayed:
ISPF;3.4;START 3.17;START 10;SWAP 1
When none of these three options (option, BASIC, and cmd_stack_var_name) is specified, the
default cmd_stack_var_name value ZSTART is used. If the ZSTART variable is defined and valid,
ISPF executes the command stack contained in the ZSTART variable as though entered on the
default primary option menu. If ISPF cannot retrieve the ZSTART variable or the variable definition
is not valid, ISPF displays the default primary option menu.
To support an initial command stack being provided in an ISPF variable, ISPF puts the variable
name (or "ZSTART DEFAULT" when the default cmd_stack_var_name value ZSTART is used) into
the ZCMD variable. Then, the )PROC section of the default primary option menu is executed before
the initial display of the panel. The default primary option menu must not perform verification
of the ZCMD variable in the )PROC section unless the verification allows for the initial command
stack variable name (for example, ZSTART) to be stored in ZCMD.
CODEPAGE(codepage) CHARSET(character_set)
If your terminal or emulator does not support code pages, these values are used as the host code
page and character set. Otherwise, these values are ignored.
NEWAPPL(application_id)
Specifies a 1- to 4-character code that identifies the application that is being invoked. The code
is to be prefixed to the user and edit profile names or to the command table associated with the
application, as follows:
User Profile   -  xxxxPROF
Edit Profile   -  xxxxEDIT
Command Table  -  xxxxCMDS
Chapter 2. Controlling ISPF sessions  13

## Page 42

where xxxx is the application_id. If the application_id is omitted, or if the NEWAPPL keyword is
omitted, the application_id defaults to ISP.
SHRPROF
Specifies that ISPF is to enable the multi-logon profile sharing support. The parameter is optional.
EXCLPROF
Specifies that ISPF is to disable the multi-logon profile sharing support. The parameter is optional
SCRNAME(screen_name)
Specifies a screen name to be used with the SWAP command and the ISPF task list. The name can be
from 2 to 8 characters in length, must satisfy the rules for a member name, but cannot be LIST, PREV,
or NEXT.
TEST
Specifies that ISPF is to be operated in TEST mode, described under “ISPF test and trace modes” on
page 23.
TESTX
Specifies that ISPF is to be operated in extended TEST mode, described under “ISPF test and trace
modes” on page 23.
TRACE
Specifies that ISPF is to be operated in TRACE mode, described under “ISPF trace modes” on page
24.
TRACEX
Specifies that ISPF is to be operated in extended TRACE mode, described under “ISPF trace modes”
on page 24.
LOGO(logo_panel_name)
Specifies that ISPF displays the named panel before invoking the specified dialog object. Subsequent
SELECT service requests that identify a LOGO panel will not result in the indicated panel being
displayed. This includes a repeat of the first SELECT as a result of a split-screen request or a logical
screen restart following a severe dialog error.
Applications can choose to display their own LOGO panel directly. These applications can determine
whether the user specified the NOLOGO keyword on ISPSTART by retrieving the ISPF system variable
ZLOGO. Applications that choose to display their own LOGO panel are responsible for controlling that
display operation during split-screen operations and logical-screen restart situations.
NOLOGO
Specifies that ISPF is to bypass the display of the message pop-up window containing the product
title and copyright statement.
screen_width
For batch mode, specifies screen width in character positions. The default value is 80. This parameter
is ignored when not running in batch mode.
All screen sizes from 24 x 80 to 62 x 160 are valid.
screen_depth
For batch mode, specifies screen depth in lines. The default value is 32. This parameter is ignored
when not running in batch mode.
max_number_of_displays
For batch mode, specifies the maximum number of displays that can occur during a session. This
number includes the total of all SELECT PANEL calls, plus all DISPLAY and TBDISPL calls (with or
without panel name). This number does not include redisplays related to the .MSG control variable.
The largest number that can be specified is 999999999. The batch default value is 100. This
parameter is ignored when not running in batch mode.
max_number_of_redisplays
For batch mode, specifies the maximum number of redisplays allowed for a .MSG-redisplay loop. The
largest number that can be specified is 255. The batch default value is 2. This parameter is ignored
when not running in batch mode.
14  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 43

BDBCS
For batch mode, specifies that Double-Byte Character Set (DBCS) terminal support is required. This
parameter is ignored when not running in batch mode.
DANISH, ENGLISH, GERMAN, JAPANESE, PORTUGUE, SPANISH, KOREAN, FRENCH, ITALIAN,
CHINESET, CHINESES, SGERMAN, UPPERENG
Specifies the national language that is to override the default language for this session. The JAPANESE
keyword specifies that the KANJI character set is to be used. The CHINESET keyword stands
for Traditional Chinese, CHINESES stands for Simplified Chinese, and SGERMAN stands for Swiss-
German. The UPPERENG keyword specifies that the uppercase English character set is to be used.
For information about establishing the default session language, refer to z/OS ISPF Planning and
Customizing.
Note:
1. Attempting to run a dialog under a session language other than that for which it was intended may
produce unexpected results.
2. When the Korean, French, Italian, Traditional Chinese, Simplified Chinese, Spanish, Brazilian-
Portuguese, Danish, German or Swiss-German session language is specified, its respective literal
module is used. However, the ISPF product panels and messages are displayed in English.
NESTMACS
Specifies that all REXX and CLIST edit macros invoked during the ISPF session are to run as nested
commands, allowing output from these macros to be trapped using either the REXX OUTTRAP
function or the CLIST &SYSOUTTRAP control variable.
Using the ISPSTART command
ISPSTART command parameters specify the first menu to be displayed or the first function to receive
control. For example, this command invokes ISPF and specifies that dialog processing is to begin by
displaying a selection panel named ABC, which must be stored in the panel library:
ISPSTART PANEL(ABC)
The next example invokes ISPF and specifies that dialog processing is to begin with a CLIST command
procedure function named DEF:
ISPSTART CMD(%DEF)
The final example invokes ISPF and specifies that dialog processing is to begin with a program function
named GHI:
ISPSTART PGM(GHI)
Note: If you specify the CMD (command) or PANEL (panel) keyword more than once on an ISPSTART
command line, ISPF uses the last value specified. For example:
ISPSTART PANEL(PANELA) PANEL(PANELX)
ISPF interprets this command as:
   ISPSTART PANEL(PANELX)
The ISPSTART command is typically entered during logon or from a command procedure. For example,
suppose you begin an application from a terminal by invoking a command procedure named ABC.
Procedure ABC allocates the libraries for the application, and then issues an ISPSTART command to begin
ISPF processing. The ABC procedure cannot use ISPF dialog services, because it does not run under ISPF.
ISPF is a command processor that can be attached by another command processor as a subtask. You
should always specify SZERO=NO in the MVS ATTACH macro, as ISPF does when it attaches a subtask,
to ensure that at ISPF termination the storage that was acquired by ISPF will be released. For more
information on the ATTACH macro, refer to z/OS MVS Programming: Assembler Services Reference ABE-
Chapter 2. Controlling ISPF sessions  15

## Page 44

HSP. For more information on using MVS macros, refer to z/OS MVS Programming: Assembler Services
Guide.
Invoking a dialog from a selection panel
Figure 7 on page 16 shows a selection panel on which the user has selected option 3. When the user
presses Enter, option 3, the INVENTORY application, is given control.
 ------------------------------- BUILDING 661 ----------------------------
 SELECT OPTION ===> 3_
   1  PAYROLL    - Add, update, or delete employee records
   2  MAILING    - Add, delete, or change address of employee
   3  INVENTORY  - Status of stock
   4  SCHEDULE   - Building maintenance
 ENTER END COMMAND TO TERMINATE.
Figure 7. Sample selection panel
Invoking a dialog from a master application menu
If your installation provides an ISPF master application menu, you can invoke a dialog from that menu.
A master application menu is one from which any of the installation's applications can be invoked. It
generally is displayed at the beginning of each ISPF session. Figure 8 on page 16 is an illustration of the
sample master application menu that is included with ISPF.
                          ISPF Master Application Menu
 1 Sample 1    Sample application 1                         Userid . : LSACKV
 2 .           (Description for option 2)                   Time . . : 11:12
 3 .           (Description for option 3)                   Terminal : 3278
 4 .           (Description for option 4)                   Pf keys  : 24
 5 .           (Description for option 5)                   Screen . : 1
 X Exit        Terminate ISPF using list/log defaults       Language : ENGLISH
                                                            Appl ID  : ISP
                                                            Release  : ISPF 5.6
 Enter END command to terminate application
 5694-A01 (C) COPYRIGHT IBM CORP 1982, 2003
┌──────────────────────────────────────────────┐
│ Licensed Materials - Property of IBM         │
│ 5637-A01 (C) Copyright IBM Corp. 1980, 2004. │
│ All rights reserved.                         │
│ US Government Users Restricted Rights -      │
│ Use, duplication or disclosure restricted    │
│ by GSA ADP Schedule Contract with IBM Corp.  │
⋘──────────────────────────────────────────────┘
 Option ===>
  F1=Help      F2=Split     F3=Exit      F9=Swap     F10=Actions  F12=Cancel
Figure 8. ISPF master application menu (ISP@MSTR)
You usually invoke the master menu by using the ISPSTART command with no operands. ISPSTART can
be issued automatically as part of a user's logon procedure or from a CLIST or REXX command procedure.
16  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 45

Controlling ISPF sessions
This topic describes how you can control ISPF sessions with the SHRPROF system command.
Using the SHRPROF system command
The SHRPROF command allows you to modify settings for shared ISPF profiles.
You can display a panel (Figure 9 on page 17) for selecting command options by entering the SHRPROF
command with no parameters, or by selecting the Shared Profile settings… choice from the Environ
pull-down on the ISPF Settings panel. This panel includes the current values of the SHRPROF command
parameters. You can change these values by entering new values directly on the panel.
  Log/List  Function keys  Colors  Environ  Identifier  Help
┌─────────────────────────────── ISPF Settings ───────────────────────────────┐
│ ISPISSA            Multi-Logon Profile Sharing Settings                     │
│ Command ===>                                                                │
│                                                                             │
│ Profile Enqueue settings                                                    │
│   Enter "/" to select option           ENQ Lock Wait . . . . . . 1000       │
│   /  Prompt for Profile ENQ Lockout    ENQ Lock Retry Count  . . 1          │
│                                                                             │
│ Profile conflicts                                                           │
│   System Profile conflicts             Reference List conflicts             │
│   1  1. Keep                           1  1. Keep                           │
│      2. Discard                           2. Discard                        │
│      3. Prompt                            3. Prompt                         │
│                                                                             │
│   ISPF Profile conflicts               Edit Profile conflicts               │
│   1  1. Keep                           1  1. Keep                           │
│      2. Discard                           2. Discard                        │
│      3. Prompt                            3. Prompt                         │
│                                                                             │
│   Application Profile conflicts        Batch Profile conflicts              │
│   1  1. Keep                           1  1. Keep                           │
│      2. Discard                           2. Discard                        │
│      3. Prompt                                                              │
│                                                                             │
│   Other Profile conflicts                                                   │
│   1  1. Keep                                                                │
│      2. Discard                                                             │
│      3. Prompt                                                              │
│  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
│  F9=Swap       F12=Cancel                                                   │
⋘─────────────────────────────────────────────────────────────────────────────┘
 F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
F10=Actions  F12=Cancel
Figure 9. Multi-logon pr o file  sharing settings (ISPISSA)
You can issue the SHRPROF command at any time during an ISPF session.
SHRPROF command syntax and parameter descriptions
The general syntax for the SHRPROF command is:
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  17

## Page 46

SHRPROF
RESET
WAIT
n
RETRY
n
PROMPT
NOPROMPT
CONFLICT SYSTEM
ISPF
APPLID
REFLIST
EDIT
OTHER
BATCH
KEEP
DISCARD
PROMPT
The parameter descriptions for the SHRPROF command are as follows:
RESET
Resets all the Shared Profile settings to the values specified in the ISPF Configuration options.
WAIT
The wait time in milliseconds that ISPF is to wait before retrying when it is unable to obtain an
enqueue on a member of the ISPF profile data set. If specified, n must an integer in the range 0 to
9999. A value of 0 indicates that no wait is to occur. The ISPF default is 1000.
RETRY
The number of times that ISPF is to retry to obtain an enqueue on a member of the ISPF profile data
set when it is unable to obtain the enqueue. If specified, n must an integer in the range 0 to 99. The
ISPF default is 1.
PROMPT
ISPF prompts you when it is unable to enqueue on a member of the ISPF profile data set, and the
retry count has been reached. You are then given the option to either retry again, or cancel the
request.
NOPROMPT
ISPF fails the enqueue request when it is unable to obtain the enqueue on a member of the ISPF
profile data set and the retry count has been reached.
CONFLICT
The required action to be taken when a conflict is found updating a member of the profile data set,
where the last updated information has changed. You can specify a different actions for different
types of profile members. When you specify the CONFLICT parameter, you must also specify a conflict
type (see following list). The conflict action parameter (see following list) is optional; if you do not
specify a conflict action, ISPF use the value specified in the ISPF configuration settings.
The supported conflict types are:
SYSTEM
The ISPF System profile member, ISPSPROF.
ISPF
The ISPF profile, normally ISPPROF.
APPLID
An application profile member, being a member with "PROF" as the suffix, other than the SYSTEM
and ISPF profiles.
Controlling ISPF sessions
18  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 47

REFLIST
Any of the ISPF Reference lists: ISRLLIST, ISRPLIST, or ISRSLIST.
EDIT
An ISPF Edit profile member, being a member with "EDIT" as the suffix.
BATCH
Any batch ISPF job.
OTHER
Any other ISPF table in the ISPF profile data set.
The supported conflict actions are:
KEEP
The current changes are kept, replacing any other changes previously saved by another ISPF
session sharing the profile.
DISCARD
The current changes are discarded, retaining those already updated in the profile data set.
PROMPT
A panel is displayed prompting you to either KEEP or DISCARD the changes.
What the SELECT service does
The SELECT service initiates dialog execution. Selection keywords, passed to the SELECT service, specify
whether the dialog begins with the display of a menu (PANEL keyword) or the execution of a dialog
function (CMD or PGM keyword). The dialog terminates when the selected menu or function terminates.
The action at termination depends on how the SELECT service was originally invoked.
SELECT is both a control facility and a dialog service. ISPF uses SELECT during its initialization to invoke
the function or selection panel that begins a dialog. During dialog processing, SELECT displays selection
panels and invokes program functions or command procedure functions.
The principal SELECT parameters are:
PANEL(panel-name)
CMD(command)
PGM(program-name)
See z/OS ISPF Services Guide for a full description of the SELECT service syntax.
The panel-name parameter specifies the name of the next selection panel to be displayed. You must use
the ISPF panel definition statements (described in Chapter 5, “Panel definition statement guide,” on page
87) to define the panel.
The command and program-name parameters specify a function, coded as a CLIST command procedure
or program, respectively, to receive control. Input parameters can be passed to the function as part of the
command specification or, for programs, by the use of the PARM parameter.
Figure 10 on page 20 shows how the SELECT service is used when invoking or processing a dialog. After
SELECT starts a dialog, the dialog uses it as a service to invoke a function or to display a selection panel.
In turn, that function or menu can use SELECT to invoke another function or to display another menu.
This function or menu can, in turn, using SELECT, invoke still another function or menu. This process
can continue for many levels and establishes a hierarchy of invoked functions and menus. There is no
restriction on the number of levels allowed in this hierarchy.
Subtasks attached by the SELECT service do not share subpools. ISPF specifies SZERO=NO when issuing
the ATTACH macro to ensure that at SELECT termination the storage that was acquired by ISPF is
released.
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  19

## Page 48

Figure 10. SELECT service used to invoke and process a dialog
When a lower-level function or menu in the hierarchy completes its processing, control returns to
the higher-level function or menu from which it was invoked. The higher-level function resumes its
processing, or the higher-level menu is redisplayed for the user to make another selection. Thus,
SELECT is used in a dialog to establish a hierarchy of functions and menus. This hierarchy determines
the sequence in which functions and menus are processed, including the sequence in which they are
terminated.
Dialog functions written as command procedures can directly invoke other functions written as command
procedures without using the SELECT service. They are not treated as new functions by ISPF.
Dialog functions written as programs can invoke another function only through using the SELECT service.
Thus, when a program-coded function calls another program directly, without using the SELECT service,
the called program is treated as part of the function that called it. It is not treated as a new function by
ISPF.
Invoking the SELECT service
The SELECT service can be invoked in these ways:
• During initialization, the dialog manager automatically invokes the SELECT service to start the first
dialog. The selection keywords originally specified on the ISPSTART command are passed to the
SELECT service.
For dialogs invoked by ISPSTART, ISPF error processing is not put into effect until ISPF is fully
initialized. ISPF is considered to be fully initialized when the Enter key on the primary option menu
has been processed without a severe error occurring.
• If you enter split-screen mode, the dialog manager again invokes the SELECT service and again passes
the selection keywords from the ISPSTART command. This causes the first dialog, specified in the
ISPSTART command, to be initiated on the new logical screen.
• The SELECT service recursively invokes itself when you select an option from a menu displayed by the
SELECT service. In this case, the selection keywords are specified in the panel definition for the menu.
Controlling ISPF sessions
20  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 49

• The SELECT service can be invoked from a dialog function. In this case, the selection keywords are
passed as calling sequence parameters.
Terminating a dialog
The action taken at dialog termination is as follows:
• If a dialog function invoked the SELECT service, control returns to that function and the function
continues execution.
• If a menu invoked the SELECT service, that menu is redisplayed, including execution of the INIT section
in the panel definition.
• If you are terminating split-screen mode, the original dialog ends on that logical screen, and the other
logical screen expands to the full size of the viewing area.
• If you are terminating ISPF, which can be done only in single-screen mode, and neither the ISPLOG
nor the ISPLIST data set was pre-allocated, either the ISPF termination panel is displayed or the ISPF
SETTINGS defaults for list/log processing are used.
ISPF displays the termination panel if:
• The dialog started with the display of a menu and you entered the END command on that menu.
• The dialog started with the execution of a function, and the function ended with a return code of 0.
The list/log defaults are used if:
• The dialog started with the display of a menu and you entered the RETURN command or selected the
EXIT option.
• The dialog started with the execution of a function and the function ended with a return code of 4 or
higher. A return code other than 0 or 4 causes an error message to be displayed.
If you have not specified valid list/log defaults, the ISPF termination panel is displayed in all cases.
See Pre-allocation of List/Log data sets in z/OS ISPF Planning and Customizing for more information.
Return Codes from Terminating Dialogs
The return code from ISPSTART for a successful dialog completion is either 0 or a value returned by the
executing dialog in the system variable ZISPFRC. ZISPFRC is a shared-pool input variable of length 8. The
dialog can set ZISPFRC to any value in the range of 0 to 16777215, except the values reserved for ISPF
use (900 through 999, and 9000 through 9100). This value must be left-justified and padded with blanks.
At termination, ISPF copies the value from ZISPFRC and passes it to the invoking application (or Terminal
Monitor Program) in register 15. If the value in ZISPFRC is not within the valid range or is otherwise not
valid, such as a value that is not numeric, ISPF issues an appropriate line message and passes a return
code of 908. If the dialog has not set ZISPFRC to a value, ISPF returns a value of 0.
Note:
1. CLIST procedures that invoke ISPSTART can check the CLIST variable LASTCC for the ISPF return
code. In REXX, check the variable rc after an ISPF function.
2. Even though ISPF restricts the return code value to the range 0 to 16777215, other products or
subsystems, such as JES when processing JCL condition codes, can be more restrictive on return code
values. See documentation for the affected product for more information.
3. ZISPFRC should not be confused with the normal dialog return code set by the function; it has no
effect on ISPF log/list termination processing.
ZISPFRC is intended to be used by applications that invoke a dialog dedicated to a single task or function.
However, it is valid to set ZISPFRC from a selection panel invoked by the ISPSTART command.
ISPF checks for the existence of ZISPFRC only at ISPF termination. If ZISPFRC is set by any dialog other
than the one invoked by the ISPSTART command, ISPF ignores the value.
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  21

## Page 50

Return Codes from Termination Dialogs
Error codes that ISPF can return in register 15 to an application are:
908
ZISPFRC value not valid.
920
ISPSTART command syntax not valid.
930
ISPSTART Program not found.
940
ISPSTART Command not found.
950
An ISPF session running on behalf of a z/OS client had to be abnormally terminated.
988
An error occurred initializing IKJSATTN.
990
An error occurred running in batch mode. If ZISPFRC has not been set previously, and ISPF
encounters a severe error that terminates the product, then 990 is set.
997
Uncorrectable TPUT error.
998
ISPF initialization error. A 998 error code can result from:
• Required ISPF data element library not preallocated
• Error opening ISPF data element library
• ISPF data element library has invalid data set characteristics
• Error loading literals module
• Recursive ISPF call
ISPF issues a line message that indicates which of these errors caused the 998 return code.
999
ISPF environment not valid. A 999 error code can result from:
• TSO/MVS environment not valid
• Unsupported screen size
ISPF issues a line message that indicates which of these errors caused the 999 return code.
When running in batch, ISPF can also return the following return codes:
9008
Abend termination.
9012
Attach error.
9014
Authorized command invocation error, or TSO CMD START exit routine rejected the command.
9016
Command not found, or was otherwise unable to execute, or an exit routine returned an invalid return
code.
9018
Invalid command: LOGOFF, ISPF, etc.
9020
TSO RTN IKJTBLS (called from CAU) abended.
Controlling ISPF sessions
22  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 51

An example using the ZISPFRC return code
Figure 11 on page 23 shows a portion of a background job that invokes ISPF. The final job step runs only
if the job step that invoked the ISPF dialog terminates with a return code of 8 or less.
⋮
//************************************************
//*                                              *
//*  INVOKE ISPF TO EXECUTE DIALOG "DIALOG1".    *
//*  DIALOG1 PASSES BACK A RETURN CODE OF        *
//*  20 IF IT DID NOT PROCESS SUCCESSFULLY.      *
//*                                              *
//************************************************
//ISPFSTEP EXEC PGM=IKJEFT01,DYNAMNBR=30,REGION=2048K
//*
//*  ALLOCATE DIALOG AND ISPF PRODUCT LIBRARIES, *
//*  ISPF LOG DATA SET, AND TSO OUTPUT DATA SET. *
//*                                              *
//ISPPROF  DD DSN=USER1.ISPF.TABLES,DISP=SHR
⋮
//*  ALLOCATE TSO INPUT DATA SET.                *
//*                                              *
//SYSTSIN  DD *
  PROFILE PREFIX(USER1)     /* ESTABLISH PREFIX  */
  ISPSTART CMD(%DIALOG1)    /* INVOKE DIALOG1    */
/*
//************************************************
//*                                              *
//*  EXECUTE NEXT JOB STEP ONLY IF THE ISPF STEP *
//*  ENDED WITH A RETURN CODE LESS THAN OR EQUAL *
//*  TO 8.  THAT IS, BYPASS THE STEP IF 8 IS     *
//*  LESS THAN THE ISPF RETURN CODE.             *
//*                                              *
//************************************************
//NEXTSTEP EXEC PGM=IKJEFT01,DYNAMNBR=30,REGION=2048K,
//              COND=(8,LT,ISPFSTEP)
⋮
Figure 11. Sample background ISPF job
The portion of the invoked dialog, DIALOG1, that establishes the value in system variable ZISPFRC is
shown in Figure 12 on page 23.
PROC 0
⋮
IF &MAXCC > 8 THEN +
  DO
    SET &ZISPFRC = 20
    VPUT (ZISPFRC) SHARED
  END
EXIT CODE(0)
Figure 12. Sample dialog using system variable ZISPFRC
ISPF test and trace modes
The testing modes of ISPF provide special processing actions to help debug a dialog. Consider using the
Dialog Test (option 7) facility.
You can specify any one of four mutually exclusive keyword parameters on the ISPSTART command to
control the operational mode when testing a dialog:
TEST
Test mode
TESTX
Extended test mode; logged messages are displayed
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  23

## Page 52

TRACE
Trace mode; ISPF service calls are logged
TRACEX
Extended trace mode; ISPF service calls are logged and displayed
Test modes
In TEST mode, ISPF operates differently from normal mode in these ways:
• Panel and message definitions are fetched again from the panel and message files when a panel name
or message ID is specified in an ISPF service. In normal mode, the most recently accessed panel
definitions are retained in virtual storage. If you have modified the panel or message file, use of TEST
mode ensures that the latest version of each panel or message is accessed during a test run.
Using an editor to modify a panel, message, or skeleton can result in an additional DASD extent being
required for the associated data set. DASD rarely (if ever) gains new extents as the result of the
execution of software (with the possible exception of DASD formatting software). It can also be caused
by link-editing a module. When a new extent is allocated, you can access the modification only by first
terminating and then invoking ISPF again.
• Tutorial panels are displayed with current panel name, previous panel name, and previous message ID
on the bottom line of the display screen. This assists you in identifying the position of the panel in the
tutorial hierarchy.
• Screen printouts, obtained through use of the PRINT or PRINT-HI commands, include line numbers,
current panel name, and message ID.
• In PDF, the index listing (option 3.1) for a partitioned data set includes TTR data for each member of the
data set.
• If a dialog function is operating in the CANCEL error mode (the default), the panel that is displayed
on an error allows you to force the dialog to continue in spite of the error. Results from that point on,
however, are unpredictable and ISPF can abend.
If a dialog function is operating in any other error mode, and a command run from the SELECT service
abends, any ISPF-detected error, abend, or program interrupt forces an abend of ISPF. You can also
force an abend by entering ABEND or CRASH in the command line of any panel. For more information
about the SELECT service, refer to the z/OS ISPF Services Guide.
• The PA1 key causes an immediate exit from ISPF.
The ISPF controller task attaches one ISPF subtask for each logical screen. Any additional logical screens
are created by the SPLIT command and there can be up to four screens on a 3290 terminal.
If an ISPF subtask abends, pressing Enter after the abend message appears generates a dump, provided
that a SYSUDUMP, SYSMDUMP, or SYSABEND data set has been allocated.
Dialogs invoked with the SELECT CMD(XXX) cause an attach of a new subtask under the ISPF subtask. If
an abend occurs under the new subtask, an immediate dump is taken.
In TESTX mode, ISPF operates the same as it does in TEST mode, except that all messages written to the
ISPF log file are also displayed at the terminal.
ISPF provides the ENVIRON command, which allows you to cause a dump following an abend condition,
even if ISPF is not running in TEST mode. See “Using the ENVIRON system command” on page 331 for a
description of using the ENVIRON command.
ISPF trace modes
In TRACE mode, ISPF operates as it does in TEST mode, except that a message is written to the ISPF
log file when any ISPF service is invoked, even if CONTROL ERRORS RETURN has been issued, and when
any error is detected by an ISPF service. Note that only CLIST, APL2, and CALL ISPEXEC service requests
are recorded. This does not include service requests issued under Dialog Test option 7.6. CALL ISPLINK
requests for service are not recorded in the log file.
Controlling ISPF sessions
24  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 53

In TRACEX (extended trace) mode, ISPF operates the same as it does in TRACE mode except that all
messages written to the ISPF log file, including the trace messages, are also displayed at the terminal. If
the length of the message text exceeds the width of the terminal screen, the message will be truncated.
Invoking authorized programs
You can invoke authorized programs by using the SELECT service, a selection panel, a command table,
or by using the TSO CALL command under ISPF. ISPF uses the TSO Service Facility IKJEFTSR to invoke
authorized commands and programs. Authorized programs are invoked under the TSO TMP (Terminal
Monitor Program) and therefore should not reside in the ISPLLIB library. Authorized programs cannot
issue dialog service requests. See z/OS TSO/E Customization for information about adding authorized
programs and commands to the list maintained by your installation.
Invoking TSO commands
TSO commands can be initiated by use of the SELECT dialog service (with the CMD keyword), from a
selection panel, from a command table, by entering the ISPF TSO system command in the command field
of any panel, or be contained in a CLIST or REXX command procedure that is invoked under ISPF.
You can invoke authorized TSO commands by using the SELECT service, a selection panel, or a command
table. Authorized commands are attached under the TSO TMP (Terminal Monitor Program) and, therefore,
should not reside in the ISPLLIB library. Authorized commands cannot issue dialog service requests.
You can run most TSO commands under ISPF. These commands are not allowed:
• LOGON
• LOGOFF
• SPF
• ISPF
• PDF
• ISPSTART
• TEST
• Commands that are restricted by TSO
Note: The LOGON, LOGOFF, and TEST commands can be run within ISPF if the TSOEXEC interface is used
(for example, TSO TSOEXEC LOGOFF). In that case, the LOGON and LOGOFF commands are processed
upon ISPF termination, instead of returning to TSO READY. When the TEST command is being run, TSO
TEST is entered immediately. However, because TSOEXEC runs commands in a parallel TMP structure,
ISPF dialogs cannot be run under TSO TEST in this situation.
Compiled REXX requirements
ISPF supports compiled REXX load modules through ISPSTART and the SELECT service. The REXX
program must be compiled with the OBJECT option of the IBM Compiler for REXX/370. This OBJECT
output needs to be link-edited with the CPPL stub that is a part of the IBM Library for REXX/370.
The SELECT service and ISPSTART command contain a value, CREX, for the LANG parameter on the CMD
keyword. Specifying LANG(CREX) on the CMD keyword indicates that it is a Compiled REXX load module
and that a REXX function pool is to be used for variable manipulation. LANG(CREX) is optional if the
compiled REXX has been link-edited to include any of the stubs EAGSTCE, EAGSTCPP, or EAGSTMP.
The CPPL stub takes the parameters that are passed by the SELECT CMD service or the ISPSTART
invocation, and converts them into arguments for the REXX program. For complete details on how to
create a REXX load module, see IBM Compiler and Library for REXX on zSeries User's Guide and Reference.
Compiled REXX programs that were compiled with the CEXEC option must be started using the CMD
option of the SELECT service or ISPSTART command, and must NOT use the LANG(CREX) parameter.
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  25

## Page 54

CLIST requirements
A CLIST cannot invoke any of the restricted TSO commands. TERMIN command procedure statements
can cause unpredictable results.
Note: If a CLIST contains CONTROL MAIN, the TSO input stack is not flushed after an ISPF severe error.
Attention exits
When a CLIST command procedure is executing under ISPF, the ATTN statement in the procedure defines
how attention interrupts are to be handled. You can find information about using attention exits in z/OS
TSO/E CLISTs and z/OS TSO/E Programming Services.
Restrictions on using attention exits from CLISTs
Restrictions that apply to using attention exits from a CLIST dialog are:
• CLIST attention exits are not supported when running in ISPF TEST or TRACE modes. This is because
the ISPF attention exit routine is not established in TEST or TRACE modes.
• The CLIST must issue a null command to return from an attention exit. If the dialog issues a TSO
command to terminate the exit routine, ISPF discards the command. The ISPF dialog then resumes
execution as if CONTROL MAIN NOFLUSH were in effect for this CLIST.
• You can stack CLIST attention exits only within one SELECT CMD level. An exit applies only to the logical
screen from which the CLIST owning the attention exit was invoked. Therefore, when you are operating
in split-screen mode, invoking a CLIST attention exit from one logical screen has no effect on the other
logical screens.
• Do not invoke an ISPF dialog service from a CLIST attention exit routine. If you do, results are
unpredictable.
• Attention interrupts initiated while an exit routine is executing are not honored.
Examples of CLIST attention exit process flow
See:
• “Single CLIST with one attention exit” on page 26
• “Nested CLISTs with two attention exits (one SELECT level)” on page 26
• “Nested CLISTs with one attention exit” on page 27
• “Nested CLISTs and SELECT levels with one attention exit” on page 27
Single CLIST with one attention exit
1. From a selection panel, select a CLIST procedure named CLIST1. CLIST1 has one attention exit
routine, named ATTN1.
2. CLIST1 displays PANEL1.
3. Press the attention key.
4. Exit routine ATTN1 runs and PANEL1 redisplays.
Nested CLISTs with two attention exits (one SELECT level)
1. From a selection panel, select a CLIST procedure named CLIST1. CLIST1 has one attention exit
routine, named ATTN1.
2. CLIST1 invokes procedure CLIST2 by using the TSO EXEC command. CLIST2 has one attention exit
routine, named ATTN2.
3. CLIST2 displays PANEL2.
Controlling ISPF sessions
26  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 55

4. Press the attention key.
5. Exit routine ATTN2 runs and PANEL2 redisplays.
6. Press Enter to return control to CLIST2. CLIST2 then terminates processing and control returns to
CLIST1.
7. CLIST1 displays PANEL1.
8. Press the attention key.
9. Exit routine ATTN1 runs and PANEL1 redisplays.
Nested CLISTs with one attention exit
1. From a selection panel, select a CLIST procedure named CLIST1. CLIST1 has one attention exit
routine, named ATTN1.
2. CLIST1 invokes procedure CLIST2 by using the TSO EXEC command. CLIST2 has no attention exit
routine.
3. CLIST2 displays PANEL2.
4. Press the attention key.
5. Exit routine ATTN1 runs and PANEL2 redisplays.
6. Press Enter to return control to CLIST2. CLIST2 then terminates processing and control returns to
CLIST1.
7. CLIST1 displays PANEL1.
8. Press the attention key.
9. Exit routine ATTN1 runs and PANEL1 redisplays.
Nested CLISTs and SELECT levels with one attention exit
1. From a selection panel, select a CLIST procedure named CLIST1. CLIST1 has one attention exit
routine, named ATTN1.
2. CLIST1 invokes procedure CLIST2 by using the ISPEXEC SELECT CMD(CLIST2) command. CLIST2 has
no attention exit routine.
3. Press the attention key.
4. Because CLIST2 has no attention exit routine, and ISPF does not propagate attention exits across
SELECT levels:
• An error message indicates that a CLIST was interrupted by an attention condition.
• The logical screen terminates and restarts, causing the primary option menu to redisplay.
Using APL2
ISPF permits the use of APL2, as follows:
• ISPF dialogs can be written in an APL2 workspace.
• APL2 can be selected as a command, initializing an ISPF-APL2 environment.
• APL2 functions can be selected as options (from a selection panel), as ISPF commands (from an
application command table), or from another dialog function, once the ISPF-APL2 environment has
been established.
• All dialog manager services available to the command language dialog writer are executable from the
APL2 workspace after the ISPF-APL2 environment has been established.
• ISPF views the APL2 workspace variables as the dialog function pool whenever an ISPF dialog service is
executing.
• ISPF supports APL on a DBCS device with an APL keyboard.
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  27

## Page 56

The ISPF/GDDM interface is not available to an APL2 dialog. However, the APL2 dialog can interface
directly with GDDM and interleave the ISPF and GDDM services.
Invoking APL2
You can invoke APL2 by specifying the APL2 command and its appropriate keywords as the value of the
CMD keyword of the SELECT service. You must also code the SELECT keyword and the value LANG(APL)
on the SELECT statement. The LANG(APL) parameter provides the basis for establishing an ISPF-APL2
environment. It is required if any ISPF dialog services are to be used.
You can code any of the APL2 command keywords. However, be aware of:
APNAMES
ISPF and APL2 communicate through an APL2 Auxiliary Processor (AP), ISPAPAUX, which is released
with the ISPF product. This AP, number 317, must be made available to APL2 when APL2 is invoked,
as follows:
• The dialog writer can specify ISPAPAUX in the APNAMES list of auxiliary processors to be
dynamically loaded.
When APL2 is invoked, ISPAPAUX must exist as a load module in a system library, or in a private
library named by the LOADLIB keyword.
LOADLIB
Keep in mind that if this keyword is used, the dialog must be changed or accept this keyword's value
dynamically (for example, through a variable), if the name of the private library containing the AP is
changed.
TERMCODE (code)
The user is prompted to enter an appropriate character if this keyword is not coded. This allows APL2
to identify the terminal type that is currently being used.
Typically, a dialog ensures that the user does not have to perform this extra step by identifying the
terminal type through the TERMCODE keyword.
ISPF system variable ZTERM contains this information. However, ISPF terminal types are different
from those of APL2. For those dialog writers who wish to make use of currently available ISPF
information, program dialog ISPAPTT can be selected before the call of APL2. ISPAPTT expects one
parameter, which is the ISPF variable name into which the corresponding APL2 terminal type is
returned. The variable is created in the shared variable pool.
For a CLIST, the use of ISPAPTT can look as follows:
⋮
ISPEXEC SELECT PGM(ISPAPTT) PARM(APLTT)
ISPEXEC VGET APLTT
ISPEXEC SELECT CMD(APL2.....TERMCODE(&APLTT)) LANG(APL)
⋮
These ISPF to APL2 mappings are supported:
   ISPF                    APL2
  (ZTERM)
  ------------------------------
   3277                    3277
   3278                    3279
   3277A                   32771
   3278A                   32791
   3278T                   32791
   3278CF                  3279
   3277KN                  3277
   3278KN                  3279
If ISPF is executing in the background, then ISPAPTT will return a terminal code of 1.
Controlling ISPF sessions
28  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 57

If ZTERM contains a value other than those previously listed, the specified variable is set to a value of
3277 in the shared variable pool.
FREESIZE, WSSIZE
Some combination of these keywords should be coded to accommodate the user's storage
requirements; however, remember that ISPF and the ISPF-APL2 AP require storage (beyond that
currently allocated) to run, especially if ISPF split-screen facilities are to be used.
INPUT
A user dialog can specify the INPUT keyword to load a given workspace, start an APL2 dialog function,
and terminate APL2. This allows a user to enter APL2, use APL2 dialog capabilities, and leave APL2
without needing special APL2 expertise.
For example, to start a dialog named EMPLOY in workspace MYWS:
 ......INPUT(')LOAD MYWS' 'EMPLOY' ')OFF HOLD')......
Note that a dialog function can also be started through the latent function definition in the workspace.
In addition, the Alternate Input Auxiliary Processor, AP101, can be used to stack commands for
execution.
If INPUT is coded and QUIET and PROFILE are not coded, the first ISPF panel can be refreshed before
the keyboard is unlocked.
QUIET
A dialog can specify the QUIET keyword to suppress the APL2 entry and exit information, so that the
user does not see non-dialog APL2 messages.
PROFILE
A dialog can specify the PROFILE keyword with a value of null to suppress any entry and exit APL2
session manager screens, so that the user does not see any non-dialog panels.
Executing APL2 functions
It is possible to start an APL2 function dialog by using the INPUT keyword, as described in “Invoking
APL2” on page 28. However, for many applications it is necessary to invoke additional APL2 functions as
options (from a selection panel), as commands (from an application command table), or from other dialog
functions.
Such functions are selected by specifying the function request as the value of the SELECT CMD
keyword, and once again, specifying LANG(APL). Because APL2 has already been started, and the APL2
environment established, the string is passed back to the APL2 workspace, and an APL2 EXECUTE
function is performed on the string. For example, option 5 on a selection panel can be defined to APL2
function AVG (assuming that APL2 has already been started) as follows:
⋮
5,'CMD(AVG 1 2 3 4 5) LANG(APL)'
⋮
The return code for the selected function is passed back as a fullword of 0 (zero) if no terminating (to a
quad-EA) APL2 errors have occurred. Otherwise, a fullword consisting of the quad-ET values in the two
halfwords is returned.
APL2 cannot be invoked more than once, either within the same screen or on more than one screen. ISPF
does nothing to prevent the second call. If APL2 is invoked a second time while running under ISPF, the
results are unpredictable. Note that ISPF's split-screen capabilities can be used as long as APL2 is not
invoked on a second screen.
Invoking ISPF dialog services in the APL2 environment
A dialog service can be invoked by using the function form of ISPEXEC:
[n]   lastrc◄ISPEXEC   character-vector
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  29

## Page 58

lastrc
Specifies the name of an APL2 variable in which the return code from the service is to be stored.
character-vector
Specifies a vector of characters that contains parameters to be passed to the dialog service. The
format of the vector is the same as that for dialog service statements for command procedures written
in CLIST.
A workspace containing the ISPEXEC function is provided with ISPF. All dialog writers must use this
ISPEXEC function, as it contains the interface to ISPF and handles the implementation of commands
(through the APL2 EXECUTE function); otherwise, results are unpredictable.
For example:
APL2 workspace as the ISPF function pool
When an APL2 function invokes an ISPF dialog service, the APL2 workspace is considered to be the ISPF
function pool. The dialog writer need not do anything special to make use of this mechanism. However,
these restrictions apply:
• Any variable retrieved or set is the most local to the currently executing APL2 function.
• The dialog writer should not use variables whose names begin with the three characters ISP; these
names are reserved for ISPF. All variables used in the ISPEXEC function have names that start with
these three characters.
• Only those variables whose names and formats fit both ISPF and APL2 protocols can be used for ISPF
entities such as panels or tables:
– All variable names must be 1 to 8 characters in length, composed of alphanumeric characters (A-Z,
0-9), of which the first must not be numeric. Note that #, $, and @ are not allowed.
– All variable values must be simple character strings; APL2 general data types are not allowed. Note
that the only acceptable null vector is that for character strings (‘’).
If an attempt is made to use a name or format incompatible with ISPF for an ISPF entity, a severe error
occurs. Any APL2 name or format can be used within a dialog function, as long as that variable is not
used for an ISPF entity.
• Whenever an APL2 function is selected after APL2 is started, the original APL2 function pool (the
APL2 workspace) is used. This implies that information can remain in the function pool from previous
SELECTs, and the dialog writer must handle any such cases. Moreover, this rule is unaffected by
SELECTs where new shared or profile pools are created; it is the responsibility of the dialog writer to
ensure that the integrity of the workspace is maintained.
• If the PDF component is installed, and the Dialog Test Variables option is requested, only those
variables that have the correct name and format are displayed; if an attempt is made to enter a variable
with a name that is not valid (to ISPF or APL2), an error occurs. The variables displayed are the most
local to the currently executing function.
• A maximum of 64K bytes can be retrieved from the APL2 workspace during the execution of a DM
service.
Interface between ISPF and APL2
The interface between ISPF and APL2 is like a telephone call. If one side of the communication is broken,
any attempt to use the interface causes error messages to be generated. The link between the two
products can be broken by:
Controlling ISPF sessions
30  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 59

• The APL2 user "hanging up". For example, if a new workspace is loaded and there are still ISPF service
requests that have not completed (for example, options in the selection panel process), the ISPF
Auxiliary Processor (ISPAPAUX) issues an error message, informs ISPF and waits for the process to
begin again (by "hanging up" until another ISPF request is made). ISPF issues a severe error message
telling the user that the link has been damaged.
If the user is in ISPF TEST mode, then, on user request, ISPF attempts to reshow all panels traversed in
an effort to unnest all service requests. When all requests have been unnested, ISPF will again wait for
the ISPF Auxiliary Processor to make a request. During the unnesting process, any attempts to invoke
APL2 functions are rejected, severe error messages are issued, and any requests for APL2 variables are
logged.
• The APL2 user "cutting the line". For example, if the user terminates APL2 while there are still
outstanding APL2 function requests from ISPF (for example, options in the selection panel process),
the ISPF Auxiliary Processor (ISPAPAUX) issues an error message, informs ISPF, and terminates. ISPF
issues a severe error message telling the user that the link has been damaged, and if in TEST mode,
proceeds to unnest as previously described. When all requests have been unnested, APL2 will be
terminated. During the unnesting process, any attempts to invoke APL2 functions are rejected, severe
error messages are issued, and any requests for APL2 variables are logged.
• An APL2 failure. This is handled as if the line were cut, assuming APL2 performs recovery and returns to
ISPF.
• An ISPF failure. In this case, ISPF or the logical screen can fail, causing APL2 termination.
Subtasking support
A dialog attached by ISPF, as described in “Invoking TSO commands” on page 25, can invoke a dialog
service. It does this by a call to either the ISPLINK or ISPEXEC interfaces from any subtask level. For
subtasks to issue ISPF services, the program that attaches these subtasks must be invoked with the
SELECT(cmd) service.
In addition, ISPF allows a task to detach its subtask at any time, even if an ISPF service invoked by that
subtask is processing. The SUBTASK keyword of the CONTROL service, described in z/OS ISPF Services
Guide, provides additional information. Multiple dialog services issued from multiple tasks executing
asynchronously are not supported, and results will be unpredictable. This also applies to attention exit
routines given control by STAX which may receive asynchronous control while an ISPF service is already
active.
ESTAE restrictions
Programs that code their own ESTAE routines should not issue ISPF services within the MVS ESTAE
routine. Unpredictable results can occur. For more information on ESTAE, refer to z/OS MVS Programming:
Assembler Services Reference ABE-HSP. For more information on using MVS macros, refer to z/OS MVS
Programming: Assembler Services Guide.
ISPF services in batch mode
When initiated in a batch environment, ISPF services run as a background command. Background calls
are generally used to invoke ISPF table and file tailoring services. However, access to other dialog services
is also available.
Command processors in the TSO batch environment
TSO provides facilities for executing command processors in the batch environment. The JCL stream
provides for data sets to be preallocated before the call of any command. Invoke the Terminal Monitor
Program (TMP) using the EXEC statement to establish the necessary control blocks for the TSO
environment. The command input stream is accessed from the SYSTSIN DD statement. All terminal line
I/O outputs issued by the TSO I/O service routines are directed to the SYSTSPRT DD statement definition.
Allocate ISPF libraries by using DD statements. Panel, message, skeleton, table, and profile data sets
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  31

## Page 60

must be preallocated. While not required, it is recommended that the log data set also be preallocated. If
a log data set is dynamically allocated, it is always kept at ISPF termination.
To invoke ISPF, place the ISPSTART command in the SYSTSIN input stream with the PANEL, CMD, or PGM
keywords that name the dialog to be invoked.
Note: When running on MVS with TSO/E Version 2 Release 1, ISPF does not read and run the CLIST
statements that follow the ISPSTART command. With ISPF running in batch (background) mode in the
MVS environment with TSO/E Version 2 Release 1, you can select a CLIST procedure.
A user ID is selected for the background job as follows:
1. If available, the user ID supplied during RACF® authorization checking is used.
2. If a user ID is not available from RACF, the prefix supplied with the TSO PROFILE command is used.
3. If neither of these is available, the default is BATCH.
Although the user ID defaults to BATCH, the prefix used by ISPF when dynamically allocating a data set
has no default. Therefore, a prefix should always be supplied on the TSO PROFILE command. At various
times, ISPF attempts dynamic allocation and if no prefix has been supplied, allocation will fail and the job
will abend. Multiple jobs executing concurrently must have unique prefixes.
The contents of positions 17-24 in system variable ZENVIR indicate whether ISPF is running interactively
(TSO followed by five blanks) or background (BATCH followed by three blanks).
Sample batch job
Figure 13 on page 32 shows a sample batch job. This job invokes the MVS/TSO Terminal Monitor
Program (TMP) which, in MVS, establishes the environment necessary to attach command processors.
The ISPSTART command is specified in the TSO background input stream (SYSTSIN) with the name of a
CLIST (TBUPDATE) that contains the ISPF services to be run.
//USERAA JOB (AA04,BIN1,000000),'I. M. USERAA',
// CLASS=L,MSGCLASS=A,NOTIFY=USERAA,MSGLEVEL=(1,1)
//*-------------------------------------------------------*/
//*  EXECUTE ISPF COMMAND IN THE BACKGROUND               */
//*-------------------------------------------------------*/
//*
//ISPFBACK EXEC PGM=IKJEFT01,DYNAMNBR=25,REGION=1024K
//*- - ALLOCATE PROFILE, PANELS, MSGS, PROCS, AND TABLES -*/
//ISPPROF  DD DSN=USERAA.ISPF.PROFILE,DISP=OLD
//ISPPLIB  DD DSN=ISP.SISPPENU,DISP=SHR
//ISPMLIB  DD DSN=ISP.SISPMENU,DISP=SHR
//ISPSLIB  DD DSN=ISP.SISPSENU,DISP=SHR
//         DD DSN=ISP.SISPSLIB,DISP=SHR
//ISPTLIB  DD DSN=USERAA.ISPF.TABLES,DISP=SHR
//         DD DSN=ISP.SISPTENU,DISP=SHR
//         DD DSN=ISP.SISPTLIB,DISP=SHR
//ISPTABL  DD DSN=USERAA.ISPF.TABLES,DISP=SHR
//*
//*- - ALLOCATE ISPF LOG DATA SET  - - - - - - - - - - - -*/
//ISPLOG   DD DSN=USERAA.ISPF.LOG,DISP=SHR
//*
//*- - ALLOCATE DIALOG PROGRAM AND TSO COMMAND LIBRARIES -*/
//ISPLLIB  DD DSN=USERAA.ISPF.LOAD,DISP=SHR
//SYSEXEC  DD DSN=ISP.SISPEXEC,DISP=SHR
//SYSPROC  DD DSN=ISP.SISPCLIB,DISP=SHR
//*
//*- - ALLOCATE TSO BACKGROUND OUTPUT AND INPUT DS - - - -*/
//SYSTSPRT DD DSNAME=USERAA.ISPF.ISPFPRNT,DISP=SHR
//SYSTSIN  DD *
  PROFILE PREFIX(USERAA)         /* ESTABLISH PREFIX      */
  ISPSTART CMD(%TBUPDATE)        /* INVOKE CLIST DIALOG   */
/*
Figure 13. MVS batch job
Controlling ISPF sessions
32  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 61

Processing errors
ISPF terminates with an error message if a required library is not available. The ISPSTART command
must also be invoked naming either a CLIST, PGM function, or selection panel. If no dialog is specified, a
message is issued. These messages are directed to the data set defined by the SYSTSPRT DD statement.
Errors encountered during background dialog execution are handled in the same manner as errors
encountered during foreground execution. Messages normally written to the ISPF log data set for severe
errors are also written to the SYSTSPRT file. This is useful when executing a CLIST dialog because any
error messages are listed immediately after the ISPEXEC service in which the error occurred.
If a function encounters an abend, the entire ISPF batch job stream terminates. A message is issued to
the SYSTSPRT file indicating the type of abend.
Batch display facility for background panel processing
The Batch Display Facility allows applications to simulate full-screen write operations while ISPF is
executing in the background. This requires that dialogs provide the input to ISPF that would normally be
supplied by the user or by information associated with the type of terminal being used. Much of this is
done by having the dialog assign values to panel input variables, and by supplying screen size information
through keywords on the ISPSTART command.
Batch execution has traditionally not allowed the use of services that require user interaction. Any
full-screen write operation would result in an error condition.
The Batch Display Facility overcomes these limitations. Although there is no user interaction during
execution; the Batch Display Facility does allow background execution of interactive services. These
services include:
• DISPLAY
• TBDISPL
• SELECT PANEL
• SETMSG
• PQUERY
These services are issued for batch just as they are issued for dialogs running in interactive mode. ISPF
GDDM services do not run in the background, and thus, cannot be requested in a batch environment.
All ISPF commands except SPLIT and SPLITV can be executed in dialogs running in batch mode.
Installations can easily convert current interactive applications that use these services so they run in a
batch environment.
Supplying input in lieu of interactive users
When an application is running in batch, there is no user to respond to panel input operations. Therefore,
the primary requirement for running interactive applications in batch is to supply expected input data
by an alternate means. For example, panel variables can be given values by dialog function statements
or by the processing specified in the panel's executable sections. This processing is begun in the batch
environment as though a user had pressed Enter. In the absence of an alternative action on the dialog's
part, ISPF assumes an ENTER condition following a panel display.
A dialog can override the ENTER condition and establish an END condition by performing any of these
actions:
• Using the .RESP control variable
• Setting the panel command field to END
• Issuing a CONTROL NONDISPL END before the display operation
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  33

## Page 62

Supplying batch terminal characteristics
In a batch environment there is no terminal from which ISPF can get screen width and screen depth
values, so you must supply to ISPF data related to terminal type. You can include two optional keywords,
BATSCRW and BATSCRD, on the ISPSTART command line to specify, respectively, screen width and
screen depth values. The default values, if you do not include these keywords, are a screen width of
80 characters and a screen depth of 32 lines. The width and depth values, whether specified on the
ISPSTART command or through the default values, establish the values in system variables ZSCREENW,
ZSCREEND, ZSCRMAXW, and ZSCRMAXD.
In addition to the display services, use of the PQUERY service requires that the screen width and depth
values be supplied to ISPF, either through default values or as defined on the ISPSTART command.
When running batch, terminal characteristics cannot be changed during a session, although some
characteristics can be changed during an interactive session. For example, when ISPF is running
interactively you can specify 3278 Model 5 and 3290 screen formatting. In batch mode, a dialog does not
interact with a physical screen. Therefore, screen size, specified by including the BATSCRW and BATSCRD
keywords on the ISPSTART command, is fixed for the duration of the batch session.
When running in batch mode, you can include the BDBCS keyword on the ISPSTART command. ISPF then
processes the dialog as though it were running on a DBCS terminal.
The value in system variable ZCOLORS defines the number of colors (either 1 or 7) that a terminal can
display. In batch mode, ISPF sets ZCOLORS to 1.
The value in system variable ZHILITE (YES or NO) determines if a terminal is to have extended
highlighting capability, including underscore, blinking, and reverse video. In batch mode, ISPF sets
ZHILITE to NO.
Message processing in the batch environment
In an interactive environment ISPF displays two types of messages:
• Informational messages, normally those resulting from the MSG keyword specified on the SETMSG,
DISPLAY, or TBDISPL service
• Error messages, including those resulting from the .MSG control variable in an executable panel section.
When running in a batch environment, ISPF writes any informational or error messages to the ISPF log
data set at the processing point that the messages would normally be displayed to a user. The information
logged includes the name of the panel associated with the message, followed by the short message and
the long message.
A .MSG-initiated error message plus an ENTER condition causes a panel redisplay. In a batch
environment, there is no interactive user to correct the error, so it must be handled by statements
in the panel's )REINIT or )PROC sections. This leads to the possibility of a .MSG-redisplay loop if the
error condition is not corrected. Some panel language functions that can lead to this problem are VER,
TRANS, ADDSOSI, DELSOSI, .MSG, and PANEXIT. To prevent this loop, a BREDIMAX keyword on the
ISPSTART command is available to specify the maximum number (default 2) of redisplays. If this number
of redisplays is exceeded, a severe error condition (return code 20) results and the related error message
is written to SYSTSPRT.
Command processing in the batch environment
ISPF processes most commands when running in the batch environment in the same way it processes
them when running interactively, except that:
• The SPLIT and SPLITV commands are disabled.
• The ENVIRON, LOG, LIST, ISPPREP, KEYS, ZKEYS, and PFSHOW TAILOR commands can result in display
loops.
Controlling ISPF sessions
34  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 63

Display error processing in the batch environment
When ISPF is running interactively with CONTROL ERRORS CANCEL in effect, a return code of 12 or higher
causes the ISPF error panel to display. These same conditions in the batch environment cause the error
panel message to be written to the SYSTSPRT data set, after which ISPF terminates. In the interactive
or batch environment with CONTROL ERRORS RETURN in effect, control returns to the dialog for error
processing following a return code of 12 or higher.
How ISPF handles log and list data sets in the batch environment
If ISPF allocates a log or list data set in the batch environment, it is always kept at termination, regardless
of the disposition specified on SETTINGS Option 0.
Avoiding panel loop conditions in the batch environment
When writing new dialogs or altering existing dialogs to run in the batch environment, dialog developers
must be very careful not to create functions that result in a processing loop where user input is expected
and none is supplied. See “Supplying input in lieu of interactive users” on page 33 for more information.
For example, running the ISPPREP command causes ISPF to call an interactive ISPPREP dialog, which will
cause a loop condition in a batch environment. Instead, you should invoke the non-interactive ISPPREP
facility directly by using the SELECT PGM(ISPPREP) service request as described for batch mode under
Figure 46 on page 128.
The KEYS command can cause a loop condition because its processing termination depends on an END
or RETURN command. An ENTER condition, which ISPF assumes in absence of an END or RETURN being
forced, results only in another panel display, which leads to a loop condition.
To help deal with possible looping situations, the BDISPMAX keyword on the ISPSTART command is
available to specify the maximum number of panel displays that can occur during a session. The default
value is 100. You can test the current number of displays in a batch mode session by reading the
ZBDMXCNT system variable. The value of BDISPMAX is stored in the ZBDMAX system variable.
If the number specified in BDISPMAX is exceeded, a severe error condition (return code 20) results and
an error message, stating that the maximum number of displays has been exceeded, is written to the
SYSTSPRT data set.
Controlling ISPF sessions
Chapter 2. Controlling ISPF sessions  35

## Page 64

Controlling ISPF sessions
36  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
