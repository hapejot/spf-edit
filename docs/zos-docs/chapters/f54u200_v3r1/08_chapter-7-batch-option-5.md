# Chapter 7. Batch (option 5)

Source file: f54u200_v3r1.md
Start page: 377
Page span: 377-388

## Page 377

Chapter 7. Batch (option 5)
The Batch option (5) allows ISPF to run the batch processors shown on the Batch Selection panel,
Figure 204 on page 339, as batch jobs. ISPF generates job control language (JCL) for the job, based on
information you enter on the batch processing panels, and then submits the job for processing. All these
processors, plus SCRIPT/VS, COBOL interactive debug, and FORTRAN interactive debug, are also available
with the Foreground option (4).
When you run a batch processor, you can continue using ISPF while the program is running. However,
if you run these processors by using the Foreground option, you must wait for processing to end before
doing anything else with ISPF. The Foreground Selection panel is shown in Figure 185 on page 309.
   Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                             Batch Selection Panel
 1  Assembler              7  *Binder/Link editor    15 *ADA/370
 2  COBOL                  10 *VS COBOL II debug     16 *AD/Cycle C/370
 3  VS FORTRAN             12  Member Parts List     18  ISPDTLC
 5  PLI                    13 *C/370                 19 *OS/390 C/C++
 6  VS PASCAL              14 *REXX/370
     Enter "/" to select option          * No packed data support
     /  Source data online
        Source data packed
 Job Statement Information: Verify before proceeding
 ===> //LSACKV1  JOB (ACCT),CLASS=A                                           
 ===> //*                                                                     
 ===> //*                                                                     
 ===> //*                                                                     
 Option ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 204. Batch Selection panel (ISRJPA)
The names of the batch processors on this panel are point-and-shoot fields. See the information about
Point-and-Shoot Text Fields in the ISPF User Interface topic in the z/OS ISPF User's Guide Vol I for more
information.
Batch selection panel action bar
The Batch Selection Panel action bar choices function as follows:
Menu
See the information about the Menu Action Bar Choice in the ISPF User Interface topic in the z/OS
ISPF User's Guide Vol I for more information about the Menu pull-down.
Utilities
See the information about the Utilities Action Bar Choice in the ISPF User Interface topic in the z/OS
ISPF User's Guide Vol I for more information about the Utilities pull-down.
Help
The Help pull-down provides general information about foreground processing as well as information
about each available choice on the Batch Selection Panel.
Batch processing sequence
This topic describes the main sequence for batch processing.
Batch—processing sequence
© Copyright IBM Corp. 1980, 2024 339

## Page 378

1. If you do not know whether the source data is in packed format, find out by editing the data set and
entering the PROFILE command. If the source data is in packed format, the profile shows PACK ON.
If the data is not packed, continue with the next step.
If the data is packed, you should read “Expanding packed data” on page 312, paying close attention
to information that applies to the batch processor you plan to use. When you are satisfied that the
data set is ready to be processed, save the data set if you are in Edit and continue with the next step.
2. Select a batch processor. If you bypass the Batch Selection panel, you cannot verify or change the job
statement parameters, or generate multiple compilations (multiple job steps) or link-edits within the
same job.
3. Select the Source data online option to tell ISPF that the data to be processed resides on a currently
mounted volume. ISPF checks the data set information that you entered on the Batch Selection panel
and allows you to display a member list. If you do not select this option, ISPF assumes that the
data cannot be accessed except by the batch job and does not verify the existence or validity of the
specified data set.
4. Select the Source data packed option to tell ISPF that it needs to expand the source data. This option
has no effect on the member parts list option (5.12). Member parts list can read both packed and
unpacked data sets, so no expansion is needed.
5. Enter any job statement information you need. See the information about Job Statement Information
in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I for more information.
6. Press Enter. ISPF displays the data entry panel for the processor you selected.
Note: The remainder of this processing sequence applies to all batch processors except Member
Parts List. See “Member parts list (option 5.12)” on page 348 for more information.
7. Enter the appropriate ISPF library and concatenation sequence or data set names.
For VS COBOL II interactive debug, enter the name of the input object module or load module. The
TYPE, or last qualifier, must be either OBJ or LOAD. However, if you specify an OBJ data set as your
input data set, you must include a load library or data set in the input search sequence (see step “11”
on page 341).
ISPF displays a member list if you omit the member name or use a pattern. See the information about
Naming ISPF Libraries and Data Sets in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I if you need help entering library or data set names, “Input data sets” on page 316
for more information about the concatenation sequence, and the Displaying Member Lists section of
the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I for more information
about displaying member lists.
Note:
a. VS FORTRAN has no LIB option. However, the concatenation sequence is still used to find the
member to be compiled.
b. Password protection is not supported from the Batch option. Therefore, if your input or output
data sets are password-protected, use the Foreground option, which does support passwords.
If you submit a job requiring a password-protected data set, the system operator will be
requested to enter the required password.
8. The List ID field tells ISPF what to name the output listing. Leave this field blank and enter a SYSOUT
class to send the listing to a printer. See “List data sets” on page 316 for more information.
9. Enter a SYSOUT class to generate hardcopy of the listing. You can enter any valid SYSOUT parameter.
If a List ID is entered, this field is ignored.
10. The Options field, whether ASSEMBLER, COMPILER, or LINKAGE EDITOR, is remembered from
one session to another. Therefore, you do not need to change these fields unless the options or
parameters you need are not displayed.
If you need information about the options available for your processor, refer to the documentation
provided with the processor.
Batch—processing sequence
340  z/OS: z/OS ISPF User's Guide Vol II

## Page 379

11. Enter any additional input libraries you need. For VS COBOL II interactive debug, enter any input
LOAD libraries that you need to complete the search. These libraries must be LOAD libraries only. See
“Input data sets” on page 316 if you need help.
12. Once all the input fields have been specified, press Enter to call the batch processor. ISPF generates
the appropriate JCL statements. See “JCL generation—compilers” on page 342 and “JCL generation—
assemblers and linkage editor” on page 343 for more information.
Note: You can leave the entry panel without generating any JCL by entering the END command
instead of pressing Enter.
13. One of these actions occurs:
• If you used the jump function to bypass the Batch Selection panel, ISPF submits the generated JCL
and returns directly to the ISPF Primary Option Menu.
ISPF calls the TSO SUBMIT command to submit a job. The SUBMIT command displays this
message:
JOB jobname(jobid) SUBMITTED
***
When you press Enter or any other interrupt key, ISPF returns to the previous panel.
• Otherwise, ISPF returns to the Batch Selection panel with the message Job step generated
displayed in the short message area on line 1, as shown in Figure 205 on page 341. 
   Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                             Batch Selection Panel           Job step generated
                                                                    More:     +
 1  Assembler              7  *Binder/Link editor    15 *ADA/370
 2  COBOL                  10 *VS COBOL II debug     16 *AD/Cycle C/370
 3  VS FORTRAN             12  Member Parts List     18  ISPDTLC
 5  PLI                    13 *C/370                 19 *OS/390 C/C++
 6  VS PASCAL              14 *REXX/370
    Enter "/" to select option           * No packed data support
    /  Source data online
       Source data packed
 Job Statement Information:
 ===> //LSACKV1  JOB (ACCT),CLASS=A                                           
 ===> //*                                                                     
 ===> //*                                                                     
 ===> //*                                                                     
 Option ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 205. Batch Selection Panel with JCL generated (ISRJPB)
The job statement parameters are shown for information only. They are no longer intensified, and
you cannot type over them because the JOB statement has already been generated. At this point,
you can:
– Select the same or another processor to cause more JCL to be generated.
– Go to the ISPF Primary Option Menu by:
- Canceling the batch job by entering the CANCEL command
- Entering the END or RETURN command to cause the generated JCL to be submitted for
processing.
– Use the jump function (=) to choose any primary option. If any JCL has been generated, it is
submitted for batch processing.
Batch—processing sequence
Chapter 7. Batch (option 5)  341

## Page 380

JCL generation—compilers
Figure 209 on page 346 shows an example for the PL/I optimizing compiler. This panel is typical of
the batch compiler entry panels. After you fill in an entry panel and press Enter, ISPF generates the
appropriate JCL statements. The JCL that would be generated for the PL/I example is:
 //SCAN   EXEC PGM=ISRLEMX,COND=(12,LE),
 //   PARM=('PLI,TOPSEG,B,N,E,4, ,00,ENU,4,7',
 //        '1,/,VIO')
 //*
 //* INSERT STEPLIB DD CARDS HERE FOR ISRLEMX AND THE NATIONAL
 //* LANGUAGE LITERAL LOAD MODULE IF THEY ARE NOT IN YOUR SYSTEM
 //* LIBRARY
 //*
 //ISRLCODE DD  DSN=ISPFDEMO.XXX.PLIO,DISP=SHR
 //         DD  DSN=ISPFDEMO.A.PLIO,DISP=SHR
 //         DD  DSN=ISPFDEMO.PROD.PLIO,DISP=SHR
 //ISRLEXPD DD  UNIT=SYSDA,DISP=(NEW,PASS),SPACE=(CYL,(2,2)),
 //             DSN=&&TEMP1
 //ISRLMSG  DD  SYSOUT=(A)
 //PLIO   EXEC  PGM=IEL0AA,REGION=1024K,COND=(12,LE),
 //             PARM='MACRO,XREF'
 //SYSPRINT DD  DSN=ISPFDEMO.LISTPLIO.LIST,UNIT=SYSDA,
 //             SPACE=(CYL,(2,2)),DISP=(MOD,CATLG),
 //             DCB=(RECFM=VBA,LRECL=125,BLKSIZE=3129)
 //SYSIN    DD  DSN=&&TEMP1,DISP=(OLD,DELETE)
 //SYSLIB   DD  DSN=ISPFDEMO.XXX.PLIO,DISP=SHR
 //         DD  DSN=ISPFDEMO.A.PLIO,DISP=SHR
 //         DD  DSN=ISPFDEMO.PROD.PLIO,DISP=SHR
 //         DD  DSN=ISPFTEST.FLAG.PLIO,DISP=SHR
 //SYSUT1   DD  UNIT=SYSDA,SPACE=(CYL,(2,2))
 //SYSLIN   DD  DSN=ISPFDEMO.XXX.OBJ(TOPSEG),DISP=OLD\
The JCL is generated in two steps:
1. The first step processes one of these scan programs, which are distributed as part of ISPF:
ISRSCAN
Copies one member.
ISRLEMX
Copies the primary member, expands any included members, and unpacks any packed members.
The selected scan program searches the user-specified sequence of concatenated libraries to find
the designated member. If the scan program finds the member, it copies the member to a temporary
sequential data set that is shown by &&TEMP1 and generated by the system. The scan program then
exits with a return code of zero, if no errors are found. If any errors are found, the scan program exits
with one of these return codes, which prevents the processing of the second job step. Table 23 on
page 342 and Table 24 on page 342 describe ISRSCAN and ISRLEMX return codes: 
Table 23. ISRSCAN return codes
ISRSCAN
12 Member not found.
16 OPEN error on DDNAME=IN.
20 I/O error on DDNAME=IN.
24 OPEN error on DDNAME=OUT.
28 I/O error on DDNAME=OUT.
Table 24. ISRLEMX return codes
ISRLEMX
1-15 Parameter n was too long, where n = 1 to 15.
16 Too many parameters.
Batch—JCL generation for compilers
342  z/OS: z/OS ISPF User's Guide Vol II

## Page 381

Table 24. ISRLEMX return codes (continued)
ISRLEMX
17 Too few parameters.
20 Severe error in expand module. An error message
should be printed in the ISRLMSG data set.
2. In this example, the second step calls the PL/I optimizing compiler by using the temporary data set
designated by &&TEMP1 as the input data set. The concatenation sequence is passed to the compiler
through SYSLIB DD statements, to allow inclusion of subsidiary members referenced by %INCLUDE
statements in the source text.
The object module is directed to a partitioned data set with a three-level name composed of the
project name, the first library name, and a type qualifier of OBJ. The member name for the object
module is the same as the primary member to be compiled.
The compiler listing is directed to SYSOUT class A, as specified.
JCL generation—assemblers and linkage editor
For batch assembly and link-edit, an optional SYSTERM DD statement is generated (if you specify TERM)
besides the JCL shown in “JCL generation—compilers” on page 342, as follows:
//SYSTERM  DD  DSN=prefix.member.TERM,DISP=(MOD,CATLG)
where:
prefix
The data set prefix in your TSO user profile
member
For members of partitioned data sets, this is the same member name specified on the entry panel. For
sequential data sets, this name is TEMPNAME.
Assembler (option 5.1)
Batch Assembler enables you to invoke either High Level Assembler or Assembler H. Both are called
from the Batch Assembler panel, shown in Figure 206 on page 343. For information about Assembler
allocation data sets, see the topic about Allocation Data Sets in the z/OS ISPF User's Guide Vol I.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                Batch Assembler
                                                                    More:     +
 ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . LSACKV   . . .          . . .          . . .         
    Type  . . . . ASM     
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
                                                     Assembler
 List ID  . . . . .           (Blank for hardcopy)   1  1. High Level Assembler
 SYSOUT class . . .                  (For hardcopy)     2. Assembler H
 Assembler options:
   Term  . . .         (TERM or NOTERM)
   Other . . .                                                              
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 206. Batch Assembler panel (ISRJP01)
Batch—JCL generation for assemblers and Linkage Editor
Chapter 7. Batch (option 5)  343

## Page 382

All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I, except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained here and in “Input data sets” on page 316:
Term
In the Term field, enter TERM if you want ISPF to generate a terminal data set. A terminal data
set contains a synopsis of the error messages produced by the Assembler. If the input data set is
partitioned, the terminal data set name is:
prefix.member.TERM
where pr efix  is the data set name prefix in your TSO user profile, if you have one, and member is the
name of the member being assembled. However, if the input data set is sequential, the terminal data
set name is:
prefix.TEMPNAME.TERM
Enter NOTERM in the Term field to avoid generating the terminal data set. This is a required field.
Other
Enter any other options you need in the Other field.
COBOL compile (option 5.2)
ISPF generates an ISPEXEC SELECT PGM(IGYCRCTL) statement to invoke a COBOL compiler using the
values you enter on the Batch COBOL Compile panel, shown in Figure 207 on page 344. For information
about COBOL allocation data sets, see the topic about Allocation Data Sets in the z/OS ISPF User's Guide
Vol I.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                              Batch COBOL Compile
                                                                    More:     +
 ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . LSACKV   . . .          . . .          . . .         
    Type  . . . . COBOL   
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
 List ID . . . . . . .           (Blank for hardcopy)
 SYSOUT class  . . . .                  (If hardcopy requested)
 Compiler options:
   Term  . . . NOTERM         (TERM or NOTERM)
   Other . . .                                                              
 Additional input libraries:
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 207. Batch COBOL Compile panel (ISRJP02)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I, except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained here and in “Input data sets” on page 316:
Term
In the Term field, enter TERM if you want ISPF to generate a terminal data set. A terminal data set
contains a synopsis of the error messages produced by the COBOL compiler. If the input data set is
partitioned, the terminal data set name is:
prefix.member.TERM
Batch— COBOL compile (option 5.2)
344  z/OS: z/OS ISPF User's Guide Vol II

## Page 383

where pr efix  is the data set name prefix in your TSO user profile, if you have one, and member is the
name of the member being assembled. However, if the input data set is sequential, the terminal data
set name is:
prefix.TEMPNAME.TERM
Enter NOTERM in the Term field to avoid generating the terminal data set. This is a required field.
Other
If you plan to run VS COBOL II interactive debug after you compile your program, enter TEST,
RESIDENT, and any other options you need in the Other field.
VS FORTRAN compile (option 5.3)
The Batch VS FORTRAN Compile panel is shown in Figure 208 on page 345.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                            Batch VS FORTRAN Compile
                                                                    More:     +
 ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . LSACKV   . . .          . . .          . . .         
    Type  . . . . FORT    
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
 List ID . . . . . . .           (Blank for hardcopy)
 SYSOUT class  . . . .                  (If hardcopy requested)
 Compiler options:
   Term  . . . NOTERM         (TERM or NOTERM)
   Other . . .                                                              
 Additional input libraries:
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 208. Batch VS FORTRAN Compile panel (ISRJP03)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained here and in “Input data sets” on page 316:
Term
In the Term field, enter TERM if you want ISPF to generate a terminal data set. A terminal data set
contains a synopsis of the error messages produced by the VS FORTRAN compiler. If the input data
set is partitioned, the terminal data set name is:
prefix.member.TERM
where pr efix  is the data set name prefix in your TSO user profile, if you have one, and member is the
name of the member being assembled. However, if the input data set is sequential, the terminal data
set name is:
prefix.TEMPNAME.TERM
Enter NOTERM in the Term field to avoid generating the terminal data set. This is a required field.
Other
If you plan to run FORTRAN interactive debug after you compile your program, enter TEST in the Other
field, along with any other options you need.
Batch—VS FORTRAN compile (option 5.3)
Chapter 7. Batch (option 5)  345

## Page 384

PL/I compile (option 5.5)
The Batch PL/I Compile option enables you to invoke either OS PL/I Version 2 or PL/I for MVS and VM,
using the values specified on the Batch PL/I compile panel shown in Figure 209 on page 346.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                               Batch PL/I Compile
                                                                    More:     +
 ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . LSACKV   . . .          . . .          . . .         
    Type  . . . . PLI     
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
                                                     Compiler
 List ID  . . . . .           (Blank for hardcopy)   1  1. OS PL/I V2R3
 SYSOUT class . . .                  (For hardcopy)     2. PL/I for MVS and VM
                                                        3. VA PL/I for OS/390
 Compiler options:
        ===>                                                                
 Additional input libraries:
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 209. Batch PL/I Compile panel (ISRJP05)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316.
Compiler
Choose the compiler you want to use from the list presented.
VS Pascal compile (option 5.6)
The Batch VS Pascal Compile panel is shown in Figure 210 on page 346.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                            Batch VS PASCAL Compile
                                                                    More:     +
 ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . LSACKV   . . .          . . .          . . .         
    Type  . . . . PASCAL  
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
 List ID . . . . . . .           (Blank for hardcopy)
 SYSOUT class  . . . .                  (If hardcopy requested)
 Compiler options:
        ===>                                                                
 Additional input libraries:
        ===>                                                                  
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 210. Batch VS Pascal Compile panel (ISRJP06)
Batch—PL/I compile (option 5.5)
346  z/OS: z/OS ISPF User's Guide Vol II

## Page 385

All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, and "Additional input
libraries", which is explained in “Input data sets” on page 316.
Binder/linkage editor (option 5.7)
The Batch Binder or Linkage Editor is called from the Batch Binder/Linkage editor panel. The panel in
Figure 211 on page 347 shows entries you might make when link-editing a VS Pascal program.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                          Batch Binder/Linkage Editor
                                                                    More:     +
 ISPF Library:
    Project . . . PDFTDEV 
    Group . . . . COMMON   . . .          . . .          . . .         
    Type  . . . . LEL     
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
                                                       Processor
 List ID . . . . .           (Blank for hardcopy)         1. Binder
 SYSOUT class  . .                  (For hardcopy)        2. Linkage Editor
 Linkage editor/binder options:
   Term  . . .           (TERM or blank)
   Other . . .                                                              
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 211. Batch Binder Linkage/Editor panel (ISRJP07B)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, and here:
Binder
Determines whether the Linkage Editor (NOBINDER) or Binder (BINDER) is invoked.
Term
In the Term field, enter TERM if you want ISPF to generate a terminal data set. A terminal data set
contains a synopsis of the error messages produced by the linkage editor. If the input data set is
partitioned, the terminal data set name is:
prefix.member.TERM
where pr efix  is the data set name prefix in your TSO user profile, if you have one, and member is the
name of the member being assembled.
Note: Sequential data sets are invalid when using the Linkage Editor.
Leave the Term field blank to avoid generating the terminal data set.
Other
Enter any other options you need in the Other field.
SYSLIB
The name of the data set that is to contain the ISPF library concatenation sequence used to resolve
any copy statements specified in your program. See “Input data sets” on page 316 and the SYSLIB
Data Set section of the topic about Allocation Data Sets in the z/OS ISPF User's Guide Vol I for more
information.
SYSLIN
The name of the data set that is to contain the object module. The SYSLIN field is provided to
accommodate the VS Pascal XA and NOXA processing options. See “Input data sets” on page 316 and
Batch—binder/linkage editor (option 5.7)
Chapter 7. Batch (option 5)  347

## Page 386

the SYSLIN Data Set section of the topic about Allocation Data Sets in the z/OS ISPF User's Guide Vol I
for more information.
VS COBOL II interactive debug (option 5.10)
Before you can run VS COBOL II interactive debug in batch, you must first perform these tasks in the order
shown:
1. Compile the program using the VS COBOL II compiler (option 4.2 or option 5.2) with the TEST and
RESIDENT options.
2. Use the linkage editor (option 4.7 or option 5.7) to generate an output load module, which VS COBOL II
interactive debug will use as input.
The VS COBOL II Interactive Debug panel is shown in Figure 212 on page 348.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                         VS COBOL II Interactive Debug
                                                                    More:     +
 ISPF Library:
    Project . . . LSACKV  
    Group . . . . PRIVATE   (Type = LOAD assumed)
    Member  . . .           (Blank or pattern for member selection list)
 Other Partitioned or Sequential Data Set:
    Data Set Name  . .                                                         
 List ID . . . . . . .           (Blank for hardcopy)
 SYSOUT class  . . . . A                (If hardcopy requested)
 Debug command data set:
        ===>                                                                
 Additional input libraries:
        ===>                                                                  
        ===>                                                                  
 Command ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 212. VS COBOL II Interactive Debug panel (ISRJP10)
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I except List ID, which is explained in “List data sets” on page 316, "Additional input
libraries", which is explained in “Input data sets” on page 316, and here:
Note: For VS COBOL II interactive debug, any additional input libraries that you enter to complete the
search sequence must be LOAD libraries only.
Debug command data set
In the "Debug command data set" field, enter the name of the data set that contains the DEBUG
command that you want VS COBOL II interactive debug to enter during batch processing. See VS
COBOL II Application Programming Debugging Guide for more information.
Member parts list (option 5.12)
When you select the Batch Member Parts List option (5.12), the panel shown in Figure 213 on page 349 is
displayed.
The only difference between this panel and the Foreground Member Parts List panel is that option 1 (print
member parts) is called Browse/Print member parts list in foreground. The foreground version does not
print your member parts list unless you use the Foreground Print Options panel to do so.
Otherwise, this version operates the same as the foreground version. See “Member parts list (option
4.12)” on page 333 for more information about using the member parts list function.
The listing is 120 characters wide and uses ANSI printer controls.
Batch—VS COBOL II interactive debug (option 5.10)
348  z/OS: z/OS ISPF User's Guide Vol II

## Page 387

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                            Batch Member Parts List
 1 Print member parts
 2 Write member parts data set
 ISPF Library:
    Project . . . LSACKV  
    Group . . . . PRIVATE  . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .           (Blank or pattern for member selection list)
 Language . . . . COB       (Defaults to Type value)
 Groups for Primary members . . . 1  (1, 2, 3, or 4)
 SYSOUT class . . .                  (Defaults to A )
 Output Data Set:          (option 2 only)
    Data Set Name  . .                                                         
 Option ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 213. Batch Member Parts List panel (ISRJP12)
C/370 compile (option 5.13)
ISPF supports the C/370 compiler through dialogs supplied with the C/370 compiler (5688-040). See C
Compiler User's Guide for MVS, SC09-1129 for additional information.
REXX/370 compile (option 5.14)
ISPF supports the REXX/370 compiler through dialogs supplied with the REXX/370 compiler (5695-013).
See IBM Compiler and Library for REXX/370 User's Guide and Reference, SH19-8160, for additional
information.
Ada/370 compile (option 5.15)
ISPF supports the Ada/370 compiler and its tools through dialogs supplied with the Ada/370 compiler
(5706-292). See IBM Ada/370 User's Guide, SC09-1415, for additional information.
AD/Cycle C/370 compile (option 5.16)
ISPF supports the AD/Cycle C/370 compiler through dialogs supplied with the AD/Cycle C/370 compiler
(5688-216). See IBM SAA AD/Cycle C/370 Programming Guide SC09-1356, for additional information.
ISPDTLC compile (option 5.18)
ISPF supports the ISPF Dialog Tag Language compiler by running the ISPDTLC function. See z/OS ISPF
Dialog Tag Language Guide and Reference for more information.
The ISPDTLC interface panels are identical to those in the Foreground option. The first panel can be seen
in Figure 203 on page 337.
The fields on this panel are explained in the topic "Using the Conversion Utility" in z/OS ISPF Dialog Tag
Language Guide and Reference.
OS/390 C/C++ compile (option 5.19)
ISPF supports the OS/390 C/C++ compiler and its tools through dialogs supplied with the OS/390 C/C++
compiler (5647-A01). For information about OS/390 C/C++, refer to the OS/390 C/C++ User's Guide.
Batch C/370 compile (option 5.13)
Chapter 7. Batch (option 5)  349

## Page 388

Batch OS/390 C/C++ (option 5.19)
350  z/OS: z/OS ISPF User's Guide Vol II
