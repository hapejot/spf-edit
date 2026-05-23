# Chapter 9. Defining file-tailoring skeletons

Source file: f54dg00_v3r1.md
Start page: 305
Page span: 305-326

## Page 305

Chapter 9. Defining file-tailoring skeletons
ISPF skeleton definitions are stored in a skeleton library and accessed through the ISPF file-tailoring
services. You create or change skeletons by editing directly into the skeleton library. ISPF interprets the
skeletons during execution. No compilation or preprocessing step is required.
There are two types of records that can appear in the skeleton file:
Data records
A continuous stream of intermixed text, variables, and control characters that are processed to create
an output record.
Control statements
Control the file-tailoring process. Control statements start with a right parenthesis in column 1.
Records containing a ) in column 1 and a blank in column 2 are interpreted as data records.
Records containing a ) in column 1 and a nonblank character in column 2, are interpreted as control
statements.
A )DEFAULT control statement can be used to assign different special characters for syntactical
purposes. The available control statements are:
  )BLANK          )CM             )DEFAULT
  )DO             )DOT            )ELSE
  )ENDDO          )ENDDOT         )ENDREXX
  )ENDSEL         )IF             )IM
  )ITERATE        )LEAVE          )NOP
  )REXX           )SEL            )SET
  )SETF           )TB             )TBA
You can use the ISPFTTRC command to trace both the execution of file tailoring service calls (FTOPEN,
FTINCL, FTCLOSE, and FTERASE) and the processing that occurs within the file tailoring code and
processing of each statement. For more information, refer to “File tailoring trace command (ISPFTTRC)”
on page 324.
Control characters
The characters listed are control characters and have special meanings in a skeleton. They can appear in
either a data record or a control statement.
)  (right parenthesis)
Defines:
• The start of a control statement when placed in column 1 and followed by a nonblank character in
column 2.
• The start of a data record when placed in column 1 and followed by a blank in column 2.
?  (question mark)
The question mark is used as a continuation character when more than one input record maps to a
single output record or control statement.
Data records
A question mark in the last input column of a data record indicates record continuation. If any
character other than a question mark appears in the last input column of an input data record, it
is copied to that column of the output record. Continuation of data records is not permitted for
variable-length input records.
Control statements
Continuation of control statements is permitted for both fixed-length and variable-length input
records.
Control characters
© Copyright IBM Corp. 1980, 2025 277

## Page 306

In a fixed-length record, continuation of a control statement is identified by a question mark in
the last input column:
)SEL &RC = 0                                                           ?
  && &VARNAME = &ZUSER                                                 ?
  && &VARI <= 10
In a variable-length record, continuation of a control statement is identified by a question mark in
the last nonblank input column that is preceded by a space:
)SEL &RC = 0 ?
  && &VARNAME = &ZUSER ?
  && &VARI <= 10
&  (ampersand)
Indicates the start of a variable name. The value of the corresponding dialog variable is substituted in
the output record. A value of all blanks is treated as null. These characters implicitly delimit the end of
a variable name:
(blank) ø < ( + | & ! * ) ; ¬ - / , % _ > : ' = "
Note: File tailoring treats an ampersand-blank combination in the input record as an invalid variable
name.
.  (period)
Causes the value of the variable to be concatenated with the character string following the period
when used at the end of a variable name.
Example:
If variable V has the value ABC, then "&V.DEF" yields "ABCDEF".
Two consecutive ampersand or period control characters in the input record result in one ampersand or
period character being placed in the output record:
  &&  yields  &
  ..  yields  . immediately following a variable name.
Note: If any of these characters is overridden by the )DEFAULT control statement, the same rule applies
to the new control character. For example, if a )DEFAULT statement substitutes the ^ character for ), then
two consecutive ^ characters in the input record will result in one ^ character being placed in the output
record.
Considerations for data records
Input records can have a maximum length of 255 bytes. For fixed-length records, the last eight character
positions are considered to be a sequence number. The character preceding the last eight characters is
considered to be the last input column. Variable-length input records are scanned up to the end of the
record.
If variable substitution results in an output record larger than the logical record length of the output file,
file tailoring terminates and a message is displayed.
Any blank data records in the input data are deleted from file-tailoring output. However, the )BLANK
control statement can be used to produce blank lines in the output file.
Control characters for data records
These characters have special meanings in data records:
!  (exclamation point)
Serves as the default tab character for the )TB and the )TBA control statements. The file-tailoring
tabbing function works either similarly to that of a typewriter tabbing operation, or you can specify in
Considerations for data records
278  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 307

the )TB syntax that tabbing is not to take place if a tab stop is sensed at the same record position as
the tab character.
<  (less-than)
|  (vertical bar)
>  (greater-than)
Specify, respectively, the beginning, middle, and end of a conditional substitution string. For example:
<string1|string2>
where string1 must contain at least one variable name. string2 can be null.
If the first variable in string1 is not null, string1 is substituted in the output record. If the first variable
in string1 is null, string2 is substituted in the output record.
Example:
An input skeleton contains these lines:
)SET I = &Z
)SET J = VALUE_OF_J
)SET K = VALUE_OF_K
FIRST CONDITIONAL SUBSTITUTION RESULT: <&J|&K>;
SECOND CONDITIONAL SUBSTITUTION RESULT: <&I|&J>;
After processing, the file-tailoring output file contains:
FIRST CONDITIONAL SUBSTITUTION RESULT: VALUE_OF_J
SECOND CONDITIONAL SUBSTITUTION RESULT: VALUE_OF_J
Two consecutive control characters in the input record result in one control character being placed in the
output record:
  !!  yields  !
  <<  yields  <
  ||  yields  |
  >>  yields  >
Note: If any of these characters is overridden by the )DEFAULT control statement, the same rule applies
to the new control character. For example, if a )DEFAULT statement substitutes the ^ character for !, then
two consecutive ^ characters in the input record will result in one ^ character being placed in the output
record.
Considerations for control statements
The general format of a control statement is:
)control-word   parameter1 parameter2 … parameter63
where each parameter represents a name, value, operator, or keyword.
Notes about formatting control statements:
1. Control statements must begin in column 1. Note that an )IF or )ELSE control statement can contain
another control statement on the same line, as long as the )IF or )ELSE statement begins in column 1.
2. All control words must be entered in uppercase.
3. The parameters must be separated by one or more blanks, and cannot contain embedded blanks. A
parameter can be coded as:
• A character string
• A dialog variable name, preceded by an ampersand
• A concatenation of variable names and character strings
Considerations for data records
Chapter 9. Defining file-tailoring skeletons  279

## Page 308

4. The current value of each variable is substituted before the control statement is evaluated. The rules
for delimiting variable names and for using ampersands, periods, double ampersands, and double
periods are the same as for data records, as described in “Control characters for data records” on page
278.
The )N comment statement of PDF edit models is not a valid control statement for file tailoring and will
cause file tailoring to terminate with a severe error.
Control statements
This topic describes each of the ISPF file tailoring control statements:
)BLANK
)BLANK
variable
The specified number of blank lines are placed in the output file at the point where the )BLANK
statement is encountered. The number parameter can be specified as a symbolic variable. If number
is omitted, the default value is 1.
Example:
)BLANK
)BLANK &SPACER
The first example inserts one blank line into the output file. In the second example, the number of
blank lines inserted is equal to the current value of the variable SPACER.
)CM
)CM
comment
The statement is treated as a comment. No tailoring is performed, and the record is not placed in the
output file. Comment statements cannot be continued.
In addition, comment control statements are ignored in these cases:
• When specified as the control statement for either the )IF or )ELSE control statements.
• When embedded within another control statement that includes continuation across two or more
input records
)DEFAULT
)DEFAULT abcdefg
The seven characters represented by abcdefg override the use of the ), &, ? , !, <, |, and > characters,
respectively. Exactly seven characters must be specified.
If you are using a non-U.S. keyboard, refer to Appendix A, “Character translations for APL, TEXT, and
Katakana,” on page 311 for text keyboard character translations.
The )DEFAULT statement takes effect immediately when it is encountered. It remains in effect until
the end of FTINCL processing, or until another )DEFAULT statement is encountered. If the )DEFAULT
statement is used to change defaults during an imbed, it is only in effect for that imbed level. It does
not apply to deeper or previous imbed levels. The defaults will not be in effect for any imbedded
skeletons but will be in effect for any data in the skeleton after the )IM. The )DEFAULT statement
cannot be continued.
Example 1:
Considerations for data records
280  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 309

This example demonstrates that defaults changed using )DEFAULT do not take effect in imbedded
skeletons.
This skeleton changes the variable name control character & to the ø sign:
   )DEFAULT )ø?!<|>
   )SET A = USERNAME
   A: øA
   )IM SKEL2
   A: øA
An FTINCL of this skeleton imbeds SKEL2, which contains:
   AA: øA
   AA: &A
This results in this data in the output data set:
   A: USERNAME
   AA: øA
   AA: USERNAME
   A: USERNAME
Example 2:
This example demonstrates that defaults changed in an imbedded skeleton are not passed back to
the skeleton doing the )IMBED.
An FTINCL of this skeleton imbeds SKEL3:
   )SET A = USERNAME
   A: øA
   )IM SKEL3
   A: øA
SKEL3 changes the variable name control character & to the ø sign:
   )DEFAULT )ø?!<|>
   AA: øA
   AA: &A
This results in this data in the output data set:
   A: øA
   AA: USERNAME
   AA: &A
   A: øA
Example 3:
This example demonstrates how to use the NT parameter to prevent tailoring from occurring when
imbedding a file. Using NT eliminates having to change defaults in the imbedded skeleton when it
contains default control characters.
An FTINCL of this skeleton imbeds a skeleton with the NT parameter:
     )SET A = LBL1
     &A:
     )IM SKEL4 NT
       GO TO &A
The imbedded skeleton SKEL4 contains:
     IF (&A < 0) | (&A > 10) THEN
       &A = 0
     ELSE
This results in this data in the output data set:
Considerations for data records
Chapter 9. Defining file-tailoring skeletons  281

## Page 310

LBL1:
     IF (&A < 0) | (&A > 10) THEN
       &A = 0
     ELSE
       GO TO LBL1 
)DO )ENDDO
)DO
)ENDDO
The skeleton input records between the )DO and the corresponding )ENDDO statements are
repeatedly processed until a condition causes the )DO loop to terminate. Processing then continues
with the input record immediately following the )ENDDO statement.
The processing of a )DO loop can be prematurely ended using the )LEAVE statement, or the current
iteration of the )DO loop can be terminated using the )ITERATE statement.
There are several different formats of the )DO statement supported by file tailoring. The possible
syntaxes are:
)DO
var =  n
TO m BY incr FOR cnt WHILE while_expression
UNTIL until_expression
FOREVER
count
var
The control variable name.
n
The starting value, which can be either a positive or a negative integer in the range -2147483648
to 2147483647.
m
The ending value, which can be either a positive or a negative integer in the range -2147483648
to 2147483647.
incr
The increment value, which can be either a positive or a negative integer in the range
-2147483648 to 2147483647. Default value is 1.
cnt
The maximum number of iterations of the )DO loop to be performed. The number can be either
a positive or a negative integer in the range -2147483648 to 2147483647. If cnt is less than 1,
the )DO statement is skipped.
until_expression is a relational expression that is evaluated for a true or false condition. The )DO loop
continues while the until_expression evaluates to a false condition. The test is performed at the end of
each loop prior to updating the control variable. The loop is always performed at least once.
while_expression is a relational expression that is evaluated for a true or false condition. The )DO loop
continues while the while_expression evaluates to a true condition. The test is performed at the start
of each loop, once the control variables are initialized.
count is an integer number used to control the number of iterations of the )DO loop. The number can
be either a positive or a negative integer in the range -2147483648 to 2147483647. If the count is
less than 1, the )DO statement is skipped. The default value for count is 1.
Considerations for data records
282  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 311

FOREVER continues processing the )DO loop until a )LEAVE statement within the loop terminates
the )DO loop. All other parameters are ignored when using the FOREVER parameter. File tailoring
makes no attempt to determine if a )DO FOREVER loop can be suitably terminated.
Example 1
This example performs a loop 10 times with the control variable, I, starting at 1 and increasing by 1
each time. The control variable will have the value 11 at the end of the loop.
)DO I = 1 TO 10
. . .
)ENDDO
Example 2
This example shows a )DO loop that is to continue until the variable RC is nonzero.
)SET RC = 0
)DO FOREVER
. . .
)IF &RC ¬= 0 THEN )LEAVE
. . .
)ENDDO
Example 3
This is another example of a )DO loop that is to continue until the variable RC is nonzero. Note that
testing of the variable RC is performed at the start of each loop.
)SET RC = 0
)DO WHILE &RC = 0
. . .
)ENDDO
Example 4
This example performs a loop 10 times. There is no control variable.
)DO 10
. . .
)ENDDO 
)DOT )ENDDOT
)DOT table-name
SCAN
( name-cond-pairs )
OPT
)ENDDOT
Note: The )DOT command parameter table-name must be in uppercase for use with ISPF table
services.
The skeleton input records between the )DOT and the corresponding )ENDDOT are iteratively
processed as follows:
• Where the SCAN keyword is not provided, the skeleton input records are processed for each row of
the table, beginning with the first row.
• Where the SCAN keyword is provided, the skeleton input records are processed for only those rows
of the table that match the current scan arguments.
– Where the additional name-cond-pairs parameter is not specified, a search argument must
have already been established for the ISPF table, table-name. This requires table-name to have
been opened and a valid search argument established using the TBSARG service before the file
Considerations for data records
Chapter 9. Defining file-tailoring skeletons  283

## Page 312

tailoring services are invoked. A severe dialog error will occur if the SCAN keyword is specified
and valid search arguments have not yet been established for the table.
– Where the additional name-cond-pairs parameter is specified, file tailoring services will use the
variable names and condition values to process the table. The dialog variables must already be
initialized to the required values for the TBSCAN service. The syntax of the name-cond-pairs is
exactly the same as for the TBSARG name-cond-pairs parameter.
• Where the OPT keyword is not provided, if the table does not exist the file tailoring is terminated
with an error message.
• Where the OPT keyword is provided, if the table does not exist the file tailoring processing is the
same as for an empty table.
• Where both the SCAN and OPT keywords are provided, the SCAN keyword must immediately follow
the table-name.
At the start of each iteration, the contents of the current table row are stored into the corresponding
dialog variables. Those values can then be used as parameters in control statements or substituted
into data records. Up to four levels of )DOT nesting are permitted. The same table cannot be
processed recursively. The list of records must end with the )ENDDOT statement.
If the table was already open, it remains open after file tailoring with the CRP positioned at TOP. If it
was not open, it is opened automatically and then closed upon completion of file tailoring.
Any of the other control statements can be used between the )DOT and the )ENDDOT control
statements.
Example 1
This example takes the information from table ABC, and writes any blank table row as a blank line:
)DOT ABC
)SEL &LNAME = &Z && &FNAME = &Z
)BLANK 1
)ENDSEL
    &FNAME &LNAME
)ENDDOT
Example 2
This example takes the information from table ABC, and writes out the records containing the value
in the dialog variable &VAR2, where the table variable VAR1 matches the current value in the dialog
variable &VAR1:
)DOT ABC SCAN(VAR1,EQ)
&VAR2
)ENDDOT 
)IF )ELSE
)IF relational-expression THEN
control-statement
)ELSE
control-statement
The relational-expression is evaluated for a true or false condition.
• If the condition is true, then either the control-statement on the )IF control statement is processed
or the next non-comment line is processed. The )ELSE statement, if one is present, is skipped.
• If the condition is false, the control-statement or next non-comment line is skipped and the
subsequent )ELSE statement, if one is present, is processed.
Up to 32 levels of )IF and )SEL nesting are permitted.
Considerations for data records
284  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 313

The control-statement can be any ISPF file tailoring control statement, except )CM (comment),
which is ignored. Some control statements, namely )DO, )SEL, and )DOT require more than one input
record. Similarly, the )IM control statement imbeds another ISPF skeleton member. The processing of
the )IF or )ELSE statement is not completed until the control statement specified on the )IF or )ELSE
statement is also completed.
Only a control statement can be included on the same input record after the THEN parameter or )ELSE
control word. Put data records that are to be processed as part of the )IF or )ELSE on the next input
record. The control-statement is optional on the same line as either the )IF or )ELSE control words,
but a valid statement must be supplied for an )IF and )ELSE control statement before the end of
the skeleton member. A severe error will occur if the control statement is missing after the THEN
parameter or )ELSE control word. Use the )NOP control statement to provide a null statement.
Example 1
This example combines the )IF and )DO statements to process a block of input records when the
variable RC has a value of zero, or another block of input records when its value is nonzero.
)IF &RC = 0 THEN )DO
. . .
)ENDDO
)ELSE )DO
. . .
)ENDDO
Example 2
This example sets the dialog variable RC back to zero when it has a value of 4. Note that the comment
statement is ignored.
)IF &RC = 4 THEN
)CM RESET RETURN CODE TO ZERO
)SET RC = 0 
)IM
)IM skel-name
NT OPT EXT
NOEXT
The specified skeleton is imbedded at the point where the )IM statement is encountered. Up to 15
levels of imbedding are permitted.
The optional NT parameter indicates that no tailoring is to be performed on the imbedded skeleton.
Because the NT parameter causes the data to be imbedded as it is, without any processing of control
characters or control statements, using the NT option improves performance.
The optional OPT parameter indicates that the skeleton is not required to be present in the skeleton
library. If OPT is coded and the skeleton is not present, no error indication is given, and the record is
ignored. If OPT is not coded, and the skeleton is not present, a severe error occurs.
The EXT parameter enables the use of the extended built-in functions within the skeleton skel-name.
The NOEXT parameter disables the use of the extended built-in functions. These two parameters
only apply to the specified skeleton name. Nested imbeds do not inherit the state. Both parameters
are optional. When neither the EXT or NOEXT parameter is specified, the ability to use the built-in
functions is determined by the FTINCL service call:
Considerations for data records
Chapter 9. Defining file-tailoring skeletons  285

## Page 314

Table 27. EXT and NOEXT, effect on built-in functions support
)IM control statement
FTINCL service
Not specified EXT
Built-in functions supported?
Not specified No Yes
EXT Yes Yes
NOEXT No No
)ITERATE
)ITERATE
The )ITERATE statement terminates the current iteration of the )DO structure and repeats the loop,
providing any conditions that would cause the loop to terminate have not yet been reached. A severe
dialog error will occur if the )ITERATE statement is used outside a )DO structure.
)LEAVE
)LEAVE
DOT
The )LEAVE statement immediately terminates the innermost )DO statement. A severe dialog error
will occur if the )LEAVE statement is used outside a )DO structure.
The optional DOT parameter permits the termination of the current table via the )DOT … )ENDDOT
control statements. The )LEAVE DOT statement must be found within an active )DOT … )ENDDOT
sequence.
)NOP
)NOP
The )NOP control statement does not generate any output and can be used anywhere in a skeleton
input file. It can be used as a null control-statement for either the )IF or )ELSE control statements.
)REXX )ENDREXX
)REXX
variable
REXX=
%
 rexxname TSOENV
)ENDREXX
The )REXX control statement is used to invoke REXX code from within a file tailoring skeleton. The
REXX can be coded within the skeleton immediately after the )REXX control statement, or the name of
a member containing a REXX exec can be supplied.
variable1 … variablen are optional parameters that specify the names of dialog variables to be passed
to the REXX code for processing. Each variable can itself be a variable name, whose value is a list of
one or more dialog variables, separated by either a space or a comma, that are to be passed to the
REXX code.
rexxname specifies the name of a member in the standard search sequence used to load REXX
programs. This member can contain interpreted REXX or compiled REXX. Compiled REXX can be
either the output generated by the REXX compiler when using the CEXEC option, or a load module
Considerations for data records
286  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 315

generated by link-editing the output generated by the REXX compiler when using the OBJECT option.
This is an optional parameter.
If a percent sign (%) is specified before rexxname, it will bypass the attempt to load the REXX as a
load module and attempt to load it directly from the standard SYSEXEC/SYSPROC allocations.
TSOENV indicates that you want ISPF to use the current TSO environment. If your dialog does an
IRXINIT to create its own REXX environment, but this keyword is not specified, that environment is
not used to process the REXX code. The REXX code is instead invoked in ISPFs REXX environment.
Note:
1. The REXX code cannot access any other dialog variables except those specified on the )REXX
control statement.
2. The REXX code cannot issue requests for ISPF services.
3. REXX coded within the skeleton must be terminated by a )ENDREXX control statement within the
same skeleton member.
ISPF dialog variables can be processed by file tailoring REXX code. Dialog variables are made
available to the REXX code via the parameters specified on the )REXX control statement:
The variable values must be in character format when passed to file tailoring REXX code, and must
remain in character format.
The ISPF module ISPFTRXV is used to make ISPF dialog variables available to the file tailoring REXX
code, and to update the dialog variables after they have been processed by file tailoring REXX.
When the file tailoring REXX is interpreted REXX (that is, the REXX statements are coded directly
in a skeleton or the member specified on )REXX control statement contains interpreted REXX), ISPF
creates calls to ISPFTRXV to perform these tasks:
1. Set up corresponding REXX variables for the ISPF dialog variables before the file tailoring REXX is
invoked.
2. Update the ISPF dialog variables with any changes made by the file tailoring REXX after it has
finished.
To do this, ISPF generates these REXX statements before and after the supplied file tailoring REXX
code:
    Call ISPFTRXV 'I'
    If rc=0 then do
    say 'ISPFTRXV Init failed rc='rc
    return
    end
    Call ft_0003B060
    Call ISPFTRXV 'T'
    If rc=0 then
    say 'ISPFTRXV Term failed rc='rc
    return
    ft_0003B060:
    ⋮
       file tailoring REXX code
    ⋮
    return
(Bold text indicates REXX code generated by ISPF.)
Note:
1. A "trace i" statement is also inserted into the REXX code generated by ISPF when the file tailoring
trace command (ISPFTTRC) is used with the debug option.
2. The 11 or 12 lines of REXX code generated by ISPF before the supplied file tailoring REXX code
and the line of REXX code generated by ISPF after the supplied file tailoring REXX code will
affect the results obtained from the SOURCELINE function. For example using SOURCELINE() in
interpreted file tailoring REXX code returns a value that is 12 or 13 more than the number of
source lines of file tailoring REXX.
Considerations for data records
Chapter 9. Defining file-tailoring skeletons  287

## Page 316

If the interpreted file tailoring REXX code uses the EXIT statement to terminate REXX processing, the
termination call to ISPFTRXV generated by ISPF will not be executed. This means that any changes
made to REXX variables will not be applied to the corresponding ISPF dialog variables. If you need
to use the EXIT statement in your file tailoring REXX code and you want changes to be applied to the
ISPF dialog variables, ensure that a termination call to ISPFTRXV (that is, Call ISPFTRXV 'T') is
executed before the EXIT statement.
When the file tailoring REXX code is compiled REXX, ISPF does not create these initialization and
termination calls to ISPFTRXV. Therefore, file tailoring developers must include these calls in their file
tailoring REXX code.
ISPF provides these system dialog variables for processing errors and return codes in file tailoring
REXX:
ZFTXRC
Available for file tailoring REXX code to pass a return code back to ISPF. Length is 2 bytes. The
corresponding REXX variable is initialized with a value of 0.
ZFTXMSG
Available for file tailoring REXX to return a message ID to file tailoring and the invoking
application. Length is 8 bytes. The corresponding REXX variable is initialized with a value of 8
blanks.
ISPF recognizes these return codes passed back by the file tailoring REXX code in the dialog variable
ZFTXRC:
0
Successful operation.
8
File tailoring REXX defined failure. File tailoring continues.
other
Severe error in the file tailoring REXX. File tailoring terminates.
When control returns to ISPF after the file tailoring REXX code has executed, if ZFTXRC contains a
return code of 8 and the value in ZFTXMSG is blank, then ZFTXMSG is set to ISPF222.
If the return code in ZFTXRC is other than 0 or 8, the FTINCL service terminates with a severe error
condition. ISPF sets the ZERRMSG system variable using this search order:
1. If the value in ZFTXMSG is not blank when control returns to ISPF, it is used to set the ZERRMSG
system variable. This allows the file tailoring REXX code to define the message to be used if a
severe error occurs.
2. If the value in ZFTXMSG is blank when control returns to ISPF, ZFTXMSG and ZERRMSG are set to
ISPF223. This is the default ISPF message for severe errors relating to file tailoring REXX.
If CONTROL ERRORS CANCEL is in effect, ISPF displays on the severe error panel the message
indicated by the value of ZERRMSG.
)SEL )ENDSEL
)SEL relational-expression
)ENDSEL
The relational expression is evaluated for a true or false condition.
• If the condition is true, the skeleton input records between the )SEL and the corresponding )ENDSEL
are processed.
• If the condition is false, these records are skipped.
Up to 32 levels of )SEL and )IF nesting are permitted. The list of records must end with an )ENDSEL
statement.
Considerations for data records
288  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 317

Any of the other control statements can be used between the )SEL and the )ENDSEL control
statements. For example, if you want to write information from a table only if variable ABC is set
to the name of that table, specify:
)SEL &ABC = &TABNAME
)DOT TABNAME
   &FNAME &LNAME
)ENDDOT
)ENDSEL
The relational expression consists of a simple comparison of the form:
value1 operator value2
or a combination of up to eight simple comparisons joined by connectors. The system variable Z can
be used to represent a null or blank value.
The allowable operators are:
EQ  or  =               LE  or  <=
NE  or  ¬=              GE  or  >=
GT  or  >               NG  or  ¬>
LT  or  <               NL  or  ¬<
The allowable connectors are | (OR) and && (AND). ISPF evaluates connected expressions from left to
right and evaluates the connectors with equal priority.
Examples:
)SEL  &COND = YES
)SEL  &TEST1 ¬= &Z   |  &ABC = 5
)SEL  &TEST1 ¬= &Z  &&  &ABC = 5 
)SET
)SET variable = expression
)SET allows a value to be assigned to a dialog variable. The variable name should not be preceded by
an ampersand, unless the variable name is itself stored as a variable. A blank is required between the
variable and the equal sign and between the equal sign and the expression.
The expression can be specified in either of these ways:
 value1
 value1  operator  value2  operator  …  value31
where operator can be a plus sign ( + ) or a minus sign ( - ).
To assign a null value to a dialog variable, use the system variable &Z.
Example:
An input skeleton file contains:
)SET A = 1
)SET B = 2
)SET C = &A + &B
)SET D = &Z
A is &A, B is &B, C is &C, D is &D
The resulting output file contains:
A is 1, B is 2, C is 3, D is 
)SETF
Considerations for data records
Chapter 9. Defining file-tailoring skeletons  289

## Page 318

)SETF variable = expression
The )SETF control statement is the same as the )SET control statement, except that it does not require
the use of the EXT parameter on either the FTINCL service or )IM control statement that is processing
the skeleton to use any of the built-in functions. In other words, the extended built-in functions can
always be used on the )SETF control statement.
The expression can be specified in either of these ways:
   value1
   value1  operator  value2  operator  …  value31
where operator can be a plus sign ( + ) or a minus sign ( - ). Each value of the expression can be a
built-in function or a value.
If you need more arithmetic capabilities, use the &EVAL() built-in function (“&EVAL()” on page 292) or
use the )REXX control statement to invoke a REXX exec.
Examples:
)SETF  TOTAL = &EVAL(&SUB1 * (&N-1)) + 2
)SETF NAME = &STR($FNAME &SNAME) 
)TB
The )TB control statement has 3 forms: 
Syntax - standard tabbing
)TB value
Syntax - alternate tabbing: designated positions
)TB value
A
Syntax - alternate tabbing: all positions
)TBA value
An exclamation point (!) is used as the default tab character for the )TB control statement.
It tabs the output record to the next tab stop and fills the intervening spaces with blanks. The next
character following an exclamation point in the input record is put at the tab stop location in the
output record. Up to 16 tab stops can be specified. A tab stop specifies a tab position in the output
record, and must be in the range 1-255. The default is one tab stop at location 255.
When you use the standard tabbing syntax, )TB value1 … value16, and the tab stop value equals
the current output position, the tabbing skips to the next tab stop value that is greater than the
current output position. The input character following the tab character is then inserted into the
position skipped to in the output record.
When you use alternate tabbing syntax, specified with an 'A' in the )TB tabbing syntax, and the tab
stop value equals the current output position, the input character following the tab character is
inserted into the current position in the output record. This allows you to write to the current position
of the output record if a tab character in the input record is encountered at the same time as a tab
stop is encountered in the output record.
Considerations for data records
290  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 319

The way you specify alternate tabbing syntax on the )TB control statement determines whether only
designated or all tab stop values are affected, even if the tab stop value equals the current position in
the output record when a tab character is encountered in the input record. If you specify:
   )TB value1A … value16A
only the tab stop values to which the character A is appended selectively cause tabbing to stop in any
of those positions. If you specify:
   )TBA  value1 … value16
any tab stop value that equals the current position in the output record when a tab character is
encountered in the input record causes tabbing to stop.
Be sure the character that you append for alternate tabbing is an uppercase A. Appending an A to
the )TB control word (that is, )TBA ) has the same effect as appending an A to all individual tab stop
values. When you use the )TBA control word, appending an A to an individual tab stop value has no
additional effect.
Example 1:
This example uses the standard tabbing syntax:
An input skeleton file contains:
  )TB 5 10 20
  !ABCDE!F
After processing, the file-tailoring output record contains these characters:
• Positions 1-4 contain the blanks inserted by the first tab operation.
• Positions 5-9 contain ABCDE. Standard tabbing occurs between E and F because tab stop 10 is at
the same (not greater than) position of the output record at which the tab character is encountered
in the input record.
• Positions 10-19 contain blanks inserted by the second tab operation.
• Position 20 contains F.
Example 2:
This example uses alternate tabbing syntax for designated tab positions:
An input skeleton file contains:
  )TB 5 10A 20
  !ABCDE!F
After processing, the file-tailoring output record contains these characters:
• Positions 1-4 contain the blanks inserted by the first tab operation.
• Positions 5-10 contain ABCDEF. F immediately follows E because alternate tabbing is specified for
tab position 10. This allows tabbing to stop in the current output record position (10) when the tab
character was encountered in the input record.
Example 3:
This example uses the alternate tabbing syntax for all tab positions:
)TBA value1 … value16
An input skeleton file contains:
  )TBA 3 6 10
  !ABC!DEF!GH
After processing, the file-tailoring output record contains:
Considerations for data records
Chapter 9. Defining file-tailoring skeletons  291

## Page 320

• Positions 1-2 contain the blanks inserted by the first tab operation.
• Positions 3-5 contain ABC. D immediately follows C because alternate tabbing is specified and a tab
stop is set at the current output position (6).
• Positions 6-8 contain DEF.
• Position 9 contains a blank inserted by normal tabbing.
• Positions 10-11 contain GH.
Built-in functions
ISPF skeletons support the built-in functions listed. These can be used in place of any single parameter
on a control statement as follows:
• All )SETF statements.
• Other control statements when EXT is specified on the FTINCL service or )IM statement that caused this
member (or any earlier levels) to be processed.
They cannot be used:
• On the )DEFAULT control statement.
• As the control statement keyword itself.
• On data records.
A built-in function name is defined as a variable name, including the ampersand and immediately followed
by an open bracket "(". Built-in functions can be nested up to 32 levels.
The built-in functions are:
• “&EVAL()” on page 292
• “&LEFT()” on page 293
• “&LENGTH()” on page 294
• “&RIGHT()” on page 294
• “&STR()” on page 294
• “&STRIP()” on page 295
• “&SUBSTR()” on page 295
• “&VSYM()” on page 296
• “&SYMDEF()” on page 296
Examples
)DO &I = 1 TO &EVAL(20*&J-1)
)SEL &SUBSTR(&MYVAR,3,6) = ABCDEF
)DOT TAB&SYMDEF(SYSCLONE)
&EVAL()
The &EVAL() function evaluates an arithmetic expression. Only integer calculations are supported.
Syntax
&EVAL( expression)
Built-in functions
292  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 321

expression
An arithmetic expression that is to be evaluated. Only integers with values in the range (-2147483647
to +2147483646) are supported. All intermediate results are also truncated to an integer. The
expression can include these operators:
+
addition
-
subtraction
*
multiplication
/
division
**
raised to the power of
/ /
remainder
The expression can include up to 32 levels of nested parentheses.
Examples
&EVAL(&SUB1 * (&N-1))
&EVAL(&YEAR//4)
&LEFT()
The &LEFT() function returns a string of characters starting at the left of the specified string. Where the
string is shorter than the required length, the resulting string is padded at the right with a pad character.
Syntax
&LEFT(
string
, length
, pad
)
string
The string from which the leftmost characters are to be obtained. This can be a null parameter.
length
The length of the resulting string. It must be a positive integer or zero. The length parameter can be an
expression and will be automatically evaluated using the &EVAL() function (“&EVAL()” on page 292).
This parameter is required.
pad
A single character used to extend the resulting string to the required length when the length of string
is less than length. The default pad character is a blank. This parameter is optional.
Examples
&LEFT(,80,+)
&LEFT(&NAME,1)
Built-in functions
Chapter 9. Defining file-tailoring skeletons  293

## Page 322

&LENGTH()
The &LENGTH() function returns the length of a string.
Syntax
&LENGTH(
string
)
string
The string used to obtain the required length. This can be a null parameter.
Examples
&LENGTH(&NAME)
&RIGHT()
The &RIGHT() function returns a string of characters starting at the right of the specified string. Where the
string is shorter than the required length, the resulting string is padded at the left with a pad character.
Syntax
&RIGHT(
string
, length
, pad
)
string
The string from which the rightmost characters are to be obtained. This can be a null parameter.
length
The length of the resulting string. It must be a positive integer, or zero. The length parameter can
be an expression and will be automatically evaluated using the &EVAL() function (“&EVAL()” on page
292). This parameter is required.
pad
A single character used to extend the resulting string, at the left, to the required length when the
length of string is less than length. The default pad character is a blank. This parameter is optional.
Examples
&RIGHT(25,6,0)
&RIGHT(&DSN,1)
&STR()
The &STR() function returns a string. The resulting string can include embedded blanks.
Syntax
&STR(
string
)
string
The string of characters to be returned. This can be a null parameter.
Built-in functions
294  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 323

Examples
&STR(&FNAME &SNAME)
&STRIP()
The &STRIP() function removes leading and trailing characters that match a supplied character.
Syntax
&STRIP(
string
, option
, char
)
string
The string of characters to be processed. This can be a null parameter.
option
This parameter is required. It must contain one of these values:
L
remove leading characters only
T
remove trailing characters only
B
remove both leading and trailing characters
char
A single character that is the character to be removed from the string. The default character is a blank.
This parameter is optional.
Examples
&STRIP(&NUM,L,0)
&SUBSTR()
The &SUBSTR() function obtains a substring of another string, starting at a specified position and
obtaining either the remainder of the string a specified number of characters.
Syntax
&SUBSTR(
string
, position
, length , pad
)
string
The string of characters to be processed. This can be a null parameter.
position
The starting position within the string from which to obtain the resulting value. It must be a positive
integer. The position parameter can be an expression and will be automatically evaluated using the
&EVAL() function. This parameter is required.
length
The length of the resulting string. It must be a positive integer or zero. The length parameter can be an
expression and will be automatically evaluated using the &EVAL() function (“&EVAL()” on page 292).
The default length is to return the remainder of the string. This parameter is optional.
Built-in functions
Chapter 9. Defining file-tailoring skeletons  295

## Page 324

pad
A single character used to extend the resulting string to the required length when the remaining
length of string is less than length. The default pad character is a blank. This parameter is optional.
Examples
&SUBSTR(&DATE,5,2)
&VSYM()
The &VSYM() function processes the value of a dialog variable found in the function pool and resolves
the values of any system symbols. This includes all system static symbols and dynamic symbols and any
user defined static symbols. z/OS MVS Initialization and Tuning Reference has details on system static
and dynamic symbols. Consult your system programmer for any locally defined user symbols as these are
system and installation dependent.
Syntax
&VSYM( varname)
varname
The name of a dialog variable whose value in the function pool is processed to resolve the values for
system symbols.
Examples
&VSYM(DSNL)
&SYMDEF()
The &SYMDEF() function obtains the value for the corresponding system symbolic symbol. This includes
all system static symbols and dynamic symbols and any user defined static symbols. z/OS MVS
Initialization and Tuning Reference has details on system static and dynamic symbols. Consult your
system programmer for any locally defined user symbols as these are system and installation dependent.
Syntax
&SYMDEF(  symname)
symname
The name of the system or user symbol that is to be obtained. If the symbol name is not found file
tailoring processing returns a null value and processing continues. This parameter is required.
Examples
&SYMDEF(SYSCLONE)
&SYMDEF(LHHMMSS)
Sample skeleton file
Figure 75 on page 297 shows a sample skeleton file. The sample skeleton refers to several dialog
variables (for example, ASMPARMS, ASMIN, and MEMBER). It also illustrates use of the select
statements )SEL and )ENDSEL to conditionally include records. The first part of the example has nested
Sample skeleton file
296  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 325

selects to include concatenated macro libraries if the library names have been specified by the user, that
is, if variables ASMMAC1 and ASMMAC2 are not equal to the null variable Z.
In the second part of the example, )IF … )ELSE statements are used to conditionally run a load-and-go
step. An imbed statement, )IM, is used to bring in a separate skeleton for the load-and-go step.
//ASM    EXEC  PGM=IFOX00,REGION=128K
//             PARM=(&ASMPARMS)
//SYSIN    DD  DSN=&ASMIN(&MEMBER),DISP=SHR
//SYSLIB   DD  DSN=SYS1.MACLIB,DISP=SHR
)SEL  &ASMMAC1 ¬= &Z
//         DD  DSN=&ASMMAC1,DISP=SHR
)SEL  &ASMMAC2 ¬= &Z
//         DD  DSN=&ASMMAC2,DISP=SHR
)ENDSEL
)ENDSEL
//SYSUT1   DD  UNIT=SYSDA,SPACE=(CYL,(5,2))
//SYSUT2   DD  UNIT=SYSDA,SPACE=(CYL,(2,1))
//SYSUT3   DD  UNIT=SYSDA,SPACE=(CYL,(2,1))
//SYSPRINT DD  SYSOUT=(&ASMPRT)
)CM   IF USER SPECIFIED "GO," WRITE OUTPUT IN TEMP DATA SET
)CM   THEN IMBED "LINK AND GO" SKELETON
)IF  &GOSTEP = YES THEN )DO
//SYSGO    DD  DSN=&&&&OBJSET,UNIT=SYSDA,SPACE=(CYL,(2,1)),
//             DISP=(MOD,PASS)
)IM   LINKGO
)ENDDO
)CM   ELSE (NOGO), WRITE OUTPUT TO USER DATA SET
)ELSE )DO
//SYSGO    DD  DSN=&ASMOUT(&MEMBER),DISP=OLD
)ENDDO
//*
Figure 75. Sample skeleton file 
DBCS-related variables in file skeletons
These rules apply to substituting DBCS-related variables in file skeletons (they also apply to messages
and file-tailoring operations):
• If the variable contains MIX format data, each DBCS subfield must be enclosed with shift-out and
shift-in characters.
Example:
eeee[DBDBDBDBDB]eee[DBDBDB]
ee... represents a field of EBCDIC characters
DBDB... represents a field of DBCS characters
-[ ]- represent shift-out and shift-in characters.
• If the variable contains DBCS format data only, the variable must be preceded by the ZE system
variable, without an intervening blank.
Example:
 ...text...&ZE&DBCSVAR..text...
• If the variable contains EBCDIC format data and is to be converted to the corresponding DBCS
format data before substitution, the variable must be preceded by the ZC system variable, without
an intervening blank.
Example:
 ...text...&ZC&EBCSVAR..text...
The ZC and ZE system variables can be used only for the two purposes described. For file skeleton
definition and file tailoring, these two variables can be used only between )DOT and )ENDDOT statements.
When variable substitution causes a subfield-length of zero, the adjacent shift-out and shift-in characters
are removed.
DBCS-related variables
Chapter 9. Defining file-tailoring skeletons  297

## Page 326

DBCS-related variables
298  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
