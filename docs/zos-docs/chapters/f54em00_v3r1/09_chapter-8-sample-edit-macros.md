# Chapter 8. Sample edit macros

Source file: f54em00_v3r1.md
Start page: 151
Page span: 151-166

## Page 151

Chapter 8. Sample edit macros
This chapter documents general-use programming interfaces and associated guidance information.
ISRBOX macro
The ISRBOX macro draws a box with its upper left corner at the cursor position. This macro comes in
handy when you want to make a note to yourself or others reading the data. You can start the ISRBOX
macro in one of two ways:
• Type ISRBOX on the command line as an edit primary command and press Enter.
• Type KEYS on the command line, press Enter, set a function key to the ISRBOX macro, and enter the
END command.
If you have defined a function key for ISRBOX, position the cursor on a data line where you want the box
drawn. Press the function key that you have defined to start the ISRBOX macro. After the box is drawn,
the cursor is positioned inside, ready for you to type enough text to fill the box.
If any of the macro commands fail, a warning message appears.
/*********************************************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRBOX - Draw a box with its upper left corner at the             */
/*          cursor position                                          */
/*                                                                   */
/*********************************************************************/
ISREDIT MACRO
ISREDIT (ROW,COL) = CURSOR             /* Get cursor position     */
ISPEXEC CONTROL ERRORS RETURN          /* No macro error panel    */
                                       /* Draw box over existing  */
                                       /*    lines                */
ISREDIT LINE &ROW          = LINE + < &COL '+--------------------+'>
ISREDIT LINE &EVAL(&ROW+1) = LINE + < &COL '|                    |'>
ISREDIT LINE &EVAL(&ROW+2) = LINE + < &COL '|                    |'>
ISREDIT LINE &EVAL(&ROW+3) = LINE + < &COL '|                    |'>
ISREDIT LINE &EVAL(&ROW+4) = LINE + < &COL '|                    |'>
ISREDIT LINE &EVAL(&ROW+5) = LINE + < &COL '+--------------------+'>
IF &MAXCC > 0 THEN DO                  /* If error occurred while */
   SET ZEDSMSG = &STR(INCOMPLETE BOX)  /*    overlaying lines     */
   SET ZEDLMSG = &STR(NOT ENOUGH LINES/COLUMNS TO DRAW COMPLETE BOX)
   ISPEXEC SETMSG MSG(ISRZ001)         /* Issue error message     */
END
SET &COL = &COL + 2                    /* Position cursor within  */
SET &ROW = &ROW + 1                    /*    the box              */
ISREDIT CURSOR = (ROW,COL)
EXIT CODE(0)
Figure 41. ISRBOX macro
This list explains the logical sections of the ISRBOX macro:
1. The variables &ROW and &COL are set to the cursor position.
ISREDIT (ROW,COL) = CURSOR
2. The dialog service allows the macro to handle severe errors, allowing a message to be displayed when
the cursor is placed too close to the end of the data. The LINE assignment statement fails if the row it
is setting does not exist.
ISRBOX macro
© Copyright IBM Corp. 1984, 2024 119

## Page 152

ISREDIT CONTROL ERRORS RETURN
3. The LINE assignment statements overlay existing data on a line with the characters which form a box.
LINE uses a merge format to include the existing line data and then a template to put the overlaying
data at the cursor column position. The CLIST &EVAL function increments the relative line numbers
before the statement is passed to the editor.
ISREDIT LINE &ROW          = LINE + < &COL '+----------------+'>
ISREDIT LINE &EVAL(&ROW+1) = LINE + < &COL '|                |'>
ISREDIT LINE &EVAL(&ROW+2) = LINE + < &COL '|                |'>
ISREDIT LINE &EVAL(&ROW+3) = LINE + < &COL '|                |'>
ISREDIT LINE &EVAL(&ROW+4) = LINE + < &COL '|                |'>
ISREDIT LINE &EVAL(&ROW+5) = LINE + < &COL '+----------------+'>
4. The CLIST IF statement checks the &MAXCC variable, and if it is nonzero, calls the dialog service
SETMSG to display a message. &MAXCC is a variable updated by the CLIST processor to contain the
highest condition code.
IF &MAXCC > 0 THEN
5. The message used in SETMSG is one of two messages (ISRZ000 and ISRZ001) reserved for macro use.
Each message uses two variables:
• &ZEDSMSG to set the text for the short message (up to 24 characters) that is displayed when the
macro ends.
• &ZEDLMSG to set the text for the long message that appears when the HELP command is entered.
Message ISRZ001 sounds the alarm to indicate an error; message ISRZ000 does not sound the alarm.
  DO
    SET ZEDSMSG = &STR(INCOMPLETE BOX)
    SET ZEDLMSG = &STR(NOT ENOUGH LINES/COLUMNS +
    TO DRAW COMPLETE BOX)
    ISPEXEC SETMSG MSG(ISRZ001)
  END
6. These statements position the cursor within the box to simplify entering text when the panel is
redisplayed.
SET &COL = &COL + 2
SET &ROW = &ROW + 1
ISREDIT CURSOR = (ROW,COL)
This example shows the cursor placed on line 000009 next to the number 9 before starting the macro. 
ISRBOX macro
120  z/OS: z/OS ISPF Edit and Edit Macros

## Page 153

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.DATA(TESTDATA) - 01.00           Columns 00001 00072
 Command ===> isrbox                                           Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000100 TEST-1
 000200 TEST-2
 000300 TEST-3
 000400 TEST-4
 000500 TEST-5
 000600 TEST-6
 000700 TEST-7
 000800 TEST-8
 000900 TEST-9_
 001000 TEST-#
 001100 TEST-#
 001200 TEST-#
 001300 TEST-#
 001400 TEST-#
 001500 TEST-#
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 42. ISRBOX macro - before running
When you press Enter, a box appears beside the cursor. 
   File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       SBURNF.PRIVATE.DATA(TESTDATA) - 01.00           Columns 00001 00072
 Command ===>                                                  Scroll ===> CSR 
 ****** ***************************** Top of Data ******************************
 000100 TEST-1
 000200 TEST-2
 000300 TEST-3
 000400 TEST-4
 000500 TEST-5
 000600 TEST-6
 000700 TEST-7
 000800 TEST-8
 000900 TEST-9+--------------------+
 001000 TEST-#| _                  |
 001100 TEST-#|                    |
 001200 TEST-#|                    |
 001300 TEST-#|                    |
 001400 TEST-#+--------------------+
 001500 TEST-#
 ****** **************************** Bottom of Data ****************************
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 43. ISRBOX macro - after running
ISRIMBED macro
The ISRIMBED macro (Figure 44 on page 122) builds a list of imbed (.im) statements found in the
member that is entered as an operand. The list is created at the end of the member currently being
edited. The imbed statements are indented under a MEMBER identifier line.
You can start this macro by editing a member, typing ISRIMBED and the name of the member that
contains the imbed statements as the operand, and pressing Enter.
ISRIMBED macro
Chapter 8. Sample edit macros  121

## Page 154

/*********************************************************************/
/*                                                                   */
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */
/*                                                                   */
/* ISRIMBED - creates a list of imbed statements.                    */
/*                                                                   */
/*********************************************************************/
ISREDIT MACRO (MEMBER)                   /* Member name passed  */
                                         /*   as input          */
ISREDIT LINE_AFTER .ZL='MEMBER &MEMBER'  /* Add member ID line  */
ISREDIT (LINENBR) = LINENUM .ZL          /* Get line number     */
ISREDIT COPY AFTER .ZL &MEMBER           /* Copy member at end  */
ISREDIT (NEWLL) = LINENUM .ZL            /* Get new last line#  */
IF &LINENBR = &NEWLL THEN                /* If no data was      */ -
   EXIT CODE(8)                          /*   copied, then exit */
ELSE DO
   ISREDIT LABEL &EVAL(&LINENBR + 1) = .FIRST  /* Label first   */
                                         /*   line copied       */
   ISREDIT RESET EXCLUDED                /* Make sure there are */
                                         /*   no previously     */
                                         /*   excluded lines    */
   ISREDIT EXCLUDE ALL .FIRST .ZL        /* Exclude newly       */
                                         /*   copied lines      */
   ISREDIT FIND ALL .IM 1 .FIRST .ZL     /* Show lines          */
   SET FINDRC = &LASTCC                  /*   containing ".im"  */
                                         /*   in column 1       */
   ISREDIT DELETE ALL X .FIRST .ZL       /* Delete any lines    */
                                         /*   still excluded    */
   ISREDIT (NEWLL) = LINENUM .ZL         /* Update last line    */
                                         /* number after delete */
   IF &FINDRC = 0 THEN                   /* If ".im" was found  */ -
     DO WHILE (&LINENBR < &NEWLL)        /*   for all remaining */
                                         /*   copied lines      */
       SET LINENBR = &LINENBR + 1        /*   Shift all .im     */
       ISREDIT SHIFT &LINENBR ) 8        /*   lines right 8     */
     END
END
EXIT CODE(1)                             /* Place cursor on     */
                                         /*   command line      */
Figure 44. ISRIMBED macro
This list explains the logical sections of the ISRIMBED macro:
1. Add a line that identifies the member to be searched at the end of ISRIMBED. The .ZL (or .ZLAST) is
always associated with the last line in the data.
ISREDIT LINE_AFTER .ZL = 'MEMBER &MEMBER'
2. Retrieve the line number of the identifier line just added into &LINENBR.
ISREDIT (LINENBR) = LINENUM .ZL
3. Now copy, at the end of ISRIMBED, the member name that was passed as an input parameter.
ISREDIT COPY AFTER .ZL &MEMBER
4. &NEWLL is set to the new last line number of ISRIMBED.
ISREDIT (NEWLL) = LINENUM .ZL
5. Check to see if any lines were added by the copy. Exit from the macro if no lines were added.
IF &LINENBR = &NEWLL THEN
    EXIT CODE(8)
6. Set the .FIRST label on the first line copied. This label is available only to this macro; you do not see
it.
ISRIMBED macro
122  z/OS: z/OS ISPF Edit and Edit Macros

## Page 155

ISREDIT LABEL &EVAL(&LINENBR + 1) = .FIRST
7. Excluded lines are deleted later. Therefore, make sure that no lines in the data set are excluded.
ISREDIT RESET EXCLUDED
8. Exclude all lines that were just copied: all the lines in the range .FIRST to .ZL.
ISREDIT EXCLUDE ALL .FIRST .ZL
9. The FIND command is used to find all occurrences of .im starting in column 1 of the copied lines. This
shows (unexcludes) the lines to keep. If .im was not found on any line, &FINDRC will be 4.
ISREDIT FIND ALL .IM 1 .FIRST .ZL
SET FINDRC = &LASTCC
10. All the lines still excluded are now deleted.
ISREDIT DELETE ALL X .FIRST .ZL
11. Obtain the last line number again, because it will have changed if lines were deleted.
ISREDIT (NEWLL) = LINENUM .ZL
12. If .im lines were found, loop using a column shift to indent them under the member identifier line.
Note that &LINENBR is still associated with the identifier line.
IF &FINDRC = 0 THEN
   DO WHILE (&LINENBR < &NEWLL)
      SET LINENBR = &LINENBR + 1
      ISREDIT SHIFT &LINENBR ) 8
           END
LIST is a member with several imbed statements. 
Figure 45. LIST with imbed statements
When you run the ISRIMBED macro by typing ISRIMBED LIST on the command line of ISRTDATA, a list
of the imbeds in LIST appears at the end of the data. See Figure 46 on page 124. 
ISRIMBED macro
Chapter 8. Sample edit macros  123

## Page 156

Figure 46. ISRIMBED macro - after running
ISRMBRS macro
The ISRMBRS macro (Figure 47 on page 125) uses PDF library access services to determine each
member name in the library being edited.
This macro invokes the edit service for each member in the library, except the member currently being
edited, passing a user-specified edit macro on the edit service invocation. The ISRMBRS macname
command, where macname is the name of the macro to be invoked against each member, starts the
service.
This macro can aid in making repetitive changes to all members of a data set, or in searching all members
for a specific string of data.
ISRMBRS macro
124  z/OS: z/OS ISPF Edit and Edit Macros

## Page 157

/*REXX****************************************************************/
/*   ISPF edit macro to process all members of partitioned data set, */
/*   running a second, user-specified, ISPF edit macro against each  */
/*   member.                                                         */
/*                                                                   */
/*   To run:                                                         */
/*    Enter "ISRMBRS macname" on the command line, where macname is  */
/*    the macro you want run against each member.                    */
/*********************************************************************/
'ISREDIT MACRO (NESTMAC)'
/*********************************************************************/
/* Get dataid for data set and issue LMOPEN                          */
/*********************************************************************/
'ISREDIT (DATA1) = DATAID'
'ISREDIT (CURMEM) = MEMBER'
Address ispexec 'LMOPEN DATAID('data1') OPTION(INPUT)'
member = ' '
lmrc = 0
/*********************************************************************/
/* Loop through all members in the PDS, issuing the EDIT service for */
/* each.  The macro specified on the ALLMEMS invocation is passed as */
/* an initial macro on the EDIT service call.                        */
/*********************************************************************/
Do While lmrc = 0
  Address ispexec 'LMMLIST DATAID('data1') OPTION(LIST),
                  MEMBER(MEMBER) STATS(NO)'
  lmrc = rc
  If lmrc = 0 & member ^= curmem Then
    do
      Say 'Processing member' member
      Address ispexec 'EDIT DATAID('data1') MEMBER('member')
                      MACRO('nestmac')'
    end
End
/*********************************************************************/
/* Free the member list and close the dataid for the PDS.            */
/*********************************************************************/
Address ispexec 'LMMLIST DATAID('data1') OPTION(FREE)'
Address ispexec 'LMCLOSE DATAID('data1')'
Exit 0
 
Figure 47. ISRMBRS macro
To start the ISRMBRS macro, edit a new or existing member and enter ISRMBRS macname, where
macname is the name of the macro you wish to invoke against each member of the data set. For example,
if the macro is named ISRIMBED, enter: ISRMBRS ISRIMBED
This list explains the logical sections of the ISRMBRS macro:
1. The MACRO command identifies NESTMAC as the variable to contain the name of the macro that
is passed on the edit service invocation for each member. If no parameter is passed to ISRMBRS,
NESTMAC is blank.
     ISREDIT MACRO (NESTMAC)
2. The DATAID assignment statement returns a data ID in the variable DATA1. The data ID identifies the
concatenation of data sets currently being edited.
     ISREDIT (DATA1) = DATAID
3. The name of the member currently being edited is returned in CURMEM.
     ISREDIT (MEMBER) = CURMEM
ISRMBRS macro
Chapter 8. Sample edit macros  125

## Page 158

4. The data set (or sets) identified by the data ID obtained earlier is opened for input to allow the
LMMLIST service to be called later. No return code checking is done because it is presumed that if the
data set is being edited, it can be successfully processed by LMOPEN.
 Address ispexec 'LMOPEN DATAID('data1') OPTION(INPUT)'
5. The variable to hold the name of the next member to be processed, and the return code from the
LMMLIST service are initialized.
     member = ' '
     lmrc = 0
6. The exec loops to process all members returned by LMMLIST. Variable LMRC is set to 4 when the end
of the member list is reached, stopping the loop.
     Do While lmrc = 0
7. Obtain the next member in the list. If this is the first invocation of LMMLIST, the first member in the
list is returned. The member name is returned in variable MEMBER, and variable LMRC is set to the
return code from LMMLIST.
     Address ispexec 'LMMLIST DATAID('data1') OPTION(LIST),
                     MEMBER(MEMBER) STATS(NO)'
     lmrc = rc
8. If LMMLIST returns a 0, indicating a member name was returned, and if the member returned is not
the member currently being edited, the member is processed.
     If lmrc = 0 Then
       do
9. The REXX SAY statement is used to write line-I/O messages. As the macro processes each member,
the member name appears on the terminal to keep you informed about what is happening. An
alternative to the SAY statement would be to display a panel showing the member name after issuing
the ISPEXEC CONTROL DISPLAY LOCK service.
     Say 'Processing member' member
10. The EDIT service is invoked on the member returned by LMMLIST. The macro specified on invocation
of ISRMBRS is passed as an initial macro on the edit service.
     Address ispexec 'EDIT DATAID('data1') MEMBER('member')
                     MACRO('nestmac')'
11. When the LMMLIST service returns a nonzero value, the loop is exited and the cleanup begins.
LMMLIST is called to free the member list, and the LMCLOSE service is called to close the data sets
associated with the data ID.
     Address ispexec 'LMMLIST DATAID('data1') OPTION(FREE)'
     Address ispexec 'LMCLOSE DATAID('data1')'
ISRCHGS macro
The ISRCHGS macro (Figure 48 on page 127) identifies the lines most recently changed by showing only
those lines and excluding all others. When no level is passed, the latest level is assumed. A label range
can also be passed to ISRCHGS to limit the search. This macro relies on the modification level maintained
by the editor for members with numbers and ISPF statistics.
Operands can also be specified. For example, to show lines with level 8 or greater on a line range:
 Command ===> ISRCHGS 8 .FIRST .LAST
ISRCHGS macro
126  z/OS: z/OS ISPF Edit and Edit Macros

## Page 159

/*********************************************************************/ 00010003
/*                                                                   */ 00020003
/* 5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                        */ 00030003
/*                                                                   */ 00040003
/* ISRCHGS - shows the most recent changes to a data set             */ 00050003
/*                                                                   */ 00060003
/*********************************************************************/ 00070003
 ISREDIT MACRO (SEARCH,PARMS)                                           00080003
                                                                        00090003
 ISREDIT (SAVE) = USER_STATE                                            00100003
 ISREDIT (NUMBER, NUMTYPE) = NUMBER                                     00110003
 SET SYSDVAL = &NUMTYPE                                                 00120003
 READDVAL STD COBOL DISPLAY                                             00130003
 ISREDIT (STATS) = STATS                                                00140003
 ISREDIT (LEVEL) = LEVEL                                                00150003
                                                                        00160003
 IF &SEARCH = &STR() | &SUBSTR(1:1,&STR(&SEARCH. )) = &STR(.) THEN DO   00170008
   SET PARMS = &STR(&SEARCH &PARMS)                                     00180003
   SET SEARCH = &LEVEL                                                  00190003
 END                                                                    00200003
                                                                        00210003
 IF &STATS = OFF | &NUMBER = OFF | &STD = NOSTD THEN DO                 00220003
   SET ZEDSMSG = &STR(INVALID DATA)                                     00230003
   SET ZEDLMSG = &STR(BOTH NUMBER AND STATS MODE MUST BE ON)            00240003
   ISPEXEC SETMSG MSG(ISRZ001)                                          00250003
   EXIT CODE(8)                                                         00260003
 END                                                                    00270003
                                                                        00280003
 IF &DATATYPE(&SEARCH) = CHAR THEN DO                                   00290003
   SET ZEDSMSG = &STR(INVALID ARG)                                      00300003
   SET ZEDLMSG = &STR(SEARCH STRING MUST BE FIRST)                      00310003
   ISPEXEC SETMSG MSG(ISRZ001)                                          00320003
   EXIT CODE(8)                                                         00330003
 END                                                                    00340003
                                                                        00350003
 ISREDIT NUMBER = OFF                                                   00360007
 ISREDIT (RECFM) = RECFM                                                00370003
 IF &RECFM = F THEN DO                                                  00380003
   ISREDIT (LRECL) = LRECL                                              00390003
   SET COL1 = &LRECL - 1                                                00400003
   SET COL2 = &LRECL                                                    00410003
 END                                                                    00420003
 ELSE DO                                                                00430003
   SET COL1 = 7                                                         00440003
   SET COL2 = 8                                                         00450003
 END                                                                    00460003
                                                                        00470003
 ISREDIT EXCLUDE ALL                                                    00480003
                                                                        00490003
 DO WHILE &SEARCH <= &LEVEL                                             00500003
   ISREDIT FIND ALL '&SEARCH' &COL1 &COL2 &PARMS                        00510003
   SET SEARCH = &SEARCH + 1                                             00520005
 END                                                                    00530003
                                                                        00530107
 ISREDIT NUMBER = ON                                                    00531007
 ISREDIT USER_STATE = (SAVE)                                            00550003
 EXIT CODE(1)                                                           00560003
Figure 48. ISRCHGS macro
This list explains the logical sections of the ISRCHGS macro:
1. ISRCHGS allows three optional parameters to be passed: a search level and two labels (a label
range). If all three are passed, PARMS contains two labels.
ISREDIT MACRO (SEARCH,PARMS)
2. The statements shown here save user information, number mode and type, last find string, cursor
location, and other profile and status information. Also, stats mode and the current modification level
for parameter checking are retrieved, and the three-part number type is divided into three variables.
ISREDIT (SAVE) = USER_STATE
ISREDIT (NUMBER, NUMTYPE) = NUMBER
SET SYSDVAL = &NUMTYPE
ISRCHGS macro
Chapter 8. Sample edit macros  127

## Page 160

READDVAL STD COBOL DISPLAY
ISREDIT (STATS) = STATS
ISREDIT (LEVEL) = LEVEL
3. ISRCHGS requires that the modification level be entered first if it is specified. This check allows the
level to default to the current (highest) modification level. A label range can be specified without a
level number; PARMS is reset to capture both labels.
IF &SEARCH = &STR() | &SUBSTR(1:1,&SEARCH) = &STR(;) THEN -
  DO
    SET PARMS = &STR(&SEARCH &PARMS)
    SET SEARCH = &LEVEL
  END
4. Check to see if the member modification level is maintained. If not, issue an error message and exit
the macro.
IF &STATS = OFF | &NUMBER = OFF | &STD = NOSTD THEN -
  DO
    SET ZEDSMSG = &STR(INVALID DATA)
    SET ZEDLMSG = &STR(BOTH NUMBER AND STATS MODE MUST BE ON)
    ISPEXEC SETMSG MSG(ISRZ001)
    EXIT CODE(8)
  END
5. A CLIST DATATYPE function is used to check if the first parameter is valid (a number). If it is not valid,
issue an error message and exit from the macro.
IF &DATATYPE(&SEARCH) = CHAR THEN -
  DO
    SET ZEDSMSG = &STR(INVALID ARG)
    SET ZEDLMSG = &STR(SEARCH STRING MUST BE FIRST)
    ISPEXEC SETMSG MSG(ISRZ001)
    EXIT CODE(8)
  END
6. Now that validity checks have been passed you can set number mode off. This allows you to treat the
number field, which contains the level number, as data.
ISREDIT NUMBER = OFF
7. Set &COL1 and &COL2 to the columns containing the level numbers.
ISREDIT (RECFM) = RECFM
IF &RECFM = F THEN -
  DO
    ISREDIT (LRECL) = LRECL
    SET COL1 = &LRECL - 1
    SET COL2 = &LRECL
   END
  ELSE DO
    SET COL1 = 7
    SET COL2 = 8
   END
8. Exclude all lines.
ISREDIT EXCLUDE ALL
9. For each level, find all occurrences of the current modification level. If a label range was specified, it
is in the PARMS variable. All lines with matching levels are excluded.
DO WHILE &SEARCH <= &LEVEL
  ISREDIT FIND ALL '&SEARCH' &COL1 &COL2 &PARMS
  SEARCH = &SEARCH + 1
END
10. Restore user values, especially number mode.
ISREDIT USER_STATE = (SAVE)
ISRCHGS macro
128  z/OS: z/OS ISPF Edit and Edit Macros

## Page 161

In the example in Figure 49 on page 129 the data contains lines that you have changed.
Figure 49. ISRCHGS macro - before running
When you press Enter, the FINDGHGS macro displays the changed lines and excludes the others, as
shown in Figure 50 on page 129.
Figure 50. ISRCHGS macro - after running
ISRMASK macro
The ISRMASK macro (Figure 51 on page 130) allows data in the mask line to overlay lines. It can be used
to place a comment area over existing lines in a member.
ISRMASK macro
Chapter 8. Sample edit macros  129

## Page 162

Before starting this macro, you must specify two things: a mask line and the range of lines it overlays. See
“MASKLINE—Set or Query the Mask Line” on page 367 for information on creating mask lines.
Specify the range of lines by using either an OO or $$ line command. You can use O, OO, On, or $, $$, $n,
where n is the number of lines.
An O line command specifies that mask line data overlays only blanks in the line data. A $ line command
specifies that nonblank mask line data overlays the line data. Once the mask line and range of lines have
been specified, type ISRMASK on the command line and press Enter.
/*********************************************************************/ 
/*                                                                   */ 
/* 5650-ZOS     COPYRIGHT IBM CORP 1995, 2021                        */ 
/*                                                                   */ 
/* ISRMASK - Overlay a line with data from the mask line.            */ 
/*           Use either line command 'O' or '$' to indicate          */ 
/*           which line to overlay. 'O' causes a nondestructive      */ 
/*           overlay, and '$' causes a destructive overlay.          */ 
/*                                                                   */ 
/*********************************************************************/ 
ISREDIT MACRO NOPROCESS                 /* Wait to process       */     
ISREDIT PROCESS RANGE O $               /* "O" and "$" reserved  */     
IF &LASTCC = 0 THEN                     /*   for macro           */ +   
  DO                                    /* If specified, get     */     
    ISREDIT (CMD)   = RANGE_CMD         /*   command entered and */     
    ISREDIT (FIRST) = LINENUM .ZFRANGE  /*   line number range   */     
    ISREDIT (LAST)  = LINENUM .ZLRANGE                                  
    DO WHILE &FIRST LE &LAST            /* Loop to merge data    */     
                                        /*   based on which line */     
                                        /*   command was entered.*/   
       ISREDIT (LINE) = LINE &FIRST     /* Retrieve current line */   
       IF &CMD = $ THEN                 /*  If $ overlay data    */ + 
          ISREDIT LINE &FIRST = (LINE) + MASKLINE                     
       ELSE                                           /*  - else */ + 
          ISREDIT LINE &FIRST = MASKLINE + (LINE)                     
                                        /*     do not overlay    */   
       SET FIRST = &FIRST + 1           /* Increment line num    */   
      END                                                             
    SET RC = 0                                                        
  END                                                                 
ELSE                                    /* Set prompt messages   */ + 
  DO                                                                  
    SET ZEDSMSG = &STR(ENTER "O"/"$" LINE CMD)                        
    SET ZEDLMSG = &STR("ISRMASK" REQUIRES AN "O" OR +                 
        "$" CMD TO INDICATE LINE(S) MERGED WITH MASKLINE)             
    ISPEXEC SETMSG MSG(ISRZ001)                                       
    SET RC = 12                         /* Set return code to 12 */   
  END                                   /*   to keep command in  */   
EXIT CODE(&RC)                          /*   command area        */  
Figure 51. ISRMASK macro
This list explains the logical sections of the ISRMASK macro:
1. The NOPROCESS keyword on the MACRO command allows the macro to control when user input
(changes to data and line commands) is processed.
ISREDIT MACRO NOPROCESS
2. Now process user input and check if certain line commands are entered. The O and $ following the
RANGE keyword specify the line commands to be processed by this macro.
ISREDIT PROCESS RANGE O $
3. A zero return code shows that you entered an O or $ in any of its valid forms: OO-OO, On, and so forth.
IF &LASTCC = 0 THEN
4. &CMD is set to O or $, whichever command was entered.
ISRMASK macro
130  z/OS: z/OS ISPF Edit and Edit Macros

## Page 163

ISREDIT (CMD) = RANGE_CMD
5. &LINE1 and &LINE2 contain the first and last line numbers of the lines specified by the user line
commands.
ISREDIT (FIRST) = LINENUM .ZFRANGE
ISREDIT (LAST) = LINENUM .ZLRANGE
DO WHILE &FIRST LE &LAST
6. Each line that you specify is merged with data from the mask line. Note the use of the LINE keyphrase
on both sides of the assignment. The line command entered controls how the data is merged. An
O specifies that the mask line data only overlays where the line contains blanks. A $ specifies that
nonblank mask line data overlays line data.
IF &CMD = $ THEN
  ISREDIT LINE &FIRST = (LINE) + MASKLINE
ELSE
  ISREDIT LINE &FIRST = MASKLINE + (LINE)
7. When no line command is entered, issue a prompt message. Set a return code of 12 to keep ISRMASK
displayed on the command line.
SET ZEDSMSG = &STR(ENTER "O"/"$" LINE CMD)
SET ZEDLMSG = &STR("ISRMASK" REQUIRES AN "O" OR +
    "$" CMD TO INDICATE LINE(S) MERGED WITH MASKLINE)
ISPEXEC SETMSG MSG(ISRZ001)
SET RC = 12
In the example shown in Figure 52 on page 131, the mask line is specified and the range of lines is set
with the destructive $$ line command.
Figure 52. ISRMASK macro - before running
When you press Enter, the macro overlays the mask line onto the specified range of lines, as shown in
Figure 53 on page 132.
ISRMASK macro
Chapter 8. Sample edit macros  131

## Page 164

Figure 53. ISRMASK macro - after running
ISRMASK macro
132  z/OS: z/OS ISPF Edit and Edit Macros

## Page 165

Part 3. Command reference
© Copyright IBM Corp. 1984, 2024 133

## Page 166

134  z/OS: z/OS ISPF Edit and Edit Macros
