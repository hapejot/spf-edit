# Appendix B. Understanding the listings

Source file: f54u200_v3r1.md
Start page: 515
Page span: 515-530

## Page 515

Appendix B. Understanding the listings
SuperC allows you to produce a range of listings (reports) which provide detailed information about the
results of your comparison or search.
General listing format
The format and content of each type of listing depends on:
• Whether you are using the SuperC Comparison or the SuperC Search
• The listing type used
Note: The NOLIST listing type suppresses the generation of any listing output or listing file.
• Whether you are comparing (or searching) a sequential file or a partitioned data set
• The compare type used (in the case of the SuperC Comparison)
• The process options used
• The process statements used
Note: Dates in the heading lines on the sample listing output in this document appear in the format
MM/DD/YYYY. The dates in the heading lines appear in the format YYYY/MM/DD.
How to view the listing output
The listing output is always written to a listing file  (unless the NOLIST listing type is used), from which you
can print the listings.
The following pages contain:
• A description of the general format of the comparison listing (see “The comparison listing” on page
477), followed by examples of various listings produced by the SuperC Comparison.
• A description of the general format of the search listing (ssee “The search listing” on page 487),
followed by examples of various listings produced by the SuperC Search.
The comparison listing
SuperC comparison listings consist of four basic parts (although not all parts are present for all types of
listing output produced):
• Page Headings (see “Page headings” on page 477)
• Listing Output Section (see page “Listing output section” on page 478)
• Member Summary Listing (see “Member summary section” on page 480)
• Overall Summary Section (see “Overall summary section” on page 482)
Page headings
SuperC generates a page heading at the top of each page. The heading consists of two lines of
information. 
1  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS         2021/06/21  14.12    PAGE     1    NEW: USER1.CLIST                                             
Figure 254. Example of page heading lines for the comparison listing
Figure 254 on page 477 shows typical page heading lines. The first line contains:
Understanding the listings
© Copyright IBM Corp. 1980, 2024 477

## Page 516

• Printer control page eject character ("1" in column one. Not present when the NOPRTCC process option
is specified)
• "Platform-identifier". This shows "MVS".
• Program identification title: COMPARE UTILITY - ISPF FOR z/OS 
• The date and time of the compare
• The page number
Note: The program version and program date are important when reporting suspected SuperC problems.
The second heading line identifies the new and old data sets. Normally this line shows the names of the
new and old data sets. However, if the NTITLE and OTITLE process statements have been specified then
the corresponding alternative data set titles are shown instead of the data set names.
Listing output section
The listing output section shows where and what the changes are. Figure 255 on page 478 is an example
of a Listing Output Section for a LINE comparison with a listing type of DELTA. 
 1              LISTING OUTPUT SECTION (LINE COMPARE)
 2  ID      SOURCE LINES          |…|        TYPE  LEN N-LN# O-LN#
 3      ----+----1----+----2----+-|…|-+----8
 4  I - 970521                    |…|        RPL=    1 00001 00001
 5  INFO  Date cols 11:15 packed 2|…|
 6  D - 970522
 5  INFO  Date cols 11:14 packed 1|…|
Figure 255. Example of the listing output section of the comparison listing
 1 
Section title line. It tells you that this is a LINE comparison. Possible compare types are BYTE, FILE,
LINE, and WORD.
 2 
Column header line.
ID
A two-column prefix code that identifies the status of the line. See “Listing prefix codes” on page
479.
SOURCE LINES
The actual text or data from the source data sets. Under this heading, the actual data from the
data sets is listed.
TYPE
Further breakdown of the ID field. See “Type of difference codes” on page 480 for information
about TYPE codes.
LEN
The "length" or number of consecutive lines of the selected type.
N-LN#
Indicates the relative record (line) number of this line (or where it is to be inserted) in the new
source data set. Numbers are in decimal.
O-LN#
Indicates the relative record (line) number of this line (or where it was deleted from) in the old
source data set. Numbers are in decimal.
 3 
The scale of the column positions of the input source lines.
Understanding the listings
478  z/OS: z/OS ISPF User's Guide Vol II

## Page 517

4 
An inserted (I) line. The RPL in TYPE indicates that it is a replacement line. This replacement involves
the line 00001 in both files.
Note: Occasionally, you may see some "unusual" characters on the inserted (I) and deleted (D) lines.
These characters represent data that is in a non-character (and therefore not directly printable)
format in the input record. Ignore them.
 5 
An information line that is generated on a comparison listing when a Date Definition process
statement is used (see “Date definitions” on page 464) and when the preceding inserted (I) line
or deleted (D) line contains a date. The information line shows you the content of the date field as it
exists on the input file and the date as used in the comparison. For a full example, see Figure 258 on
page 484.
 6 
A deleted (D) line.
Listing prefix codes
SuperC output lines are flagged with the following prefix codes listed under the ID column:
(space)
Match No prefix code means the data is the same in both data sets.
I
Insert Data that is in the new data set, but is missing 5 from the sequence in the old data set.
D
Delete Data that is in the old data set, but is missing 5 from the sequence in the new data set.
DR
Delete Replace For BYTE compare type only. The bytes in the old data set that were replaced by the
bytes shown in the preceding insert (I) line.
RN
Reformat New For LINE compare type only. A reformatted line in the new data set. This line contains
the same data as the old data set line, but with different spacing.
RO
Reformat Old For LINE compare type only. A line in the old data set that is reformatted in the new data
set. This line is not shown if the DLREFM process option is used.
MC
Match Compose For WORD compare type only. A line containing words that match. The line may also
contain spaces to show the relationship between the matching words and any inserted or deleted
words. Inserted and deleted words are shown in following insert compose (IC) and delete compose
(DC) lines.
IC
Insert Compose For WORD compare type only. A line containing words from the new data set that are
not in the old data set. This line normally follows a match compose (MC) line.
DC
Delete Compose For WORD compare type only. A line containing words from the old data set that are
not in the new data set. This line normally follows a match compose (MC) or insert compose (IC) line.
IM
Insert Moved For comparison listings created using the FMVLNS (flag moved lines) process option. A
line in the new data set that also appears in the old data set, but has been moved. If the line was
reformatted, this is indicated by a flag to the right of the listing.
5 "Missing" data is data that is missing from the data sequence but may exist in some other part of the data
set.
Understanding the listings
Appendix B. Understanding the listings  479

## Page 518

DM
Delete Moved For comparison listings created using the FMVLNS (flag moved lines) process option. A
line in the old data set that also appears in the new data set, but has been moved. If the line was
reformatted, this is indicated by a flag to the right of the listing.
|
Change Bar For comparison listings created using the GWCBL (generate WORD/LINE comparison
change bar listing) process option. A change bar showing that words/lines were either inserted or
deleted.
Type of difference codes
At the far right of some listings are headings that provide additional information about the numbers and
types of differences SuperC has found. Headings you may see are:
MAT=
Number of matched lines.
RFM=
Number of reformatted lines.
RPL=
Number of replaced lines.
INS=
Number of lines that are in the new data set, but missing in the old data set.
DEL=
Number of lines that are in the old data set, but missing in the new data set.
IMR=
Number of lines in the new data set that have been moved from where they were in the old data set
and reformatted. The listing shows a matching "DMR=" flag for a line in the old data set.
DMR=
Number of lines in the old data set that have been moved and reformatted in the new data set. The
listing shows a matching "IMR=" flag for a line in the new data set.
IMV=
Number of lines in the new data set that have been moved from where they were in the old data set.
The listing shows a matching "DMV=" flag for a line in the old data set.
DMV=
Number of lines in the old data set that have been moved in the new data set. The listing shows a
matching "IMV=" flag for a line in the new data set.
Member summary section
SuperC generates the member summary section when you specify a partitioned data set. The member
summary section is really two sections with a page separator between them.
Figure 256 on page 481 shows an example of the two member summary sections for a FILE compare
type.
The first section indicates which files were compared and whether they were found to be
different or the same. In Figure 256 on page 481, PRJ0005.RELEASE.SOURCE was compared to
PRJ0009.RELEASE.SOURCE. The members compared were:
• FLM01EQU
• FLM01MD1
• FLM01MD3
• FLM01MD4
• FLM01MD5
• FLM01MD6
Understanding the listings
480  z/OS: z/OS ISPF User's Guide Vol II

## Page 519

Differences were found in:
• FLM01MD1
• FLM01MD5
• FLM01MD6
Following the member statistics are the group statistics. As this was a FILE comparison, the statistics are
in terms of files and the number of bytes in each file.
Note: Different compare types produce slightly different results in the first section.
The second part of the member summary section shows all the members from both the new and old data
sets which were not paired (and hence not compared). In Figure 256 on page 481, only FLM01MD2 from
the new data set was not compared to any file from the old data set. 
 1                        MEMBER SUMMARY LISTING (FILE COMPARE)
 2  DIFF SAME              MEMBERS-COMPARED  N-BYTES O-BYTES N-LINES O-LINES N-HASH-SUM O-HASH-SUM
 3   **       FLM01MD1             1520    1520      19      19  75A20517   75A20509
     **       FLM01MD3             1520    1520      19      19  75A20919   75A20919
     **       FLM01MD4             1520    1520      19      19  75A20B1A   75A20B1A
  **          FLM01MD5             1520    1520      19      19  75A20D1B   75A20D0D
  **          FLM01MD6             1520    1520      19      19  75A20F1C   75A20F0E
              -------------      ------- ------- ------- -------
 4   MEMBER TOTALS        7600    7600      95      95
 5      5   TOTAL MEMBER(S) PROCESSED AS A PDS           
 6      3   TOTAL MEMBER(S) PROCESSED HAD CHANGES
 7      2   TOTAL MEMBER(S) PROCESSED HAD NO CHANGES
 8      1   TOTAL NEW FILE MEMBER(S) NOT PAIRED
 9      0   TOTAL OLD FILE MEMBER(S) NOT PAIRED
   ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS         2021/06/23  13.46    PAGE     2
NEW: PRJ0005.RELEASE.SOURCE                                  OLD: PRJ0009.RELEASE.SOURCE
                       MEMBER SUMMARY LISTING (FILE COMPARE)
        NON-PAIRED NEW FILE MEMBERS           |     NON-PAIRED FILE MEMBERS
 10  FLM01MD2                           | 
Figure 256. Example of the member summary section of the comparison listing
 1 
Section Header.
 2 
Header line. Consists of several column headers.
DIFF
Contains "**" when the new and old data sets differ.
SAME
Contains "**" when the new and old data sets are the same.
MEMBERS-COMPARED
The paired members of the data sets compared.
N-BYTES
Number of bytes processed in the new member.
O-BYTES
Number of bytes processed in the old member.
N-LINES
Number of lines processed in the new member.
O-LINES
Number of lines processed in the old member.
N-HASH-SUM
SuperC generated a hash value for the new member.
Understanding the listings
Appendix B. Understanding the listings  481

## Page 520

O-HASH-SUM
SuperC generated a hash value for the old member.
Note: The hashsums of files can be used to compare two members that are not physically on the
same system. If the hashsum of a member on system A is different from the hashsum of a member
on system B, then the members can be said to be different. If the hashsum of the members are
identical, there is a high probability that the members are the same. As secondary confirmation that
the members are the same, compare the number of lines and number of bytes.
 3 
Member comparison statistics.
 4 
Member totals header line.
 5 
Total number of members.
 6 
Total number of members compared that had differences.
 7 
Total number of files compared that had no differences.
 8 
Total number of new file members that were not paired (and therefore were not compared).
 9 
Total number of old file members that were not paired (and therefore were not compared).
FLM01MD2 was present in the new data set. It could not be paired with a similarly named member in the
old data set and was not processed.
Overall summary section
The overall summary section gives the overall statistics of the comparison process. Figure 257 on page
482 is a representative example of an overall summary section. 
 1         PDS LINE OVERALL TOTALS
 2  95 NUMBER OF LINE MATCHES    8  21  TOTAL CHANGES (PAIRED+NONPAIRED
 3   0 REFORMATTED LINES         9  21  PAIRED CHANGES (REFM+PAIRED INS
 4  21 NEW FILE LINE INSERTIONS 10   0  NON-PAIRED INSERTS
 5  21 OLD FILE LINE DELETIONS  11   0  NON-PAIRED DELETES
 6 116 NEW FILE LINES PROCESSED
 7 116 OLD FILE LINES PROCESSED
 12  LISTING-TYPE = OVSUM   13  COMPARE-COLUMNS =    1:80   14  LONGEST-LINE = 80
 15  PROCESS OPTIONS USED: NOSEQ NOPRTCC
Figure 257. Example of the overall summary section of the comparison listing
Figure 257 on page 482 shows the following information about the comparison:
 1 
The second word of the title tells you the type of comparison. The overall summary is provided for
BYTE, FILE, LINE, and WORD compare types.
 2 
Of the lines in each data set, 95 from the new data set matched 95 corresponding lines of the old data
set. These are called matching lines.
 3 
There are no reformatted lines.
 4 
There are 21 inserted lines in the new file.
Understanding the listings
482  z/OS: z/OS ISPF User's Guide Vol II

## Page 521

5 
The old file contains 21 lines that are missing from the new source file.
 6 
116 lines from the new file were processed.
 7 
The old file also has a total of 116 lines.
 8 
The total number of changes is a summation of items  9 ,  10 , and  11 . It is a convenient number that
best represents the change activity of the two compared files.
 9 
The total number of reformats and paired changes. This represents a sum of items that may be
considered to be a single change. That is, some changes are made in pairs and need only be counted
as a single instance of a change.
 10 
There were no non-paired inserts. Non-paired inserts are changes to the new file that have no
relationship to the old file (that is, no deletes from the old file occurred in the same area).
 11 
There were no non-paired deletes. Non-paired deletes are changes to the old file that have no
relationship to the new file (that is, no inserts to the new file occurred in the same area).
 12 
The listing type is OVSUM. This is the listing type option selected for the comparison. Other options
are: DELTA, CHNG, and LONG.
 13 
SuperC compared columns 1 through 80. This value provides a convenient reference for confirming if
all the columns in the line have been compared or only some portion of the line.
 14 
The longest line length of any line in either file is 80 characters.
 15 
The process options used were NOSEQ and NOPRTCC.
Examples of comparison listings
The following represent some of the output types available from SuperC. 
Understanding the listings
Appendix B. Understanding the listings  483

## Page 522

1  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS
 NEW: USER1.DATA                                              OLD: USER1.DATA2   
                      LISTING OUTPUT SECTION (LINE COMPARE)
ID      SOURCE LINES                                                                
    ----+----1----+----2----+----3----+----4----+----5----+----6----+----7----+----8
                                                                                     
I - FLM01CDT                     7   
2017/03/02                                                                       
INFO   Date cols 34:43  char 2017/03/02       Comp=(2017/03/02)
D - FLM01CDT    
INFO   Date cols 34:41  char 99/05/02         Comp=(1999/05/02)
   FLM01CD7                    23   2017/03/02
1  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS
 NEW: USER1.DATA                                              OLD: USER1.DATA2
                      LINE COMPARE SUMMARY AND STATISTICS
      1 NUMBER OF LINE MATCHES               1  TOTAL CHANGES (PAIRED+NONPAIRED CHNG)
      0 REFORMATTED LINES                    1  PAIRED CHANGES (REFM+PAIRED INS/DEL)
      1 NEW FILE LINE INSERTIONS             0  NON-PAIRED INSERTS
      1 OLD FILE LINE DELETIONS              0  NON-PAIRED DELETES
      2 NEW FILE LINES PROCESSED
      2 OLD FILE LINES PROCESSED
LISTING-TYPE = CHNG  COMPARE-COLUMNS =    1:72     LONGEST-LINE = 80
PROCESS OPTIONS USED: SEQ(DEFAULT)
THE FOLLOWING PROCESS STATEMENTS (USING COLUMNS 1:72) WERE PROCESSED:
   Y2PAST 1987
   NY2C 34:43 YYYY/MM/DD
   OY2C 34:41 YY/MM/DD
Figure 258. Example of comparison listing with dates being compared
In Figure 258 on page 484, the two date definition process statements have each caused an information
("INFO") line to be generated. The information line shows:
• The position of the defined date in the record.
• The contents of the defined date field.
• The date as it was actually compared. In the second information line, you can see the defined date has a
2-digit year portion ("97") but has actually been compared using a 4-digit year portion ("1997").
For further details, see “Date definitions” on page 464.
Note: Occasionally, you may see some "unusual" characters on the inserted (I) and deleted (D) lines.
These characters represent data that is in a non-character (and therefore not directly printable) format in
the input record. Ignore them.
Understanding the listings
484  z/OS: z/OS ISPF User's Guide Vol II

## Page 523

ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS
NEW: USER1.DATA2                                             OLD: USER1.DATA
                      LISTING OUTPUT SECTION (LINE COMPARE)
ID      SOURCE LINES                                                                 
    Account  Birth       Surname
    Number   Date                                                                    
I - 111222   1989/02/15  JONES                                                       
D - 111111   63/04/07    JONES 
  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS      
NEW: USER1.DATA2                                             OLD: USER1.DATA
                      LINE COMPARE SUMMARY AND STATISTICS
      0 NUMBER OF LINE MATCHES               1  TOTAL CHANGES (PAIRED+NONPAIRED CHNG)
      0 REFORMATTED LINES                    1  PAIRED CHANGES (REFM+PAIRED INS/DEL)
      1 NEW FILE LINE INSERTIONS             0  NON-PAIRED INSERTS
      1 OLD FILE LINE DELETIONS              0  NON-PAIRED DELETES
      1 NEW FILE LINES PROCESSED
      1 OLD FILE LINES PROCESSED
LISTING-TYPE = CHNG  COMPARE-COLUMNS =    1:72     LONGEST-LINE = 80
PROCESS OPTIONS USED: SEQ(DEFAULT) NOPRTCC
THE FOLLOWING PROCESS STATEMENTS (USING COLUMNS 1:72) WERE PROCESSED:
   COLHEAD 'Account','Number',1:8,N 1:8 C,O 1:6 C
   COLHEAD 'Birth','Date',10:20,N 10:19 C,O 10:17 C
   COLHEAD 'Surname',,22:61,N 22:61 C,O 22:61 C
Figure 259. Example of comparison listing with column headings (Using COLHEAD)
In Figure 259 on page 485, COLHEAD process statements have been used to generate column headings
("Account Number", "Birth Date", and "Surname") for the corresponding input data. For further details,
see “Define column headings” on page 454. 
ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS         2021/06/24  16.3
NEW: USER1.DATA                                              OLD: USER1.DATA2                           
                      LISTING OUTPUT SECTION (LINE COMPARE)
ID          NEW FILE LINES          ID          OLD FILE LINES     
   ----+----1----+----2----+----3----+----4----+-      ----+----1----+----2----+----3----+----4
RN-This line is reformatted.                      | RO-    This line is reformatted.           
   This line is the same in both data sets.       |    This line is the same in both data sets.
I -This line differs between data sets.           | D -This line differs between the data sets.
   This line is the same in both data sets.       |    This line is the same in both data sets.
I -This line is only in the new data set.         |                                            
  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS         2021/06/24  16.3
NEW: USER1.DATA                                              OLD: USER1.DATA2                           
                   LINE COMPARE SUMMARY AND STATISTICS
   2 NUMBER OF LINE MATCHES            3  TOTAL CHANGES (PAIRED+NONPAIRED CHNG)
   1 REFORMATTED LINES                 2  PAIRED CHANGES (REFM+PAIRED INS/DEL)
   2 NEW FILE LINE INSERTIONS          1  NON-PAIRED INSERTS
   1 OLD FILE LINE DELETIONS           0  NON-PAIRED DELETES
   5 NEW FILE LINES PROCESSED
   4 OLD FILE LINES PROCESSED
LISTING-TYPE = CHNG   COMPARE-COLUMNS =    1:72     LONGEST-LINE = 80
PROCESS OPTIONS USED: SEQ(DEFAULT) NARROW NOPRTCC
ISRS004I LISTING LINES MAY BE TRUNCATED DUE TO LIMITING OUTPUT LINE WIDTH.
Figure 260. Example of a NARROW side-by-side listing
In Figure 260 on page 485, the new and old files are shown side-by-side. The NARROW listing type allows
SuperC to output 55 columns from each file. Notice how the inserts and deletes are horizontally aligned
with each other. 
Understanding the listings
Appendix B. Understanding the listings  485

## Page 524

ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS         2021/06/24  16.44 
NEW: USER1.DATA                                              OLD: USER1.DATA2                             
                                                                                                          
                     LISTING OUTPUT SECTION (LINE COMPARE)                                                
                                                                                                          
ID          NEW FILE LINES                                   ID           OLD FILE LINES                  
   ----+----1----+----2----+----3----+----4----+      ----+----1----+----2----+----3----+----4--
RN-This line is reformatted.                     | RO-    This line is reformatted.             
   This line is the same in both data sets.      |                                              
I -This line differs between data sets.          | D -This line differs between the data sets.  
   This line is the same in both data sets.      |                                              
I -This line is only in the new data set.        |                                              
  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS         2021/06/24  16.44 
NEW: USER1.DATA                                              OLD: USER1.DATA2                             
                                                                                                          
                       LINE COMPARE SUMMARY AND STATISTICS                                                
                                                                                                          
       2 NUMBER OF LINE MATCHES               3  TOTAL CHANGES (PAIRED+NONPAIRED CHNG)                    
       1 REFORMATTED LINES                    2  PAIRED CHANGES (REFM+PAIRED INS/DEL)                     
       2 NEW FILE LINE INSERTIONS             1  NON-PAIRED INSERTS                                       
       1 OLD FILE LINE DELETIONS              0  NON-PAIRED DELETES                                       
       5 NEW FILE LINES PROCESSED                           
       4 OLD FILE LINES PROCESSED                                              
                                                                               
LISTING-TYPE = CHNG       COMPARE-COLUMNS =    1:72        LONGEST-LINE = 80   
PROCESS OPTIONS USED: SEQ(DEFAULT) NARROW DLMDUP NOPRTCC                       
                                                                               
ISRS004I LISTING LINES MAY BE TRUNCATED DUE TO LIMITING OUTPUT LINE 
WIDTH.                                                   
Figure 261. Example of a NARROW side-by-side listing (with DLMDUP)
Figure 261 on page 486, is like the previous example (Figure 260 on page 485) except that the process
option DLMDUP has been used to suppress the matched lines from the old file section. This simplifies the
combined listing output, allowing the changes to stand out more clearly. 
  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS       
NEW: USER1.DATA                                              OLD: USER1.DATA2         
                                                                                      
                     LISTING OUTPUT SECTION (WORD COMPARE)                            
                                                                                      
ID           SOURCE LINES (COMPARED COLUMNS)                                          
                                                                                      
   This line is reformatted; the spacing  in  the  "new" file   differs.              
   This line is the same in both data sets.                                           
MC-This line differs from the text in the       file.                                 
IC-                                       "old"                                       
DC-                                       "new"                                       
   This line is the same in both data sets.                                           
I -This line is only in the "new" data set.                                           
  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS       
NEW: USER1.DATA                                              OLD: USER1.DATA2         
                                                                                      
                       WORD COMPARE SUMMARY AND STATISTICS                            
                                                                                      
      38 NUMBER OF WORD MATCHES              10  TOTAL CHANGES (PAIRED+NONPAIRED CHNG)
      10 NEW FILE WORD INSERTIONS             2  NEW FILE LINES CHANGED/INSERTED      
       1 OLD FILE WORD DELETIONS              1  OLD FILE LINES CHANGED/DELETED       
      48 NEW FILE WORDS PROCESSED             5  NEW FILE LINES PROCESSED      
      39 OLD FILE WORDS PROCESSED             4  OLD FILE LINES PROCESSED      
                                                                               
LISTING-TYPE = CHNG       COMPARE-COLUMNS =    1:80        LONGEST-LINE = 80   
PROCESS OPTIONS USED: NOPRTCC 
Figure 262. Example of a WIDE side-by-side listing
In Figure 262 on page 486, the new and old files are shown side-by-side in a WIDE listing. SuperC lists 80
columns from each file. Notice how the inserts and deletes are horizontally aligned with each other.
Note: The output file has a LRECL of 202/203 and may require special processing and printer capability to
obtain a hard copy. Refer to the previous NARROW option examples if the large LRECL requirement cannot
be satisfied and a side-by-side listing is still required.
Understanding the listings
486  z/OS: z/OS ISPF User's Guide Vol II

## Page 525

The search listing
The typical search listing is composed of three parts:
• Page Heading
• Source Lines Section
• Summary Section
Page heading
SuperC generates a page heading at the top of each page. 
1  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS         2020/07/20   9.21    PAGE     1
Figure 263. Example of the page heading line for the search listing
Figure 263 on page 487 shows a typical page heading line. It contains:
• Printer control page eject character ("1" in column one. not present when the NOPRTCC process option
is specified).
• "Platform-identifier". This shows "MVS".
• Program identification title: COMPARE UTILITY - ISPF FOR z/OS 
• The date and time of the search
• The page number.
Note: The program version and program date are important when reporting suspected SuperC problems.
Source lines section
The source lines section provides detailed information about the results of the Search. 
 1    SOURCE SECTION                    SRCH DSN: USER1.SF
 2         1     This NEW file is FIXED 80 with sequence numbers
           2  /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT
           3  (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.
           4  ! * NEW: Use DPPDCMT for this comment.
           5  * * NEW: Use DPACMT to remove this assembler type comment
           6  -- *NEW: Use DPADCMT to remove this line.
           7  * NEW: FORTRAN comment. Remove with DPFTCMT.
           8  &&& This NEW line comes out with a DPLINE '&&&'
Figure 264. Example of the source lines section of a search listing
Figure 264 on page 487 is an example showing the source line section. Only one character string ("NEW")
was specified for the search.
 1 
Column Header Line.
LINE-#
Relative line number of the line where the string was found.
SOURCE LINES
Up to 106 characters of the source line where the string was found.
SRCH DSN:
Identifies the data set which was searched. In this example, it is USER1.SF.
Understanding the listings
Appendix B. Understanding the listings  487

## Page 526

2 
Text Lines. Relative line numbers and text lines from the search file where the string "NEW" was
found.
The format of the source lines section changes when certain process options are used:
IDPFX ("Identifier Prefixed")
The member name is prefixed to each line of source text. See “Source lines section (IDPFX)” on page
488.
LMTO ("List Group Member Totals")
Only the totals of lines found and processed are listed. See “Source lines section (LMTO)” on page
488.
XREF ("Cross-reference Strings")
Creates a cross-reference listing by search string. See “Source lines section (XREF)” on page 489.
Note: The XREF process option also generates additional totals for each search string in the summary
section.
Source lines section (IDPFX)
The source line section generated when the IDPFX process option is used is like the normal source line
section but with the search file ID preceding each line of source text. See Figure 265 on page 488. 
 1  LINE-#  SOURCE LINE              SRCH DSN: USER1.CLIST
 2  SRCHFORT       1      This NEW member is FIXED 80 with sequence numbers             
SRCHFORT       2   /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT   
SRCHFORT       3   (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.    
Figure 265. Example of the IDPFX source lines section of a search listing
 1 
Column Header Line.
MEMBER
Name of the member where in the string was found.
LINE-#
Relative line number of the line where the string was found.
SOURCE LINE
Up to 106 characters of the source line where the string was found.
SRCH DSN:
In this example, the search data set name is USER1.CLIST.
 2 
The search member, relative line number, and text line from the search data set where the string was
found.
Source lines section (LMTO)
The LMTO process option generates a listing showing the total number of lines found and processed for
each data set. (The individual lines found are not listed.) See Figure 266 on page 488. 
 1  MEMBER-SEARCHED         LINES-FOUND   LINES-PROC
 2  SRCHFORT                        3            3     
TEST                            8            8     
TEST2                           2            2    
Figure 266. Example of the LMTO source lines section of a search listing
Understanding the listings
488  z/OS: z/OS ISPF User's Guide Vol II

## Page 527

1 
Column Header Line.
MEMBERS-SEARCHED
Identifies the members which were searched.
LINES-FOUND
Number of the lines found containing one or more of the search strings. The line is only counted
once no matter how many search strings were found in the line.
LINES-PROC
Number of lines in the member that were searched. Does not include "Do not Process" lines.
 2 
Individual member totals.
Source lines section (XREF)
The XREF process option creates a cross-reference listing where the source lines are listed by search
strings.
In Figure 267 on page 489, lines which contain the string "NEW" in NEW1 TESTCASE C1 are listed first,
then lines which contain the string "NEW" in NEW13 TESTCASE C1, then lines which contain the string
"USE" in NEW1 TESTCASE C1, and finally those lines which contain the string "USE" in NEW13 TESTCASE
C1. 
 1 ----- STRING="NEW"              IN TEST                                  
                                                                         
 2       1      This NEW member is FIXED 80 with sequence numbers           
      2  /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT  
      3  (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.   
      4  ! * NEW: Use DPPDCMT for this comment.                          
      5  * * NEW: Use DPACMT to remove this assembler type comment       
      6  -- *NEW: Use DPADCMT to remove this line.                       
      7  * NEW: FORTRAN comment. Remove with DPFTCMT.                    
      8  &&& This NEW line comes out with a DPLINE '&&&'                 
                                                                         
 3                                IN TEST2                                 
                                                                         
 4       1   /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT 
      2   (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.  
                                                                         
 5 ----- STRING="use"              IN TEST                                  
                                                                         
 6       2  /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT  
      3  (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.   
                                                                         
 7                                 IN TEST2                                 
                                                                           
 8       1   /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT   
      2   (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.    
Figure 267. Example of the XREF source lines section (with ANYC)
 1 
Sub-section line showing string "NEW" and member TEST.
 2 
Line number and text of line where string was found.
 3 
Sub-section line showing member TEST2 (string is still "NEW").
 4 
Line number and text of line where string was found.
 5 
Sub-section line showing string "USE" and member TEST.
Understanding the listings
Appendix B. Understanding the listings  489

## Page 528

6 
Line number and text of line where string was found.
 7 
Sub-section line showing member TEST2 (string is still "USE").
 8 
Line number and text of line where string was found.
Summary section
The summary section (see Figure 268 on page 490) provides various totals resulting from the search and
shows any process statements which were used. 
 1       SEARCH-FOR SUMMARY SECTION            SRCH DSN: ISP.SISPSAMP                     
                                                                                       
 2  LINES-FOUND  LINES-PROC  MEMBERS-W/LNS  MEMBERS-WO/LNS  COMPARE-COLS  LONGEST-LINE    
 3          7        45339            5            216           1:80           80        
                                                                                       
                                                                                       
 4  THE FOLLOWING PROCESS STATEMENTS (USING COLUMNS 1:72) WERE PROCESSED:                 
    SRCHFOR  'PLI',W                                                                   
Figure 268. Example of the summary section of a search listing
The summary section consists of:
 1 
A section heading line.
 2 
A column heading line.
 3 
One line of totals.
 4 
A multi-line section (two lines in Figure 268 on page 490) showing the process statements which
were used.
XREF summary section
When the XREF process option ("Cross-reference Strings") is used, additional lines are included in the
summary section. In Figure 269 on page 490, these are lines  2 ,  3 , and  4 . The totals are listed
according to each search string.
Note: The XREF summary section may be produced without the XREF source line section by using the
LMTO process option.
 1      SEARCH-FOR SUMMARY SECTION            SRCH DSN: USER1.CLIST                       
                                                                                       
 2  XREF STRING-FOUND           LINES-FOUND   MEMBERS-W/LNS                               
 3  "NEW"                             10              2                                   
                                                                                       
 4  "use"                              4              2                                   
                                                                                       
 5 LINES-FOUND  LINES-PROC  MEMBERS-W/LNS  MEMBERS-WO/LNS  COMPARE-COLS  LONGEST-LINE     
 6        14           10            2              0           1:80           80         
Figure 269. Example of the XREF summary section of a search listing
 1 
Section header line. Identifies the data set which was searched. In this example, it is USER1.CLIST.
Understanding the listings
490  z/OS: z/OS ISPF User's Guide Vol II

## Page 529

2 
Column header line.
XREF STRING-FOUND
Column indicating the search string.
LINES-FOUND
Lines which contained one or more occurrences of the search string.
MEMBERS-W/LNS
Total number of files in the group in which the string was found.
 3 
Totals for string "NEW"
 4 
Totals for string "use"
 5 
Column header line.
LINES-FOUND
The summation of lines found for the individual search strings.
LINES-PROC
Number of lines that were part of the search set.
MEMBERS-W/LNS
Number of members where lines were found to contain one or more of the search strings.
MEMBERS-WO/LNS
Number of members where no lines were found to contain any of the search strings.
COMPARE-COLS
The column range that was searched.
LONGEST-LINE
Number of bytes in the longest line searched.
 6 
Totals statistics arranged under the columns specified in  5 .
Examples of search listings
Search of a sequential data set
Figure 270 on page 491 shows the 3 parts of a search listing: page heading, source lines section, and
summary section. 
1  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS  
  LINE-#  SOURCE SECTION                    SRCH DSN: USER1.DB2PROC               
                                                                                  
     128  //ISPCTL1  DD  DISP=NEW,UNIT=VIO,SPACE=(CYL,(1,1)),                     
     130  //ISPCTL2  DD  DISP=NEW,UNIT=VIO,SPACE=(CYL,(1,1)),                     
     132  //ISPLST1  DD  DISP=NEW,UNIT=VIO,SPACE=(CYL,(1,1)),                     
     134  //ISPLST2  DD  DISP=NEW,UNIT=VIO,SPACE=(CYL,(1,1)),                     
                                                                                  
1  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS  
      SEARCH-FOR SUMMARY SECTION            SRCH DSN: USER1.DB2PROC               
                                                                                  
 LINES-FOUND  LINES-PROC  DATASET-W/LNS  DATASET-WO/LNS  COMPARE-COLS  LONGEST-LIN
         4          163            1              0           1:252          80   
                                                                                  
                                                                                  
 THE FOLLOWING PROCESS STATEMENTS (USING COLUMNS 1:72) WERE PROCESSED:            
    SRCHFOR  'ISPLST'                                                             
    SRCHFOR  'ISPCTL'                                                             
Figure 270. Example of the search listing for a sequential data set
Understanding the listings
Appendix B. Understanding the listings  491

## Page 530

LTO search of a partitioned data set
LTO produces the overall totals section of the search results. 
1  ISRSUPC   -   MVS/PDF FILE/LINE/WORD/BYTE/SFOR COMPARE UTILITY- ISPF FOR z/OS      
      SEARCH-FOR SUMMARY SECTION            SRCH DSN: ISP.SISPSAMP                    
                                                                                      
 LINES-FOUND  LINES-PROC  MEMBERS-W/LNS  MEMBERS-WO/LNS  COMPARE-COLS  LONGEST-LINE   
         7        45805            5            217           1:80           80       
                                                                                      
 PROCESS OPTIONS USED: LTO                                                            
                                                                                      
 THE FOLLOWING PROCESS STATEMENTS (USING COLUMNS 1:72) WERE PROCESSED:                
    SRCHFOR  'HLASM'                                                                  
    SRCHFOR  'PLI',W                                                                  
Figure 271. Example of LTO search on file  group
LPSF search of a partitioned data set
The process option LPSF ("List Previous-Search-Following Lines") lists lines before and after the search
text detected line. The "*" in the line number column indicate they were part of the extra lines listed. 
  TEST                        --------- STRING(S) FOUND ------------------- 
                                                                            
       *      This NEW member is FIXED 80 with sequence numbers             
       2  /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT    
       3  (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.     
       *  ! * NEW: Use DPPDCMT for this comment.                            
       *  * * NEW: Use DPACMT to remove this assembler type comment         
       *  -- *NEW: Use DPADCMT to remove this line.                         
       *  * NEW: FORTRAN comment. Remove with DPFTCMT.                      
       *  &&& This NEW line comes out with a DPLINE '&&&'                   
                                                                            
  TEST2                       --------- STRING(S) FOUND ------------------- 
                                                                            
       1   /** NEW: To get rid of this PLI/REXX type comment, use DPPLCMT   
       2   (** NEW: To get rid of this PASCAL type comment, use DPPSCMT.    
Figure 272. Example of LPSF search on file  group
Understanding the listings
492  z/OS: z/OS ISPF User's Guide Vol II
