# Appendix C. Update files

Source file: f54u200_v3r1.md
Start page: 531
Page span: 531-544

## Page 531

Appendix C. Update files
An update file contains information relating to the result of a comparison and is generated when one of
the update process options is specified:
UPDCMS8 (“Update CMS sequenced 8 file” on page 495)
UPDCNTL (“Update control data sets” on page 496)
UPDLDEL (“Update long control” on page 499)
UPDMVS8 (“Update MVS sequenced 8 data set” on page 500)
UPDPDEL (“Update prefixed delta lines” on page 501)
UPDREV (“Revision file” on page 493)
UPDREV2 (“Revision file (2)” on page 494)
UPDSEQ0 (“Update sequenced 0 data set” on page 501)
UPDSUMO (“Update summary only data sets” on page 502)
Note:
1. UPDCMS8, UPDMVS8, UPDPDEL, UPDREV, UPDREV2, and UPDSEQ0 do not generate an update file
after a comparison of matching files (Return Code = 0).
2. Dates, where applicable, in the heading lines of update files are in the format MM/DD/YYYY.
3. All "do not process" options, and DPLINE or CMPLINE process statements are invalid when used with
the process options UPDCMS8, UPDMVS8, UPDSEQ0, UPDLDEL, and UPDPDEL. The "do not process"
options are cancelled with error notification ASMF014.
Update files are normally used as input to post-processing programs and can be specified besides the
normal listing output file.
On the following pages, descriptions and examples are given of the contents of the update file produced
for each of the update (UPD…) process options.
In most of the examples shown, the same two input files are used. The contents of the old file are shown
in Figure 273 on page 493. The contents of the new file are shown in Figure 274 on page 493. 
This line is reformatted; the  spacing  in  the  "new"  file  differs.  00000100
This line is the same in both files.                                    00000200
This line differs from the text in the "new" file.                      00000300
This line is the same in both files.                                    00000400
Figure 273. The “Old” input file  used in most of the update examples
This line is reformatted; the spacing in the "new" file differs.        00000100
This line is the same in both files.                                    00000200
This line differs from the text in the "old" file.                      00000300
This line is the same in both files.                                    00000400
This line is in the "new" file, but not in the "old".                   00000500
Figure 274. The “New” input file  used in most of the update examples
Revision file
The process option UPDREV produces an update file containing a copy of the new source text with revision
tags delimiting the changed text lines.
The UPDREV process option is available for LINE and WORD compare types.
Update files
© Copyright IBM Corp. 1980, 2024 493

## Page 532

UPDREV supports two different types of revision tags, one for SCRIPT/VS files and one for BookMaster
files. (Use the REVREF process statement (“Revision code reference” on page 459) to specify which type
of revision tag you want.)
Figure 275 on page 494 shows a SuperC UPDREV file with SCRIPT/VS revision tags (.rc on/off). 
.rc 1 &vbar.
.rc 1 on
This line is reformatted; the spacing in the "new" file differs.
.rc 1 off
This line is the same in both files.
.rc 1 on
This line differs from the text in the "old" file.
.rc 1 off
This line is the same in both files.
.rc 1 on
This line is in the "new" file, but not in the "old".
.rc 1 off
Figure 275. Example of a UPDREV update file  for SCRIPT/VS documents
When the UPDREV update file in Figure 275 on page 494 is processed by SCRIPT/VS, the final scripted
output has "|" revision characters in the left margin of the output document identifying the changed lines
(those between the SCRIPT/VS revision tags .rc 1 on and .rc 1 off).
Note: The revision character ("|" in the example in Figure 275 on page 494) can be specified either
by using a REVREF process statement (see “Revision code reference” on page 459) or by having a
SCRIPT/VS .rc. revision tag as the first record in the new file. Subsequent changes to the source can
therefore be separately identified by using different revision characters.
Figure 276 on page 494 shows a SuperC UPDREV file with BookMaster revision tags (:rev/:erev). 
:rev  refid=!.                                                  
This line is reformatted; the spacing in the "new" file differs.
:erev refid=!.                                                  
This line is the same in both files.                            
:rev  refid=!.                                                  
This line differs from the text in the "old" file.              
:erev refid=!.                                                  
This line is the same in both files.                            
:rev  refid=!.                                                  
This line is in the "new" file, but not in the "old".           
:erev refid=!.                                                  
Figure 276. Example of a UPDREV update file  for bookmaster documents
When the UPDREV update file in Figure 276 on page 494 is processed by BookMaster, the final formatted
output has the revision character associated with the revision ID abc (as specified by a :revision.
BookMaster tag in the new input file) in the left margin of the output document identifying the changed
lines (those between the BookMaster revision tags :rev and :erev).
Note: The revision ID (abc in the example in Figure 276 on page 494) is controlled by the REVREF
process statement (see “Revision code reference” on page 459). Subsequent changes to the source
can therefore be separately identified by using different revision IDs (which are associated with unique
revision characters).
Revision file (2)
The process option UPDREV2 is identical to UPDREV with the exception that data between the following
BookMaster tags are not deleted in the update file:
:cgraphic.
:ecgraphic.
Update files
494  z/OS: z/OS ISPF User's Guide Vol II

## Page 533

:fig.
:efig.
:lblbox.
:elblbox.
:nt.
:ent.
:screen.
:escreen.
:table.
:etable.
:xmp.
:exmp.
Update CMS sequenced 8 file
The process option UPDCMS8 produces update files that are generally created for input to the CMS
UPDATE command. The CMS UPDATE command is described in z/VM® CMS Command Reference.
The UPDCMS8 process option is available for the LINE compare type only.
The old input file must have fixed-length 80-byte records with valid sequence numbers in columns 73
through 80. The new file must be fixed but may have a length less than or equal to 80.
The UPDCMS8 update file is fixed-length 80.
If the sequence numbers do not allow adequate room to insert changes from the new file, SuperC
changes the status of adjacent matched lines to find the room.
UPDCMS8 update files contain both CMS UPDATE control statements and source lines from the "new"
file. All UPDCMS8 control statements are identified by the characters "./" in columns 1 and 2 of the
80-byte record, followed by one or more spaces and a one-character control line identifier. The control
line identifiers are sequence (S), insert (I), delete (D), replace (R), and comment (*). Figure 277 on page
495 shows an example of a UPDCMS8 update file. 
 1  ./ * NEW:  JLEVERIN TEST2    A                          07/11/2008 11.35
 2  ./ * OLD:  JLEVERIN TEST1    A
 3  ./ R 00000100 00000100 $ 00000140 00000040
 4  This line is reformatted; the spacing in the "new" file differs.     00000100
 5  ./ R 00000300 00000300 $ 00000340 00000040
 6  This line differs from the text in the "old" file.                   00000300
 7  ./ I 00000400          $ 00001400 00001000
 8  This line is in the "new" file, but not in the "old".                00000500
Figure 277. Example of a UPDCMS8 update file 
The example in Figure 277 on page 495, has the following lines:
 1 
Comment line. Lists the new file name and the date and time of the comparison.
 2 
Comment line. Lists the old file name.
 3 
Replacement control line. The first 8-digit numeric field is the sequence number (of the old file) of
the first input number to be replaced. The second 8-digit numeric field is the sequence number of the
old file that is the last record to be replaced. The dollar sign is an option separator field. The third
and fourth 8-digit fields represent the first decimal number to be used for sequencing the substitute
records and the decimal increment to be used in the sequencing.
In this example, the first line of the old file is being replaced with one line from the new file.
 4 
The new record which has replaced the old record at sequence number 00000100.
Update files
Appendix C. Update files  495

## Page 534

5 
Another replacement control line.
 6 
The new record which has replaced the old record at sequence number 00000300.
 7 
Insert control line. After old line 4, there is a line inserted in the new file.
 8 
The text of the inserted line.
Update control data sets
The process option UPDCNTL produces a control data set that relates matches, insertions, deletions, and
re-formats to:
• The relative line numbers of the old and new data sets (LINE compare type); see Figure 278 on page
496.
• The relative word position of the old data set (WORD compare type); see Figure 279 on page 497.
• The relative byte offset (BYTE compare type); see Figure 280 on page 498.
Note: No source or data from either input data set is included in the update data set produced by
UPDCNTL.
Update control data set (LINE Compare Type)
 1 *    NEW:  USER1.TEXT.NEW                                       2021/06/30 11.06
 2 *    OLD:  USER1.TEXT.OLD                                                       
 3 *  N-LINE-# O-LINE-# MAT-LEN  INS-LEN  DEL-LEN REFM-LEN                         
 4    00000001 00000001                            00000001                        
 5    00000002 00000002 00000001                                                   
 6    00000003 00000003          00000001 00000001                                 
 7    00000004 00000004 00000001                                                   
 8    00000005 00000005          00000001                                          
 9 *  END
Figure 278. Example of a UPDCNTL update data set using line compare type
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
 3 
Header Comment line. For information about the columns, see Table 32 on page 497.
 4 
Shows that line 1 of the new data set is a reformatted line of line 1 of the old data set.
 5 
Line 2 from both data sets match.
 6 
Line 3 of the new data set replaces line 3 of the old data set.
 7 
Line 4 from both data sets match.
 8 
At line 5 of the new data set is an inserted line.
Update files
496  z/OS: z/OS ISPF User's Guide Vol II

## Page 535

9 
Comment line. This is the end of the update data set.
The following table shows the column numbers used for the UPDCNTL data set:
Table 32. UPDCNTL update file  format using LINE compare type
Column # Identifier Data Item
4-11 N-LINE-# New line number
13-20 O-LINE-# Old line number
22-29 MAT-LEN Match length
31-38 INS-LEN Insert length
40-47 DEL-LEN Delete length
49-56 REFM-LEN Reformat length
58-65 N-DP-LEN (Not shown) New “Do not Process” length
67-74 O-DP-LEN (Not shown) Old “Do not Process” length
76-80 N-MVL (Not shown) New “moved” line length.
Update control data set (WORD compare type)
 1 *    NEW:  USER1.TEXT.NEW                                       2021/06/30 11.22
 2 *    OLD:  USER1.TEXT.OLD                                                       
 3 *  N-LINE-# N-LN-LEN N-COL WD-MAT-# N-WD-INS O-WD-DEL O-LINE-# O-LN-LEN O-COL   
 4    00000001 00000003 00001 00000029                   00000001 00000003 00001   
 5    00000003 00000001 00040          00000001 00000001 00000003 00000001 00040   
 6    00000003 00000002 00046 00000011                   00000003 00000002 00046   
 7    00000005 00000001 00001          00000013                                    
 8 *  END     
Figure 279. Example of a UPDCNTL update data set using WORD compare type
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
 3 
Header comment line. For information about the columns, see Table 33 on page 498.
 4 
Beginning with line one column 1, of both files, the first twenty-seven words match. This takes us to
line 3.
 5 
There is 1 word replaced in line 3. It begins in column forty of each file.
 6 
Beginning from the change in  5 , there are 9 more words that match.
 7 
A line of thirteen words was inserted at line 5.
 8 
Comment line. Ends the update data set.
The following table shows the column numbers used for the UPDCNTL file:
Update files
Appendix C. Update files  497

## Page 536

Table 33. UPDCNTL update file  format using WORD compare type
Column # Identifier Data Item
4-11 N-LINE-# Beginning new line number
13-20 N-LN-LEN Number of lines
22-26 N-COL New column number (beginning of word)
28-35 WD-MAT-# Number of matching words
37-44 N-WD-INS Number of new inserted words
46-53 O-WD-DEL Number of old deleted words
55-62 O-LINE-# Beginning old line number
64-71 O-LN-LEN Number of old lines
73-77 O-COL Old column number
Update control data set (BYTE compare type)
 1 *    NEW:  USER1.TEXT.NEW                                       2021/06/30 11.29
 2 *    OLD:  USER1.TEXT.OLD                                                       
 3 *  N-BYTE-O O-BYTE-O MAT-LEN  INS-LEN  DEL-LEN                                  
 4    00000000 00000000 00000026                                                   
 5    00000026 00000026                   00000001                                 
      00000026 00000027 00000002                                                   
      00000028 00000029                   00000001                                 
      00000028 0000002A 00000004                                                   
      0000002C 0000002E                   00000001                                 
      0000002C 0000002F 00000007                                                   
      00000033 00000036                   00000001                                 
      00000033 00000037 00000004                                                   
      00000037 0000003B                   00000001                                 
      00000037 0000003C 00000009                                                   
      00000040 00000045          00000005                                          
      00000045 00000045 00000083                                                   
 6    000000C8 000000C8          00000003 00000003                                 
      000000CB 000000CB 00000075                                                   
 7    00000140 00000140          00000050                                          
 8 *  END                                                      
Figure 280. Example of a UPDCNTL update data set using BYTE compare type
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
 3 
Header comment line. For more information about the columns, see Table 34 on page 499.
 4 
First 38 (26 hex) bytes match.
 5 
1 byte is deleted.
 6 
(Skipping several lines). 3 bytes of the new data set replace 3 bytes of the old data set.
 7 
Fifty bytes inserted.
Update files
498  z/OS: z/OS ISPF User's Guide Vol II

## Page 537

8 
Comment line. Ends the update data set.
The following table shows the column numbers used for the UPDCNTL file:
Table 34. UPDCNTL update file  format using BYTE compare type
Column # Identifier Data Item
4-11 N-BYTE-O New byte offset
13-20 O-BYTE-O Old byte offset
22-29 MAT-LEN Number of matching bytes
31-38 INS-LEN Number of inserted bytes
40-47 DEL-LEN Number of deleted bytes
Update long control
The process option UPDLDEL produces an update data set that contains control records, matching new
data set source records, inserted new data set source records, and deleted old data set source records.
The UPDLDEL process option is available for the LINE compare type only.
Figure 281 on page 500 shows an example of a UPDLDEL update data set.
The control records are titled as follows:
*HDR1, *HDR2, *HDR3
Header titles and data
*M-
Matched line sequence header
*I-
Inserted line sequence header
*I-RP
Inserted line sequence header for replacement lines
*I-RF
Inserted line sequence header for reformatted lines
*D-
Deleted line sequence header
*D-RP
Deleted line sequence header for replacement lines
*D-RF
Deleted line sequence header for reformatted lines
Header control records are full length records that delimit the copied file records. This allows you to
quickly find changed areas. The records look like the information about a LONG listing. The two input data
set must both have the same fixed record length or each have a variable record length. 
Update files
Appendix C. Update files  499

## Page 538

*HDR1  USER1.TEXT.NEW                                       2021/06/30 11.46    
*HDR2  USER1.TEXT.OLD                                       TYPE = UPDLDEL      
*I-RF  INS#=  1      N-REF#=000001 O-REF#=000001  *****SUPERC CHANGE HEADER*****
This line is reformatted; the spacing in the "new" file differs.        00000100
*D-RF  DEL#=  1      N-REF#=000001 O-REF#=000001  *****SUPERC CHANGE HEADER*****
This line is reformatted; the spacing  in  the  "new"  file  differs.   00000100
*M-    MAT#=  1      N-REF#=000002 O-REF#=000002  *****SUPERC CHANGE HEADER*****
This line is the same in both files.                                    00000200
*I-RP  INS#=  1      N-REF#=000003 O-REF#=000003  *****SUPERC CHANGE HEADER*****
This line differs from the text in the "old" file.                      00000300
*D-RP  DEL#=  1      N-REF#=000003 O-REF#=000003  *****SUPERC CHANGE HEADER*****
This line differs from the text in the "new" file.                      00000300
*M-    MAT#=  1      N-REF#=000004 O-REF#=000004  *****SUPERC CHANGE HEADER*****
This line is the same in both files.                                    00000400
*I-    INS#=  1      N-REF#=000005 O-REF#=000004  *****SUPERC CHANGE HEADER*****
This line is in the "new" file, but not in the "old".                   00000500      
Figure 281. Example of a UPDLDEL update data set
Update MVS sequenced 8 data set
The process option UPDMVS8 produces a data set that contains both control records and new data set
source lines using sequence numbers from old data set columns 73 to 80.
The UPDMVS8 process option is available for the LINE Compare Type only.
The format of the generated data may be suitable as input to the IEBUPDTE utility. See MVS/DFP Utilities
for information about the contents of this data set. Figure 282 on page 500 shows an example of a
UPDMVS8 update data set created on CMS. 
 1 ./  CHANGE LIST=ALL OLD:USER1.TEXT.OLD                                          
 2 ./ DELETE  SEQ1=00000100,SEQ2=00000100                                          
 3 This line is reformatted; the spacing in the "new" file differs.        00000100
 4 ./ DELETE  SEQ1=00000300,SEQ2=00000300                                          
 5 This line differs from the text in the "old" file.                      00000300
 6 This line is in the "new" file, but not in the "old".                   00000500
Figure 282. Example of a UPDMVS8 update data set
 1 
Control record. Lists old data set name.
 2 
Control record. Shows record deleted at sequence number 100 on the old data set.
 3 
Inserted line from the new data set.
 4 
Control record. Shows record deleted at sequence number 300 on the old data set.
 5 
Inserted line from the new data set.
 6 
Inserted line from the new data set.
The data sets to be compared must have fixed-length 80-byte records. They must also contain sequence
numbers.
Update files
500  z/OS: z/OS ISPF User's Guide Vol II

## Page 539

Update prefixed delta lines
The process option UPDPDEL produces a variable-length update file that contains header records and
complete delta lines from the input files, up to a maximum of 32K bytes in each output line.
The UPDPDEL process option is available for the LINE compare type only.
Figure 283 on page 501 shows an example of a UPDPDEL update data set.
Prefix codes (I for insert and D for delete) together with the line number precede lines from the input data
sets. Sub-totals are shown before each group of flagged records:
• INS#= for the number of consecutive inserted records,
• DEL#= for the number of consecutive deleted records,
• RPL#= for the number of consecutive pairs of replaced records, and
• MAT#= for the number of intervening matched records.
 1 *    NEW:  USER1.TEXT.NEW                                       2021/06/30 15.13               
 1 *    OLD:  USER1.TEXT.OLD                                                                      
 3 *ID-   LINE#       SOURCE LINE                                                                 
 4 *                                   RPL#= 00000001                                             
 5 I - 00000001  This line is reformatted; the spacing in the "new" file differs.        00000100 
 6 D - 00000001  This line is reformatted; the spacing  in  the  "new"  file  differs.   00000100 
 4 *                                   RPL#= 00000001                MAT#= 00000001               
 5 I - 00000003  This line differs from the text in the "old" file.                      00000300 
 6 D - 00000003  This line differs from the text in the "new" file.                      00000300 
 7 *                                   INS#= 00000001                MAT#= 00000001               
 8 I - 00000005  This line is in the "new" file, but not in the "old".                   00000500 
 9 *   END                                                                                        
Figure 283. Example of a UPDPDEL update data set
The example in Figure 283 on page 501 has the following lines:
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
 3 
Header comment line.
 4 
Sub-total line showing that 1 replaced pair of records follow.
 5 
The line that has replaced the line in the old data set.
 6 
The line in the old data set that has been replaced.
 7 
Sub-total line showing that 1 inserted record follows.
 8 
The line that has been inserted in the new data set.
 9 
Comment line. Ends the update data set.
Update sequenced 0 data set
The process option UPDSEQ0 produces a control data set that relates insertions and deletions to the
relative line numbers of the old data set. UPDSEQ0 is like UPDCMS8, but uses relative line numbers
instead of sequence numbers from the old data set.
The UPDSEQ0 process option is available for the LINE compare type only.
Update files
Appendix C. Update files  501

## Page 540

This update data set is characterized by control statements followed by source lines from the new
data set. All UPDSEQ0 control statements are identified by the characters "./" in columns 1 and 2 of
the 80-byte record, followed by one or more spaces and additional space-delimited fields. The control
statements are insert (I), delete (D), replace (R), and comment (*). Control statement data does not
extend beyond column 50. Figure 284 on page 502 shows an example of a UPDSEQ0 update data set. 
 1 ./ * NEW:  USER1.TEXT.NEW                                       2021/06/30 15.38 
 2 ./ * OLD:  USER1.TEXT.OLD                                                        
 3 ./ R 00000001 00000001 $ 00000001                                                
 4 This line is reformatted; the spacing in the "new" file differs.        00000100 
 5 ./ R 00000003 00000003 $ 00000001                                                
 6 This line differs from the text in the "old" file.                      00000300 
 7 ./ I 00000004          $ 00000001                                                
 8 This line is in the "new" file, but not in the "old".                   00000500
Figure 284. Example of a UPDSEQ0 update data set
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
 3 
Replacement control record. Beginning at the first record of the old data set, replace 1 record. The
numeric value after the dollar sign specifies the number of new data set source lines that follow the
control record.
 4 
Text of new data set line to replace line 1.
 5 
Replace the third record with 1 record.
 6 
Text of new data set line to replace line 3.
 7 
Insert control line. Insert 1 line after record 4 of old data set.
 8 
Text of inserted line.
Update summary only data sets
The process option UPDSUMO produces an update data set of 4 lines: new data set name, old data set
name, column headers, and a summary totals line.
The UPDSUMO process option is available for the LINE, WORD, and BYTE compare types.
The summary totals line has a "T" in column 1. The summary statistics are located at fixed offsets in the
output line. The data set has a line length of 132 bytes.
Update files
502  z/OS: z/OS ISPF User's Guide Vol II

## Page 541

Update summary only data set (LINE compare type)
 1 *    NEW:  USER1.TEXT.NEW                                       2021/06/30
 2 *    OLD:  USER1.TEXT.OLD                                                 
 3 *  NEW-PROC OLD-PROC NEW-INS  OLD-DEL  TOT-CHG  TOT-RFM  FI-PROC  FI-DIFF 
 4 T  00000005 00000004 00000002 00000001 00000003 00000001 00000001 00000001
. . (Continuation of previous data lines) . . . . . . . . . . . .
 1 15.55                                  
 2                                        
 3 N-NOT-PD O-NOT-PD N-DP-LNS O-DP-LNS    
 4 00000000 00000000 00000000 00000000    
Figure 285. Example of a UPDSUMO data set using LINE compare type
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
 3 
Comment line. Header line. Columns are explained in Table 35 on page 503.
 4 
Totals line.
In Figure 285 on page 503, the update summary data set is shown in split screen mode. The bottom half
of the screen shows the result of scrolling right to see the remainder of the member.
The following table shows the column numbers used to display the update information:
Table 35. UPDSUMO format using LINE compare type
Column # Identifier Data Item
NEW-PROC Number of new lines processed
OLD-PROC Number of old lines processed
NEW-INS Number of new line insertions
OLD-DEL Number of old line deletions
TOT-CHG Total number of line changes
TOT-RFM Total number of reformats
FI-PROC Total number of data sets/members processed
FI-DIFF Total number of data sets/members different
N-NOT-PD Total new data sets/members not processed
O-NOT-PD Total old data sets/members not processed
N-DP-LNS Total number of new "do not process" lines
O-DP-LNS Total number of old "do not process" lines
Update files
Appendix C. Update files  503

## Page 542

Update summary only data set (WORD compare type)
 1 *    NEW:  USER1.TEXT.NEW                                       2021/06/30 16.11              
 2 *    OLD:  USER1.TEXT.OLD                                                                     
 3 *  NEW-PROC OLD-PROC NEW-INS  OLD-DEL  TOT-CHG           FI-PROC  FI-DIFF  N-NOT-PD O-NOT-PD  
 4 T  00000054 00000041 00000014 00000001 00000014          00000001 00000001 00000000 00000000  
Figure 286. Example of a UPDSUMO data set using WORD compare type
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
 3 
Comment line. Header line. Columns are explained in Table 36 on page 504.
 4 
Totals line.
In Figure 286 on page 504, the UPDSUMO data set is shown in split screen mode. The bottom half of the
screen is scrolled right to show the remainder of the member.
The following table shows the column numbers used to display the update information:
Table 36. UPDSUMO format using WORD compare type
Column # Identifier Data Item
NEW-PROC Number of new words processed
OLD-PROC Number of old words processed
NEW-INS Number of new word insertions
OLD-DEL Number of old word deletions
TOT-CHG Total number of word changes
FI-PROC Total number of data sets/members processed
FI-DIFF Total number of data sets/members different
N-NOT-PD Total new data sets/members not processed
O-NOT-PD Total old data sets/members not processed
Update summary only data sets (BYTE compare type)
 1 *    NEW:  USER1.TEXT.NEW                                       2021/06/30 16.27             
 2 *    OLD:  USER1.TEXT.OLD                                                                    
 3 *  NEW-PROC OLD-PROC NEW-INS  OLD-DEL  TOT-CHG           FI-PROC  FI-DIFF  N-NOT-PD O-NOT-PD 
 4 T  00000400 00000320 00000088 00000008 00000093          00000001 00000001 00000000 00000000
Figure 287. Example of a UPDSUMO data sets using BYTE compare type
 1 
Comment line. Lists the new data set name and the date and time of the comparison.
 2 
Comment line. Lists the old data set name.
Update files
504  z/OS: z/OS ISPF User's Guide Vol II

## Page 543

3 
Comment line. Header line. Columns are explained in Table 37 on page 505.
 4 
Totals line.
In Figure 287 on page 504, the UPDSUMO file is shown in split screen mode. The bottom half of the
screen shows the result of scrolling right to see the remainder of the member.
The following table shows the column numbers used to display the update information:
Table 37. UPDSUMO format using BYTE compare type
Column # Identifier Data Item
NEW-PROC Number of new bytes processed
OLD-PROC Number of old bytes processed
NEW-INS Number of new byte insertions
OLD-DEL Number of old byte deletions
TOT-CHG Total number of byte changes
FI-PROC Total number of data sets/members processed
FI-DIFF Total number of data sets/members different
N-NOT-PD Total new data sets/members not processed
O-NOT-PD Total old data sets/members not processed
Update files
Appendix C. Update files  505

## Page 544

Update files
506  z/OS: z/OS ISPF User's Guide Vol II
