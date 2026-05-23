# Chapter 5. Using edit macros

Source file: f54em00_v3r1.md
Start page: 111
Page span: 111-118

## Page 111

Chapter 5. Using edit macros
This topic documents general-use programming interfaces and associated guidance information. It also
describes edit macros and describes several examples of their use.
What are edit macros?
You can use edit macros, which look like ordinary editor commands, to extend and customize the editor.
You create an edit macro by placing a series of commands into a data set or member of a partitioned data
set. Then you can run those commands as a single macro by typing the defined name in the command line
or, if you have specified a user line command table to the editor, by entering a user line command in the
line command field of one or more lines of the data set.
Edit macros can be either CLISTs or REXX EXECs written in the CLIST or REXX command language,
or program macros written in a programming language (such as FORTRAN, PL/I, or COBOL). This
documentation uses the CLIST command language for most of its examples, with a few examples in
REXX. Examples of program macros are in “Program macros” on page 89.
Edit macros can also contain edit assignment statements that communicate between a macro and the
editor. These statements are made up of two parts, keyphrases and values, that are separated by an equal
sign. Edit assignment statements are described in “Edit assignment statements” on page 96.
Edit macros have access to the dialog manager and system services. Because edit macros are CLISTs, or
REXX EXECs, programs, they have unlimited possibilities.
Note: All edit macros must have an ISREDIT MACRO statement as the first edit command. For more
information see “Syntax” on page 364.
You can use edit macros to:
• Perform repeated tasks
• Simplify complex tasks
• Pass parameters
• Retrieve and return information
The remainder of this topic presents examples of these tasks.
Note: To run an edit macro against all members of a PDS you can use a program containing a loop that
uses a LMMLIST service to obtain the names of PDS members. For each member issue an ISPEXEC edit
command with the initial macro keyword. For an example, see Figure 47 on page 125.
Performing repeated tasks
You can use an edit macro to save keystrokes when you frequently perform a task. A simple example
would be using a macro to delete every line that begins with a dash (-) in column 1. You could scan the
data and manually delete each line, or you could write a macro that does the same thing much faster. The
edit macro in Figure 24 on page 80 processes the commands necessary to delete the lines and requires
only that you enter the ISRDASH macro.
What are edit macros?
© Copyright IBM Corp. 1984, 2024 79

## Page 112

/*********************************************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRDASH  Delete lines with a '-' in column 1                      */
/*          except the first '-'                                     */
/*                                                                   */
/*********************************************************************/
ISREDIT MACRO
  ISREDIT RESET EXCLUDED        /* Ensure no lines are excluded  */
  ISREDIT EXCLUDE ALL '-' 1     /* Exclude lines with '-' in col1*/
  ISREDIT FIND FIRST '-' 1      /* Show the first such line      */
  ISREDIT DELETE ALL EXCLUDED   /* Delete all lines left excluded*/
EXIT CODE (0)
Figure 24. ISRDASH macro
When you run this macro, it deletes all lines beginning with a dash, except the first one. To run the macro,
type isrdash on the command line (Figure 25 on page 80). The dash macro deletes all lines that began
with a dash except the first one (Figure 26 on page 81).
Figure 25. ISRDASH macro - before running
What are edit macros?
80  z/OS: z/OS ISPF Edit and Edit Macros

## Page 113

Figure 26. ISRDASH macro - after running
Simplifying complex tasks
If you need to perform an involved task, you can include logic in your edit macro. For instance, the
ISRTDATA macro shown in Figure 27 on page 81 creates variations of the same line by first finding
the succeeding test string number, and then changing each occurrence, using ascending numbers one
through nine.
/*********************************************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRTDATA generates test data                                      */
/*                                                                   */
/*********************************************************************/
ISREDIT MACRO
  SET &COUNT = 1                      /* Start loop counter         */
  DO WHILE &COUNT <= 9                /* Loop up to 9 times         */
    ISREDIT FIND 'TEST - # '          /* Search for 'TEST-#'        */
    SET &RETCODE = &LASTCC            /* Save the FIND return code  */
    IF &RETCODE = 0 THEN              /*                            */ -
      DO                              /* If the string is found,    */
        ISREDIT CHANGE '#' '&COUNT'   /* change '#' to the value    */
        SET &COUNT = &COUNT + 1       /* of '&COUNT', increment     */
      END                             /* the counter by one, and    */
    ELSE                              /* continue the loop.         */ -
      SET &COUNT = 10                 /* If the string is not       */
  END                                 /* found, set the counter to  */
EXIT CODE (0)                         /* exit the loop.             */
Figure 27. ISRTDATA macro
To run the test macro, type isrtdata on the command line (Figure 28 on page 82). The macro numbers
the first nine lines of data (Figure 29 on page 82).
What are edit macros?
Chapter 5. Using edit macros  81

## Page 114

Figure 28. ISRTDATA macro - before running
Figure 29. ISRTDATA macro - after running
Passing parameters, and retrieving and returning information
You can also write macros to get information from other users and from the editor, and to display
messages to other users. The ISRCOUNT macro, as shown in Figure 30 on page 83, finds occurrences of
the string TEST from the previous example, counts them, and prepares a return message.
What are edit macros?
82  z/OS: z/OS ISPF Edit and Edit Macros

## Page 115

/*********************************************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRCOUNT counts the number of occurrences of a string, and        */
/*          returns a message.                                       */
/*                                                                   */
/*********************************************************************/
ISREDIT MACRO (PARMSTR)
  ISREDIT SEEK ALL &PARMSTR
  IF &LASTCC > 12 THEN DO
    SET &ZEDSMSG = &STR(SEEK ERROR )
    SET &ZEDLMSG = &STR(STRING NOT FOUND )
  END
  ELSE DO
    ISREDIT (COUNT) = SEEK_COUNTS
    SET &COUNT = &COUNT
    SET &ZEDSMSG = &STR("&PARMSTR" FOUND &COUNT TIMES)
    SET &ZEDLMSG = &STR(THE STRING "&PARMSTR " WAS FOUND +
                     &COUNT TIMES.)
  END
  ISPEXEC SETMSG MSG(ISRZ000)
EXIT CODE (0)
Figure 30. ISRCOUNT macro
To run the ISRCOUNT macro, type isrcount TEST on the command line (Figure 31 on page 83). The
macro does not change the data but displays return messages to show the number of times it found the
string. The editor always displays the short message in the upper right corner of the screen. Enter HELP
(the default is F1) to produce the long message (Figure 32 on page 84).
Figure 31. ISRCOUNT macro - before running
What are edit macros?
Chapter 5. Using edit macros  83

## Page 116

Figure 32. ISRCOUNT macro - after running
Working with an edit line command table
You can create an edit line command table to store your own line commands. Each line command is
associated with a macro that you want to run when you enter the specified line command. The associated
macro uses the PROCESS macro statement to determine the lines the command applies to and the
destination to be used by the macro.
You can edit an existing line command table to add, delete, or modify your line commands.
When you invoke an Edit or View session, you can specify the name of the edit line command table to be
used for that session. For information on specifying the name of the edit line command table on the EDIF,
EDIT, VIEW, or VIIF service, refer to the topics describing those services in z/OS ISPF Services Guide. For
information on specifying the name of the edit line command table on the View Entry Panel or on the Edit
Entry Panel, refer to topics "View (option 1) " and "Edit (option 2) " in z/OS ISPF User's Guide Vol II.
For each line command you add to the table, you specify:
• The name of the line command you want to add.
• The name of the associated edit macro.
• If it supports a multiple line format. That is, if a numeric suffix can be included on the command to
indicate the number of lines that the command applies to.
• If it is a block format. That is, if the command applies to a block of lines.
• If it requires a destination line command as well.
If you have specified an edit line command table to be used in an Edit or View session, when you enter a
line command that is in the edit line command table, ISPF invokes the associated macro.
To create a new edit line command table or to edit an existing edit line command table, in the table editor
(3.16):
1. Specify the name of the table in the Table Name field.
2. Select the Table is an EDIT line command table option.
3. Type "E" on the command line and press Enter.
For a new table, ISPF displays the entry fields for the first line command.
What are edit macros?
84  z/OS: z/OS ISPF Edit and Edit Macros

## Page 117

a. In the User Command field, type the name of the line command you want to create. This can be a 1
to 6 character value. The name must not conflict with any of the ISPF edit internal line commands.
b. In the MACRO field, type the name of the program, REXX, or CLIST edit macro that you want to
execute when the specified edit line command is entered. The macro can VGET variable ZLMACENT
to obtain the edit line command that was entered to run the macro, excluding any suffixes that were
entered to indicate a block command. For more information about edit line command tables, see
Line command table support under ISPF table utility (option 3.16) in z/OS ISPF User's Guide Vol II.
c. In the Program Macro field, type one of these values:
Y
The macro is a program.
N
The specified macro is CLIST or REXX.
d. In the Block Format field, type one of these values:
Y
The macro permits a block format for the line command by you repeating the last character of
the line command. This is not possible if the line character is 6 characters long.
N
The macro does not permit a block format.
e. In the Multi line field, type one of these values:
Y
The macro allows a multiple line format where you can indicate a range of lines by providing a
numeric suffix on the line command.
N
The macro does not allow a multiple line format.
f. In the Dest Used field, type one of these values:
Y
The line command requires a destination line command as well.
N
The line command does not use a destination line command.
The value Y causes a return code of 8 to be returned by the PROCESS DEST macro command when
a destination line command is not specified. See “PROCESS—Process Line Commands” on page
381 for more information.
For an existing table, ISPF displays each line command and its associated fields on a separate line.
a. You can modify the details for existing line commands in the table, or add or delete lines by
entering any of these commands in the table row selection field:
I
Insert one or more rows after the row where the command was entered.
B
Insert one or more rows before the row where the command was entered.
R
Repeat a row by creating one or more copies of the row where the command was entered.
D
Delete one or more rows.
Note: An optional number from 1 to 9 can be added as a suffix to each of these command
characters to cause processing against multiple rows.
4. Press PF3 to save the new or updated table.
What are edit macros?
Chapter 5. Using edit macros  85

## Page 118

What are edit macros?
86  z/OS: z/OS ISPF Edit and Edit Macros
