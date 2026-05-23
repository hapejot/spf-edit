# Chapter 6. Creating edit macros

Source file: f54em00_v3r1.md
Start page: 119
Page span: 119-144

## Page 119

Chapter 6. Creating edit macros
This topic documents general-use programming interfaces and associated guidance information.
Edit macros are ISPF dialogs that run in the ISPF editor environment.
CLIST edit macros must be in partitioned data sets in at least one of these concatenations: SYSUPROC,
ALTLIB (for data sets activated as CLISTs), or SYSPROC. Data sets in these concatenations can contain
CLIST edit macros, REXX edit macros, or a combination of both. However, REXX edit macros in these
concatenations must include a REXX comment line (/* REXX */) as the first line of each edit macro
to distinguish them from CLIST edit macros. This comment line can contain other words or characters if
necessary, but it must include the string REXX.
Note: For more information about the ALTLIB concatenation, refer to z/OS TSO/E Command Reference.
REXX edit macros must also be in partitioned data sets. Besides the concatenations in the previous list for
CLIST edit macros, REXX edit macros can exist in these concatenations: SYSUEXEC, ALTLIB (for data sets
activated as EXECs), and SYSEXEC. Data sets in these concatenations can contain only REXX EXECs.
For example, if an application activates an application-level library with these commands:
ALTLIB ACTIVATE APPLICATION(EXEC) DA(DS1 DS2 DS3)
ALTLIB ACTIVATE APPLICATION(CLIST) DA(DSA DSB DSC)
then data sets DS1, DS2, and DS3 must contain only REXX EXECs. However, DSA, DSB, and DSC can
contain either REXX EXECs or CLISTs; if these data sets contain REXX EXECs, the first line of each EXEC
must be a REXX comment line.
As in an ISPF dialog, program macros must be made available as load modules in either the ISPLLIB,
STEPLIB, or LINKLST library.
CLIST and REXX edit macros
A CLIST edit macro is made up of CLIST statements. A REXX edit macro is made up of REXX statements.
Each statement falls into one of these categories:
• Edit macro commands
• CLIST or REXX command procedure statements and comments
• ISPF and PDF dialog service requests
• TSO and other environment commands
All statements are initially processed by the TSO command processor, which scans them and does
symbolic variable substitution. It is important to recognize the different kinds of CLIST and REXX
statements listed because:
• They are processed by different components of the system
• They have different syntax rules and error handling
• Their descriptions are in different manuals
Edit macros are invoked by the editor using the ISPF SELECT service. For REXX macros, the BARRIER
keyword is specified to ensure the REXX data stack is preserved across macro invocations.
Edit macro commands and assignment statements
Any statement in an edit macro that begins with ISREDIT is assumed to be an edit macro command or
assignment statement. When such a statement is found, the CLIST or REXX command processor does
symbolic substitution and then passes it to the editor. The editor processes it, performing any requested
functions. Examples of two edit macro commands are:
CLIST and REXX edit macros
© Copyright IBM Corp. 1984, 2024 87

## Page 120

Table 4. Example edit macro commands
CLIST Statements REXX Statements
 
ISREDIT FIND "TEST475"
ISREDIT PROCESS
ADDRESS ISPEXEC
'ISREDIT FIND TEST475'
'ISREDIT PROCESS'
Examples of two edit macro assignment statements are:
Table 5. Example edit macro assignment statements
CLIST Statements REXX Statements
 
ISREDIT BOUNDS = 1,60
ISREDIT (WIDTH) = LRECL
ADDRESS ISPEXEC
'ISREDIT BOUNDS = 1,60'
'ISREDIT (WIDTH) = LRECL'
A description of each edit macro command and assignment statement is in Chapter 11, “Edit macro
commands and assignment statements,” on page 295.
Using the REXX ADDRESS instruction
Use the REXX ADDRESS ISREDIT instruction to send instructions to the ISPF editor. You can send an
individual instruction to the editor, like this:
Address ISREDIT "FIND PROC"
If you have several edit macro commands within a REXX exec, you can change the command
environment to the ISPF editor with the instruction ADDRESS ISREDIT without any parameters.
Subsequent commands in the exec are passed directly to the editor, until the next ADDRESS statement.
These examples show how you can pass the same edit macro commands using different command
environments:
Table 6. Passing commands using a different environment
ISPEXEC Environment ISREDIT Environment
ADDRESS ISPEXEC
'ISREDIT BOUNDS = 1,60'
'ISREDIT (WIDTH) = LRECL'
ADDRESS ISREDIT
'BOUNDS = 1,60'
'(WIDTH) = LRECL'
For information on using the REXX ADDRESS instruction, refer to z/OS TSO/E REXX Reference.
Command procedure statements
Command procedure statements handle CLIST and REXX variables and control flow within a CLIST
or REXX exec. Command procedure statements are processed by the TSO command processor. Some
command procedure statements commonly used in edit macros are:
• Assignment statements
• IF-THEN-ELSE statements
• DO-WHILE-END statements
• EXIT statements
For a complete list and description of command procedure statements for CLIST and REXX, refer to z/OS
TSO/E CLISTs, z/OS TSO/E REXX Reference, and z/OS TSO/E REXX User's Guide.
CLIST and REXX edit macros
88  z/OS: z/OS ISPF Edit and Edit Macros

## Page 121

ISPF and PDF dialog service requests
Any statement in an edit macro beginning with ISPEXEC is assumed to be a request for an ISPF service.
When such a statement is found, the TSO command processor does symbolic substitution. It then passes
the command to the specified ISPF service to be processed. Some examples of service requests that
might be in an edit macro are:
Table 7. Service requests in an edit macro
CLIST Statements REXX Statements
 
ISPEXEC SETMSG ...
ISPEXEC VPUT ...
ISPEXEC DISPLAY ...
ISPEXEC EDIT ...
ISPEXEC LMINIT ...
ADDRESS ISPEXEC
'SETMSG ...'
'VPUT ...'
'DISPLAY ...'
'EDIT ...'
'LMINIT ...'
For more information on ISPF services, refer to z/OS ISPF Services Guide.
TSO commands
Any statement that is not recognized as a command procedure statement and does not begin with
ISPEXEC or ISREDIT is assumed to be a TSO command. TSO commands can be either CLISTs, REXX
EXECs, or programs. When the command processor finds a TSO command, it processes the command.
Examples of TSO commands are:
Table 8. TSO commands
CLIST Statements REXX Statements
 
ALLOCATE ...
FREE ...
DELETE ...
RENAME ...
ADDRESS TSO
'ALLOCATE ...'
'FREE ...'
'DELETE ...'
'RENAME ...'
For more information on TSO commands, refer to z/OS TSO/E Command Reference.
Program macros
Not all edit macros are written in CLIST or REXX. You can also write edit macros in a programming
language such as PL/I, COBOL, FORTRAN, APL2®, Pascal, or C. These are called program macros.
There are four basic reasons to write and debug a program macro:
• A macro runs faster in a language that can be precompiled than in CLIST or REXX. This can be valuable
for macros that you run many times.
• A macro that must read data containing symbols can confuse an interpretive language processor.
Particularly, ampersands in the data can cause problems.
• Complex logic can be handled better in a programming language.
• To pass mixed data or strings (those that contain both EBCDIC and DBCS characters) as parameters,
you must use a program macro. Although CLIST does not allow mixed data strings, these edit macro
commands and assignment statements allow you to supply data or string operands:
  CHANGE     EXCLUDE      FIND
  LINE       LINE_AFTER   LINE_BEFORE
  MASKLINE   SEEK         TABSLINE
Program macros
Chapter 6. Creating edit macros  89

## Page 122

Differences between program macros, CLISTs, and REXX EXECs
Program macros have special characteristics that you should consider before coding:
• Variables are not self-defining in program macros, as they are in CLISTs and REXX EXECs. The VDEFINE,
VCOPY, and VREPLACE dialog services must be called to identify variables looked at or set by the
program.
• If you write a REXX exec or a program macro that accepts parameter input, the macro must be aware
that the input may be in lowercase. Variable values are automatically converted to uppercase by the
CLIST processor.
• Program macros are not implicitly defined, while CLIST and REXX macros are. When you use a
command name that is not a built-in or previously defined primary command, the editor searches the
SYSUEXEC, SYSUPROC, ALTLIB, SYSEXEC, and SYSPROC concatenations (for CLISTs and REXX EXECs)
for a member with the same name. If it exists, it is assumed to be a macro.
No automatic search is done for program macros. Therefore, there are two ways to tell the editor to run
a macro as a program macro. You can precede the name with an exclamation point (!) if it is less than
8 characters, or you can use the DEFINE command to define the name as a program macro. Program
macros are treated as ISPF dialogs, and must be made available as load modules in either the ISPLLIB,
STEPLIB, or LINKLST library.
• Program macros can run without being verified as macros; the MACRO statement can follow calls to
dialog services.
• The editor scans edit statements within program macros to do variable substitution similar to the
CLIST processor. Only one level of substitution is done. This is the default; use the SCAN assignment
statement to prevent it.
Passing parameters in a program macro
Program macros process edit commands by using the ISPLINK or ISPEXEC interface. ISPLNK and ISPEX
are the interface names used in FORTRAN and Pascal programs. Parameters are passed to the ISREDIT
service as follows:
• CALL ISPLINK ('ISREDIT',length,buffer)
CALL ISPEXEC (length,'ISREDIT command')
where these definitions apply:
'ISREDIT'
The service name.
length
A fullword number indicating the length of the command buffer. When a zero length is passed, the
maximum buffer length is 255 bytes.
buffer
Can contain any edit command that is valid from a macro, typed with the same syntax used in a CLIST
or REXX exec.
command
Any PDF edit command that is valid from a macro, typed with the same syntax used in a CLIST or
REXX exec.
Program macro examples
These examples show three different methods of coding a FIND command for a program macro. They are
typed using PL/I syntax:
CALL ISPLINK ('ISREDIT',LEN0,'¢FIND XYZ¢')
CALL ISPLINK ('ISREDIT',LEN8,'FIND XYZ')
Program macros
90  z/OS: z/OS ISPF Edit and Edit Macros

## Page 123

CALL ISPEXEC (LEN16,'ISREDIT FIND XYZ')
where:
• LEN0
A fullword program variable with a value of 0.
LEN8
A fullword program variable with a value of 8.
LEN16
A fullword program variable with a value of 16.
In each of these examples, the rest of the command is typed as a literal value.
The first two examples use the ISPLINK syntax. In the ISPLINK call, ISREDIT is passed as the first
parameter and is omitted from the command buffer.
The first example uses a special interface. A zero length can be passed, but only when the command is
delimited by a special character. A special character cannot be an alphanumeric character. If the length is
zero and if a valid delimiter is the first character in the command buffer, a scan of the command is done to
find the next occurrence of that character. The command length is the number of characters between the
two delimiters. Here, the cent sign (¢) is used as a delimiter. When a zero length is passed, the maximum
buffer length is 255 bytes.
In the second example, an explicit length of 8 is used and the command buffer contains the command
without delimiters.
The third example uses the ISPEXEC syntax. This syntax always requires the length of the command
buffer to be passed. The command buffer includes the ISREDIT prefix, and is typed the same way as a
CLIST or REXX command.
Writing program macros
When you write a program macro, it can help to first type it as a CLIST or REXX macro to debug the logic
and the command statements. The example that follows is a simple macro that separates each line in a
set of data with a line of dashes. The REXX version, called ISRSLREX, is shown in Figure 33 on page 92.
The PL/I program is shown in Figure 34 on page 93, and the COBOL program is shown in Figure 35 on
page 94. Notice that a VDEFINE is not required for the variable SAVE, which is referenced only by the
ISPF editor.
Program macros
Chapter 6. Creating edit macros  91

## Page 124

/* Rexx **************************************************************/
/***** Sample Edit Macro *********************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRSLREX - separates lines with a line of dashes.                 */
/*                                                                   */
/*********************************************************************/
TRACE
ADDRESS ISPEXEC
'ISREDIT MACRO'
   'ISREDIT (SAVE) = USER_STATE'
   'ISREDIT RESET'
   'ISREDIT EXCLUDE ----- 1 ALL'
   'ISREDIT DELETE ALL X'
   LASTL = 1
   LINE = 0
   LINX = COPIES('-',70)
   LL = LASTL + 1
   DO WHILE LINE < LL
     'ISREDIT LINE_AFTER 'LINE' = (LINX)'
     'ISREDIT (LASTL) = LINENUM .ZLAST'
     LL = LASTL + 1
     LINE = LINE + 2
   END
   'ISREDIT USER_STATE = (SAVE)'
EXIT
Figure 33. ISRSLREX REXX macro
Program macros
92  z/OS: z/OS ISPF Edit and Edit Macros

## Page 125

/*                                                                */
 /* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                     */
 /*                                                                */
 /* ISRSEPP - EDIT MACRO PROGRAM TO INSERT SEPARATOR LINES         */
 /*            PL/I                                                */
 /*                                                                */
 ISRSEPP: PROC OPTIONS (MAIN);
 /*                                                            */
 DECLARE
   LINEX CHAR (70) INIT ((70)'-') ,     /* SEPARATOR LINE ---  */
   LASTL FIXED BIN(31,0) INIT (0),      /* LAST LINE OF TEXT   */
   LINE  FIXED BIN(31,0) INIT (0),      /* CURRENT LINE NUMBER */
   LEN0  FIXED BIN(31,0) INIT (0),      /* LENGTHS - 0         */
   LEN1  FIXED BIN(31,0) INIT (1),      /* LENGTHS - 1         */
   LEN4  FIXED BIN(31,0) INIT (4),      /* LENGTHS - 4         */
   LEN70 FIXED BIN(31,0) INIT (70);     /* LENGTHS - 70        */
                                        /*                     */
 DECLARE                                /*                     */
   ISPLINK ENTRY OPTIONS(ASM,INTER,RETCODE); /* LINK TO ISPF   */
                                        /*                     */
   CALL ISPLINK('VDEFINE','(LASTL)',LASTL,'FIXED',LEN4);
   CALL ISPLINK('VDEFINE','(LINE)', LINE, 'FIXED',LEN4);
   CALL ISPLINK('VDEFINE','(LINEX)',LINEX,'CHAR', LEN70);
   CALL ISPLINK('ISREDIT',LEN0,'¢ MACRO ¢');
   CALL ISPLINK('ISREDIT',LEN0,'¢ (SAVE) = USER_STATE ¢');
   CALL ISPLINK('ISREDIT',LEN0,'¢ RESET ¢');
   CALL ISPLINK('ISREDIT',LEN0,'¢ EXCLUDE ------ 1 ALL ¢');
   CALL ISPLINK('ISREDIT',LEN0,'¢ DELETE ALL X ¢');
   LASTL = 1;
   LINE = 0;
 DO WHILE (LINE < (LASTL + 1));
   CALL ISPLINK('ISREDIT',LEN0,'¢ LINE_AFTER &LINE = (LINEX) ¢    ');
   CALL ISPLINK('ISREDIT',LEN0,'¢ (LASTL) = LINENUM .ZLAST ¢');
   LINE = LINE + 2;
 END;
   CALL ISPLINK('ISREDIT',LEN0,'¢ USER_STATE = (SAVE) ¢');
 END IISRSEPP;
Figure 34. ISRSEPP PL/I macro
Program macros
Chapter 6. Creating edit macros  93

## Page 126

ID DIVISION.
       PROGRAM-ID. ISRSEPC.
      *
      *           EDIT MACRO PROGRAM TO INSERT SEPARATOR LINES
      *
       ENVIRONMENT DIVISION.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  LINEX   PIC  X(70) VALUE  ALL  "-".
      *   SEPARATOR LINE ------
       01  LASTL   PIC  9(6)  VALUE    0  COMP.
      *   LAST LINE OF TEXT
       01  LYNE    PIC  9(6)  VALUE    0  COMP.
      *   CURRENT LINE NUMBER
       01  ISREDIT PIC  X(8)  VALUE  "ISREDIT ".
       01  VDEFINE PIC  X(8)  VALUE  "VDEFINE ".
       01  ZLASTL  PIC  X(8)  VALUE  "(LASTL )".
       01  ZLINE   PIC  X(8)  VALUE  "(LINE  )".
       01  ZLINEX  PIC  X(8)  VALUE  "(LINEX )".
       01  FIXED   PIC  X(8)  VALUE  "FIXED   ".
       01  CHAR    PIC  X(8)  VALUE  "CHAR    ".
       01  LEN0    PIC  9(6)  VALUE    0  COMP.
       01  LEN4    PIC  9(6)  VALUE    4  COMP.
       01  LEN70   PIC  9(6)  VALUE   70  COMP.
       01  EM1     PIC  X(10) VALUE  "¢  MACRO ¢".
       01  EM2     PIC  X(24) VALUE  "¢  (SAVE) = USER_STATE ¢".
       01  EM3     PIC  X(10) VALUE  "¢  RESET ¢".
       01  EM4     PIC  X(25) VALUE  "¢  EXCLUDE ------ 1 ALL 0".
       01  EM5     PIC  X(18) VALUE  "¢  DELETE ALL X ¢".
       01  EM6     PIC  X(30) VALUE  "¢ LINE_AFTER &LINE = (LINEX) ¢".
       01  EM7     PIC  X(28) VALUE  "¢ (LASTL) = LINENUM .ZLAST ¢".
       01  EM8     PIC  X(23) VALUE  "¢ USER_STATE = (SAVE) ¢".
       PROCEDURE DIVISION.
           CALL  "ISPLINK" USING  VDEFINE  ZLASTL  LASTL  FIXED LEN4.
           CALL  "ISPLINK" USING  VDEFINE  ZLINE   LYNE   FIXED LEN4.
           CALL  "ISPLINK" USING  VDEFINE  ZLINEX  LINEX  CHAR  LEN70.
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM1.
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM2.
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM3.
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM4.
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM5.
           MOVE  1 TO LASTL.
           MOVE  0 TO LYNE.
           PERFORM LOOP UNTIL LYNE IS NOT LESS THAN (LASTL + 1).
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM8.
           GOBACK.
       LOOP.
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM6.
           CALL  "ISPLINK" USING  ISREDIT  LEN0    EM7.
           ADD 2 TO LYNE.
Figure 35. ISRSEPC COBOL macro
Running program macros
The ISPF editor assumes that any unknown primary command is a macro, and it also assumes that the
macro has been implemented as a CLIST or REXX exec. You can define a macro as a program macro either
by entering a DEFINE command or by prefixing the macro name with an exclamation point (!) when you
type the macro name on the command line.
If a macro named FINDIT is a CLIST or REXX exec macro, for example, you can run it by typing FINDIT on
the command line and pressing Enter. If it is a program macro, you can type !FINDIT, or FINDIT if it had
previously been defined as a program macro by the DEFINE command. The first time you enter a macro
with an exclamation point (!) prefix implicitly defines that macro as a program macro. Thereafter, you can
omit the prefix.
To use the DEFINE command to define a program as a macro, type this command and press Enter:
Program macros
94  z/OS: z/OS ISPF Edit and Edit Macros

## Page 127

DEFINE name PGM MACRO
The operands can be typed in either order. That is, DEFINE name MACRO PGM is also valid.
Using commands in edit macros
You can use most primary commands in an edit macro if you precede it with ISREDIT. Table 19 on page
295 shows the macro commands available to use. There are differences, though, between entering a
command on the command line and processing the same command in a macro as one of a series:
• When you enter a command on the command line, the result of the command is displayed in either an
informational or an error message. If you process the same command in a macro, messages are not
displayed, and the lines actually displayed may be different from a command entered on the command
line.
• When you issue a series of commands as a macro, the display does not change with each command.
The lines displayed are the end result of the macro running, not the individual commands.
• Some commands have additional operands permitted in a macro that cannot be used interactively.
Besides these differences, there are certain guidelines to remember when creating edit macros. These
topics apply to CLIST, REXX, and program macros.
Naming edit macros
Edit macro names can be any valid CLIST, REXX, or program name. Using the DEFINE ALIAS command,
you can assign command names for running the edit macros that are different from the actual name.
When choosing names and aliases, avoid defining names that might conflict with the DEFINE command
operands and their abbreviations. You can do this by implicitly defining the macros: precede program
macros with an exclamation point (!); do not use explicit definitions for CLIST or REXX macros.
Variables
Variables function in edit macros in the same way as in CLISTs and REXX EXECs. The only exceptions are
dialog variables—variables that communicate with ISPF and the PDF component—which can only have
names from 1 to 8 characters in length. This topic presents a brief introduction on using variables; for
more detailed information on variables in CLISTs, refer to z/OS TSO/E CLISTs. For information on variables
in REXX EXECs, refer to z/OS TSO/E REXX Reference and z/OS TSO/E REXX User's Guide.
When coding macros in CLIST or REXX, remember that all ISREDIT statements are processed for variable
substitution before the editor sees the statements. Enclose the variables in parentheses when variable
substitution should not occur, such as in cases when ISREDIT statements expect a variable name and not
its value. For CLIST variables, omit the ampersand; for REXX variables, use quotes.
Variable substitution
Scan mode controls the automatic replacement of variables in command lines passed to the editor. Use
the SCAN assignment statement either to set the current value of scan mode (for variable substitution), or
to retrieve the current value of scan mode and place it in a variable.
When scan mode is on, command lines are scanned for ampersands (&). If an ampersand followed by
a nonblank character is found, the characters between the ampersand and the next blank or period are
treated as the name of a dialog variable. The value from the variable pool is substituted in the command
for the variable name before the command is processed. For example, &DVNAME. and &DVNAME are both
interpreted as a dialog variable called DVNAME.
The period after the variable allows concatenation of the variable value without an intervening blank
delimiter. Remember this when using program macros that do not have the CLIST processor to substitute
variable values.
Using commands in edit macros
Chapter 6. Creating edit macros  95

## Page 128

Character conversion
A CLIST automatically converts all character strings to uppercase before passing them to the editor.
Therefore, if you want an edit macro command or assignment statement that you process from a CLIST to
find a character string in lowercase, you must precede the command or statement with the TSO CONTROL
ASIS statement. This statement passes lowercase characters to the editor.
Edit assignment statements
You use edit assignment statements to communicate between macros and the editor. An assignment
statement consists of two parts, values and keyphrases, which are separated by an equal sign. The value
segment represents data that is in the macro, and the keyphrase segment represents data in the editor.
You can use assignment statements to pass data from the edit macro to the editor, or to transfer data
from the editor to the edit macro.
Data is always transferred from the right side of the equal sign in an assignment statement to the left side.
Therefore, if the keyphrase is on the right, data known to the editor is put into CLIST or REXX variables on
the left. In this situation, the yyy would be a keyphrase, and the xxx would be the value.
Table 9. Edit assignment statements
CLIST Statement REXX Statements
 
ISREDIT  xxx = yyy
ADDRESS ISPEXEC
'ISREDIT xxx = yyy'
Value
The value part of an edit macro assignment statement can be:
• A literal character string can be one of these types:
Simple
Any series of characters not enclosed within quotes (either ' or "), parentheses, or less-than (<) and
greater-than signs (>), and not containing any embedded blanks or commas.
Delimited
Any string starting and ending with a quote (either ' or "), but not containing embedded quotes. The
delimiting quotes are not considered to be part of the data.
• A dialog variable name enclosed in parentheses (varname). If the dialog variable name is on the right,
the entire contents of the variable are considered part of the data, including any quotes, apostrophes,
blanks, commas, or other special characters. If the dialog variable name is on the left, its content is
totally replaced.
Note:
1. In the CLIST environment, the CLIST variable pool and the dialog function variable pool are merged.
Therefore, variables in parentheses are the same as ampersand variables, except that the editor
does the symbolic substitution rather than the CLIST processor.
2. In the REXX environment, the REXX variable pool and the dialog function variable pool are also
merged. Therefore, quoted variable names in parentheses are the same as unquoted variable names,
except that the editor does the symbolic substitution rather than the REXX processor.
3. In a program macro, you must use the VDEFINE service for any variables that are passed to the
editor.
Keyphrase
A keyphrase is either a single keyword, or a keyword followed by a line number or label. The keyphrase
can be either a single-valued keyphrase or a double-valued keyphrase.
Using commands in edit macros
96  z/OS: z/OS ISPF Edit and Edit Macros

## Page 129

Keyphrase syntax
Single-valued keyphrases can have this syntax:
ISREDIT keyphrase = keyphrase
ISREDIT keyphrase = value
ISREDIT keyphrase = keyphrase + value
ISREDIT keyphrase = value + value
Double-valued keyphrases can have this syntax:
ISREDIT (varname,varname) = keyphrase
ISREDIT keyphrase = value-pair
where value-pair is one of these:
• Two literals, which can be separated by a comma or blank. For example: 
Table 10. Separating two literals
CLIST Statements REXX Statements
 
ISREDIT CURSOR = 1,40
ISREDIT CURSOR = 1 40
ADDRESS ISPEXEC
'ISREDIT CURSOR = 1,40'
'ISREDIT CURSOR = 1 40'
Apostrophes or quotes cannot be used when specifying two numeric values. All of these, for example,
are incorrect:
Table 11. Invalid syntax for specifying numeric values
CLIST Statements REXX Statements
ISREDIT CURSOR = '1','40' 
ISREDIT CURSOR = '1,40'
ADDRESS ISPEXEC
'ISREDIT CURSOR = ''1'',''40'''
'ISREDIT CURSOR = ''1,40''' 
• Two variable names enclosed in parentheses and separated by a comma or blank, where each variable
contains a single value:
(varname,varname) or (varname varname)
In any edit assignment statement containing a two-valued keyphrase, either of the variables or values in a
pair can be omitted. The general syntax then becomes:
ISREDIT  (varname) = keyphrase
ISREDIT  keyphrase = single-value
ISREDIT  (,varname) = keyphrase
ISREDIT  keyphrase = ,single-value
Note: Even though you can use blanks instead of commas to separate paired variables or values, you
must use a leading comma whenever the first variable or value has been omitted.
Overlays and templates
The transfer of information from one side of the equal sign to the other can involve combining several
variables or values. This transfer is called an overlay. When you perform overlays, there are certain
guidelines to remember.
When two values (or a keyphrase and a value) are on one side of an equal sign and separated by a plus
sign (+), only nonblank characters in the value on the right overlay corresponding positions in the value on
the left. For example:
Using commands in edit macros
Chapter 6. Creating edit macros  97

## Page 130

CLIST statements
ISREDIT LINE .ZCSR = LINE + '//'
ISREDIT MASKLINE = MASKLINE + <40 '&STR(/*)' 70 '&STR(*/)'>
REXX statements
ADDRESS ISPEXEC
"ISREDIT LINE .ZCSR = LINE + '//'"
"ISREDIT MASKLINE = MASKLINE + <40 '/*' 70 '*/'>"
The first example causes two slashes to replace the first two column positions of the current line (the line
containing the cursor). The remainder of the line is unchanged. The second example uses a template to
cause columns 40-41 of the current mask line to be replaced with /* and columns 70-71 to be replaced
with */. Again, remember that the template replaces the corresponding positions on the left only if those
left positions are blank. The template shown in the preceding example has the form:
<col1 literal1 col2 literal2 ... >
It can be designed with col1 and col2 indicating a starting column position, and literal1 and literal2
indicating the data to start in that column. The entire template is delimited with less-than (<) and
greater-than (>) signs. A template can be designed by using variable names (enclosed in parentheses) for
either col1, col2, literal1, literal2, or for all four. All of these forms are valid:
<(colvar1) (datavar1) (colvar2) (datavar2) ... >
<(colvar1,datavar1)   (colvar2,datavar2)   ... >
<(colvar1)  literal1   col2     (datavar2) ... >
Using edit assignment statements
You can use an assignment statement to pass edit parameters to a macro or to allow a macro to set
an edit parameter. If the edit parameter keyphrase is on the right of the assignment statement, the
edit parameter is passed to the macro. If the edit parameter keyphrase is on the left of the assignment
statement, the edit parameter is changed to the value on the right. In the assignment statement shown,
the edit parameter keyphrase is CAPS. The editor assigns the current CAPS edit mode status (ON or OFF)
to the variable CAPMODE. 
Table 12. Assigning a value to a variable
CLIST Statement REXX Statements
 
ISREDIT (CAPMODE) = CAPS
ADDRESS ISPEXEC
'ISREDIT (CAPMODE) = CAPS'
In the preceding example statements, the parentheses around CAPMODE indicate to the ISPF editor that
the enclosed name is the name of a symbolic variable. If the name happened to be preceded by an
ampersand (&), rather than enclosed in parentheses, the CLIST processor would replace the name of the
variable with its actual value, and the editor would not see the name. In a REXX statement, the variable
name must be within quotes so that the name, not the value, is passed. Only names with 8 or fewer
characters are allowed by the ISPF editor.
When the editor finds a variable name in parentheses in a position where a value is required, it substitutes
the value assigned to that variable. In these examples the edit macro sets the edit CAPS mode: 
Using commands in edit macros
98  z/OS: z/OS ISPF Edit and Edit Macros

## Page 131

Table 13. Substituting a value in a variable
CLIST Statements REXX Statements
 
ISREDIT CAPS = ON
ISREDIT CAPS = (CAPMODE)
ISREDIT CAPS = &CAPMODE
ADDRESS ISPEXEC
'ISREDIT CAPS = ON'
'ISREDIT CAPS = (CAPMODE)'
'ISREDIT CAPS = 'capmode
The CLIST and REXX command processors replace the variable CAPMODE with its assigned value before
the ISPF editor processes the statement. This makes the last statement equivalent to the first statement;
in this case, the variable has a value of ON.
The second statement differs in that the editor receives the variable name and retrieves its value from the
dialog variable pool.
Passing values
Some information can best be passed back and forth between the editor and the macro in pairs. These
examples show assignment statements that pass two values:
CLIST Statements REXX Statements
 
ISREDIT (LB,RB) = BOUNDS
ISREDIT BOUNDS = (LB,RB)
ADDRESS ISPEXEC
'ISREDIT (LB,RB) = BOUNDS'
'ISREDIT BOUNDS = (LB,RB)'
In the first statement, the current left and right boundaries are stored into the variables LB (LEFTBND)
and RB (RIGHTBND). In the second statement, the values from the variables LB and RB are used to
change the current boundaries.
For more information on which edit macro commands take one variable and which take two, see Chapter
11, “Edit macro commands and assignment statements,” on page 295.
Manipulating data with edit assignment statements
You can use assignment statements to obtain, replace, or add data being edited.
To copy a line, use:
CLIST Statement REXX Statements
 
ISREDIT LINE_AFTER 5 = LINE 2
ADDRESS ISPEXEC
'ISREDIT LINE_AFTER 5 = LINE 2'
To copy line 1 from the data set into the variable LINEDATA, use:
CLIST Statement REXX Statements
 
ISREDIT (LINEDATA) = LINE 1
ADDRESS ISPEXEC
'ISREDIT (LINEDATA) = LINE 1'
To replace the first line in the data set, using the data from the variable LINEDATA, use:
CLIST Statement REXX Statements
 
ISREDIT LINE 1 = (LINEDATA)
ADDRESS ISPEXEC
'ISREDIT LINE 1 = (LINEDATA)'
Using commands in edit macros
Chapter 6. Creating edit macros  99

## Page 132

To add a new line after line 1 in the data set using the variable NEWDATA, use:
CLIST Statement REXX Statements
 
ISREDIT LINE_AFTER 1 = (NEWDATA)
ADDRESS ISPEXEC
'ISREDIT LINE_AFTER 1 = (NEWDATA)'
Differences between edit, CLIST, and REXX assignment statements
• Edit assignment statements are preceded by ISREDIT. CLIST assignment statements are preceded by
SET. If the ADDRESS ISREDIT command is in effect, edit assignment statements within a REXX exec do
not need to be preceded by ISREDIT.
• In edit assignment statements, a keyphrase must appear on either the left or right side of the equal
sign. A keyphrase is either a single keyword, or a keyword followed by a line number or label. See
“Keyphrase” on page 96 if you need more information.
• When coding edit assignment statements, variable names to be passed to the editor are enclosed in
parentheses so that the PDF component is passed the name of the variable, not its value. Sometimes
two variable names may appear within the parentheses.
• Arithmetic expressions are not allowed in an edit assignment statement, but in certain cases a plus sign
(+) can be used to show partial overlay of a line. See “Overlays and templates” on page 97 if you need
more information.
Performing line command functions
You cannot issue line commands directly from an edit macro. For example, you cannot use the M (move)
line command within an edit macro.
However, you can perform most of the functions provided by line commands by writing an edit macro. By
using edit assignment statements or by issuing primary commands, you can perform functions such as
move, copy, or repeat. For example, if you want to move a line, you can assign the line to a CLIST or REXX
variable, delete the original line using the DELETE command, and assign the variable to a new line in the
data.
Some commands can be processed only from within a macro. These commands provide functions
done with line commands from the keyboard. Table 14 on page 100 identifies the commands, the
corresponding line commands, and the functions performed.
Table 14. Edit macro commands corresponding to line commands
Edit Macro Statement Corresponding Line Command Function
INSERT I Inserts temporary lines
SHIFT ( ( Shifts columns left
SHIFT ) ) Shifts columns right
SHIFT < < Shifts data left
SHIFT > > Shifts data right
TENTER TE Starts text entry mode
TFLOW TF Performs text flow
TSPLIT TS Performs text split
For example:
Using commands in edit macros
100  z/OS: z/OS ISPF Edit and Edit Macros

## Page 133

CLIST Statement REXX Statements
 
ISREDIT TFLOW 1
ADDRESS ISPEXEC
'ISREDIT TFLOW 1'
causes the paragraph starting on line 1 to be flowed in the same way as a TF (text flow) line command
would if entered on the first line.
For more information on line command functions in edit macros, see Chapter 11, “Edit macro commands
and assignment statements,” on page 295.
Parameters
If you want to supply information to a macro as parameters, you must identify these parameters on
the ISREDIT MACRO statement by enclosing them in parentheses. For example, if you have this macro
command in an edit macro named FIXIT:
CLIST Statement REXX Statements
 
ISREDIT MACRO (MEMNAM)
ADDRESS ISPEXEC
'ISREDIT MACRO (MEMNAM)'
when you enter:
Command ====> FIXIT ABCD
the value ABCD is assigned to the variable MEMNAM.
Passing parameters to a macro
A parameter can be either a simple string or a quoted string. It can be passed by using the standard
method of putting variables into shared and profile pools (use VPUT in dialogs and VGET in initial macros).
This method is best suited to parameters passed from one dialog to another, as in an edit macro.
You can enter parameters along with an edit macro name as a primary command by using the MACRO
command. This command allows you to identify the names of one or more variables to contain any passed
parameters.
Note: For edit line macros, only one parameter is passed to the macro. This parameter is the line
command, including any repetition, as it was entered on the line.
For more information, see “Working with an edit line command table” on page 84.
It is an error to enter parameter values for a macro without parameter variables. If you make this
mistake, the editor displays a message. It is not an error if you supply more or fewer parameters than the
number of variables that are included on the MACRO command. When you are writing a macro, check for
omissions and the order of parameters.
Multiple parameters are placed into one or more variables based on the number of variables specified in
the MACRO command. If you include more than one variable name, the editor stores the parameters in
order (the first parameter in the first variable, the second in the second, and so on). Note that assignment
to variables is by position only.
If there are more parameters entered than there are variables available, the editor stores the remaining
parameters as 1 character string in the last variable. If you include only one variable name on the MACRO
command, that variable contains all the parameters entered with the macro name. If there are more
variable names than parameters, the unused variables are set to nulls.
Multiple parameters are separated by a blank or comma, or a quoted string that is separated by a blank or
comma. Quotes can be single (') or double ("). If you want your FIXIT macro to accept two parameters,
for example, you can include this command:
Using commands in edit macros
Chapter 6. Creating edit macros  101

## Page 134

CLIST Statement REXX Statements
 
ISREDIT MACRO (PARM1,PARM2,REST)
ADDRESS ISPEXEC
'ISREDIT MACRO (PARM1,PARM2,REST)'
This means that if you enter:
FIXIT GOOD BAD AND UGLY
variable PARM1 is assigned the value "GOOD", PARM2 is assigned the value "BAD", and REST is assigned
the value "AND UGLY".
If the parameters passed were GOOD BAD, variable REST would be null. Also, if the parameters are
enclosed in quotation marks, such as:
FIXIT 'GOOD BAD' 'AND UGLY'
PARM1 would be set to "GOOD BAD", PARM2 would be set to "AND UGLY", and REST would be null.
For another example, see the ISRTRYIT macro (Figure 38 on page 115). If the MACRO statement contains
two variables (ISREDIT MACRO (command,parm)), entering:
ISRTRYIT RESET
sets the variables command to "RESET" and parm to null. Conversely, this command:
ISRTRYIT FIND A
sets command to "FIND" and parm to "A". To find out what was actually typed on the command line,
a macro may examine the variable ZEDITCMD, which is in the shared variable pool. ZEDITCMD is a
character variable, the length if which depends on the length of the command entered. Therefore, you
should either VDEFINE ZEDITCMD to be sufficiently large to hold the expected command, or use the
VCOPY service to get the length.
Using edit macros in batch
You can run edit macros in batch by submitting JCL which allocates all of the necessary ISPF libraries
(refer to z/OS ISPF Dialog Developer's Guide and Reference ), and runs a command which calls the EDIT
service with an initial macro. This initial macro can do anything that can be done by an initial macro in an
interactive session. However, in batch, the macro should end with an ISREDIT END or ISREDIT CANCEL
statement. These statements ensure that no attempt is made to display the edit screen in batch.
A simple initial macro to change strings in batch might look like this:
ISREDIT MACRO
ISREDIT CHANGE JANUARY FEBRUARY ALL
ISREDIT END
Edit macro messages
You can display messages from an edit macro the same way you do from an ISPF dialog.
• Use SETMSG, which causes the message to appear on whatever panel is displayed next.
• Use DISPLAY with the MSG keyword. This is useful if the macro displays panels of its own.
ISPF provides three generic messages for use in dialogs where you want to generate the message text or
when you do not want a separate message library.
ISRZ000  '&ZEDSMSG'  .ALARM = NO   .HELP = ISR2MACR
'&ZEDLMSG'
Using commands in edit macros
102  z/OS: z/OS ISPF Edit and Edit Macros

## Page 135

ISRZ001  '&ZEDSMSG'  .ALARM = YES  .HELP = ISR2MACR
'&ZEDLMSG'
ISRZ002  '&ZERRSM'  .ALARM = &ZERRALRM  .HELP = &ZERRHM
'&ZERRLM'
For example, if you want your macro to sound an alarm and to issue the short message INVALID
PARAMETER and the long message PARAMETER MUST BE 4 DIGITS, use these statements:
CLIST statements
SET &ZEDSMSG = &STR(INVALID PARAMETER)
SET &ZEDLMSG = &STR(PARAMETER MUST BE 4 DIGITS)
ISPEXEC SETMSG MSG(ISRZ001)
REXX statements
ADDRESS ISPEXEC
zedsmsg = 'Invalid Parameter'
zedlmsg = 'Parameter must be 4 digits'
'SETMSG MSG(ISRZ001)'
Note: ZEDLMSG only displays when you enter the HELP command.
Macro levels
Each macro operates on a separate and unique level. A person at the keyboard always operates at level 0.
If that person starts a macro, it operates at level 1; the macro started by a level-1 macro operates at level
2, and so on. The level is the degree of macro nesting. Edit macros are primary commands; thus, nested
macros are started by prefixing them with ISREDIT.
A macro can determine its own level with this assignment statement:
ISREDIT (varname) = MACRO_LEVEL
The current level number is stored in the specified variable. ISPF supports up to 255 levels of macro
nesting.
Labels in edit macros
A label is an alphabetic character string used to name lines. It is especially useful for keeping track of a
line whose relative line number may change because labels remain set on a line even when relative line
numbers change. The special labels shown are automatically assigned by the editor. A label must begin
with a period (.) and be followed by no more than 8 alphabetic characters, the first of which cannot be Z.
No special characters or numeric characters are allowed.
The special labels that are automatically assigned by the editor all begin with the letter Z. Labels
beginning with Z are reserved for editor use only.
The editor-assigned labels are:
.ZCSR
The data line on which the cursor is currently positioned.
.ZFIRST
The first data line (same as relative line number 1). Can be abbreviated .ZF.
.ZLAST
The last data line. Can be abbreviated .ZL.
.ZFRANGE
The first line in a range specified by you.
.ZLRANGE
The last line in a range specified by you.
Using commands in edit macros
Chapter 6. Creating edit macros  103

## Page 136

.ZDEST
The destination line specified by you.
Note: Unlike other labels, .ZCSR, .ZFIRST, and .ZLAST do not stay with the same line. Label .ZCSR stays
with the cursor, and labels .ZFIRST and .ZLAST point to the current first and last lines, respectively.
Using labels
In a macro, you can assign a label to a line by using the LABEL assignment statement. For example:
CLIST Statements REXX Statements
 
SET &LNUM = 10
ISREDIT LABEL &LNUM = .HERE
ADDRESS ISPEXEC
lnum = 10
'ISREDIT LABEL' lnum '= .HERE'
This assigns the label .HERE to the line whose relative line number is contained in variable LNUM (line 10
here). The .HERE label allows the macro to keep track of a line whose relative line number may change.
When the macro finishes running, the .HERE label is removed.
Labels can be used as part of a keyphrase instead of a line number. For example:
CLIST Statements REXX Statements
 
ISREDIT LINE .NEXT = (DATAVAR)
ISREDIT LINE_AFTER .XYZ = (DATAVAR)
ADDRESS ISPEXEC
'ISREDIT LINE .NEXT = (DATAVAR)'
'ISREDIT LINE_AFTER .XYZ = (DATAVAR)'
The first example stores new data into the line that currently has the label .NEXT. The second example
creates a new line after the line whose label is .XYZ, and stores data into the new line.
A macro can determine if a label exists. Using the LINENUM assignment statement, you can obtain the
current relative line number of a labeled line. If the label does not exist, the return code (&LASTCC for
CLIST or RC for REXX) is 8. For example:
CLIST Statements REXX Statements
 
ISREDIT (LNUM2) = LINENUM .ABC
IF &LASTCC = 8 THEN WRITE NO .ABC LABEL
ADDRESS ISPEXEC
'ISREDIT (LNUM2) = LINENUM .ABC'
IF RC = 8 THEN SAY 'No .ABC label'
This example stores the relative line number of the line with label .ABC into variable LNUM2 and tests to
see if that label did exist.
Labels have a variety of uses. For example, because both the FIND and SEEK commands position the
cursor at the search string after the macro has been started, you may want to assign the data from the line
on which the cursor is positioned to the variable CSRDATA. To do so, use this statement:
CLIST Statements REXX Statements
 
ISREDIT FIND 'IT'
ISREDIT (CSRDATA) = LINE .ZCSR
ADDRESS ISPEXEC
'ISREDIT FIND IT'
'ISREDIT (CSRDATA) = LINE .ZCSR'
The label .ZCSR names the line in which the cursor is positioned. The .ZCSR label is moved to a new line
when one of these commands moves the cursor: FIND, CHANGE, SEEK, EXCLUDE, TSPLIT or CURSOR.
The labels .ZFIRST and .ZLAST can also move when data is added or deleted.
Using commands in edit macros
104  z/OS: z/OS ISPF Edit and Edit Macros

## Page 137

If you assign a labeled line a new label that is blank, the previous label becomes unassigned (if both
labels are at the same level). For example:
CLIST Statement REXX Statements
 
ISREDIT LABEL .HERE = ' '
ADDRESS ISPEXEC
"ISREDIT LABEL .HERE = ' '"
removes the label from the line.
If a label in use is assigned to another line, the label is moved from the original line to the new line (if the
new assignment is at the same level as the original).
Referring to labels
A nested macro can refer to all labels assigned by higher-level macros and to labels that you assign.
When a macro assigns labels, they are associated by default with the assigning macro level. The labels are
automatically removed when the macro finishes running. The labels belong to the level at which they are
assigned and can have the same name as the labels at other levels without any conflict.
When a macro ends, the labels at the current nesting level are deleted. To set a label for the next
higher level, the macro can issue the MACRO_LEVEL assignment statement to obtain the current level and
decrease the level by 1.
A macro can determine the level of a label with the LABEL assignment statement, as shown in this syntax:
ISREDIT (varname1,varname2) = LABEL lptr
The label assigned to the referenced line is stored in the first variable and its level is stored in the second
variable. If a label is not assigned to the line, a blank is stored in both variables.
Passing labels
You can create a label at any level above its current level by explicitly stating the level:
ISREDIT LABEL lptr = label
level
Here, if the label previously existed at the explicitly specified level, its old definition is lost. A label
assigned at a higher level remains after the macro ends and is available until the level at which it was
assigned ends or the label is explicitly removed.
If a macro sets a label without indicating a level, or if its value is equal to or greater than the level at which
the macro is running, the label is set at the macro level that is currently in control and does not affect any
labels set in a higher level.
If a macro queries a label without specifying a level, or uses the label as a line pointer, the search for the
label starts at the current macro level and goes up, level by level, until the label defined closest to the
current level is found.
If you specify a level parameter that is outside the currently active levels, it is adjusted as follows: a value
less than zero is set to zero; a value greater than the current nesting level is set to the current nesting
level. This means that a higher-level macro cannot set a label at the level of the macro that it is going to
start.
Referring to data lines
You can refer to data lines either by a relative line number or by a symbolic label. Note that special lines
(MASK lines, TABS lines, COLS lines, BOUNDS lines, MSG lines, and others) are not considered data lines.
You cannot assign labels to them, and they do not have relative line numbers. Also, you cannot directly
Using commands in edit macros
Chapter 6. Creating edit macros  105

## Page 138

reference these lines in a macro, even though they are displayed. Excluded lines are regarded as data
lines.
Relative line numbers are not affected by sequence numbers in the data, nor are they affected by the
current setting of number mode. The first line of data is always treated as line number 1, the next line is
line number 2, and so on. The TOP OF DATA line is considered line number 0.
When you insert or delete lines, the lines that follow change relative line numbers. If you insert a new line
after line 3, for example, it becomes relative line 4 and what was relative line 4 becomes relative line 5,
and so on. Similarly, if line 7 is deleted, the line that was relative line 8 becomes relative line 7, and so on.
Referring to column positions
Column positions in edit macros are not the same as they appear on the panel; they refer only to the
editable portions of the data. When number mode is on, sequence numbers are not part of the data,
and thus are not editable. For example, if NUMBER COBOL ON mode is in effect, the first six positions
of each line contain the sequence number. The first data character is in position 7, which is considered
relative column 1. When number mode is off, the line number portion is editable, so here position 1
becomes column 1 and position 7 becomes column 7. These are not the column values displayed on the
edit panel. This discrepancy can influence the use of column numbers as parameters from the keyboard.
Column numbers must be converted according to number mode. See “Edit boundaries” on page 23 for the
conversions.
If your macro must access the sequence numbers as data, include statements that save the current
number mode, set number mode off, and then restore the original number mode.
When a macro retrieves the current cursor position, a relative column number of zero is returned if the
cursor is outside the data portion of the line. When a macro sets the cursor column to zero, the cursor is
placed in the Line Command field on the left side of the designated line.
Defining macros
You can use DEFINE to give macros names that are different from their data set names, make aliases
for built-in edit commands, identify macros as program macros, or set a command as disabled. DEFINE
commands are usually issued in an initial macro.
For more information, refer to the description of the DEFINE command in Chapter 11, “Edit macro
commands and assignment statements,” on page 295.
Defining an alias
To establish an alias or alternate name for a primary command, enter a DEFINE followed by the new
name, the ALIAS operand, and then the original command name. For example, this command:
DEFINE FILE ALIAS SAVE
establishes FILE as an alias for SAVE, allowing you to enter FILE to save the data currently being edited
instead of SAVE.
Resetting definitions
To reset the last definition for a command and return the command to its previous status, use the DEFINE
command with the RESET operand. For example, having established FILE as an alias for SAVE, you can
enter this command to cause FILE to be flagged as an invalid command:
DEFINE FILE RESET
When defining a command as DISABLED, you cannot reset the disabled function.
Using commands in edit macros
106  z/OS: z/OS ISPF Edit and Edit Macros

## Page 139

Replacing built-in commands
You also use DEFINE to replace an existing edit command with a macro. This links the command name to
an edit macro. For example:
CLIST Statement REXX Statements
 
ISREDIT DEFINE FIND ALIAS MYFIND
ADDRESS ISPEXEC
'ISREDIT DEFINE FIND ALIAS MYFIND'
To use the built-in edit command, precede the command with BUILTIN. For example, to process the
built-in FIND command, include this statement:
REXX Statements
 
ISREDIT BUILTIN FIND …
ADDRESS ISPEXEC
'ISREDIT BUILTIN FIND …'
The ellipses (…) represent other FIND command operands such as the search string.
Implicit definitions
When you or your macro issue a command unknown to the editor, PDF searches for a CLIST or REXX exec
with that name. If the editor finds the command, it is implicitly defines it as an edit macro.
Program macros can be implicitly defined by preceding the name of the macro with an exclamation point
(!). Remember that the name must be 7 characters or less, excluding the exclamation point. Program
macros are similar to ISPF dialogs in that they must be made available as load modules in either the
ISPLLIB, STEPLIB, or LINKLST library. See “Program macros” on page 89 for more information.
Using the PROCESS command and operand
The PROCESS command provides a way to alter the usual sequence of events in an edit macro. It
is related to the PROCESS operand on the MACRO command. PROCESS is the default for the MACRO
command. PROCESS specifies that display data and line commands be processed before another
statement is processed. If you specify NOPROCESS, the editor defers processing the panel data and
line commands until it finds an ISREDIT PROCESS command later in the macro, or until the macro ends.
You can use PROCESS to create a "before-and-after" effect. If you specify NOPROCESS at the beginning
of a macro, edited data appears without the changes made from the keyboard—creating a "before" effect.
Once you specify PROCESS, changes that were made from the keyboard appear—creating an "after"
effect.
The syntax of the ISREDIT MACRO statement is:
ISREDIT MACRO
(
variable
)
PROCESS
NOPROCESS
Using the PROCESS command with edit line macros
The PROCESS command is used within edit line macros to set the .ZFRANGE, .ZLRANGE, and .ZDEST
labels for use by the macro. For edit user line commands, you must specify NOPROCESS on the
MACRO statement and include a PROCESS statement within the macro. For macros run by your own line
commands, the PROCESS statement does not delay or control the execution of other line commands, as
the editor executes all the line commands in sequential order and executes any preceding line commands
prior to invoking the user line macro.
Using commands in edit macros
Chapter 6. Creating edit macros  107

## Page 140

Specifying NOPROCESS in the macro statement
NOPROCESS is useful if you want to process statements before the display data or line commands are
processed. It enables you to perform initial verification of parameters or capture lines before they are
changed from the panel.
It is also useful if you want to include an ISREDIT PROCESS command to specify whether the macro
expects, and handles, line commands that identify either a range of lines, a destination line, or both. This
linking is the method by which the editor allows a macro command to interact with line commands in the
same way that the built-in MOVE and REPLACE commands do. With the ISREDIT PROCESS command, the
editor can process line commands that you have entered, performing significant error and consistency
checking.
Specifying a destination
If you include this process statement in an edit macro:
CLIST Statement REXX Statements
 
ISREDIT PROCESS DEST
ADDRESS ISPEXEC
'ISREDIT PROCESS DEST'
the macro expects you to specify a destination line. A destination line is always specified using either
A (after) or B (before). The editor sets the dialog variable .ZDEST to the line preceding the destination.
However, if neither A nor B is specified, .ZDEST is set to the last data line. In this situation, a return code
shows that no destination was specified.
Specifying a range
If you use this syntax for a PROCESS macro command in an edit macro:
ISREDIT PROCESS RANGE operand
the macro expects to receive a specified range of lines to process. The operand following the RANGE
operand identifies either one or two commands that are to be accepted. For example, the command
PROCESS RANGE Q Z allows the line commands Q or Z (but not both) to be processed with this macro.
The line commands could take any of these forms:
• Q or Z, to specify a single line.
• QQ or ZZ, to specify a block of lines. This form is obtained by doubling the last letter of the single-line
command.
• Qn or Zn where n is a number that specifies a series of lines.
After the PROCESS command is completed, the dialog variable .ZFRANGE is automatically set to the first
line of the specified range. The dialog variable .ZLRANGE is set to the last line of the specified range.
These labels can refer to the same line. If no range is entered, the range defaults to the entire data set. In
this situation, a return code shows that no range was specified.
Two line command names can be specified for PROCESS In this situation, use the RANGE_CMD
assignment statement to return the value of the command entered. For example, if you issue this
PROCESS command:
CLIST Statement REXX Statements
 
ISREDIT PROCESS RANGE Z $
ADDRESS ISPEXEC
'ISREDIT PROCESS RANGE Z $'
The RANGE_CMD assignment statement returns either a Z or a $.
Using commands in edit macros
108  z/OS: z/OS ISPF Edit and Edit Macros

## Page 141

The names of line commands that define the range can be 1 to 6 characters, but if the name is 6
characters long, it cannot be used as a block format command by doubling the last character. The name
can contain any alphabetic or special character except blank, hyphen (-), apostrophe ('), or period (.). It
cannot contain any numeric characters.
Example
In the example that follows, the NOPROCESS operand on the MACRO command defers processing of the
panel data until the line with the cursor is assigned to a variable. After the PROCESS command, the line
contains any changes that you made.
CLIST Statements REXX Statements
 
ISREDIT MACRO NOPROCESS
ISREDIT (BEFORE) = LINE .ZCSR
ISREDIT PROCESS
ISREDIT (AFTER) = LINE .ZCSR
IF &STR(&BEFORE) = &STR(&AFTER) THEN -
     ...
ELSE -
     ...
ADDRESS ISPEXEC
'ISREDIT MACRO NOPROCESS'
'ISREDIT (BEFORE) = LINE .ZCSR'
'ISREDIT PROCESS'
'ISREDIT (AFTER) = LINE .ZCSR'
IF BEFORE = AFTER THEN
     ...
ELSE
     ...
See “PROCESS—Process Line Commands” on page 381.
Recovery macros
After a system failure, you might want to restore the command definitions and aliases that you were using
when the system failed, but you do not want to destroy the profile changes you made during the edit
session before the failure.
To help to recover after a system failure, you can provide a recovery macro which can restore command
definitions and aliases while not destroying profile changes made before the failure. The recovery macro,
like an initial macro, runs after the data has been read but before it is displayed. However, the macro is
run whenever the recovery data set is being edited.
You can specify a recovery macro:
• By entering the RMACRO primary command:
RMACRO name
• In your initial macro by using the RMACRO assignment statement:
ISREDIT RMACRO = name
where name sets the name of the macro for the edit session. The name operand is used to specify the
name of the macro to be run after a data set has been recovered.
Note: Recovery macros are only in effect for the duration of a particular Edit session. They must be
specified again each time a new member or data set is edited.
Return codes from user-written edit macros
A macro can issue the return codes shown here. These return codes affect the command line and cursor
position on the next display of edit data:
0
Shows normal completion of the macro. The cursor position is left as set by the macro. The command
line is blanked.
Return Codes from User-Written Edit Macros
Chapter 6. Creating edit macros  109

## Page 142

1
Shows normal completion of the macro. The cursor is placed on the command line and the line
is blanked. Use this return code to make it easy to enter another macro or edit command on the
command line.
4 and 8
Treated by the ISPF editor as return code 0. No special processing is done.
12 and higher
Error return codes. The cursor is placed on the command line and the macro command remains. When
used with these return codes, the dialog manager SETMSG service prompts you for an incorrect or
omitted parameter.
Any invocation of a disabled macro command issues a return code of 12. See the DEFINE command
for more information on disabled commands.
20 and higher
Indicate a severe error. The meanings of the severe return codes are:
20
Command syntax error or Dialog service routine error.
24
Macro nesting limit of 255 exceeded (possible endless loop; see the BUILTIN macro command).
28
Command found either preceding the ISREDIT MACRO command, or following the ISREDIT END
or ISREDIT CANCEL command.
Each command description in Chapter 11, “Edit macro commands and assignment statements,” on page
295 includes a list of return codes that are possible for the command. Because &LASTCC (CLIST) or RC
(REXX) is set for every statement, you must either test it in the statement immediately following the
command that sets it, or you must save its value in another variable. Use a command such as:
SET &RETCODE = &LASTCC
The variable (&RETCODE or RETCODE) can then be tested anywhere in the macro until it is changed.
Return codes from PDF edit macro commands
Every CLIST edit macro command sets variable &LASTCC with a return code. REXX edit macros set
variable RC. The return codes range from 0 to 20.
0
Shows normal completion of the command.
2, 4, and 8
Information return codes. They show a special condition that is not necessarily an error. These return
codes can be tested or ignored, depending on the requirements of the macro.
For some cases of RC=8, the ISPF system variables ZERRSM (short error message text) and ZERRLM
(long error message text) are set. For more information on ZERRSM and ZERRLM, see z/OS ISPF Dialog
Developer's Guide and Reference.
12 and higher
Error return codes. Normally an error return code causes the macro to end abnormally and an error
panel to appear. The error panel shows the kind of error and lists the statement that caused the error
condition.
The ISPF system variables ZERRSM (short error message text) and ZERRLM (long error message text)
are set for error return codes. For more information on ZERRSM and ZERRLM, see z/OS ISPF Dialog
Developer's Guide and Reference.
Often, the only two possible return codes are 0 and 20. The CAPS command is an example of such a
command. Any valid form of CAPS issues a return code of 0.
Return codes from PDF edit macro commands
110  z/OS: z/OS ISPF Edit and Edit Macros

## Page 143

The dialog variables ZEDMSGNO (message identifier), ZEDISMSG (short message text) and ZEDILMSG
(first 240 bytes of the long message text) are available to be tested for or displayed within edit macros.
These variables contain values relating to any message that would have been displayed at the terminal
had the user issued the command directly from the command line. They can be useful in situations where
the return code does not provide enough detail.
Selecting control for errors
As explained in “Return codes from PDF edit macro commands” on page 110, every edit macro statement
causes variable &LASTCC (CLIST) or RC (REXX) to be set to a return code. Return codes of 12 or higher are
considered errors (except for the PROCESS edit macro command return code of 12), and the default is to
end macros that issue those return codes.
Sometimes you need to handle errors at the time that they occur. The error is expected and the edit
macro logic can handle the problem. If you want to handle all errors that might occur in your macro, you
can include this statement:
ISPEXEC CONTROL ERRORS RETURN
If errors occur, control returns to the macro. On the other hand, to return error handling to the default
mode, include this statement:
ISPEXEC CONTROL ERRORS CANCEL
If an error occurs, the macro ends.
If you want to do both, you can include any number of ISPEXEC CONTROL statements in your macro to
turn error handling on and off.
Selecting control for errors
Chapter 6. Creating edit macros  111

## Page 144

Selecting control for errors
112  z/OS: z/OS ISPF Edit and Edit Macros
