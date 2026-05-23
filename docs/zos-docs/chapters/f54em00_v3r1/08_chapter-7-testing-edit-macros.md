# Chapter 7. Testing edit macros

Source file: f54em00_v3r1.md
Start page: 145
Page span: 145-150

## Page 145

Chapter 7. Testing edit macros
This chapter documents general-use programming interfaces and associated guidance information. It
also tells you how to include statements in your edit macros to capture and handle error conditions.
Using the information in the preceding chapters , you should be able to write and run an edit macro
that uses CLIST or REXX logic and processes simple edit commands. However, even an experienced edit
macro writer occasionally includes a bug that causes a macro to end abnormally (ABEND), or writes a
macro that does not work as expected. When this occurs, you must debug your macro, just as you would
debug any other kind of program you write.
Handling errors
There are two kinds of errors that you may encounter when you debug macros: edit command errors
and dialog service errors. Both kinds of errors are controlled by the ISPEXEC CONTROL ERRORS RETURN
command. For more information about the CONTROL service, refer to z/OS ISPF Services Guide.
Edit command errors
The editor detects edit command errors and displays either an edit macro error panel with an error
message, or a return code. If an edit command error occurs, the macro ends abnormally with these
results:
• When you are using the ISPF editor with ISPF test mode off, you return to the edit session.
• If ISPF test mode is on, the PDF component is also in test mode. You can override the abnormal
end and attempt to continue by typing YES on the PDF edit macro error panel and pressing Enter. If
ISPEXEC CONTROL ERRORS RETURN has been processed, the error panel does not appear, and the
macro automatically continues.
Dialog service errors
ISPF detects dialog service errors and displays a message identifying the error with the statement which
caused the error. If a dialog service error occurs, the edit session ends abnormally with these results:
• When you are using the PDF component with ISPF test mode off, the ISPF Primary Option Menu is
displayed.
• If you are using the PDF component with ISPF test mode on, you can override the abnormal end and
attempt to continue by typing YES on the ISPF dialog error panel and pressing Enter. In either case,
if ISPEXEC CONTROL ERRORS RETURN has been processed, no panel appears and the editor sends a
return code instead of ending the dialog.
Note: If you enter ISPF with TEST as an operand, or use Dialog Test (option 7), ISPF remains in test mode
until you end the ISPF session.
Using CLIST WRITE statements and REXX SAY statements
The CLIST WRITE statement and the REXX SAY statement can be valuable tools in tracking down edit
macro problems. A WRITE statement or a SAY statement is simply a line of text inserted into your macro
that creates a message on your screen while the macro is running. With these statements, you can
identify the position of the statement within the macro, and display the value of variables.
For example, if you are having trouble debugging the CLIST ISRTDATA macro from Figure 27 on page 81,
adding some WRITE statements may help locate the problem. Sample macro ISRTDWRI is the same as
ISRTDATA, with CLIST WRITE statements added.
Handling errors
© Copyright IBM Corp. 1984, 2024 113

## Page 146

/*********************************************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRTDWRI - generates test data                                    */
/*                                                                   */
/*********************************************************************/
ISREDIT MACRO
  SET &COUNT = 1                    /* Initialize loop counter    */
  DO WHILE &COUNT <= 9              /* Loop up to 9 times         */
    ISREDIT FIND 'TEST-#'           /* Search for 'TEST-#'        */
    SET &RETCODE = &LASTCC          /* Save the FIND return code  */
    WRITE RESULT OF FIND, RC = &RETCODE
    IF &RETCODE = 0 THEN            /* If string was found,       */ -
      DO                            /*                            */
        ISREDIT CHANGE '#' '&COUNT' /*   Change # to a digit and  */
        SET &COUNT = &COUNT + 1     /*   increment loop counter   */
        WRITE COUNT IS NOW UP TO &COUNT
      END                           /*                            */
    ELSE                            /* If string is not found,    */ -
      SET &COUNT = 10               /*   Set counter to exit loop */
  END
EXIT CODE (0)
Figure 36. ISRTDWRI macro
Remember that the macro ISRTDATA creates test data with variations of the same line by putting
ascending numbers 1 through 9 in the data. When WRITE statements are included in the data, a step-by-
step breakdown of the procedure appears on your screen.
If there are no errors in the ISRTDWRI macro, the return codes and count appear on your screen in TSO
line mode. Asterisks at the bottom of the screen prompt you to press Enter and return to ISPF full-screen
mode (Figure 37 on page 114).
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 2
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 3
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 4
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 5
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 6
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 7
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 8
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 9
 RESULT OF FIND, RC = 0
 COUNT IS NOW UP TO 10
  ***_
Figure 37. Results of ISRTDWRI macro
Using CLIST CONTROL and REXX TRACE statements
You can display a statement from a macro as it is being interpreted and run. Use either of these:
• A CLIST CONTROL statement with the LIST, SYMLIST, or CONLIST operand
• A REXX TRACE statement with the A, I, L, O, R, or S operand
These statements produce messages on your display screen similar to the WRITE and SAY statements
discussed in the previous section. However, several differences should be noted:
• For the CLIST CONTROL statement:
Using CLIST CONTROL and REXX TRACE statements
114  z/OS: z/OS ISPF Edit and Edit Macros

## Page 147

– LIST displays commands and subcommands (including ISREDIT statements) after substitution but
before processing. This allows you to see an ISREDIT statement in the form that the editor sees the
statement.
– CONLIST displays a CLIST statement (for example, IF, DO, SET) after substitution but before
processing. You might be able to tell why an IF statement did not work properly by using CONLIST.
– SYMLIST displays both CLIST and command lines before symbolic substitution, allowing you to see
the lines as written.
Use the NOLIST, NOSYMLIST, and NOCONLIST operands to prevent the display of statements. See z/OS
TSO/E CLISTs for more details.
• For the REXX TRACE statement:
– The A operand traces all clauses displaying the results of each clause.
– The I operand traces the intermediate results, displaying both the statement and the results.
– The L operand traces labels in your edit macro.
– The O operand stops, or turns off, the trace.
– The R operand, which is used most often, traces all clauses and expressions.
– The S operand scans each statement, displaying it without processing it.
See z/OS TSO/E REXX Reference and z/OS TSO/E REXX User's Guide for more details.
Experimenting with macro commands
Use the ISRTRYIT macro (Figure 38 on page 115) to experiment with edit macros. ISRTRYIT is handy
when you want to see how a command or assignment statement works but do not actually want to
write an entire macro. ISRTRYIT processes the command and issues return codes that show whether it
succeeded. To start the macro, type ISRTRYIT on the command line, followed by a command, and press
Enter. If you enter ISRTRYIT with the RESET operand, the variable &COMMAND is set to RESET; if you
enter it as ISRTRYIT FIND A, the variable &COMMAND is set to FIND A.
/*********************************************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRTRYIT - a simple macro for trying out edit macro statements.   */
/*                                                                   */
/*********************************************************************/
ISREDIT MACRO (COMMAND)
  SET &RETCODE = 0                     /* Initialize return code  */
  IF &STR() = &STR(&COMMAND) THEN      /* If no command specified */ -
    WRITE MISSING COMMAND PARAMETER    /*   indicate problem      */
  ELSE DO                              /* Else parameter exists;  */
    ISREDIT &COMMAND                   /* Invoke edit command,    */
    SET &RETCODE = &LASTCC             /*   save the return code  */
    WRITE &COMMAND RETURN CODE IS &RETCODE  /*   and display it   */
  END                                  /*   and the command name  */
EXIT CODE(&RETCODE)
Figure 38. ISRTRYIT macro
The ISRTRYIT macro tests both the SEEK and AUTONUM commands (Figure 39 on page 116). When you
run the macro, it displays the return codes from the commands on your screen (Figure 40 on page 116).
Experimenting with macro commands
Chapter 7. Testing edit macros  115

## Page 148

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.DATA(TESTDATA) - 01.00           Columns 00001 00072
 Command ===> isrtryit seek "test"; isrtryit autonum on        Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000100 TEST-#
 000200 TEST-#
 000300 TEST-#
 000400 TEST-#
 000500 TEST-#
 000600 TEST-#
 000700 TEST-#
 000800 TEST-#
 000900 TEST-#
 001000 TEST-#
 001100 TEST-#
 001200 TEST-#
 001300 TEST-#
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 39. ISRTRYIT macro - before running
 ISREDIT SEEK "TEST"
 RETURN CODE IS 0
 ISREDIT AUTONUM ON
 RETURN CODE IS 0
  ***_
Figure 40. ISRTRYIT macro - after running
Debugging edit macros with ISREMSPY
When you run an edit macro, the editor screen is not displayed until the macro completes. To view the
status of the data being edited during execution of the edit macro, invoke the program ISREMSPY from
within the running macro.
ISREMSPY displays a simulated editor panel in which the data is presented as it exists at the time
ISREMSPY is started. You can also see the cursor location and the last edit macro command executed. In
most cases, the line that has the cursor on it is indicated by an arrow in the line command field.
Within an ISREMSPY display you can issue the commands RESET and FIND. RESET restores the display to
the current editor state, including scroll and cursor location. FIND locates a string within the data being
display.
FIND does not support all the operands of the FIND command of the real editor; it only supports the
search string as an operand. The string may be in quotes, and embedded quotes should not be doubled.
Pressing the RFIND key will repeat the last search. Only the first 256 bytes of each line are searched by
the FIND command.
Because ISREMSPY is a simulated edit session, it may not display precisely as the editor would. For
example, the numbers in the line command field are always incremented by one, and may not accurately
reflect the numbers displayed in the real edit session. Similarly, there are some cases such as TENTER
and INSERT, where the cursor location may not be correct.
ISREMSPY can be invoked in several ways:
• You can invoke it as a TSO command directly from within an edit macro.
CLIST example:
ISREMSPY
REXX example:
Experimenting with macro commands
116  z/OS: z/OS ISPF Edit and Edit Macros

## Page 149

Address TSO 'ISREMSPY'
• You can define a breakpoint for ISREDIT in dialog test (option 7.8) and then run the macro under dialog
test (option 7.1). When the breakpoint is triggered, you can type TSO ISREMSPY to view the current
state of the edit data. This technique can be used to look at edit data during execution of a macro
without having to modify the edit macro source and is particularly useful for debugging program macros
(macros not written in CLIST or REXX).
• You can define ISREMSPY as a program macro using the editor DEFINE command and then use
ISREMSPY as an editor command.
Experimenting with macro commands
Chapter 7. Testing edit macros  117

## Page 150

Experimenting with macro commands
118  z/OS: z/OS ISPF Edit and Edit Macros
