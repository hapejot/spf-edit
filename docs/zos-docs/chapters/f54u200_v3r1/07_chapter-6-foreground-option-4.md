# Chapter 6. Foreground (option 4)

Source file: f54u200_v3r1.md
Start page: 347
Page span: 347-376

## Page 347

Chapter 6. Foreground (option 4)
The Foreground option (4) allows ISPF to run the foreground processors shown on the Foreground
Selection panel, Figure 185 on page 309. All these processors except for COBOL interactive debug,
SCRIPT/VS, and FORTRAN interactive debug are also available with the Batch option (5).
When you run a foreground processor, you must wait until the processor ends before doing anything else
with ISPF. If you want to use ISPF while waiting for the processor to end, submit the input as a batch job.
You can do this by using the Batch option if the processor you need is listed on the Batch Selection panel,
Figure 204 on page 339.
   Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                           Foreground Selection Panel
 1    Assembler                          11  *FORTRAN debug
 2    COBOL                              12   Member Parts List
 3    VS FORTRAN                         13  *C/370
 5    PL/I                               14  *REXX/370
 6    VS PASCAL                          15  *ADA/370
 7   *Binder/Link editor                 16  *AD/Cycle C/370
 9    SCRIPT/VS                          18   ISPDTLC
 10  *VS COBOL II debug                  19  *OS/390 C/C++
 10A *OS/VS COBOL debug
     Enter "/" to select option          * No packed data support
        Source Data Packed
 Option ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 185. Foreground Selection Panel (ISRFPA)
The names of the foreground processors on this panel are point-and-shoot fields. For more information,
see the Point-and-Shoot Text Fields section of the ISPF User Interface topic of the z/OS ISPF User's Guide
Vol I.
Foreground selection panel action bar
The Foreground Selection Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides general information about foreground processing as well as information
about each available choice on the Foreground Selection Panel.
Foreground processing sequence
This topic describes the main sequence for foreground processing.
1. If you do not know whether the source data is in packed format, find out by editing the data set and
entering the PROFILE command. If the source data is in packed format, the profile shows PACK ON.
Foreground—processing sequence
© Copyright IBM Corp. 1980, 2024 309

## Page 348

If the data is packed, select the Source Data Packed option. If the data is not packed, deselect this
option.
Also, you should read “Expanding packed data” on page 312, paying close attention to:
• Information that applies to the foreground processor you plan to use.
• The difference between expanding a sequential data set and expanding members of a partitioned
data set.
When you are satisfied that the data set is ready to be processed, continue with the next step.
2. Select one of the foreground processors listed at the top of the Foreground Selection panel shown in
Figure 185 on page 309.
Note: A region size of 2 megabytes or more will probably be required to run the VS FORTRAN
compiler in the foreground.
3. Select the Source Data Packed option to tell ISPF if it needs to expand the source data.
Note: The Source Data Packed option has no effect on the Member parts list option (4.12). Member
parts list can read both packed and unpacked data sets, so no expansion is needed.
4. When the Session Manager licensed program, 5740-XE2, is installed, you can select the Session
Manager mode option on the ISPF Settings panel so that you enter Session Manager mode when
you call any of the foreground processors. Once you call Session Manager, it stays in effect for all
logical screens until you turn it off. For example, if you call Session Manager and then split the screen,
Session Manager will be in effect on both logical screens.
Note: If graphics interface mode is active, Session Manager does not get control of the screen.
Graphics interface mode is started when a GRINIT service has been issued, but a GRTERM service
has not been issued. See z/OS ISPF Services Guide for more information about these two services.
5. Press Enter. ISPF displays the data entry panel for the processor you selected. The remainder of this
processing sequence applies to all foreground processors except SCRIPT/VS, VS COBOL II interactive
debug, COBOL interactive debug, and Member Parts List. For these processors, use the sequence
referred to in this list:
Processor
Reference
SCRIPT/VS
“SCRIPT/VS processor (option 4.9)” on page 324
VS COBOL II interactive debug
“VS COBOL II interactive debug (option 4.10)” on page 330
COBOL interactive debug
“OS/VS COBOL debug (option 4.10A)” on page 330
Member parts list
“Member parts list (option 4.12)” on page 333
6. Enter the appropriate ISPF library and concatenation sequence or data set name. If the input data set
is partitioned, you can leave the member name blank or use a pattern to display a member list. If you
need help, see:
• The Naming ISPF Libraries and Data Sets section of the "ISPF Libraries and Data Sets" chapter of
the z/OS ISPF User's Guide Vol I for help in entering library or data set names
• The Displaying Member Lists section of the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I for information about patterns and displaying member lists
• “Input data sets” on page 316 for information about the regular concatenation sequence
• “Object data sets” on page 317 for information about object modules
• “Linkage editor concatenation sequence” on page 323 for help with the linkage editor
concatenation sequence
Foreground—processing sequence
310  z/OS: z/OS ISPF User's Guide Vol II

## Page 349

VS FORTRAN has no LIB option, which some foreground processors use to specify the input data set
concatenation sequence. Therefore, the concatenation sequence specified in the Group fields is used
to find the member to be compiled.
For FORTRAN interactive debug, the TYPE, or last qualifier, must be either OBJ or LOAD. However, if
you specify an OBJ data set as your input data set, you must include a load library or data set in the
input search sequence (see step “11” on page 311).
7. This step applies to FORTRAN interactive debug only. Use the Source Type field to tell ISPF the Type,
or last qualifier, of the data set used to create the input object module or load module.
8. Use the List ID field to tell ISPF what to name the output listing. See “List data sets” on page 316 for
more information.
9. Enter your password in the Password field if your input data set is password-protected. See
“Password protection” on page 317 for more information.
10. The Option field, whether ASSEMBLER, COMPILER, LINKAGE EDITOR, or DEBUG, is remembered
from one session to another. Therefore, you do not need to change this field unless the options you
need are not displayed.
Be careful not to enter any options that ISPF generates automatically. These options are listed on the
data entry panel. For more information about the options available for your processor, refer to the
documentation supplied with that processor.
11. Enter any additional input libraries you need. For FORTRAN interactive debug, enter any input LOAD
libraries that you need to complete the search sequence. These libraries must be LOAD libraries only.
See “Input data sets” on page 316 if you need help.
12. Once all the input fields have been specified, press Enter to call the foreground processor.
If the Session Manager is installed and you selected Session Manager mode on the ISPF Settings
panel, the foreground processor and all function keys and PA keys are under the control of the
Session Manager. When foreground processing is complete, you are prompted to enter a null line to
return to ISPF control.
If the Session Manager is not called, the PA and function keys have their usual TSO-defined
meanings; generally, the function keys are treated the same as the Enter key.
13. Communication with foreground processors is in line-I/O mode. Whenever you see three asterisks,
press Enter.
14. If the foreground processor generated an output listing, the listing is displayed automatically in
Browse mode.
Note: If a Foreground processing program ends abnormally, ISPF displays a message in the upper-
right corner of the screen and does not enter Browse mode. The list data set is retained, but the
Foreground Print Options panel (see step “15” on page 311) is not displayed.
You can scroll the output up or down using the scroll commands. All the Browse commands are
available to you. When you finish browsing the listing, enter the END command.
15. An optional print utility exit routine can be installed by your system programmer. If this exit routine is
installed, it may cause the Foreground option's response to differ from the descriptions shown here.
See z/OS ISPF Planning and Customizing for more information about the print utility exit.
Another factor that can affect the performance of the Foreground option is whether the TSO/E
Information Center Facility is installed. If the TSO/E Information Center Facility is installed, your
installation can optionally allow ISPF to display a panel for submitting TSO/E Information Center
Facility information with the print request. See Figure 187 on page 318 for an example of this panel
and “Using the TSO/E information center facility” on page 170 for information about the fields on this
panel. If the TSO/E Information Center Facility is not installed, the Foreground option displays the
panel shown in Figure 186 on page 312 to allow you to print, keep, or delete the output.
Foreground—processing sequence
Chapter 6. Foreground (option 4)  311

## Page 350

Foreground Print Options
   PK  Print data set and keep          K  Keep data set (without printing)
   PD  Print data set and delete        D  Delete data set (without printing)
   If END command is entered, data set is kept without printing.
 Data set name . :
 Print mode  . . . BATCH              (Batch or Local)
 Batch SYSOUT class . .               
 Local Printer id or
       writer-name. . .                    (For local printer)
 Local SYSOUT class . .                
 Job statement information:           (Required for system printer)
   ===>                                                                        
   ===>                                                                        
   ===>                                                                        
   ===>                                                                        
 Option  ===>                                                                 
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 186. Foreground Print Options panel (ISRFPPRT)
On this panel, the "Data set name" field shows the name of the list data set that contains the output
generated by the processor you selected. In the Option field, enter one of the options shown at the
top of the panel. The "Print mode", "Batch SYSOUT class", "Local Printer id or writer name", and Local
SYSOUT Class fields on this panel are described under “Hardcopy utility (option 3.6)” on page 167.
The "Job statement information" field is described under the Job Statement Information section of
the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I.
When you press Enter, the processor entry panel is displayed again. A message indicating completion
of the process is displayed in the upper-right corner of the screen.
16. You can perform one of these actions:
• Enter other parameters and call the same processor.
• Enter the END command to return to the Foreground Selection panel and select another processor.
• Enter the RETURN command to go to the ISPF Primary Option Menu.
• Use the jump function (=) to choose any primary option.
Expanding packed data
Packed data is data in which ISPF has replaced any repeating characters with a sequence showing how
many times the character is repeated. Packing data allows you to use direct access storage devices
(DASD) more efficiently because the stored data occupies less space than it would otherwise.
If the source data that you want to process is packed, it must be expanded before it can be successfully
processed by any of the language processors. Which expansion method you should use depends on
whether your source data is:
• A sequential data set that contains expansion triggers
An expansion trigger is a keyword that tells ISPF to expand additional data before copying, including, or
imbedding it in the source data. Examples are INCLUDE and COPY statements, and SCRIPT .IM (imbed)
control words. For information about defining your own expansion triggers, refer to z/OS ISPF Planning
and Customizing.
ISPF does not recognize expansion triggers in data stored as a sequential data set. Therefore, for this
type of data, you should follow these steps:
1. Manually expand the data that is to be copied, included, or imbedded in your source data. To do this,
edit the source data, enter the PACK OFF command, and then save the data. When you have finished
processing the data, you can repack it by editing it again and entering PACK ON.
Foreground—expanding packed data
312  z/OS: z/OS ISPF User's Guide Vol II

## Page 351

2. Select the Source Data Packed option before calling one of the language processors.
• Either of these:
– A sequential data set that does not contain expansion triggers
– Any member of a partitioned data set, either with or without expansion triggers.
ISPF does recognize expansion triggers in data stored as members of a partitioned data set. Also, if your
source data does not contain expansion triggers, you do not have to be concerned with them. Therefore,
for these two types of data, select the Source Data Packed option before calling one of the language
processors.
In each of the preceding situations, selecting the Source Data Packed option causes ISPF to expand
packed source data before it is processed. For partitioned data sets, any included members are also
expanded inline where the INCLUDE or COPY statements, .IM SCRIPT control words, or other user-
defined trigger statements are found.
Member expansion (ISRLEMX)
Member expansion uses simple language scanners to find expansion triggers. If you specify that the
source data is not packed, the ISRSCAN program is used. However, if you specify that the source data is
packed, member expansion uses the ISRLEMX program.
These scanners do not have all the sophistication of the actual language processors. Therefore, unusual
code or code that does not compile cannot be successfully processed by member expansion. Examples
are trigger statements:
• With comments that extend onto the next line
• That have compiler instructions to change the content of the code to be included.
Compiler control statements and symbolic substitution are not considered during member expansion.
Instead, ISRLEMX creates a temporary data set to be used as input to the language processor. All
members to be processed, including members imbedded with COPY, INCLUDE, or .IM statements, are
copied into this data set, expanded, and passed on to the language processor. The temporary data set will
have the same block size as the input data set that contains your source data.
When using languages that allow multiple compilations, such as VS FORTRAN, you must put the source
statement that ends the program in your original, or top-level, program. This statement cannot be in an
included member.
Table 19 on page 313 shows the languages processed by member expansion, their expansion triggers,
syntax, and the input columns processed for fixed-record data and variable-record data.
Table 19. Expansion triggers and syntax
Language
Expansion
Trigger Syntax
Input Columns
Processed for
F/FB Format
Input Columns
Processed for
V/VB Format
Assembler COPY COPY name 1 - 80 N/A
PL/I %INCLUDE %INCLUDE
DDNAME(name);
%INCLUDE name;
2 - 72 10 - 100
COBOL COPY COPY name. 7 - 72 N/A
VS FORTRAN INCLUDE INCLUDE (name) 1 - 72 N/A
Pascal %INCLUDE %INCLUDE name;
%INCLUDE
DDNAME(name);
1 - 72 1 - 100
Foreground—member expansion (ISRLEMX)
Chapter 6. Foreground (option 4)  313

## Page 352

Table 19. Expansion triggers and syntax (continued)
Language
Expansion
Trigger Syntax
Input Columns
Processed for
F/FB Format
Input Columns
Processed for
V/VB Format
SCRIPT .IM .IM name
.IM (name)
.IM ('name')
1-reclength or 9-
reclength
1-reclength or 1-
(reclength-8)
All languages User-trigger User-trigger name N/A N/A
Restrictions on member expansion and member parts lists
These restrictions apply only to the member expansion and member parts listing functions:
• These restrictions apply to all languages:
– Expansion triggers must follow their respective language coding conventions unless otherwise noted.
– Multiple names and preprocessor variables on trigger statements are not permitted.
– User triggers and their start column are specified at installation time and must be:
- No more than 20 characters long
- Uppercase with no imbedded blanks.
No part of the user trigger can be in a comment or continuation field.
– Macros cannot be in packed form.
– The trigger statement must be the only statement in the logical record. No continuation is allowed
into or from a trigger statement. Also, the trigger keyword must be the first character on the trigger
statement that is not a blank and can be followed by only one statement delimiter.
– For compilers that allow names longer than 8 characters, the name is truncated at 8.
– For compilers that allow uppercase and lowercase names, all referenced names are converted to
uppercase.
• This restriction applies to assembler only:
– The user trigger cannot start in column 1.
• This restriction applies to FORTRAN only:
– The member expansion function allows only the fixed form of coding.
• This restriction applies to PL/I, Pascal, and COBOL:
– Free form coding is allowed except in trigger statements.
• Other COBOL restrictions are:
– The name is truncated at 8 characters or the first hyphen (-), whichever comes first.
– The first statement in the COBOL program must be either an expansion trigger, a valid COBOL
division header, a TITLE, a PROCESS, or a CBL statement. The expansion trigger can precede all other
statements, but it must start in FIELD B.
If an expansion trigger is the first statement, it must eventually resolve (through multiple expansion
triggers if needed) to a valid COBOL division header, TITLE, PROCESS, or CBL statement.
– In the COPY statement, the text-name is the only value processed. The statement must end on the
same line as the COPY keyword with a period followed by a space. If any option is found, the COPY
statement is not expanded.
– In the IDENTIFICATION DIVISION, the division header or paragraph header statements must be
blank except for the division or paragraph name. The trigger statement must be on the next line that
is neither blank nor a comment.
Foreground—member expansion (ISRLEMX)
314  z/OS: z/OS ISPF User's Guide Vol II

## Page 353

– In all other divisions, the trigger statement (line) can be on any line in the division.
– If the WITH DEBUGGING MODE clause is not found in the SOURCE COMPUTER paragraph, all debug
lines are passed to the compiler without being scanned for expansion triggers, as if they were
comment lines. If the clause is found, valid trigger statements found on debug lines are expanded
and a D is inserted in column 7 of all the non-comment, non-continuation lines included.
– Any character found in FIELD A that is not a blank causes the end of the paragraph form of the NOTE
statement.
• These are SCRIPT/VS restrictions:
– The .im statement must be the only statement in the logical record and must start in the first valid
column. The first logical record is tested for line numbers, as follows:
- For fixed-length records, if the last 8 characters are all numeric, they are skipped for the complete
library.
- For variable-length records, if the first 8 characters are all numeric, processing begins with column
9.
The statements can be in either uppercase, lowercase, or mixed case.
– Because ISPF creates a sequential data set from the imbedded members, use of the .EF control
word will cause all statements in the sequential data set following the .EF to be ignored. The use
of .EF is not recommended with packed data.
Member expansion ISRSCAN and ISRLEMX return codes
Table 20 on page 315 describes the ISRSCAN return codes.
Table 20. ISRSCAN return codes
ISRSCAN
12 Member not found.
16 OPEN error on DDNAME=IN.
20 I/O error on DDNAME=IN.
24 OPEN error on DDNAME=OUT.
28 I/O error on DDNAME=OUT.
Table 21 on page 315 describes the ISRLEMX return codes.
Table 21. ISRLEMX return codes
ISRLEMX
1-15 Parameter n was too long, where n = 1 to 15.
16 Too many parameters.
17 Too few parameters.
20 Severe error in expand module. An error message
should be printed in the ISRLMSG data set.
Trigger statement errors
Some of the more common errors that occur are:
• Restricted option.
• Statement on more than one line.
• Referenced member name not found.
Foreground—member expansion (ISRLEMX)
Chapter 6. Foreground (option 4)  315

## Page 354

If an error occurs, the trigger statement is not expanded and is passed to the language processor.
In SCRIPT/VS, if the error was found in a user trigger, one blank line is inserted before and after the
statement in question.
Input data sets
Input to a foreground processor is either:
• A member of an ISPF library or other partitioned data set. If you do not specify a member name, ISPF
displays a member list, or
• A sequential data set
If an ISPF library is the input source, the member can be in any library in the concatenation sequence. You
can include additional input by using:
• The COPY statement for assembler and COBOL.
• The INCLUDE statement for PL/I, FORTRAN, and Pascal.
• The SCRIPT/VS imbed control word (.im).
• Macros
• Additional input libraries.
Whenever the input source is partitioned, you can specify additional input libraries. They must be
partitioned data sets that are not password protected. You cannot specify additional input libraries if
the input source is sequential. Specify the fully qualified data set names, enclosed in apostrophes, such
as:
Additional input libraries:
        ===> 'ABC.MACROS'
For example, in Figure 188 on page 319, a concatenation sequence of three ISPF data sets and one
additional input library has been specified. The concatenation order is:
ISPFDEMO.XXX.ASM
ISPFDEMO.A.ASM
ISPFDEMO.PROD.ASM
ISPFTEST.FLAG.ASM
The last data set in the concatenation sequence, ISPFTEST.FLAG.ASM, is entered as an additional input
library at the bottom of the panel. Additional input libraries are always last in the sequence.
Before calling a foreground processor, ISPF scans the concatenated sequence of libraries to find the
member to be processed. For this example, the member name is TOP. If member TOP first appears in data
set ISPFDEMO.A.ASM, High Level Assembler is invoked with these data sets allocated to SYSLIB.
'SYS1.MACLIB',
'ISPFDEMO.XXX.ASM',
'ISPFDEMO.A.ASM',
'ISPFDEMO.PROD.ASM',
'ISPFTEST.FLAG.ASM'
The processor options are passed to the prompter exactly as you specify them.
Note: The macro library SYS1.MACLIB is included in the concatenation sequence for Assembler only.
When included, as the preceding prompter command example shows, it is always first in the sequence
because of its large block size.
List data sets
In the List ID field, you can enter the name you want ISPF to use to identify the list data set that will
contain the foreground processor output. This name is passed to the foreground processor by either the
LIST or PRINT option. These rules apply:
Foreground—input data sets
316  z/OS: z/OS ISPF User's Guide Vol II

## Page 355

• If the input data set is partitioned the List ID field is optional:
– Leave the List ID field blank if you want ISPF to use the input member name to identify the output list
data set.
– Enter a LIST ID if you want to use a name other than the input member name to identify the output
list data set.
• If the input data set is sequential, you must enter a LIST ID.
For best results, if you plan to debug your program later using COBOL interactive debug:
• Enter the name of the member being compiled in the List ID field if the input data set is partitioned.
• If the input data set is sequential, enter the name of the sequential data set.
Then, when you debug your program, use these same names in the PROG ID fields on the COBOL
Interactive Debug panel.
ISPF names the listing:
prefix.userid.listid.LIST
where
prefix
is the data set prefix in your TSO profile, if you have one and if it is different from your user ID,
userid
is your user ID, and
listid
is the member name or the value in the List ID field.
If you are using the same list data set for multiple job steps, be aware that the DCB information can differ
between the language processors and the linkage editor, causing an I/O error when trying to read the list
data set. We suggest that you use a different list ID for each job step.
Password protection
Input, object, interpretable text (ITEXT), and symbolic debug data sets can be password-protected.
You can specify the password in the Password field on the foreground processor data entry panel. The
password does not appear on the screen when you enter it, but ISPF remembers it.
Since foreground processor panels have only one Password field, ISPF prompts you if all data sets do not
have the same password.
Object data sets
The information shown here about object data sets applies to all foreground assemblers and compilers.
However, if you are using the VS FORTRAN compiler, you must enter OBJECT in the Other field to
generate an output object module. The two assemblers and the other compilers generate object modules
automatically.
If you specify an ISPF library as the input source, ISPF writes object output from the foreground
assembler or compiler to a partitioned data set. This data set has the same name as the first library
in the concatenation sequence, but has a type of OBJ. For example, if you specify PROJECT.LIB1.ASM as
the first library name, the object output is placed in data set PROJECT.LIB1.OBJ. The member name of the
object module is the same as the input member.
If you specify another data set, the object output is placed in a data set of the same name, but with the
last qualifier replaced by OBJ. If the data set name has only one qualifier, OBJ is appended as the last
qualifier. For example, if you specify an input data set named OTHER.ASM or OTHER, the object output
Foreground—password protection
Chapter 6. Foreground (option 4)  317

## Page 356

is placed in a data set named OTHER.OBJ. For partitioned data sets, the object output is stored in a
member with the same name as the input member. For sequential data sets, the object output is stored in
a sequential data set.
Note: The object data set must exist before invoking a foreground or batch option that creates an object
module.
Foreground—TSO/E information center facility
If the TSO/E Information Center Facility is installed, your installation can optionally allow ISPF to
substitute the panel shown in Figure 187 on page 318 for the panel shown in Figure 186 on page 312.
This panel is valid for all foreground processors except SCRIPT/VS and member parts list. See “Using the
TSO/E information center facility” on page 170 for information about the fields on this panel.
                          Foreground Print Options
   PK  Print data set and keep          K  Keep data set (without printing)
   PD  Print data set and delete        D  Delete data set (without printing)
   If END command is entered, data set is kept without printing.
 Data set name  . . .:
 Printer Location  . .               
 Printer Format  . . .         
 Number of copies  . .    
 Option  ===>                                                                 
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 187. Foreground Print Options panel with TSO/E information center facility (ISRFPPRI)
Assembler (option 4.1)
Foreground Assembler enables you to use either High Level Assembler or Assembler H. Both are
called from the Foreground Assembler panel, shown in Figure 188 on page 319. For information about
Assembler data sets, see the topic about Allocation Data Sets in the z/OS ISPF User's Guide Vol I.
Foreground—TSO/E information center facility
318  z/OS: z/OS ISPF User's Guide Vol II

## Page 357

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                              Foreground Assembler
                                                                    More:     +
 ISPF Library:
    Project . . . ISPFDEMO
    Group . . . . XXX      . . . A        . . . PROD     . . .         
    Type  . . . . ASM     
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Name  . . . . . . .                                                        
 List ID . . .              Assembler
 Password  . .                1  1. High Level Assembler  2. Assembler H
 Assembler Options: (Options OBJECT and LIST generated automatically)
        ===>                                                             
 Additional input libraries:
        ===>  'ISPFTEST.FLAG.ASM'                                             
 Command ===>                                                                 
  F1=Help      F3=Exit      F10=Actions    F12=Cancel   F13=Help    F15=End
 F16=Return   F17=Rfind     F18=Rchange    F22=Left     F23=Right   F24=Cretriev
Figure 188. Foreground Assembler panel (ISRFP01)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I, except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Assembler Options
Be careful not to enter the OBJECT and LIST options in this field. ISPF generates these options
automatically. OBJECT writes the output object module to a partitioned data set. LIST writes the
output listing to a list data set. See “Object data sets” on page 317 and “List data sets” on page 316
for more information.
Assembler
Enables you to specify whether to use High Level Assembler or Assembler H. Specify 1 for High Level
Assembler or 2 for Assembler H.
COBOL (option 4.2)
ISPF generates an ISPEXEC SELECT PGM(IGYCRCTL) statement to invoke a COBOL compiler using the
values you enter on the Foreground COBOL Compile panel, shown in Figure 189 on page 320. For
information about COBOL allocation data sets, see the topic about Allocation Data Sets in the z/OS ISPF
User's Guide Vol I.
Foreground—COBOL (option 4.2)
Chapter 6. Foreground (option 4)  319

## Page 358

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                            Foreground COBOL Compile
                                                                    More:     +
 ISPF Library:
    Project . . . MYPROJ  
    Group . . . . DEV      . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                        
 List ID . . . . . . .           Password  . .
 Compiler options: (options LIB and OBJECT generated automatically)
   Test  . . . NOTEST  (TEST or NOTEST)
   Other . . .                                                             
 Additional input libraries:
        ===>                                                                 
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 189. Foreground COBOL Compile panel (ISRFP02)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Test
If you plan to run interactive debug after you compile your program, enter TEST in the Test field.
Otherwise, enter NOTEST.
Other
If you plan to run VS COBOL II interactive debug after you compile your program, enter RESIDENT in
the Other field. Otherwise, just enter any other options you need.
Be careful not to enter the LIB and OBJECT options in the Other field. ISPF generates these options
automatically. LIB specifies the input data set concatenation sequence. OBJECT writes the output
object module to a partitioned data set. See “Input data sets” on page 316 and “Object data sets” on
page 317 for more information.
VS FORTRAN compile (option 4.3)
The Foreground VS FORTRAN Compile panel is shown in Figure 190 on page 321.
Foreground—VS FORTRAN compile (option 4.3)
320  z/OS: z/OS ISPF User's Guide Vol II

## Page 359

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                         Foreground VS FORTRAN Compile
                                                                    More:     +
 ISPF Library:
    Project . . . MYPROJ  
    Group . . . . DEV      . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                        
 List ID . . . . . . .           Password  . .
 Compiler options:
   Object  . .           (OBJECT or NOOBJECT)
   Other . . .                                                             
 Additional input libraries:
        ===>                                                                 
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 190. Foreground VS FORTRAN Compile panel (ISRFP03)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Object
ISPF does not automatically generate any options for VS FORTRAN. Instead of generating an object
module automatically, the VS FORTRAN compiler allows you to decide whether to generate one. To
generate an object module, enter OBJECT in the Object field. To avoid generating an object module,
enter NOOBJECT. See “Object data sets” on page 317 for more information.
Other
If you plan to run FORTRAN interactive debug after you compile your program, enter TEST in the Other
field, along with any other options you need.
PL/I (option 4.5)
The Foreground PL/I option enables you to invoke either OS PL/I Version 2 or PL/I for MVS and VM, using
the values specified on the Foreground PL/I Compile panel shown in Figure 191 on page 321.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                            Foreground PL/I Compile
                                                                    More:     +
 ISPF Library:
    Project . . . MYPROJ  
    Group . . . . DEV      . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                        
 List ID . . .              Compiler
 Password  . .                 1. OS PL/I Version 2     2. PLI for MVS and VM
 Compiler options: (options LIB, OBJECT, and PRINT generated automatically)
        ===>                                                             
 Additional input libraries:
        ===>                                                                 
        ===>                                                                 
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 191. Foreground PL/I Optimizing Compile panel (ISRFP05)
Foreground—PL/I compile (option 4.5)
Chapter 6. Foreground (option 4)  321

## Page 360

All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Compiler
Choose the compiler you want to use from the list presented.
Compiler Options
Enter any options you need in the Other field, except LIB, OBJECT, or PRINT. ISPF generates these
options automatically. LIB specifies the input data set concatenation sequence. OBJECT writes the
output object module to a partitioned data set. PRINT writes the output listing to a list data set. See
“Input data sets” on page 316, “Object data sets” on page 317, and “List data sets” on page 316 for
more information.
VS Pascal compile (option 4.6)
The Foreground VS Pascal Compile panel is shown in Figure 192 on page 322.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                          Foreground VS PASCAL Compile
                                                                    More:     +
 ISPF Library:
    Project . . . MYPROJ  
    Group . . . . DEV      . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                        
 List ID . . . . . . .           Password  . .
 Compiler options: (options LIB, OBJECT, and PRINT generated automatically)
        ===>                                                             
 Additional input libraries:
        ===>                                                                 
        ===>                                                                 
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 192. Foreground VS Pascal Compile panel (ISRFP06)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Other
Enter any options you need in the Other field, except LIB, OBJECT, or PRINT. ISPF generates these
options automatically. LIB specifies the input data set concatenation sequence. OBJECT writes the
output object module to a partitioned data set. PRINT writes the output listing to a list data set. See
“Input data sets” on page 316, “Object data sets” on page 317, and “List data sets” on page 316 for
more information.
Binder/linkage editor (option 4.7)
The Foreground Binder/Linkage Editor is called from the Foreground Binder/Linkage Edit panel, shown in
Figure 193 on page 323.
Foreground—VS Pascal compile (option 4.6)
322  z/OS: z/OS ISPF User's Guide Vol II

## Page 361

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                        Foreground Binder/Linkage Editor
                                                                    More:     +
 ISPF Library:
    Project . . . XYZ     
    Group . . . . MYLIB    . . . PROD     . . .          . . .         
    Type  . . . . LEL     
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned Data Set:
    Name  . . . . . . .                                                        
                               Processor
 List ID . . . . .             1  1. Binder
 Password  . . . .                          2. Linkage Editor
 Linkage editor/binder options: (Options LOAD, LIB, and PRINT generated
 automatically)
          ===>                                                             
 Command ===>                                                                 
  F1=Help      F3=Exit      F10=Actions    F12=Cancel   F13=Help    F15=End
 F16=Return   F17=Rfind     F18=Rchange    F22=Left     F23=Right   F24=Cretriev
Figure 193. Foreground Binder/Linkage Editor panel (ISRFP07B)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Linkage editor/binder options
Enter any options you need, except LOAD, LIB, or PRINT. ISPF generates these options automatically.
LOAD writes the output object module to a partitioned data set.
Note: Sequential data sets are invalid when using the Linkage Editor.
LIB specifies the input data set concatenation sequence. PRINT writes the output listing to a list data
set. See “List data sets” on page 316 and “Object data sets” on page 317 for more information.
Binder
Determines whether the Linkage Editor (NOBINDER) or Binder (BINDER) is invoked.
SYSLIB
The name of the data set that is to contain the ISPF library concatenation sequence used to resolve
any copy statements specified in your program. See “Input data sets” on page 316 and the SYSLIB
Data Set section in the topic about Allocation Data Sets in the z/OS ISPF User's Guide Vol I for more
information.
SYSLIN
The name of the data set that is to contain the object module. The SYSLIN field is provided to
accommodate the VS Pascal XA and NOXA processing options. See “Input data sets” on page 316 and
the SYSLIB Data Set section in the topic about Allocation Data Sets in the z/OS ISPF User's Guide Vol I
for more information.
Linkage editor concatenation sequence
The concatenation sequence used by ISPF to find the member for input to the Linkage Editor is:
project-name.lib1-name.type
project-name.lib2-name.type
(and so forth)
where type is whatever you specify on the panel. For example, it can be OBJ or some other type
containing Linkage Editor language (LEL) control statements. If the type is not OBJ, an OBJECT DDNAME
is automatically allocated to ease the use of these Linkage Editor control statements:
INCLUDE OBJECT(member-name)
Foreground—binder/linkage editor (option 4.7)
Chapter 6. Foreground (option 4)  323

## Page 362

For example:
Project . . . XYZ
Group . . . . MYLIB   . . . PROD  . . .          . . .
Type  . . . . LEL
Member  . . . TOP
In this example, ISPF searches data sets XYZ.MYLIB.LEL and XYZ.PROD.LEL to find member TOP, which
should contain LEL control statements. Also, ISPF allocates to DDNAME OBJECT (DISP=SHR) these
concatenated sequence of object libraries:
XYZ.MYLIB.OBJ
XYZ.PROD.OBJ
This concatenated sequence is searched by the Linkage Editor if member TOP contains INCLUDE
OBJECT(member-name) statements. The concatenation sequence passed to the Linkage Editor by way of
the LIB parameter has a type qualifier of LOAD and includes the system libraries you specify, as follows:
LIB('project-name.lib1-name.LOAD',
    'project-name.lib2-name.LOAD',
 ⋮
    and so forth,
 ⋮
    'syslib1-name',
 ⋮
    and so forth)
This concatenation sequence is used by the Linkage Editor to resolve automatic call references.
SCRIPT/VS processor (option 4.9)
Use of this facility requires the installation of the Document Composition Facility (DCF) program product
and its component text processing program, SCRIPT/VS, with the Foreground Environment Feature.
Note: DCF requires the TSO profile prefix to be set. For additional information, refer to DCF
documentation.
When you select the SCRIPT/VS option, the first panel displayed is the SCRIPT/VS Processor panel shown
in Figure 194 on page 324.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                              SCRIPT/VS Processor
                                                                    More:     +
                                        Enter "/" to select option
 Style . . . .                          /  Display Style Options
                                        /  Browse Output
 ISPF Library:
    Project . . . MYPROJ  
    Group . . . . DEV      . . .          . . .          . . .         
    Type  . . . . SOURCE  
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                        
 List ID . . . . . . .           Password  . .
 Script Command  . . . SCRIPT    (SCRIPT or SCRIPTDB)
 Additional input libraries:
        ===>                                                                 
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 194. SCRIPT/VS Processor panel (ISRFP09)
Foreground—SCRIPT/VS processor (option 4.9)
324  z/OS: z/OS ISPF User's Guide Vol II

## Page 363

All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, and Style, Display
Style Options, and the Browse Output option, which are explained in subsequent sections.
SCRIPT/VS processing sequence
A style contains options that tell SCRIPT/VS how to format a document for display or printing. These
options include the use of fonts, white space, line lengths, and so forth.
The value you put in the Style field and whether or not you select the Display Style Options and Browse
Output options determine this SCRIPT/VS processing sequence:
1. For the Style field, you can perform one of these actions:
• Enter the name of an existing style.
You can enter the name of a style you have created or one of the styles SCRIPT/VS creates for you:
DRAFT and FINAL. These two styles correspond to the formatting options available in the previous
release of SCRIPT/VS Foreground Processing. If you have not defined these options before or if this
is your first release of ISPF, the default values for the SCRIPT/VS formatting options are set for you.
If you enter the name of an existing style in the Style field, that style is used for formatting.
• Enter the name of a new style you want to define.
If you enter a new style name, the name is added to your style list. The new style uses SCRIPT/VS
formatting options that are equal to the formatting options of the last style. Step “2” on page 325
explains what to do to change these options.
• Leave the Style field blank.
If you leave the Style field blank, ISPF displays the Select SCRIPT/VS Formatting Style panel. This
panel displays a list of the available styles. See “Selecting a formatting style” on page 327 for more
information.
2. Use a slash to select Display Style Options. ISPF displays the SCRIPT/VS Options for Style panel,
which shows the options that are currently being used and allows you to change them. See “Changing
style options” on page 328 for more information.
If you do not select Display Style Options, ISPF does not display the SCRIPT/VS Options for Style
panel.
3. Enter the appropriate ISPF library and concatenation sequence or data set names. You can display
a member list by omitting the member name or by using a pattern. See the "ISPF Libraries and Data
Sets" chapter of the z/OS ISPF User's Guide Vol I if you need help entering library or data set names,
“Input data sets” on page 316 for more information about the concatenation sequence, and the
Displaying Member Lists section of the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's
Guide Vol I for more information about displaying member lists.
4. Enter your password in the Password field if your input data set is password-protected. See
“Password protection” on page 317 for more information.
5. Use the List ID field to tell ISPF what to name the output SCRIPT/VS listing. See “List data sets” on
page 316 for more information.
6. Use a slash to select the Browse Output option. ISPF displays your output in Browse mode after it
has formatted.
If you do not select the Browse Output option, ISPF skips Browse mode and displays a Foreground
Print Options for Style panel, shown in Figure 195 on page 327.
7. Once all the input parameters have been specified, press Enter to call SCRIPT/VS.
8. Communication with SCRIPT/VS is in line-I/O mode. Each time you see three asterisks, press Enter.
These asterisks, which usually appear at the bottom of the screen, show that TSO is waiting for you to
clear the screen before it can proceed.
Foreground—SCRIPT/VS processor (option 4.9)
Chapter 6. Foreground (option 4)  325

## Page 364

If the Session Manager is installed and you selected the Session Manager mode option on the ISPF
Settings panel, SCRIPT/VS and all PF and PA keys are under control of the Session Manager. When
formatting is complete, you are prompted to enter a null line to return to ISPF control.
If the Session Manager is not called, the PA and function keys have their usual TSO-defined
meanings; generally, the function keys are treated the same as Enter.
9. One or both of the panels listed may appear, depending on your treatment of the Style and Display
Style Options fields. If both appear, they will be in this sequence:
a. Select SCRIPT/VS Formatting Style
b. SCRIPT/VS Options for Style
See “Selecting a formatting style” on page 327 and “Changing style options” on page 328 if you need
information about using these panels. When you are finished with each panel, press Enter.
10. If SCRIPT/VS generated an output listing and you selected the Browse Output option, the output is
displayed automatically in Browse mode. Otherwise, continue with the next step.
Note: If SCRIPT/VS formatting ends abnormally, ISPF displays a message in the upper-right corner of
the screen and does not enter Browse mode. The list data set is retained, but the Foreground Print
Options for Style panel (see step “11” on page 326) is not displayed.
You can scroll the output up or down using the scroll commands. All the Browse commands are
available to you. When you finish browsing the listing, enter the END command.
11. An optional print utility exit can be installed by your system programmer. If this exit is installed, it
may cause SCRIPT/VS's response to differ from the descriptions here. See z/OS ISPF Planning and
Customizing for more information about the print utility exit.
Another factor that can affect the performance of SCRIPT/VS is whether the TSO/E Information
Center Facility is installed. If the TSO/E Information Center Facility is installed, your installation
can optionally allow ISPF to display a panel for submitting the TSO/E Information Center Facility
information with the print request. See Figure 198 on page 330 for an example of this panel and
“Using the TSO/E information center facility” on page 170 for information about the fields on this
panel.
If the TSO/E Information Center Facility is not installed, SCRIPT/VS displays the panel shown in
Figure 195 on page 327.
The Foreground Print Options for Style panel allows you to optionally print the formatted document
and specify its disposition. On this panel, the Data Set Name field shows the name of the list data set
that contains the SCRIPT/VS output. On the Command line, enter one of the options shown at the top
of the panel.
Foreground—SCRIPT/VS processor (option 4.9)
326  z/OS: z/OS ISPF User's Guide Vol II

## Page 365

Foreground Print Options for Style:
   PK  Print data set and keep          K  Keep data set (without printing)
   PD  Print data set and delete        D  Delete data set (without printing)
   If END command is entered, data set is kept without printing.
 Data Set Name  :
 Print mode . . . BATCH              (Batch or Local)
 Batch SYSOUT class . .                
 Local Printer ID or
       writer-name. . .                    (For local printer)
 Local SYSOUT class . .                
 Job statement information:          (Required for system printer)
   ===>                                                                        
   ===>                                                                        
   ===>                                                                        
   ===>                                                                        
 Command ===>                                                                 
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 195. Foreground Print Options for Style panel (ISRFP09P)
In the Print mode field, enter either of these commands:
• BATCH to submit your print request as a background job.
If you choose BATCH, specify a valid Batch SYSOUT class and job statement information. Specifying
BATCH causes SCRIPT/VS to ignore the "Local Printer ID or writer-name" field and the "Local
SYSOUT class" field.
SCRIPT/VS list data sets are formatted DCB=RECFM=VBM. Unless the line count is altered, the
formatted page length may exceed the JES line count and cause duplicate page ejects. Therefore,
specify this job statement information to prevent JES line counting:
/*JOBPARM LINECT=0
• LOCAL to print the output on a local printer.
If you choose LOCAL, specify the "Local Printer ID or writer-name" of a local printer and optional
"Local SYSOUT class". Specifying LOCAL causes SCRIPT/VS to ignore the "Batch SYSOUT class"
field. Job statement information is ignored.
Page spacing will probably vary from the expected format because of differences between 328x
printers and 1403 or 3800 printers used as a formatting guide.
See “Hardcopy utility (option 3.6)” on page 167 if you need information about the "Print mode",
"Batch SYSOUT class", "Local Printer ID or writer-name", and "Local SYSOUT class" fields. For
information about the "Job statement information" fields, see the "ISPF Libraries and Data Sets"
chapter of the z/OS ISPF User's Guide Vol I.
When you press Enter, the SCRIPT/VS Processor panel is displayed again. A message indicating
completion of the process is displayed in the upper-right corner of the screen.
12. You can perform one of these actions:
• Enter other parameters and call SCRIPT/VS again.
• Enter the END command to return to the Foreground Selection panel and select another processor.
• Enter the RETURN command to go to the ISPF Primary Option Menu.
• Use the jump function (=) to choose any primary option.
Selecting a formatting style
Use the Select SCRIPT/VS Formatting Style panel shown in Figure 196 on page 328 to see which styles
are available and to select or delete styles as necessary.
Foreground—SCRIPT/VS processor (option 4.9)
Chapter 6. Foreground (option 4)  327

## Page 366

Select SCRIPT/VS Formatting Style         Row 1 to 2 of 2
  Valid Line Commands:  S - Use this STYLE for formatting
                        D - Delete
 Line
 Cmd   Style       Description
  '    DRAFT       Draft SCRIPT/VS document options                           
  '    FINAL       Final SCRIPT/VS document options                           
 ******************************* Bottom of data ********************************
 Command ===>                                                 Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 196. Select SCRIPT/VS Formatting Style panel (ISRFP09T)
The Select SCRIPT/VS Formatting Style panel is a list that can be scrolled and contains all the styles
available to you. Each style is a set of predefined formatting options.
Type either S or D in the Line Cmd field and press Enter to select or delete a style, respectively. You can
only select one style at a time for formatting. However, one or more styles can be deleted at the same
time.
The display fields on the Select SCRIPT/VS Formatting Style panel contain:
Style
The names of styles that you can either select or delete.
Description
A reminder of the purpose of each style. Type over the description to change it.
Changing style options
Use the SCRIPT/VS Options for Style panel to:
• See which options are currently being used for the style you chose
• Change the options as needed.
Figure 197 on page 329 shows the options available for the DRAFT style.
Foreground—SCRIPT/VS processor (option 4.9)
328  z/OS: z/OS ISPF User's Guide Vol II

## Page 367

SCRIPT/VS Options for Style: DRAFT
                                                                    More:     +
 Profile data set  . .                                                         
 FONTLIB data set  . .                                             
 SEGLIB data set . . .                                             
 User macro data set  . . .                                             
 System macro data set  . .                                             
 Bind:       #Odd  . .         
             #Even . .         
 Device type . . . . .         
 Chars (Fonts) . . . .                                                         
 SYSVAR  . . . . . . .                                                         
 Page  . . . . . . . .                                                         
 Other script parms  . . .                                                     
 Enter "/" to select option
    Twopass                              /  Uppercase only
 /  Spelling                                Unformat
    Index                                   Condensed Text
 Command ===>                                                                  
  F1=Help    F2=Split   F3=Exit    F9=Swap   F12=Cancel
Figure 197. SCRIPT/VS Options for Style: DRAFT panel (ISRFP09O)
The fields on the SCRIPT/VS Options for Style panel represent SCRIPT/VS formatting options, all of
which are optional. For a complete description of these options, refer to Document Composition Facility:
Generalized Markup Language Starter Set User's Guide.
If you enter the END command from the SCRIPT/VS Options for Style panel, changes on this panel are not
saved. If the style is new, it is saved with default formatting options.
If you press Enter from the SCRIPT/VS Options for Style panel, SCRIPT/VS processes the data set, and
then one of these actions occurs:
• A Browse panel is displayed if you selected the Browse Output option on the SCRIPT/VS Processor
panel. When you finish browsing the SCRIPT/VS formatted output, a Foreground Print Options for Style
panel is displayed.
Note: If you enter the PRINT parameter in the "Other script parms" field, the Browse panel is not
displayed.
• A Foreground Print Options for Style panel is displayed if you did not select the Browse Output option on
the Script/VS Processor panel.
See step “11” on page 326 for more information about printing SCRIPT/VS output.
Using SCRIPT/VS with the TSO/E information center facility
If the TSO/E Information Center Facility is installed, your installation can optionally allow ISPF to
substitute the panel shown in Figure 198 on page 330 for the panel shown in Figure 195 on page 327. See
“Using the TSO/E information center facility” on page 170 for information about the fields on this panel.
Foreground—SCRIPT/VS processor (option 4.9)
Chapter 6. Foreground (option 4)  329

## Page 368

Foreground Print Options for Style:
   PK  Print data set and keep         K  Keep data set (without printing)
   PD  Print data set and delete       D  Delete data set (without printing)
   If END command is entered, data set is kept without printing.
 Data Set Name . . . :
 Printer location  . .                
 Printer Format  . . .         
 Number of copies  . .    
 Command  ===>                                                                 
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT                              
Figure 198. Foreground Print Options for Style panel with the TSO/E information center facility (ISRFP09I)
VS COBOL II interactive debug (option 4.10)
To run VS COBOL II interactive debug in foreground, the VS COBOL II compiler, Release 2, must be both
installed and accessible, for these reasons:
• You must compile your program by using the VS COBOL II compiler (option 4.2 or option 5.2) with the
TEST and RESIDENT options before running VS COBOL II interactive debug. Debug output from the
compilation is stored in the object module, which ISPF generates automatically.
• The VS COBOL II compiler contains the Debug Productivity Aid (DPA) facility, which ISPF accesses when
you run VS COBOL II interactive debug in the foreground.
All VS COBOL II interactive debug processing in the foreground is under DPA's control. DPA displays a
series of interactive panels. When processing is complete, return to step “12” on page 311.
OS/VS COBOL debug (option 4.10A)
Before you can run COBOL interactive debug, you must first perform these actions in the order shown:
1. Allocate a symbolic debug data set and, optionally, a print output data set by using the Data Set utility
(option 3.2). See “Symbolic debug data sets” on page 332 and “Print output data sets” on page 332
for more information.
2. Compile the program by using the OS/VS COBOL compiler (option 4.2A or option 5.2A) with the TEST
option.
3. Use the linkage editor (option 4.7 or option 5.7) to generate an output load module, which COBOL
interactive debug will use as input.
The COBOL Debug panel is shown in Figure 199 on page 331.
Foreground—VS COBOL II Interactive Debug (option 4.10)
330  z/OS: z/OS ISPF User's Guide Vol II

## Page 369

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                            COBOL Interactive Debug
                                                                    More:     +
 ISPF Library:
    Project . . .         
    Group . . . .           (Type = LOAD assumed)
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
 Prog ID . . . . . . .          . . .          . . .          . . .         
 Print ID  . . . . . .           Password  . .
 Enter "/" to select option
 /  Source
 Execution Parms:
        ===>                                                              
 Additional input libraries:
        ===>                                                                  
        ===>                                                                  
        ===>                                                                  
 Note: 1. PREFIX.PRINTID.TESTLIST must exist if Print ID is specified.
       2. PREFIX.PROGID.LIST must exist for each program specified if / is
       specified in Source field.
 Command ===>                                                                  
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 199. COBOL Debug panel (ISRFP10A)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS
ISPF User's Guide Vol I, except Prog ID, Print ID, Source, and Execution Parms, which are explained in
subsequent topics.
COBOL debug processing sequence
Fill in the fields on the COBOL Debug panel as follows:
1. Enter the ISPF library or data set name that contains the input load module generated by the linkage
editor. You can display a member list by omitting the member name or by using a pattern. See the
"ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I if you need help entering
library or data set names, “Object data sets” on page 317 for more information about object modules,
and the Displaying Member Lists section of the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I for more information about displaying member lists.
2. The Prog ID field tells ISPF the names of the sequential list data sets generated by the OS/VS COBOL
compiler. You can enter up to four Prog ID names if you compiled a partitioned data set member or a
sequential data set for each name. See “List data sets” on page 316 for more information.
3. The Print ID field is optional. This field tells ISPF the name of a sequential data set to which it writes
the print output from the debug session. This data set must be preallocated. See “Print output data
sets” on page 332 for more information.
4. Enter your password in the Password field if your input data set is password-protected. See “Password
protection” on page 317 for more information.
5. The Source option tells ISPF whether to allocate the list data sets specified in the Prog ID field. If you
select Source, these data sets must already exist.
6. The Execution Parms field is remembered from one session to another. Therefore, you do not need to
change this field unless the parameters you need are not displayed. Enter any parameters that you
want ISPF to pass to the program being debugged.
7. To continue COBOL interactive debug, return to step “12” on page 311.
Foreground—OS/VS COBOL Debug (option 4.10A)
Chapter 6. Foreground (option 4)  331

## Page 370

Symbolic debug data sets
If you want to run COBOL interactive debug on a program compiled with the OS/VS COBOL compiler,
you must use the Data Set utility (option 3.2) to allocate a symbolic debug data set before compiling
the program. Then, when you compile the program, enter TEST in the Test field on the Foreground
OS/VS COBOL Compile panel. The TEST parameter generates the debug output, which ISPF stores in the
symbolic debug data set you allocated.
Note: You do not need to allocate a symbolic debug data set for programs compiled with VS COBOL
II because the debug output, if requested, is stored in the OBJECT module, which ISPF generates
automatically.
When you allocate the data set, specify the same name as the data set that contains your COBOL
program, but:
• For an ISPF library, enter:
– The Group name you will specify in the first Group field on the Foreground OS/VS COBOL Compile
panel
– SYM in the Type field.
• For another partitioned or sequential data set, use SYM to replace the last qualifier. For example, if
COBOL.INPUT or COBOL is the input data set name, allocate COBOL.SYM as the symbolic debug data
set.
Use these values to allocate symbolic debug data sets:
Record format . . . . . F
Record length . . . . . 512
Block size  . . . . . . 512
For partitioned data sets, including ISPF libraries, the debug output is stored in a member with the same
name as the input member. For sequential data sets, the debug output is stored in a sequential data set.
When you run COBOL interactive debug, the names you put in the Prog ID field on the COBOL Interactive
Debug panel must be the same as the input member names if you are to create a correct SYM data set.
Print output data sets
ISPF writes the print output from a debug session to a sequential data set, if you:
• Allocate the data set, using the Data Set utility (option 3.2), before you run COBOL interactive debug
• Enter, in the Print ID field on the COBOL Interactive Debug panel, the name of the data set you
allocated.
You can avoid generating the print output by leaving the Print ID field blank, even if you allocated the data
set.
The last qualifier in the name of the data set you allocate must be TESTLIST. For example, if you allocate a
sequential data set named DEBUG1.TESTLIST and then specify the Print ID as:
Print ID . . DEBUG1
ISPF writes the print output to a sequential data set named:
'prefix.userid.DEBUG1.TESTLIST'
where pr efix  is your TSO data set prefix, if you have one and if it is different from your user ID, and userid
is your TSO user ID. Use these values to allocate print output data sets:
Record format . . . . . FBA
Record length . . . . . 121
Block size  . . . . . . 3146
Foreground—OS/VS COBOL Debug (option 4.10A)
332  z/OS: z/OS ISPF User's Guide Vol II

## Page 371

The value you put in the Block Size field should be a multiple of 121, the record length. Therefore, if your
print output data is too large to fit within the recommended block size (3146), increase this amount by
using a multiple of 121, such as 3267 or 3388.
FORTRAN debug (option 4.11)
Before you can run FORTRAN interactive debug, you must first compile the program using the VS
FORTRAN compiler (option 4.3 or option 5.3) with the OBJECT and TEST options.
The FORTRAN interactive debug option supports both FORTRAN Interactive Debug Version 2 (5668-903)
and FORTRAN Interactive Debug Version 1 (5734-F05). ISPF looks for Version 2 first, then Version 1, and
finally its own Debug Dialog, which displays the panel shown in Figure 200 on page 333.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                           FORTRAN Interactive Debug
                                                                    More:     +
 ISPF Library:
    Project . . .         
    Group . . . .          . . .          . . .          . . .         
    Type  . . . . OBJ       (OBJ or LOAD)
    Member  . . .           (Blank or pattern for member selection list)
 Source Type  . .         
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
 List ID . . . . . . .           Password  . .
 Debug Options: (options LIB, SOURCE, and PRINT generated automatically)
        ===>                                                              
 Additional input libraries:
        ===>                                                                  
        ===>                                                                  
        ===>                                                                  
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 200. FORTRAN Debug panel (ISRFP11)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Debug Options
Enter any options you need, except LIB, SOURCE, or PRINT. ISPF generates these options
automatically. LIB specifies the input data set concatenation sequence. SOURCE specifies the input
source program, whose type is identified in the Source Type field. PRINT writes the output listing
to a list data set. See “Input data sets” on page 316 and “List data sets” on page 316 for more
information.
Member parts list (option 4.12)
The member parts list uses the program ISRLEMX to show this information for each source program
module specified:
• The names of the modules it calls or includes.
• The names of the modules that call or include it.
The languages permitted in the member expansion function also are permitted in the member parts list
function, and the expansion triggers have the same restrictions. See “Member expansion (ISRLEMX)”
Foreground—FORTRAN Debug (option 4.11)
Chapter 6. Foreground (option 4)  333

## Page 372

on page 313. Besides the expansion triggers, the member parts list also uses the CALL statements in
assembler, PL/I, COBOL, and VS FORTRAN. The format of the CALL statement is:
CALL name
where the delimiter after the name can be either a left parenthesis, a blank, or a valid statement delimiter.
In COBOL, the CALL statement is valid only in the PROCEDURE DIVISION, and the CALL PGMA and CALL
'PGMA' statements both result in a reference to the member name PGMA.
When you select the Foreground Member parts list option (4.12), the panel shown in Figure 201 on page
334 is displayed.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                          Foreground Member Parts List
 1 Browse/Print member parts
 2 Write member parts data set
 ISPF Library:
    Project . . .         
    Group . . . .          . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .           (Blank or pattern for member selection list)
 Language . . . .           (Defaults to Type value)
 Groups for Primary members . . . 1  (1, 2, 3, or 4)
 Output Data Set:          (option 2 only)
    Data Set Name  . .                                                         
 Option ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 201. Foreground Member Parts List panel (ISRFP12)
The member parts list does not use the Source Data Packed option on the Foreground Selection panel;
both packed and unpacked data sets can be read.
Fill in the fields on the Foreground Member Parts List panel as follows:
1. Select one of the options listed at the top of the panel by typing its number in the Option field.
2. Enter the appropriate ISPF library and concatenation sequence or data set names. A blank member
name results in a member list being displayed. You can select only one member from this list. A
pattern results in the processing of all member names matching the pattern; an asterisk results in all
members being processed.
See the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I if you need help
entering library or data set names, the Displaying Member Lists section of the "ISPF Libraries and Data
Sets" chapter of the z/OS ISPF User's Guide Vol I for information about using patterns and displaying
member lists, and “Input data sets” on page 316 for more information about the concatenation
sequence.
3. The Language field is optional. It is used to specify the language in which the source code is written. If
you leave this field blank, ISPF uses the value in the Type field as the default. However, the language
must be one of these:
• Assembler
• COBOL
• FORTRAN
• Pascal
• PLI
• SCRIPT.
Foreground—member parts list (option 4.12)
334  z/OS: z/OS ISPF User's Guide Vol II

## Page 373

4. In the "Groups For Primary members" field, enter a number from 1 to 4. This number tells ISPF
how many libraries in the concatenation sequence are to be used in locating primary members. For
example, if you enter 2, the first and second libraries specified in the Group field are used to find
primary members.
5. If you selected option 2 (write member parts data set), use the Data Set Name field to tell ISPF where
to write the output data set. The name you enter:
• Can be a sequential data set or a member of a partitioned data set
• Must follow standard TSO data set naming conventions.
If you enter the name of a data set that does not exist, ISPF allocates it for you.
6. Once all the input parameters have been specified, press Enter to call the Foreground Member Parts
List processor.
If the Session Manager is installed and if you specified Session Manager mode on the Foreground
Selection panel, the Foreground Member Parts List processor and all function keys and PA keys are
under control of the Session Manager. When processing is complete, you are prompted to enter a null
line to return to ISPF control.
If the Session Manager is not called, the PA and function keys have their usual TSO-defined meanings;
generally, the function keys are treated the same as Enter.
7. Communication with the Foreground Member Parts List processor is in line-I/O mode. Each time you
see three asterisks, press Enter. These asterisks, which usually appear at the bottom of the screen,
show that TSO is waiting for you to clear the screen before it can proceed.
8. The option you chose in step “1” on page 334 determines what happens next.
Note: If the Foreground Member Parts List processing program ends abnormally, ISPF displays a
message in the upper-right corner of the screen and does not enter Browse mode. The list data set is
retained, but the Foreground Print Options panel (see step “15” on page 311) is not displayed.
Option 1 (Browse/print member parts) creates the member parts list and displays it in Browse mode.
This figure shows an example.
 BROWSE - Parts List for ISPFPROJ.ABL.PLI(*) ----------------------------------
  From   Via         From   Via     Member       To    Via          To    Via
 ------- ---       -------- ---    --------   -------- ---       -------- ---
********************************* Top of Data **********************************
                                  (MEMBERA )
                                  (MEMBERB )  MEMBERC  C
                   MEMBERB  C     (MEMBERC )  MEMBERD  C         MEMBERE  I
                                              MEMBERG  C*
                   MEMBERC  C     (MEMBERD )  MEMBERE  I
 MEMBERC  I        MEMBERD  I     (MEMBERE )
                                  (MEMBERF )
******************************** Bottom of Data ********************************
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 202. Member parts list display (ISRFP12B)
The figure shows that:
• Library ISPFPROJ.ABL.PLI contains these members:
MEMBERA
Has no calls or includes.
Foreground—member parts list (option 4.12)
Chapter 6. Foreground (option 4)  335

## Page 374

MEMBERB
Calls MEMBERC.
MEMBERC
Calls MEMBERD and MEMBERG, and includes MEMBERE. The asterisk (*) beside the C in the third
VIA column means that MEMBERG was not found in the input library.
MEMBERD
Includes MEMBERE.
MEMBERE
Has no calls or includes.
MEMBERF
Has no calls or includes.
• A parts list is requested for all members in the first data set.
You can scroll the output up or down using the scroll commands. All the Browse commands are
available to you. When you finish browsing the listing, enter the END command and continue with step
“15” on page 311.
Option 2 (Write member parts data set) produces an intermediate sequential member parts list in the
data set you named in step “5” on page 335. This data set can be either a sequential data set or a
member of a partitioned data set.
If the data set has not been allocated, option 2 allocates it with a logical record length (LRECL) of 17, a
block size (BLKSIZE) of 3009, and a record format (RECFM) of FB. The format of the records is shown
in Table 22 on page 336: 
Table 22. Foreground member parts list record formats
Field Name Format Description
Member name CHAR(8) Subject member.
Called by or calls
member name
CHAR(8) Referenced member.
Call flag BIT(1) Found on a CALL statement.
Include flag BIT(1) Found by INCLUDE or COPY.
Not found flag BIT(1) Referenced member not found.
From flag BIT(1) Subject member called from referenced member.
To flag BIT(1) Referenced member called from subject member.
COBOL flag BIT(1) Member referenced outside valid COBOL division.
Reserved BIT(2) Field that is reserved.
9. You can perform one of these actions:
• Enter other parameters and call the same processor.
• Enter the END command to return to the Foreground Selection panel and select another processor.
• Enter the RETURN command to go to the ISPF Primary Option Menu.
• Use the jump function (=) to choose any primary option.
Member not found
A primary library is one of the number of libraries specified in the "Groups For Primary members" field.
A primary member is a member that starts the member parts explosion chain. An explosion chain is
the order in which members are nested, starting with the primary member and continuing through each
member that it includes, calls, or copies.
Foreground—member parts list (option 4.12)
336  z/OS: z/OS ISPF User's Guide Vol II

## Page 375

The chain is broken when a member cannot be found in the set of concatenated libraries or no more
members are referenced. If a member cannot be found, the name is flagged with an asterisk (*) and
processing continues. For instance, internally called routines are not found.
When no more primary members can be found, the listing is printed, written, or browsed. Calls to internal
routines or variable names result in the member not found flag being set.
C/370 compile (option 4.13)
ISPF supports the C/370 compiler through dialogs supplied with the C/370 compiler (5688-040). See C
Compiler User's Guide for MVS, SC09-1129 for additional information.
REXX/370 compile (option 4.14)
ISPF supports the REXX/370 compiler through dialogs supplied with the REXX/370 compiler (5695-013).
See IBM Compiler and Library for REXX/370 User's Guide and Reference, SH19-8160, for additional
information.
Ada/370 compile (option 4.15)
ISPF supports the Ada/370 compiler and its tools through dialogs supplied with the Ada/370 compiler
(5706-292). See IBM Ada/370 User's Guide SC09-1415, for additional information.
AD/Cycle C/370 compile (option 4.16)
ISPF supports the AD/Cycle C/370 compiler through dialogs supplied with the AD/Cycle C/370 compiler
(5688-216). See IBM SAA AD/Cycle C/370 Programming Guide SC09-1356, for additional information.
ISPDTLC (option 4.18)
ISPF supports the ISPF Dialog Tag Language compiler by running the ISPDTLC function. See z/OS ISPF
Dialog Tag Language Guide and Reference for more information about DTL.
The first ISPDTLC interface panel appears as shown in this figure.
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │   Menu  Utilities  Commands  Language  Options  Help                        │
 │ ──────────────────────────────────────────────────────────────────────────  │
 │            ISPF Dialog Tag Language Conversion Utility - 5.5                │
 │                                                                             │
 │ Click here:   Go to DTL input names 5-16        Reset DTL input names 2-16  │
 │ Enter requested information:          Current Language: ENGLISH             │
 │                                                                More:     +  │
 │ Member name . . . . . . . .           (Blank or pattern for member list)    │
 │ DTL Source data set - 1 . . 'USERID.GML'                                    │
 │ DTL Source data set - 2 . .                                                 │
 │ DTL Source data set - 3 . .                                                 │
 │ DTL Source data set - 4 . .                                                 │
 │ Panel data set  . . . . . . 'USERID.PANELS'                                 │
 │ Message data set  . . . . . 'USERID.MSGS'                                   │
 │ Log data set  . . . . . . .                                                 │
 │   Log File Member name  . .           (Required when log file is a PDS)     │
 │ List data set . . . . . . .                                                 │
 │   List File Member name . .           (Required when list file is a PDS)    │
 │ SCRIPT data set . . . . . .                                                 │
 │ Command ===>                                                                │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F10=Actions    F12=Cancel                                    │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 203. Foreground ISPDTLC compile panel (ISPCP01) Screen 1
The fields on this panel are explained in the topic "Using the Conversion Utility" in z/OS ISPF Dialog Tag
Language Guide and Reference.
Foreground—C/370 compile (option 4.13)
Chapter 6. Foreground (option 4)  337

## Page 376

OS/390 C/C++ compile (option 4.19)
ISPF supports the OS/390® C/C++ compiler and its tools through dialogs supplied with the OS/390 C/C++
compiler (5647-A01). For information about OS/390 C/C++, refer to the OS/390 C/C++ User's Guide.
Foreground—C/C++ for OS/390 compile (option 4.19)
338  z/OS: z/OS ISPF User's Guide Vol II
