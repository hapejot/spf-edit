# Appendix A. SuperC reference

Source file: f54u200_v3r1.md
Start page: 469
Page span: 469-514

## Page 469

Appendix A. SuperC reference
This topic provides information about the SuperC return codes, process options, update data set control
options, and process statements.
ISPF contains two utilities, SuperC (option 3.12) and SuperCE (option 3.13), that allow you to compare
data sets for differences. Also, ISPF contains two other utilities, Search-For (option 3.14) and Search-ForE
(option 3.15), that allow you to search data sets for strings of data.
All four of these utilities combine two major components to do their respective functions. The first
component is a dialog that provides the data entry panels, selection panels, and messages. The second
component is the program module, ISRSUPC. The CPI interface is through a standard parameter list.
You can use the SuperC program without the ISPF utilities. To do this, however, your installation must
customize a CLIST or REXX exec (for interactive use), or a PROCLIB procedure (for batch processing of
a catalog procedure). A sample CLIST has been provided to show line command processing. A sample
PROCLIB JCL catalog procedure has also been provided to show batch submission. The sample CLIST
and PROCLIB JCL are located in the ISP.SISPSAMP PDS data set as members ISRSCLST and ISRSPROC,
respectively.
Utility differences
The standard utilities, SuperC (option 3.12) and Search-For (option 3.14), are easy to use with somewhat
reduced function. The extended utilities, SuperCE (option 3.13) and Search-ForE (option 3.15), fully
exploit the SuperC program's capabilities.
Standard utilities
The standard utilities are useful for ordinary comparisons and searches. The SuperC utility (option 3.12)
uses a two-panel sequence: you specify the new input data set on the first panel and the old input data
set on the second. The Search-For utility (option 3.14) uses an optional two-panel sequence: you can
specify the input data set and one search string on the first panel, and use the second panel if you need to
specify more than one search string.
You can enter additional information on these panels as they are displayed. If you are using the
SuperC utility, you can enter the name of a previously prepared profile data set that contains additional
information to specify the comparison.
Search-For does not use a profile data set. Also, Search-For finds all occurrences without case distinction
when searching for a data string.
Extended utilities
The primary intent of the extended utilities is to provide maximum flexibility and access to all SuperC
functions. Input fields are provided to allow you to use process options and statements. Also, the Search-
ForE utility's ASIS fields allow you to specify mixed-case search strings.
The input data set name fields differ from standard ISPF format because Project, Group, Type, and
Member fields are not provided. Instead, you can enter input data set names horizontally using standard
TSO naming conventions. This includes the use of a PDS member name, if desired, as part of a data set
name.
The concatenation of input data sets is also different. Up to four data set names, as opposed to the
standard four ISPF library groups, can be entered as new or old data sets. This allows data sets with the
same attributes to be concatenated. For example, PANELS and MSGS data sets could be concatenated for
searching.
SuperC - utility differences
© Copyright IBM Corp. 1980, 2024 431

## Page 470

Besides compare functions, the SuperCE Utility panel provides access to the Search-ForE utility (option
3.15). This gives you the added advantage of the ability to search for a data string without having to leave
SuperCE, in addition to access to more functions than Search-For (option 3.14) provides.
Program description
The SuperC program is a fast and versatile program that can process:
• Two sequential data sets
• Two complete partitioned data sets
• Members of two partitioned data sets
• Concatenated data sets.
In fact, any data set that can be processed by ISPF can be processed by the SuperC program.
Note: SuperC does not support tape data sets.
SuperC can compare data sets even when there are many differences and redundant data. Some
examples of redundant data are blank lines, duplicate words, and binary data with many duplicate
characters.
Unlike many compare programs, SuperC is not limited to comparing data sets on a line-by-line basis.
Instead, it allows you to choose between the four comparison levels listed. The compare type you
select determines which kinds of data differences are presented by SuperC. See “Reasons for differing
comparison results” on page 467 for more information about comparison results.
• File comparisons produce summary information about the differences between the data sets being
compared.
• Line comparisons are record-oriented and show matching, inserted, deleted, and reformatted lines. This
level is most useful for comparing lines of program source code. It provides the least output difference
information and is least sensitive to resynchronization.
• Word comparisons show differences between data strings that are delimited by blanks or
nonalphanumeric characters, such as commas. Matching words are found, even if they are not on the
same line. This level is most useful for comparing text data sets.
• Byte comparisons determine byte differences. It is most useful for comparing unformatted and
machine-readable data.
The SuperC program requires only the names of the input data sets. However, the utility you are using
may require other information, such as a listing type. Also, you can enter these types of processing
information and options on the utility data entry panels:
• Compare type
• Listing data set name or destination
• Process options
• Statements or profile data set name
• Browse output choice.
The SuperC program allows you to create two kinds of output:
• A listing that shows the results of the comparison or search and
• A structured data set that contains update information.
Within these two categories, you can create many kinds of output that make it easy to see where your
data differs. To see your comparison results, you can generate listings that show:
• An overall summary of total changes
• The actual source code where deltas (differences) were found
• The deltas plus up to 10 (the default) matching lines before and after
• The deltas plus all matching lines.
SuperC - Program Description
432  z/OS: z/OS ISPF User's Guide Vol II

## Page 471

You can format the listings to show differences either sequentially or side-by-side.
In an update data set, output lines are identified and results are put in specific columns. An update data
set is especially useful as input to a user-written application program. It allows a program to customize
what you see, changing generalized output to information that is specific to a particular application.
The SuperC utility (options 3.12, 3.13, and 3.14) CLISTs allocate or free space under these DDNAMEs:
SYSIN, SYSIN2, OLDDD, NEWDD, OUTDD, and DELDD.
SuperC features for the year 2000 transition
SuperC includes features designed to help manage the Year 2000 transition:
• Specify a 100-year period (or "year window") so that, for dates that have only a 2-digit year, the century
can be determined. This can be based on either:
– A "fixed" year window (with a fixed starting year), or
– A "sliding" year window (starting at a specified number of years before the current year).
• Compare 2-digit year values in one data set with 4-digit year values in another data set.
• Compare compressed year values in one data set with uncompressed year values in another data set.
• Filter cosmetic differences caused by adding century digits to 2-digit years, so that you can more easily
identify real differences in content.
Applications
You can use the SuperC program for many applications other than comparing two source data sets.
This topic lists some specific applications for general users, writers and editors, and programmers and
systems administrators.
General users can:
• Compare two data sets that have been reformatted. Reformatted data sets contain such differences as
indentation level changes, spaces inserted or deleted, or lines that have been reformatted and moved to
other parts of the data set.
SuperC detects and classifies reformatted lines as special changes. You can list these lines in the
output, along with the normal insert/delete changes, or eliminate them from the listing. Reducing the
number of flagged lines may help you focus on real, rather than cosmetic, changes.
• Determine whether two PDSs, or a concatenation of PDSs, have corresponding like-named members.
Members absent from one data set but present in the other are listed, as is all change activity between
like-named members. The comparison can show changes caused by creating or deleting PDS members.
Writers and editors can:
• Detect word changes within documents.
SuperC finds word differences even if the words have moved to different lines.
• Verify that only designated areas are changed.
SuperC comparison results show all areas affected. Changes made to restricted areas may be invalid.
Therefore, unintended changes can be detected so that a complete document need not be checked for
errors again.
• Create a utility that automatically inserts SCRIPT revision codes.
You could write a program that uses Word compare to find where words in the new data set are
different, makes a copy of the new data set, and then inserts SCRIPT revision codes (.RC) before and
after the changed words. This utility could eliminate the need to insert SCRIPT revision codes manually.
Programmers and systems administrators can:
• Generate management reports that show the quantity and type of changes in program source code.
SuperC - applications
Appendix A. SuperC reference  433

## Page 472

SuperC can count the changed and unchanged lines of code in an application program. Therefore,
comparison results could be used to summarize the changes between different versions of a program.
• Retain a record of change activity.
Listing data sets can be collected and retained as a permanent record of the changes made before
a new program is released. Source code differences can help detect regressions or validate the
appropriateness of any code modifications.
• Rewrite a listing data set, including additional headers or change delimiters.
Some SuperC listings may need to be rewritten before you accept the results. For example, some
installations may require security classifications. Others may require a listing created using the WIDE
process option to have box delimiters surrounding changed sections.
• Compare data sets across nonconnected systems.
SuperC can generate a 32-bit hash sum per data set or member using the File compare type. Data sets
compared on a nonconnected processor, using SuperC, should have the same hash sums if they are
identical. A File comparison of any data set to determine a hash sum can be done by specifying the
same data set as both new and old.
• Develop additional uses for update data sets.
SuperC produces general results with generalized reports. However, your installation may have
additional requirements. There are many specialized update formats that you can use to produce
listings that match these requirements. Normal SuperC listings may not fit this type of application, but
the update data sets are more structured and should be easier to use as data input. See Appendix C,
“Update files,” on page 493 for explanations and examples of the update data sets.
Process options
You can use primary command P on either the SuperCE Utility panel or the Search-ForE Utility panel
to display one or more panels from which you can select process options. For SuperCE, the options
displayed are compatible with the compare type (File, Line, Word, or Byte) that you specified in the
Compare Type field. The compare type that you select determines the available process options (Table 29
on page 434).
Table 29. Summary of process options
Process option Valid for compare type Valid
for
Searc
h
Keyword Description FILE LINE WOR
D BYTE
ALLMEMS All members ✓ ✓ ✓ ✓ ✓
ANYC Any case ✓ ✓ ✓
APNDLST Append listing output ✓ ✓ ✓ ✓ ✓
APNDUPD Append update ✓ ✓ ✓
ASCII Process ASCII input files ✓ ✓ ✓ ✓ ✓
CKPACKL Check for packed format ✓ ✓ ✓
CNPML “1” on
page 437
Count non-paired member/file lines ✓
COBOL “2” on
page 437
For COBOL source files ✓ ✓ ✓
COVSUM Conditional summary ✓ ✓ ✓ ✓
CPnnnnn EBCDIC code page used with ASCII option ✓ ✓ ✓ ✓ ✓
Compare - process options
434  z/OS: z/OS ISPF User's Guide Vol II

## Page 473

Table 29. Summary of process options (continued)
Process option Valid for compare type Valid
for
Searc
h
Keyword Description FILE LINE WOR
D BYTE
DLMDUP Do not list matching duplicate lines ✓
DLREFM Do not list reformatted lines ✓
DPACMT Do not process asterisk (*) comment lines ✓ ✓ ✓
DPADCMT Do not process ADA-type comments ✓ ✓ ✓
DPBLKCL Do not process blank comparison lines ✓ ✓ ✓
DPCBCMT Do not process COBOL-type comment lines ✓ ✓ ✓
DPCPCMT Do not process C++ -type comment lines ✓ ✓ ✓
DPFTCMT Do not process FORTRAN-type comment lines ✓ ✓ ✓
DPMACMT Do not process PC Assembly-type comment lines ✓ ✓ ✓
DPPLCMT Do not process PL/I-type comments ✓ ✓ ✓
DPPSCMT Do not process Pascal-type comments ✓ ✓ ✓
EMPTYOK Return RC 0 in place of RC 28. See Table 31 on page
468.
✓ ✓ ✓ ✓ ✓
FINDALL Require all strings found for return code 1 ✓
FMSTOP Stop immediately a difference found ✓ ✓
FMVLNS Flag moved lines ✓
GWCBL Generate WORD/LINE comparison change bar listing ✓ ✓
IDPFX Identifier-prefixed listing lines ✓
LMCSFC “3” on
page 437
Load module CSECT file compare ✓
LMTO “4” on
page 437
List group member totals ✓
LNFMTO “4” on
page 437
List not-found member totals only ✓
LOCS List only changed entries in summary ✓ ✓ ✓ ✓
LONGLN “5” on
page 437
Long lines ✓ ✓
LPSF “4” on page
437
List previous-search-following lines ✓
LTO “4” on page
437
List totals only ✓
MIXED Mixed input (single/double byte) text ✓ ✓ ✓
NARROW “5”
on page 437
Narrow (side-by-side) listing ✓
NOPRTCC No printer control columns ✓ ✓ ✓ ✓ ✓
Compare - process options
Appendix A. SuperC reference  435

## Page 474

Table 29. Summary of process options (continued)
Process option Valid for compare type Valid
for
Searc
h
Keyword Description FILE LINE WOR
D BYTE
NOSEQ “2” on
page 437
No sequence numbers ✓ ✓ ✓
NOSUMS No summary section ✓ ✓ ✓ ✓
REFMOVR Reformat override ✓
SDUPM Search duplicate members ✓
SEQ “2” on page
437
Ignore standard sequence number columns ✓ ✓ ✓
SYSIN Provide alternative DD name for process statements. ✓ ✓ ✓ ✓ ✓
UPDCMS8 “6”
on page 437
Update CMS8 format ✓
UPDCNTL “6”
on page 437
Update control ✓ ✓ ✓
UPDLDEL “6”
on page 437
Update long control ✓
UPDMVS8 “6”
on page 437
Update MVS8 format ✓
UPDPDEL “6”
on page 437
Update prefixed delta lines ✓
UPDREV “6” on
page 437
Update revision ✓ ✓
UPDREV2 “6”
on page 437
Update revision (2) ✓ ✓
UPDSEQ0 “6”
on page 437
Update sequence 0 ✓
UPDSUMO “6”
on page 437
Update summary only ✓ ✓ ✓
VTITLE Print data set volume serial ✓ ✓ ✓
WIDE “5” on
page 437
Wide (side-by-side) listing ✓
XREF Cross reference strings ✓
XWDCMP Extended word comparison ✓
Y2DTONLY “7”
on page 437
Compare Dates Only ✓
Compare - process options
436  z/OS: z/OS ISPF User's Guide Vol II

## Page 475

Table 29. Summary of process options (continued)
Process option Valid for compare type Valid
for
Searc
h
Keyword Description FILE LINE WOR
D BYTE
Note:
1. Valid for group LINE comparisons only.
2. COBOL, SEQ, and NOSEQ are mutually exclusive.
3. Not supported for PDSE data sets.
4. LMTO, LNFMTO, LPSF, and LTO are mutually exclusive.
5. LONGLN, NARROW, and WIDE are mutually exclusive.
6. All update (UPD) process options are mutually exclusive. Also, they cannot be used with the process option
Y2DTONLY.
7. Y2DTONLY is not supported for change bar listing (process option GWCBL).
Here are the SuperC process options, listed alphabetically:
ALLMEMS
Process all members in a PDS including ALIAS members. Without this process option, when
performing a PDS compare, SuperC does not include members with the ALIAS attribute unless
explicitly specified by a SELECT process statement. The ALLMEMS process option indicates that all
directory entries including those with the ALIAS attribute are to be processed.
Valid for FILE, LINE, WORD, and BYTE compare types and Search.
ANYC
Any case. Lowercase alphabetic characters (a to z) in source files are translated to uppercase (A to Z)
before comparison processing. (The actual input files are not modified.) The ANYC option only applies
to alphabetic characters in the source files. It does not affect any strings in the statements data set.
Use this option to cause strings such as "ABC", "Abc", "ABc", to compare equally.
Valid for LINE and WORD compare types and Search.
APNDLST
The APNDLST process option appends the listing output to the specified or default listing file. If the
file does not exist, it is created.
APNDLST allows you to collect updates from multiple comparisons into one listing file.
Valid for FILE, LINE, WORD, and BYTE compare types and Search.
Note:
1. You can also do this by using the SELECT process statement (and, on CMS, SELECTF) that identifies
different files/members and produces a single listing.
APNDUPD
The APNDUPD process option appends the update output to the specified or default update file. If the
file does not exist, it is created.
APNDUPD allows you to collect updates from multiple comparisons into one update file.
Valid for LINE, WORD, and BYTE compare types.
Note:
1. You can also do this by using the SELECT process statement (and, on CMS, SELECTF) that identifies
different files/members and produces a single listing.
Compare - process options
Appendix A. SuperC reference  437

## Page 476

ASCII
Process ASCII input files. For LINE or WORD compare and for Search the input data is translated from
ASCII to EBCDIC. For BYTE compare, character data in the listing is translated from ASCII to EBCDIC.
For FILE compare, this option is accepted but has no effect. Any search string given in hexadecimal
notation is assumed to be in ASCII, matching the original input data.
The ASCII code page is assumed to be ISO 8859-1 (CCSID 819). The EBCDIC code page may be
specified using the Cpnnnnn option.
Valid for FILE, LINE, WORD, and BYTE compare types, and Search.
CKPACKL
Check for packed format. This option determines if the member or sequential data set has the
standard ISPF/PDF packed header format. If required, SuperC unpacks the input data set or member
during the comparison.
Valid for LINE and WORD compare types and Search.
CNPML
Count non-paired member/file lines for the group summary list. Use this option to inventory the total
number of processed and not-processed lines. Otherwise, only the paired entries are listed with line
counts.
Valid for LINE compare type.
Note: CNPML is only used when comparing a group of files.
COBOL
Ignore columns 1 to 6 in both COBOL source files. Data in columns 1 to 6 is assumed to be sequence
numbers.
Valid for LINE and WORD compare types and Search.
COVSUM
Conditional summary section. List the final summary section or the update file for the option
UPDSUMO only if there are differences. This is useful when used in combination with APNDLST or
APNDUPD.
Valid for FILE, LINE, WORD, and BYTE compare types.
CPnnnnn
Use the specified EBCDIC code page number (up to 5 digits) when translating data using the ASCII
option. If not specified ISPF uses the terminal code page. If the terminal code page cannot be
determined or is not supported SuperC uses CP1047. All CECP and Euro Latin-1 code pages are
supported. Therefore nnnnn can be any of the following values:
Default: 1047 (Open Systems Latin-1 EBCDIC)
CECP: 37, 273, 277, 278, 280, 284, 285, 297, 500, 871
ECECP (Euro): 1140 to 1149
Valid for FILE, LINE, WORD, and BYTE compare types, and Search.
DLMDUP
Do not list matching duplicate lines. Old file source lines that match new file source lines are omitted
from the side-by-side output listing.
Valid for LINE compare type.
DLREFM
Do not list reformatted lines. Old file source lines that have the same data content (that is, all data is
the same except the position and number of space characters) as the new file lines are omitted from
the listing. Only the new file reformatted lines are included in the output.
Valid for LINE compare type.
Compare - process options
438  z/OS: z/OS ISPF User's Guide Vol II

## Page 477

DPACMT
Do not process asterisk (*) comment lines. Lines with an "*" in column 1 are excluded from the
comparison set. Other forms of assembler comments are unaffected.
Valid for LINE and WORD compare types and Search.
DPADCMT
Do not process ADA type comments. ADA comments are whole or partial lines that appear after the
special "--" sequence. Blank lines are also considered part of the comment set. This option produces
a comparison listing with comments removed and part comments blanked.
Valid for LINE and WORD compare types and Search.
DPBLKCL
Do not process blank comparison lines. Source lines in which all the comparison columns are blank
are excluded from the comparison set.
Note: It is redundant to use this option with DPADCMT, DPPLCMT, or DPPSCMT as these process
options also bypass blank comparison lines.
Valid for LINE and WORD compare types and Search.
DPCBCMT
Do not process COBOL-type comment lines. COBOL source lines with an "*" in column 7 are excluded
from the comparison set
Valid for LINE and WORD compare types and Search.
DPCPCMT
Do not process C++ end-of-line type compiler comments. These are "/ /" delimited comments.
DPPLCMT may also be used with DPCPCMT when the source file contains "/* … */" comments
delimiters.
Valid for LINE and WORD compare types and Search.
DPFTCMT
Do not process FORTRAN-type comment lines. FORTRAN source lines with a "C" in column 1 are
excluded from the comparison set.
Valid for LINE and WORD compare types and Search.
DPMACMT
Do not process PC Assembly-type comments. This uses the IBM PC definition for assembler
comments: comments begin with either the COMMENT assembler directive or a semi-colon (;).
Valid for LINE and WORD compare types and Search.
DPPLCMT
Do not process PL/I-type comments. PL/I, C++, C, REXX comments (/* … */) and blank lines are
excluded from the comparison set. This option produces a listing with all comments removed and
blanked.
Valid for LINE and WORD compare types and Search.
DPPSCMT
Do not process Pascal-type comments. Comments of the type (* … *) and blank lines are excluded
from the comparison. DPPSCMT and DPPLCMT may be required for some Pascal compiler comments.
This option produces a comparison listing with comments removed and part comments blanked.
Valid for LINE and WORD compare types and Search.
EMPTYOK
If search or compare finds empty files, ISRSUPC will normally terminate RC 28. If this option is set,
the return code will be changed to RC 0. Any messages that are associated with empty files (such as
ISRS001I, ISRS005I) will continue to be written.
Valid for FILE, LINE, WORD, and BYTE compare types and Search.
Compare - process options
Appendix A. SuperC reference  439

## Page 478

FINDALL
All strings must be matched at least once for the overall search to be considered successful, in which
case the return code is set to one. For a search across multiple files (for example when searching PDS
members) the matches do not have to be in the same file.
Valid for Search.
Note:
1. If all searches are not satisfied, there is NO message to indicate this, other than RC=0. To find
which searches failed, specify the XREF process option.
2. If the FMSTOP option is specified, the search will stop once it has satisfied all search strings.
FMSTOP
For FILE compare, the compare is stopped with a return code of 1 when a difference is found between
the files. This option provides a quicker way of telling if two files are different.
For search, the search of each file is stopped when a search string is found. However, if the FINDALL
option is also specified, the search is stopped only when all search strings have been found at least
once (not necessarily in the same input file), so that the FINDALL return code can be set correctly.
Valid for FILE compare type and search.
FMVLNS
Flag moved lines. Identify inserted lines from the new file that match deleted lines from the old file.
Inserted-moved lines are noted with "IM" and deleted-moved lines are noted with "DM" in the listing.
Valid for LINE compare type.
Note:
1. Maximum length for lines is 256 characters.
2. Maximum length for a contiguous block of moved lines is 32K.
GWCBL
Generates WORD/LINE comparison change bar listings. SuperC lists new file lines with change bars
("|") in column 1 for lines that differ between the new and old files. Deleted lines are indicated by
flagging the lines following the deletion.
Valid for LINE and WORD compare types.
Note:
1. LINE comparison and WORD comparison may give slightly different results due to their sensitivity
to word and line boundaries. For further details, see “Reasons for differing comparison results” on
page 467.
2. GWCBL cannot be used with the process option Y2DTONLY.
IDPFX
Identifier prefixed. Member name is prefixed to the search string lines of the listing.
Valid for Search.
LMCSFC
Load module CSECT file compare list. Lists the name, number of bytes, and hash sum for each load
module CSECT. Unchanged paired CSECTs are omitted when you specify the LOCS process option.
Note:
1. LMCSFC is not supported for PDSE.
Valid for FILE compare type.
LMTO
List group member totals. Lists the member summary totals and the overall summary totals for the
entire file/group.
Valid for Search.
Compare - process options
440  z/OS: z/OS ISPF User's Guide Vol II

## Page 479

LNFMTO
List "not found" member totals only. Lists the members that have no strings found for the entire
file/group.
Valid for Search.
LOCS
List only changed entries in summary. Normally, for groups of files/members being compared, SuperC
lists all paired entries in the Member Summary Listing section of the listing file. Preceding the names
of these pairs is a CHNG field to indicate whether the comparison found any differences or not.
When LOCS is specified, only those pairs which have changes are listed in the summary section.
Valid for group FILE, LINE, WORD, and BYTE compare types.
LONGLN
Long lines. LONGLN causes SuperC to create a listing with 203 columns, reflecting up to 176 columns
from the source files. This file may exceed the maximum number of columns handled by many
printers.
Valid for LINE compare type and Search.
LPSF
List previous-search-following lines. Lists the matched string line and up to 6 preceding and 6
following lines for context. The preceding and following count may be changed by using the LPSFV
process statement. This allows a count range of 1 to 50 lines. A value of 0 is invalid, since this
produces a normal search without any options.
Valid for Search.
LTO
List totals only. List the overall summary totals for the entire file/member group. See Figure 271 on
page 492 for an example of an LTO listing.
Valid for Search.
MIXED
Mixed input. Indicates that the input text may be a mixture of both single-byte and double-byte
(DBCS) text. Double-byte strings are recognized and handled differently than if MIXED were not
specified. For instance, single byte characters are not valid within double-byte strings. Special
terminal devices (for example, 5520) allow entry of DBCS characters.
Valid for LINE and WORD compare types and Search.
NARROW
Narrow side-by-side listing. Creates a 132/133 variable listing file with only 55 columns from each
source file. Insertions and deletions are flagged and appear side-by-side in the listing output. Refer to
Figure 260 on page 485 and Figure 261 on page 486 for examples of NARROW listings.
Valid for LINE comparison.
NOPRTCC
No printer control columns. SuperC generates "normal" or NARROW listing files with record lengths
of 133 columns, or WIDE or LONGLN listing with 203 columns. These listings contain printer control
columns and page separators. NOPRTCC eliminates both the page separators and page header line.
With NOPRTCC, "normal" and NARROW listings are 132 columns, and WIDE and LONGLN listings
are 202. Section separators and title lines are still generated. This file may be preferred for on-line
"browsing".
Valid for FILE, LINE, WORD, and BYTE compare types and Search.
NOSEQ
No Sequence numbers. Process fixed-length 80-byte record standard sequence number columns (73
to 80) as data. This option is extraneous for any record size other than 80.
Valid for LINE and WORD compare types and Search.
Compare - process options
Appendix A. SuperC reference  441

## Page 480

NOSUMS
No Summary Section. Eliminates the group and final summary section from the output listing. This
allows the user to generate a better "clean" copy for program inspection. Conversely, it eliminates the
all-problem information in case of errors and option identification.
Valid for LINE, WORD, and BYTE compare types and Search.
REFMOVR
Reformat override. Reformatted lines are not flagged in the output listing. They are, however, counted
for the overall summary statistics and influence the return code since they are a special case of an
insert/delete pair.
Valid for LINE compare type.
SDUPM
Search duplicate members. Searches all members found in concatenated PDS data sets, even if more
than one member is found to have the same name. Searches duplicate names even if the search is for
a single member or if members are specified using the SELECT process statement.
Valid for Search.
SEQ
Sequence numbers. Ignore fixed-length 80-byte record standard sequence number columns.
Sequence numbers are assumed in columns 73 to 80 for such records. This option is invalid for
any record size other than 80.
Valid for LINE and WORD compare types and Search.
SYSIN
Provide alternate DD name for process statements. Syntax is SYSIN(DDNAME). The default ddname
is SYSIN. If this option is used, SuperC only accesses process statements via the supplied ddname. It
does not attempt to access additional process statements via the SYSIN2 DD card.
Valid for FILE, LINE, WORD, and BYTE compare types and Search.
UPDCMS8
Update CMS 8 format. UPDCMS8 produces an update file that contains both control records and
source lines from the new input file. UPDCMS8 requires that the old file has fixed-length 80-byte
records with sequence numbers. The new file may have a variable or fixed length format with an
LRECL ≤ 80.
SuperC may change the status of match lines to insert/delete pairs, enlarging the sequence number
gaps of the old file. The update file (when properly named) can be used as input to CMS XEDIT. For
information and an example of this update file, see “Update CMS sequenced 8 file” on page 495.
Valid for LINE compare type.
UPDCNTL
Update Control. Produces a control file which relates matches, insertions, deletions, and
reformattings using relative line numbers (for LINE compare type), relative word positions (for WORD
compare type), or relative byte offsets (for BYTE compare type) within the new and old file. No source
or data from either input file is included in the output file. "Do not" process options/statements are
compatible selections for the LINE compare type. For information and an example of this update file,
see “Update control data sets” on page 496.
Valid for LINE, WORD, and BYTE compare types.
UPDLDEL
Update Long Control with all matches and delta changes. This reflects the comparison's matches,
inserts, and deletes. You can edit this update file accepting, rejecting, or modifying the changes.
There are control records preceding each change and matching section. After the changes have been
audited, optionally modified, and the control records removed, you should be able to reuse this
control file as a composite new file.
Valid for LINE compare type.
Compare - process options
442  z/OS: z/OS ISPF User's Guide Vol II

## Page 481

UPDMVS8
Update MVS8 format. Produces a file that contains both control and new file source lines. Sequence
numbers from columns 73 to 80 of the new file are used (when possible) as insert references,
while deletes use sequence numbers from columns 73 to 80 of the old file. Both files must have
fixed-length 80-byte records. The format of the generated data may be suitable as z/OS IEBUPDTE
input. For information and an example of this update file, see “Update MVS sequenced 8 data set” on
page 500.
Valid for LINE compare type.
UPDPDEL
Update prefixed delta lines. Produces a control data set containing header records and complete (up
to 32K line length limit) delta lines from the input source files. Each output record is prefixed with
identification and information. The update data set is a variable-length data set reflecting the input
source files' characteristics.
Valid for LINE compare type.
UPDREV
Update Revision. UPDREV produces a copy of the new file with SCRIPT/VS .rc on/off or
BookMaster® :rev/:erev revision codes delimiting most script lines that contain changes.
You may wish to contrast the source lines delimited by the UPDREV option and a similar flagging of the
lines with changes from the output listing file as produced by the GWCBL process.
Note: The revision character used is controlled by using the REVREF process statement. For details,
see “Revision code reference” on page 459.
A REVREF process statement (for example, REVREF REFID=ABC or REVREF RCVAL=1) defines
the revision level (SCRIPT/VS tags) or reference ID (BookMaster tags). Alternatively, SCRIPT/VS .rc
delimiters may be controlled by the first record in the new file. (For example, .rc 2 | as the first
record causes level 2 to be used).
Note: BookMaster requires the REFID value to be defined with a :revision tag and "RUN=YES"
attribute to have the change character inserted in the processed document.
For information and an example of this update file, see “Revision file” on page 493.
Valid for LINE and WORD compare types.
UPDREV2
Update Revision (2). UPDREV2 is identical to UPDREV with the exception that data between the
following BookMaster tags are not deleted in the update file:
:cgraphic.
:ecgraphic.
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
Valid for LINE and WORD compare types.
Compare - process options
Appendix A. SuperC reference  443

## Page 482

UPDSEQ0
Update Sequence 0 (zero). UPDSEQ0 produces a control file that relates insertions and deletions to
the relative line numbers of the old file. Both control records and new file source lines are included
in the output file. This option is like UPDCMS8 except that it uses relative line numbers (starting with
zero) instead of the sequence numbers from columns 73 to 80. The control field after a "$" designates
the number of new source lines that follow in the update file.
Both fixed and variable record length lines are allowed. Fixed-length records shorter than 80 bytes are
padded with spaces to 80. Insertion lines are full fixed or variable length copies of the new input data
set lines. For information and an example of this update file, see “Update sequenced 0 data set” on
page 501.
Valid for LINE compare type.
UPDSUMO
Update Summary only. UPDSUMO produces an update file of 4 lines (new file ID, old file ID, totals
header, single summary line). The summary line is tagged with a "T" in column 1 and the summary
statistics are located at fixed offsets in the output line. The file has a record length of 132. For
information and an example of this update file, see “Update summary only data sets” on page 502.
Valid for LINE, WORD, and BYTE compare types.
VTITLE
Volume title. VTITLE modifies the compare listing so that the data set volume serial is printed below
the data set name.
For a multi-volume data set only the VOLSER of the first volume is displayed.
VTITLE is ignored if the NTITLE or OTITLE process option is specified.
Valid for LINE, WORD, and BYTE compare types.
WIDE
Wide side-by-side listing. Creates a 202/203 variable-length listing file with 80 columns from each
source file. Inserts and deletes are flagged and appear side-by-side in the listing output. For an
example of a WIDE side-by-side listing, see Figure 262 on page 486.
Valid for LINE compare type.
XREF
Cross reference strings. Creates a cross reference listing by search string. Can be used with IDPFX,
LMTO LNFMTO, and LTO. Not implemented for LPSF.
The XREF option can be useful when more than one search string (or search condition) is specified.
The XREF listing is implemented using a multiple pass operation for listing the "lines found" for each
individual string. Be aware that XREF adds some additional processing overhead to the normal search
process. For an example of a search XREF listing, see Figure 267 on page 489.
Valid for Search.
XWDCMP
Extended WORD comparison. The word delimiter set is extended to include non-alphanumeric
characters (including spaces). For example, "ABCD(EFGH) JKL" is 2 words using normal WORD
compare type, but 5 (3 words and 2 pseudo-words) with the XWDCMP process option.
Valid for WORD compare type.
Y2DTONLY
Compare Dates Only. Indicates that the comparison process is to be performed only on the dates
defined by the Date Definition process statements. That is, all data in the input files is ignored in the
comparison process apart from that defined by NY2C, NY2Z, NY2D, NY2P, OY2C, OY2Z, OY2D, and
OY2P process statements. For further details on these process statements, see “Date definitions” on
page 464.
Note:
Compare - process options
444  z/OS: z/OS ISPF User's Guide Vol II

## Page 483

1. Y2DTONLY causes a "record-for-record" comparison to be performed between the two input files,
whereby dates are checked for being equal or unequal. (The "high/low" comparison logic that
SuperC normally uses is not applied in the case of Y2DTONLY and, as such, the relative values of
the dates have no bearing on the result of the comparison.)
2. Y2DTONLY is not supported for the process option GWCBL (change bar listing).
Valid for LINE compare type.
Process statements
You can use process statements to tailor your comparison or search according to your requirements.
Process statements provide a powerful and flexible way of ensuring that only relevant data is compared
(or searched) and that meaningful results are produced.
Broadly speaking, the two major functions that process statements perform are:
• To select the data that is to be compared (or searched) and,
• To handle various date formats.
All process statements require a keyword followed by one or more operands. They are supplied to SuperC
in the Process Statements File.
Table 30 on page 445 lists each of the process statement keywords and shows for which compare type
each keyword can be used. The table also shows whether the keyword is valid for the SuperC Search.
Note: The sequence in which each of the process statements is listed (in Table 30 on page 445 and the
pages following) is primarily alphabetic according to the process statement keyword.
However, in the interest of keeping associated "pairs" and "sets" of process statements together, the
prefixes "N" and "O" (indicating the process statement applies to the new or old file) have been ignored
when sequencing the process statements alphabetically.
Table 30. Summary of process statements
Process option Valid for compare type Valid
for
SearchKeyword Description FILE LINE WORD BYTE
NCHGT Change text: new or search file ✓ ✓ ✓
OCHGT Change text: old file ✓ ✓
CHNGV Change listing value ✓ ✓ ✓
CMPBOFS Compare byte offsets ✓
CMPCOLM Compare (search) columns: new, old, search
files
✓ ✓ ✓
CMPCOLMN Compare columns: new file ✓ ✓
CMPCOLMO Compare columns: old file ✓ ✓
CMPLINE Compare lines ✓ ✓ ✓
CMPSECT “1” on
page 447
Compare sections ✓ ✓
COLHEAD “2” on
page 447
Define column headings ✓
DPLINE Do not process lines (containing a string) ✓ ✓ ✓
DPLINEC Do not process lines continuation ✓ ✓ ✓
Compare - process options
Appendix A. SuperC reference  445

## Page 484

Table 30. Summary of process statements (continued)
Process option Valid for compare type Valid
for
SearchKeyword Description FILE LINE WORD BYTE
NEXCLUDE “4” on
page 447
Exclude data: new file ✓ ✓
OEXCLUDE “4” on
page 447
Exclude data: old file ✓ ✓
NFOCUS “4” on
page 447
Focus on data: new file ✓ ✓
OFOCUS “4” on
page 447
Focus on data: old file ✓ ✓
LNCT Line count ✓ ✓ ✓ ✓ ✓
LPSFV List previous-search-following value ✓
LSTCOLM List columns ✓ ✓
REVREF Revision code reference ✓ ✓
SELECT Select PDS members (z/OS) ✓ ✓ ✓ ✓ ✓
SELECT Select members/files (CMS) ✓ ✓ ✓ ✓ ✓
SELECT Select members (z/VSE®) ✓ ✓ ✓ ✓ ✓
SLIST Statements listing option ✓ ✓ ✓ ✓ ✓
SRCHFOR Search for a string ✓
SRCHFORC Search for a string continuation ✓
NTITLE Alternative listing title: new file ✓ ✓ ✓ ✓ ✓
OTITLE Alternative listing title: old file ✓ ✓ ✓ ✓
NY2AGE Aging option: new file ✓
OY2AGE Aging option: old file ✓
NY2C Date definition: new file, character format ✓
NY2Z Date definition: new file, zoned decimal
format
✓
NY2D Date definition: new file, unsigned packed
decimal format
✓
NY2P Date definition: new file, packed decimal
format
✓
OY2C Date definition: old file, character format ✓
OY2Z Date definition: old file, zoned decimal
format
✓
OY2D Date definition: old file, unsigned packed
decimal format
✓
OY2P Date definition: old file, packed decimal
format
✓
WORKSIZE Maximum number of units for comparison ✓ ✓ ✓
Compare - process options
446  z/OS: z/OS ISPF User's Guide Vol II

## Page 485

Table 30. Summary of process statements (continued)
Process option Valid for compare type Valid
for
SearchKeyword Description FILE LINE WORD BYTE
Y2PAST Global date option ✓
* Process Statement comment to be printed ✓ ✓ ✓ ✓ ✓
.* Process Statement comment not to be
printed
✓ ✓ ✓ ✓ ✓
Note:
1. Not supported on CMS.
2. Valid only for listing types DELTA and LONG.
3. Supported only on z/VSE.
4. FILE compare type is valid only with ROWS option of NEXCLUDE, OEXCLUDE, NFOCUS, and OFOCUS.
5. Supported only on CMS.
The following sections describe each process statement in detail.
Change listing value
The CHGNV process statement specifies the number of match lines listed before and after a line with a
change: insert, delete, or reformat.
Compare Types: LINE, WORD, and BYTE
CHNGV number
number
A decimal number between 1 and 1000.
Example
Description
CHNGV 3
Lists up to 3 lines before and after change.
Change text
There are two Change Text process statements:
NCHGT
Change new (or search) input text string
OCHGT
Change old input text string
These process statements change the input source image before performing the comparison.
The relative input file ("new" or "old") is scanned for text that matches a search_string. If matching text
is found, it is replaced by a corresponding output_string before the comparison process is performed.
Question marks ("?") may be used as "wildcard" characters in the search_string or output_string.
The search_string and output_string need not be the same length. The output_string may even be a null
string.
Compare Types: LINE, WORD, and Search. OCHGT cannot be used for Search.
Change listing value
Appendix A. SuperC reference  447

## Page 486

NCHGT
OCHGT
' search_string ' , ' output_string '
, start_column
: last_start_column
search_string
A character or hexadecimal string to be replaced in the input file. The search string is used explicitly
as coded and is not affected by the ANYC process option. For one embedded apostrophe, use two
consecutive apostrophes ('').
output_string
The replacement string to be used in the comparison. The output string is used explicitly as coded
and is not affected by the ANYC process option. For one embedded apostrophe, use two consecutive
apostrophes ('').
start_column
The column in or after which the search_string must start. Must be greater than zero.
last_start_column
The last column in which the search_string may start. Must be separated from the start_column by a
colon, and must be equal to or greater than the start_column value. If not supplied, is the equivalent
of setting the value to start_column. To search from the start_column to the end of a variable length
row, set the last_start_column to a value larger than the length of the longest row.
Example
Description
NCHGT 'ABCD','XXXX'
Changes all strings "ABCD" in the new file to "XXXX" before performing the comparison.
OCHGT 'ABCD','XXXX',1:50
Changes all strings "ABCD" in the old file, that start within columns 1 to 50, to "XXXX" before
performing the comparison.
OCHGT 'ABCD','',1:50
Changes all strings "ABCD" in the old file, that start within columns 1 to 50, to a null string before
performing the comparison. (In the comparison process, this effectively ignores any "ABCD" strings
found in those positions in the old file.)
NCHGT 'ABCD','AB'
Changes all strings "ABCD" in the new file to "AB" before performing the comparison.
NCHGT X'7B01',':1',6
Changes all hexadecimal strings X'7B01' in the new file, that start in column 6, to the character string
":1" before performing the comparison.
NCHGT 'PREF???','NPREF'
Changes all 7-character strings with the prefix "PREF" in the new file, to the 5-character string
"NPREF" before performing the comparison.
NCHGT 'PREF???','NPREF??'
Changes the first 5 characters of all 7-character strings with the prefix "PREF" in the new file, to
"NPREF" before performing the comparison.
Comment lines
There are two tags that if found at the start of a line turn it into a comment line:
*
An asterisk as the first character on a process statement line begins a printable comment line.
Comment lines
448  z/OS: z/OS ISPF User's Guide Vol II

## Page 487

.*
A period-asterisk as the first two characters on a process statement line begins a comment that is not
printed in the SuperC listing.
Compare Types: FILE, LINE, WORD, BYTE, and Search
*
. *
comment
*
Must be in column 1.
.*
Must be in columns 1 and 2.
Example
Description
*
This comment prints in the SuperC listing.
.*
This comment does not print in the SuperC listing.
Compare byte offsets
The CMPBOFS process statement compares a file between byte limits. The start and stop reference
values must be hex values. The statement may be specified on one complete line or may have separate
CMPBOFS statements for each of the six keyword operands: TOP, BTM, NTOP, NBTM, OTOP, and OBTM.
Compare Type: BYTE
CMPBOFS TOP
BTM
NTOP
OTOP
NBTM
OBTM
hex_offset
keyword
The keyword may have one of the following values:
TOP
Top. Defines the first byte offset position in the new and old byte compare file. Means both NTOP
and OTOP. The lowest byte position is at offset zero.
NTOP
New Top. Defines the first byte offset position in the new file for the byte compare.
OTOP
Old Top. Defines the first byte offset position in the old file for the byte compare.
BTM
Bottom. Defines the last byte position in the new and old byte compare file. Means both NBTM and
OBTM.
NBTM
New Bottom. Defines the ending point in the new file for the compare.
OBTM
Old Bottom. Defines the ending point in the old file for the compare.
Compare byte offsets
Appendix A. SuperC reference  449

## Page 488

hex_offset
A hexadecimal value. Do not put in apostrophes, or 'bracket' it within "X'...'".
Example
Description
CMPBOFS NTOP 1000 OTOP 5E00
Compare the new file from hex offset X'1000' (to the end of file) with the old file from hex offset
X'5E00' (to the end of file).
CMPBOFS NTOP 1000CMPBOFS OTOP 5E00
These two separate process statements have the same effect as the "combined" statement above.
Compare (search) columns
There are three Compare Columns process options:
CMPCOLM
Applies to both the new and old files, or search file
CMPCOLMN
Applies to the new file
CMPCOLMO
Applies to the old file
These options compare (or search) the data between column limits of the input files (or search file). Up
to 15 compare ranges or individual columns are allowed and may be entered on additional CMPCOLM,
CMPCOLMN, or CMPCOLMO statements. All specified ranges of columns must be in ascending order.
Compare Types: LINE and WORD CMPCOLM is also valid for Search.
Note:
1. Some process options (SEQ, NOSEQ, and COBOL) also specify columns. The CMPCOLM, CMPCOLMN,
CMPCOLMO process statements override all these process options.
2. CMPCOLM, CMPCOLMN, CMPCOLMO cannot be used for WORD compare type or Search if the input
contains a mixture of DBCS and non-DBCS data.
CMPCOLM
CMPCOLMN
CMPCOLMO
,
start_column
: end_column
start_column
The starting column number to be compared or searched.
end_column
The ending column number of the range of columns to be compared or searched. (Must be separated
from the start_column by a colon.)
Example
Description
CMPCOLM 25:75
Compare columns 25 through 75 in both files (or search columns 25 through 75 in the search file).
CMPCOLM 30:60,75
Compare columns 30 through 60 and column 75 in both files (or search columns 30 through 60 and
column 75 in the search file).
CMPCOLMN 48:54
Compare columns 48 through 54 in the new file.
Compare (search) columns
450  z/OS: z/OS ISPF User's Guide Vol II

## Page 489

CMPCOLMO 87
Compare column 87 in the old file.
CMPCOLMN 17:22
CMPCOLMO 15:20
Compare columns 17 through 22 in the new file with columns 15 through 20 in the old file.
Note: Small CMPCOLM values can sometimes lead to false matches. See “How SuperC matches input
files” on page 470 for more information.
Compare lines
The CMPLINE process statement compares two files (or search) between line limits. The statement may
be specified on one complete line or may have separate CMPLINE statements for each of the six keyword
operands: TOP, BTM, NTOP, NBTM, OTOP, and OBTM. The reference values may be line numbers or data
strings.
Compare Types: LINE, WORD, and Search
Note: Keyword operands OTOP and OBTM are invalid for Search.
CMPLINE TOP
NTOP
OTOP
BTM
NBTM
OBTM
line number
String operands
String operands
, ' search_string '
, start_column
: last_start_column
keyword
The keyword may have one of the following values:
TOP
Top. Defines the beginning line in the new (or search) file and old compare file. Means both NTOP
and OTOP.
NTOP
New Top. Defines the beginning line in the new (or search) file.
OTOP
Old Top. Defines the beginning line in the old file.
BTM
Bottom. Defines the ending line in the new (or search) file and old compare file. Means both NBTM
and OBTM.
NBTM
New Bottom. Defines the ending line in the new (or search) file.
OBTM
Old Bottom. Defines the ending line in the old compare file.
line number
The relative number of the record in the file.
Compare lines
Appendix A. SuperC reference  451

## Page 490

search_string
A character or hexadecimal string enclosed within apostrophes. The search string is used explicitly
as coded and is not affected by the ANYC process option. For one embedded apostrophe, use two
consecutive apostrophes ('').
start_column
The column in or after which the search_string must start.
last_start_column
The last column in which the search_string may start. Must be separated from the start_column by a
colon.
Example
Description
CMPLINE TOP 55 BTM 99
Compare from line 55 to line 99 in both files.
CMPLINE NTOP 55 NBTM 99
Compare from line 55 to line 99 in the new file.
CMPLINE NTOP 'ABCD',5:66
Compare from where "ABCD" starts within columns 5 to 66 in new file (that is, is found within columns
5 to 69).
CMPLINE OTOP 'ABCD'
Compare from where "ABCD" first found in old file.
CMPLINE TOP X'40E2',1:1
Compare from where " S" is found for both files.
Compare sections
The CMPSECT process statement compares multiple sections from one sequential data set or PDS
member to another sequential data set or PDS member. It is not valid for a PDS group comparison of
more than one member. It is functionally similar to CMPLINE but allows you to divide the input into one
or more sections for subsequent comparison or searching. A section ID name is needed to associate
all keyword operands to a particular section. Thus, multiple sections of the input can be compared (or
searched) in a single execution of SuperC.
Compare Types: LINE, WORD, and Search
Note:
1. CMPSECT is not supported for CMS.
2. Keywords OTOP and OBTM are invalid for Search.
CMPSECT section_ID TOP
NTOP
OTOP 1
BTM
NBTM
OBTM 1
line_number
String operands
String operands
, ' search_string '
, start_column
: last_start_column
Notes:
Compare sections
452  z/OS: z/OS ISPF User's Guide Vol II

## Page 491

1 Invalid for Search-For.
section_ID
A character string identifier (up to 8 alphanumeric characters, no embedded spaces, can start with a
numeric) relating to a section (group of lines). It allows multiple keywords to be associated with the
same section.
keyword
The keyword may have one of the following values:
TOP
Top. Defines the beginning line in the new (or search) file and old compare section. Means the
same as NTOP and OTOP.
NTOP
New Top. Defines the beginning line in the new (or search) section.
OTOP
Old Top. Defines the beginning line in the old section.
BTM
Bottom. Defines the ending line in the new (or search) file and old compare section. Means both
NBTM and OBTM.
NBTM
New Bottom. Defines the ending line in the new (or search) section.
OBTM
Old Bottom. Defines the ending line in the old compare section.
line_number
The line number associated with the keyword.
search_string
A character or hexadecimal string enclosed within apostrophes. The search string is used explicitly
as coded and is not affected by the ANYC process option. For one embedded apostrophe, use two
consecutive apostrophes ('').
start_column
The column in or after which the search_string must start.
last_start_column
The last column in which the search_string may start. Must be separated from the start_column by a
colon.
Note: If a "top" condition is not found (for example, a pattern is incorrect), the compare continues but
normally reports zero lines processed for this data set.
Example
Description
CMPSECT SECT01 TOP 25 BTM 50
Compares lines 25 through 50 in both data sets or members.
CMPSECT SECT02 NTOP 60 NBTM 70
CMPSECT SECT02 OTOP 65 OBTM 75
Compares lines 60 through 70 in the new data set to lines 65 through 75 in the old data set.
CMPSECT SECTX TOP 'PART1:',2:10
CMPSECT SECTX BTM 'END PART1:',2:10
Starts the comparison of both data sets when SuperC detects the string "PART1:" starting in columns
2 through 10 and ends the comparison when SuperC detects the string "END PART1:" starting in
columns 2 through 10.
Compare sections
Appendix A. SuperC reference  453

## Page 492

CMPSECT SECTY NTOP 'PART2:',2:10
CMPSECT SECTY OTOP 'PART2:',6:20
CMPSECT SECTY BTM 'END PART2:'
Compares a section in the new data set to a section in the old data set. The section in the new data
set begins with the string "PART2:" in columns 2 through 10 and ends with the string "END PART2:"
in columns 2 through 10. The section in the old data set begins with the string "PART2:" in columns 6
through 20 and ends with the string "END PART2:" in columns 2 through 10.
Note: All the previous statements could be combined to compare multiple sections of the new and old
data sets.
Define column headings
The COLHEAD process statement defines column headings and specifies the location and format of the
corresponding data to be displayed. For an example of a listing with column headings, see Figure 259 on
page 485.
Note: COLHEAD is not available for side-by-side listings. (See “NARROW” on page 441.)
Compare Type: LINE
COLHEAD ' heading1 ' ,
' heading2 '
, start_print_column :
end_print_column , N  1 start_column : last_start_column  2
C
B
D
P
Z
, O  1 start_column : last_start_column  2
C
B
D
P
Z
Notes:
1 N and O must be followed by a space.
2 C, B, D, P, or Z must be preceded by a space.
heading1
The heading to appear on the first line for the print column.
heading2
The heading to appear on the second line for the print column.
start_print_column
The starting print column for the heading specified.
end_print_column
The ending print column for the heading specified. (Must be separated from the start_print_column by
a colon.)
Define column headings
454  z/OS: z/OS ISPF User's Guide Vol II

## Page 493

Note: If the print-column range is shorter than the heading specified, the heading is truncated.
N
Indicates the operands following relate to the new file.
start_column
The starting position in the new file of the data to be displayed.
last_start_column
The ending position in the new file of the data to be displayed. (Must be separated from the
start_column by a colon.)
Data Format Indicator
The format of the data in the new file to be displayed:
C
Character
B
Binary
D
Unsigned packed decimal
P
Packed decimal
Z
Zoned decimal
O
Indicates the operands following relate to the old file.
start_column
The starting position in the old file of the data to be displayed.
last_start_column
The ending position in the old file of the data to be displayed. (Must be separated from the
start_column by a colon.)
Data Format Indicator
The format of the data in the old file to be displayed (as for the new file).
Example
Description
COLHEAD 'START','DATE',1:7,N 1:6 P,O 11:16
Defines a print column with a heading of "START" in the first line and "DATE" in the second heading
line, headings to start in print column 1. The data to be displayed from the new file is in positions 1
through 6 and is in packed format. The data to be displayed from the old file is in positions 11 through
16 and is in (the default) character format.
Do not process lines
There are two Do Not Process Lines process statements:
DPLINE
Do not Process Lines
DPLINEC
Do not Process Lines Continuation
These options remove from the compare (or search) set all lines that can be recognized by either a unique
character string or combination of related strings all appearing on the same input line. DPLINEC is the
continuation of the immediately preceding DPLINE or DPLINEC process statement. All the strings in a
DPLINE/DPLINEC group must be found on the same input line.
The DPLINE and DPLINEC strings are used explicitly as coded and are not affected by the ANYC process
option.
Do not process lines
Appendix A. SuperC reference  455

## Page 494

A start_column or start-range can also be used to restrict the columns. Relative start_columns and
start-ranges are valid only on DPLINEC statements.
Compare Types: LINE, WORD, and Search
DPLINE ' string '
, start_column
: last_start_column
DPLINEC ' string '
, start_column
: last_start_column
, + start_column
: last_start_column
, + 
string
A character or hexadecimal string enclosed within apostrophes. For one embedded apostrophe, use
two consecutive apostrophes ('').
start_column
The column in, or after which, the string must start.
last_start_column
The last column in which the string may start. (Must be separated from the start_column by a colon.)
+start_column
The relative column, following the location of the previous string (as specified in the previous DPLINE
or DPLINEC statement), in, or after which, this string must start.
last_start_column
The relative last column, following the location of the previous string (as specified in the previous
DPLINE or DPLINEC statement), in which this string may start.
+
The specified string may appear anywhere following the location of the previous string (as specified in
the previous DPLINE or DPLINEC statement).
Example
Description
DPLINE 'ABCDE'
Scans all columns for string "ABCDE"
DPLINE 'AbCde',2
Scans only column 2 for start of string "AbCde"
DPLINE 'AbCde',2:2
DPLINEC 'BDEF'
Same as above example. String "BDEF" must be on the same line as the string "AbCde"
DPLINE 'ABCDE',2:50
Scans only columns 2 through 50 for start of string "ABCDE"
DPLINE 'AB''CD',2:50
Scans only columns 2 to 50 for start of string "AB'CD"
DPLINE X'C1C27BF1',2:50
Scans only columns 2 to 50 for start of hexadecimal string X'C1C27BF1'
DPLINE 'ABC'
DPLINEC 'BDEF',+
Scans for string "ABC"; if found, then scans for string "BDEF" in the same line (following "ABC")
Do not process lines
456  z/OS: z/OS ISPF User's Guide Vol II

## Page 495

DPLINE 'ABC'
DPLINEC 'BDEF',+5
Scans for string "ABC"; if found, then scans for string "BDEF" starting in the 5th column after the
starting column of "ABC"
DPLINE 'ABC'
DPLINEC 'BDEF',+5:12
Scans for string "ABC"; if found, then scans for string "BDEF" starting anywhere in the 5th to 12th
columns after the starting column of "ABC"
Exclude data
There are two Exclude Data process statements:
NEXCLUDE
Exclude applies to the new file
OEXCLUDE
Exclude applies to the old file
These statements exclude rows or columns of data from the comparison. Up to 254 "exclude" statements
can be entered for each file.
Note:
1. NEXCLUDE and OEXCLUDE statements are mutually exclusive to NFOCUS and OFOCUS statements if
using the same operand keyword (ROWS or COLS).
2. Do not use the NEXCLUDE or OEXCLUDE process statement if the Y2DTONLY process statement has
been specified.
Compare Types: FILE (ROWS option only) and LINE
NEXCLUDE
OEXCLUDE
ROWS
COLS
start_position : end_position
start_position
If ROWS operand used, the first row (record) to be excluded from the comparison. If COLS operand
used, the first column to be excluded from the comparison.
end_position
If ROWS operand used, the last row (record) to be excluded from the comparison. If COLS operand
used, the last column to be excluded from the comparison. (Must be separated from the start_position
by a colon.)
Example
Description
NEXCLUDE ROWS 5:900
Excludes rows (records) 5 through 900 on the new file.
OEXCLUDE ROWS 1:900
Excludes rows (records) 1 through 900 on the old file.
OEXCLUDE COLS 100:199
Excludes columns 100 through 199 on the old file.
Focus on data
There are two Focus on Data process statements:
NFOCUS
Focus applies to the new file
OFOCUS
Focus applies to the old file
Exclude data
Appendix A. SuperC reference  457

## Page 496

These two statements select (or "focus on") rows or columns of data to be compared. In other words, only
these rows or columns are considered when performing the comparison (or search) process and all other
rows or columns are ignored. Up to 254 "focus" statements can be entered for each file.
Note:
1. NFOCUS and OFOCUS statements are mutually exclusive to NEXCLUDE and OEXCLUDE statements if
using the same operand keyword (ROWS or COLS).
2. Do not use the NFOCUS or OFOCUS process statement if the Y2DTONLY process statement has been
specified.
Compare Types: FILE (ROWS option only) and LINE
NFOCUS
OFOCUS
ROWS
COLS
start_position : end_position
start_position
If ROWS operand used, the first row (record) to be selected for the comparison. If COLS operand used,
the first column to be selected for the comparison.
end_position
If ROWS operand used, the last row (record) to be selected for the comparison. If COLS operand used,
the last column to be selected for the comparison. (Must be separated from the start_position by a
colon.)
Example
Description
NFOCUS ROWS 28:90
Selects rows (records) 28 through 90 on the new file.
OFOCUS ROWS 150:165
Selects rows (records) 150 through 165 on the old file.
OFOCUS COLS 10:18
Selects columns 10 through 18 on the old file.
Line count
The LNCT process statement specifies the number of lines per page in the listing file.
Compare Types: FILE, LINE, WORD, BYTE, and Search
LNCT number
number
A decimal number between 15 and 999999.
Example
Description
LNCT 55
Lists up to 55 lines per page.
List columns
The LSTCOLM process statement selects a range of columns to be listed in the output. This statement
overrides the defaults that SuperC generates. Column selections must be contiguous and can be no wider
than the output listing line allocated (55/80/106/176).
Compare Types: LINE and Search
LSTCOLM start_column : last_start_column
start_column
The starting column to be listed.
Line count
458  z/OS: z/OS ISPF User's Guide Vol II

## Page 497

last_start_column
The ending column to be listed. (Must be separated from the start_column by a colon.)
Example
Description
LSTCOLM 275:355
Lists columns 275 through 355 in the output.
List previous-search-following value
The LPSFV process statement specifies the number of lines preceding and following the search line found
to be listed. The default value is 6.
Compare Type: Search
LPSFV number
number
A decimal number between 1 and 50.
Example
Description
LPSFV 2
Lists up to 2 lines before and after the line found.
Revision code reference
The REVREF process statement identifies the revision type (BookMaster or SCRIPT/VS) and level-ID for
delimiting UPDREV and UPDREV2 output changes. The revision delimiter may, alternatively, be specified
or indicated by using a SCRIPT/VS .rc definition statement as the first line of the new input file.
If either the UPDREV or UPDREV2 process option is specified and no REVREF process statement is in the
statements file, or the first new file source line is not a .rc script definition statement, SuperC defaults
the revision definition to a SCRIPT/VS specification of .rc 1 |.
Note: BookMaster requires the REFID value to be defined with a :revision tag. Do not forget the
"RUN=YES" attribute if you want your document to have the change-bar inserted in the processed
document.
Compare Types: LINE and WORD
REVREF REFID = name
RCVAL = number
REFID=name
Name of the revision identifier for the BookMaster :rev/:erev. tags.
RCVAL=number
Numeric revision code for SCRIPT/VS revision tags.
Example
Description
REVREF REFID=ABC
BookMaster example :rev refid=ABC. and :erev refid=ABC. tags.
REVREF RCVAL=5
SCRIPT/VS example .rc 5 on/off delimiters.
Search strings in the input file
There are two process options to search for strings in the input file:
List previous-search-following value
Appendix A. SuperC reference  459

## Page 498

SRCHFOR
Search a text string in the input file
SRCHFORC
Search a text string continuation
These statements search for a specified string in the input Search file. The string may be further qualified
as a word, prefix, or suffix, and where it must be positioned on the line.
SRCHFORC is the continuation of the immediately preceding SRCHFOR or SRCHFORC process statement.
In the case of a SRCHFOR/SRCHFORC group, all the specified strings must occur on the same line for the
search to be successful.
Compare Type: Search
SRCHFOR ' string '
, W
, P
, S
, start_column
: last_start_column
SRCHFORC ' string '
, W
, P
, S
, start_column
: last_start_column
, + start_column
: last_start_column
, + 
string
The character or hexadecimal string to be searched for (enclosed by apostrophes). Use two
consecutive apostrophes ('') for one apostrophe within the search string.
W
Word. String must appear as a separate word. That is, be delimited by one or more spaces or special
characters.
P
Prefix. String must appear as the first part of some other text.
S
Suffix. String must appear as the last part of some other text.
start_column
The column in which the string must start for the search to be successful. (If a last_start_column is
also specified, see description for that operand.)
last_start_column
The "latest" column in which the string can start for the search to be successful. (Must be separated
from the start_column by a colon.)
Search strings in the input file
460  z/OS: z/OS ISPF User's Guide Vol II

## Page 499

+start_column
The relative column (starting from the column where the string for the previous SRCHFOR/SRCHFORC
was found) in which the string must start for the search to be successful. (A corresponding
last_start_column operand can be specified in a similar way to that for the start_column.)
+
The string specified can occur anywhere after the position of the previously found string for the search
to be successful.
Example
Description
SRCHFOR 'ABC'
Searches for string "ABC"
SRCHFOR 'ABC',W
Searches for the word "ABC"
SRCHFOR X'4004'
Searches for the hexadecimal string X'4004'
SRCHFOR 'A''bc'
Searches for string "A'bc"
SRCHFOR 'ABC',5:10
Searches for string "ABC" starting in positions 5 to 10
SRCHFOR 'ABC',W,5
Searches for the word "ABC" starting in position 5
SRCHFOR 'ABC'
SRCHFORC 'DEF'
Searches for strings "ABC" and "DEF" in any order in the same line.
SRCHFOR 'ABC'
SRCHFORC 'DEF',+
Searches for the string "DEF" following the string "ABC"
SRCHFOR 'ABC'
SRCHFORC 'DEF',W,+
Searches for the word "DEF" following the string "ABC"
SRCHFOR 'ABC'
SRCHFORC 'DEF',+5
Searches for the string "DEF" in the 5th position after the string "ABC"
SRCHFOR 'ABC'
SRCHFORC 'DEF',+5
SRCHFORC 'GKL'
Searches for the string "DEF" in the 5th position after the string "ABC" with the string "GKL" also
anywhere in the same line
Select PDS members
The SELECT process statement selects members from a PDS for comparison or for being searched. You
can specify as many member names as fit on one line. If you need to select additional members, enter a
new SELECT statement.
For comparisons, the new members are normally compared with old members that have the same names.
Use the colon character (:) to compare members that are not named alike.
Any number of SELECT statements may be specified.
Compare Types: FILE, LINE, WORD, BYTE, and Search
Select PDS members (z/OS)
Appendix A. SuperC reference  461

## Page 500

SELECT
,
new_member : old_member
new_member
search_member
new_member
The name of a new PDS member that is to be compared to an old PDS member.
old_member
The name of an old PDS member that does not have a like-named member in the new PDS. This
member name, if entered, must be separated from the new_member name by a colon (:).
If the old_member name is not used, SuperC attempts to compare the new_member to a like-named
member of the old PDS.
search_member
The name of the PDS member that is to be searched.
Example
Description
SELECT NEW1,NEW2
For a comparison, compares member NEW1 from the new PDS with the member NEW1 from the old
PDS and compares member NEW2 from the new PDS with the member NEW2 from the old PDS.
For a search, selects members NEW1 and NEW2 from the PDS to be searched.
SELECT NEW1:OLD1,MEMBER2
Compares member NEW1 from the new PDS with the member OLD1 from the old PDS and compares
member MEMBER2 from the new PDS with the member MEMBER2 from the old PDS.
Statements file listing control
The SLIST process statement turns the printing of process statements in the output listing on and off.
The initial setting of this control is ON.
Compare Types: FILE, LINE, WORD, BYTE, and Search
SLIST ON
OFF
ON
Causes the lines in the process statements file following the SLIST statement to be listed in the output
listing.
OFF
Causes the lines in the process statements file following the SLIST statement to be suppressed in the
output listing.
Example
Description
SLIST OFF
Do not list following process statements.
SLIST ON
List following process statements.
Title alternative listing
There are two process statements that let you provide an alternative title:
Statements file listing control
462  z/OS: z/OS ISPF User's Guide Vol II

## Page 501

NTITLE
New (or search) listing file title identification
OTITLE
Old listing file title identification
These statements allow an alternative file identification to be used in the output listing (instead of the
default identifiers "New File ID" and "Old File ID").
Compare Types: FILE, LINE, WORD, BYTE, and Search (NTITLE only)
NTITLE
OTITLE
' title_name '
title_name
The alternative title to be used on the output listing to identify either the "new" file (NTITLE) or the
"old" file (OTITLE). The title name must be in apostrophes and may be up to 54 characters in length.
Use two consecutive apostrophes for one apostrophe within the title name.
Example
Description
NTITLE 'New Title'
Change title heading for new (or search) file to "NEW TITLE"
OTITLE 'Old Title'
Change title heading for old file to "OLD TITLE"
Work size
The WORKSIZE process statement allows the maximum size of the comparison set to be adjusted for
comparing large files.
If WORKSIZE exceeds 99999, then the SuperC LINE comparison DELTA listing type may result in wider
columns for LEN N-LN# and O-LN#. Typically, these columns contain 5-digit values. However, when
WORKSIZE exceeds 5 digits, and providing the standard record length of the listing is not affected, the
columns are extended to contain 7-digit values. If the length of the input source lines in the listing are
such that 7-digit values cannot fit, the report outputs 5-digit values by default, and only reports 7-digit
values when significant characters are otherwise lost.
Compare Type: FILE, LINE, WORD, BYTE. It is ignored if specified on a SEARCH.
WORKSIZE
32000
max_size
max_size
The maximum number of units for comparison. Maximum value is 9999999.
Year aging
There are two process statements for year aging:
NY2AGE
Aging applies to the new file
OY2AGE
Aging applies to the old file
These statements age all the defined dates in either the new or old file. That is, the number of years
specified is added to the "year" portion of each defined date in the file concerned.
Note: Dates are defined  by the Date Definition process statements NY2C, NY2Z, NY2D, NY2P, OY2C,
OY2Z, OY2D, and OY2P; see “Date definitions” on page 464.
Compare Type: LINE
Work size
Appendix A. SuperC reference  463

## Page 502

NY2AGE
OY2AGE
years
years
A number (0 to 999) indicating the number of years by which all defined dates in the file are to be
aged.
Example
Description
OY2AGE 28
Ages all defined dates in the "old" file by 28 years before being compared. The listing shows the
original date. For example, a defined date in the "old" file with a value equating to March 1, 1997, is
aged to March 1, 2025 before being compared to its equivalent in the "new" file.
Date definitions
There are eight process statements that set date definitions:
NY2C
New file, date in character format
NY2Z
New file, date in zoned decimal format
NY2D
New file, date in unsigned packed decimal format
NY2P
New file, date in packed decimal format
OY2C
Old file, date in character format
OY2Z
Old file, date in zoned decimal format
OY2D
Old file, date in unsigned packed decimal format
OY2P
Old file, date in packed decimal format
Note:
1. If any Date Definition process statements are used, also use a Y2PAST process statement, so that the
"century" portion of the date can be determined where necessary. (If a Y2PAST process statement is
not present, a default fixed window based on the current year is used.)
2. For a description of each date format (character, zone, decimal, and packed), see “Date formats
(keyword suffixes: C, Z, D, P)” on page 466.
3. If any Date Definition process statements are used, an information line is generated on the listing
output (see Figure 258 on page 484).
4. Do not use any Date Definition process statements if using the COLHEAD process statement.
Defines the location and format of a date field on the input file. Up to 254 date definition statements can
be entered for each file. The matching of the new to the old dates is performed according to the sequence
that the statements are entered. That is, the first defined old date is matched to the first defined new
date.
If the number of date definition statements for one file differ from the number of date definition
statements for the other file, the location and format details for the "missing" date definition statements
are assumed to be the same as their counterpart date definition statements for the other file.
Compare Type: LINE
Date definitions
464  z/OS: z/OS ISPF User's Guide Vol II

## Page 503

NY2C
NY2Z
NY2D
NY2P
OY2C
OY2Z
OY2D
OY2P
start_column : last_start_column date_format
 1 EMPTY
Notes:
1 The EMPTY keyword, when used, must be preceded by a space
start_column
The first position of the date in the input file.
last_start_column
The last position of the date in the input file. (Must be separated from the start_column by a colon.)
date_format
A mask representing the format of the date.
For a Julian date, the mask must be either YYDDD or YYYYDDD.
For date formats other than Julian, the mask must contain 2 "D"s (representing the day part of the
date field), 2 "M"s (representing the month), and either 2 or 4 "Y"s (representing the year) or, if the
date contains a year only, it must contain either 2 or 4 "Y"s.
If the date is character, there may also be a separator between the different parts. In this case, you
can represent the position of the separators by one of the following characters:
S (indicates that this position within the date is not used in comparison)
. (period, used in comparison)
/ (forward slash, used in comparison)
: (colon, used in comparison)
Note: The length of the date_format mask must correspond to the length of the date in the input file
as indicated by the values of start_column and last_start_column.
EMPTY
This keyword is optional. If it is entered, the defined date field is checked for containing zeros, spaces,
low-values, or high-values before commencing the comparison process. If any of these values are
found, the date is not converted according to the Y2PAST criteria but instead is converted to an
extended format with the initial value. For example, a date defined by the process statement OY2C
YYMMDD which contains all zeros is compared as "YYYYMMDD" with a value of zeros.
Example
Description
NY2C 1:8 MMDDYYYY 9:16 MMDDYYYY 21:28 YYYYMMDD
The new file has dates in character format in columns 1 to 8, 9 to 16 and 21 to 28.
OY2P 5:8 YYMMDD 9:12 YYMMDD
The old file has dates in packed decimal format in columns 5 to 8 and 9 to 12.
OY2P 101:104 MMDDYY
The old file has a date in packed decimal format in columns 101 to 104,
Date definitions
Appendix A. SuperC reference  465

## Page 504

NY2Z 101:108 YYYYMMDD
The new file has a date in zoned decimal format in columns 101 to 108.
NY2C 101:110 YYYY.MM.DD
The new file has a date in character format (with separators) in columns 101 to 110.
OY2C 93:98 DDMMYY EMPTY
The old file has a date in character format in columns 93 to 98. If the date field contains zeros,
spaces, low-values, or high-values, the date in the old file is converted before being compared to an
extended format (DDMMYYYY) with a value of all zeros, spaces, low-values, or high-values.
Date formats (keyword suffixes: C, Z, D, P)
C
Character date data.
Examples:
'96' is represented as hexadecimal X'F9F6'
If using a MMDDYY format, March 21, 1996 is represented as hexadecimal X'F0F3F2F1F9F6'
Z
Zoned decimal date data. The date can be represented as follows:
X'xyxy' to X'xyxyxyxyxyxyxyxy'
y is hexadecimal 0 to 9 and represents a date digit. x is hexadecimal 0 to F and is ignored.
Examples:
'96' is represented as hexadecimal X'F9C6' or X'0906'
'03211996' is represented as hexadecimal X'F0F3F2F1F1F9F9C6' or X'0003020101090906'
P
Packed decimal date data. The date can be represented as follows:
X'zyyx' to X'zyyyyyyyyx'
y is hexadecimal 0 to 9 and represents a date digit. x is hexadecimal A to F and is ignored. The z part is
normally zero but is not ignored.
Examples:
'96' is represented as hexadecimal X'z96F' or X'z96C'
'1996' is represented as hexadecimal X'z1996C'
'03211996' is represented as hexadecimal X'z03211996x' (the x part is ignored).
'96203' (a Julian date) is represented as hexadecimal X'96203C'
D
Unsigned packed decimal date data. The date can be represented as follows:
'yy' to 'yyyyyyyy'
y is hexadecimal 0 to 9 and represents a date digit.
Examples:
'96' is represented as hexadecimal X'96'
'03211996' is represented as hexadecimal X'03211996'
Global date
The Y2PAST process statement specifies a 100-year period (used for determining the century-part of a
date when only a 2-digit year has been specified). The Y2PAST process statement uses either a fixed or
sliding window.
Global date
466  z/OS: z/OS ISPF User's Guide Vol II

## Page 505

Note: Always use the Y2PAST process statement if one of the Date Definition process statements (NY2C,
NY2Z, NY2D, NY2P, OY2C, OY2Z, OY2D, OY2P) has also been used.
Compare Type: LINE
Y2PAST fixed
sliding
fix ed 
A 4-digit number indicating a fixed window.
sliding
A 1-digit or 2-digit number indicating a sliding window.
Example
Description
Y2PAST 1986
A fixed window specifying a 100-year period from 1986 to 2085.
Y2PAST 70
A sliding window specifying (based on the current year being 2001) a 100-year period from 1931 (70
years in the past) to 2030.
Y2PAST 5
A sliding window specifying (based on the current year being 2001) a 100-year period from 1996 (5
years in the past) to 2095.
Reasons for differing comparison results
When comparing two sets of input date, it is possible that different compare types (FILE, LINE, WORD,
and BYTE) gives slightly different results.
In order for SuperC to detect only the types of differences that are of interest to you, make sure that
you are using the most appropriate compare type and, if necessary, the appropriate process options and
process statements to select only the data that you actually want compared.
Here are some of the reasons why different compare types can produce different results:
• FILE and BYTE comparisons inspect the complete file (every byte) for differences. LINE and WORD
comparisons use designated columns that are either specified by you or are within the default range of
columns assigned by SuperC.
For example, a FILE comparison of a file with fixed-length records of eighty bytes compares all columns
(that is, all bytes), whereas a LINE comparison of the same file compares columns 1 to 72 (the default).
Since the sequence number columns in the file are ignored in the LINE compare, the final results can
be different. In this case, for consistent results, specify the LINE compare type and the NOSEQ process
option.
• LINE comparisons "pad" the shorter records with spaces when comparing files with different record
lengths. However, BYTE comparisons only "recognize" spaces when they are already present in the
input file.
• For files with input line lengths <= 256, a LINE comparison is performed after padding the lines to the
longest line length. Consequently two lines, originally of unequal length, compare equally only if the
spaces at the end of the longer line coincide with the shorter line's space padding.
• For files with input line lengths > 256, a LINE comparison is performed on the lines without space
padding. As a result, lines of unequal length are always a mismatch.
• Different compare types have different sensitivity to being resynchronized. Synchronization for a LINE
comparison begins at the beginning of a line and ends at the end of a line. Synchronization for a
WORD comparison begins anywhere on a line on any word boundary and ends at the end of a word.
Synchronization for a BYTE comparison extends only one byte anywhere on a line.
Reasons for differing comparison results
Appendix A. SuperC reference  467

## Page 506

• LINE comparisons detect lines that have been reformatted. However, reformatted lines have no effect
on WORD comparisons as spaces and blank lines are ignored.
• Results may differ depending on which input file is specified as the "new" file and which is specified
as the "old" file. The matching algorithm is sensitive to the largest matched set it finds between files.
There may be occasions where more than one set of matched data meets this criteria. The rules for
deciding which set to choose among the equals depends upon the contents of each file and which file
was nominated as the "new" file.
Return codes
SuperC displays the completion message at the top of the Primary Comparison Menu or at the top of the
Primary Search Menu. The message is an interpretation of the following return codes.
Table 31. SuperC return codes
Code Meaning
0 Normal completion.
Comparison
The input files are the same. No differences found.
Search
No matches found in the input file.
1 Normal completion.
Comparison
Differences were found in the input files.
Search
Matches found in the input file.
4 WARNING. Erroneous input was detected. Files were compared but results may not be
as expected. Check listing for more information.
6 WARNING. Old file did not contain proper sequence numbers, or the sequence
number intervals were not sufficiently large to contain insert activity (UPDCMS8 and
UPDMVS8).
8 ERROR. Error on old input file. Files were NOT compared. Check listing for more
details.
16 ERROR. Error on new or search source file. The operation was NOT performed. Check
listing for more details.
20 ERROR. I/O error writing to update file, FILEDEF missing, or APNDUPD process option
cancelled because of inconsistent file attributes.
24 ERROR. I/O error writing to the output listing file.
25 ERROR. The old output file attributes are not consistent with the new listing
requirements. The APNDLST process option can not be accepted and the operation
is immediately terminated.
26 ERROR. The output file caused a "disk full"condition. The output listing is incomplete.
27 ERROR. The output file is a "read-only" disk. All I/O operations to the disk is
suppressed.
28 ERROR. No data was compared because of invalid file names, no commonly named
members of both input file groups, or one or both input files were empty.
If you specify EMPTYOK as an option, this return code is changed to RC 0. ISRSUPC
continues to print any messages that relate to RC 28.
Return codes
468  z/OS: z/OS ISPF User's Guide Vol II

## Page 507

Table 31. SuperC return codes (continued)
Code Meaning
32 ERROR. Insufficient storage was available for SuperC to execute. Refer to output listing
for more details.
36 ERROR. z/VSE file would not open.
40 ERROR. z/VSE label information not available.
48 ERROR. z/VSE Librarian member not found.
52 ERROR. z/VSE VSAM Showcat failed.
56 ERROR. z/VSE device type not supported.
60 ERROR. Wrong length record read on tape input.
SuperC and search-for technical overview
This topic describes these SuperC and Search-For processes:
• How SuperC and Search-For filter input file lines
• How SuperC matches input file lines
• How SuperC partitions and processes large files
• Why SuperC compare types may produce different results
• How to directly invoke the SuperC and Search-For programs.
How SuperC and search-for filter input file lines
The SuperC and Search-For utilities apply process options and process statements to the input file or files
in a specific order. Figure 252 on page 470 shows schematically the effects, in the order that they occur,
of the various “filtering” process options and process statements, on the compare and Search-For input
lines. The options and statements nearer the top affect the input line before options or statements nearer
the bottom.
SuperC/search-for technical overview
Appendix A. SuperC reference  469

## Page 508

Figure 252. Priority for filt ering  input lines
How SuperC matches input files
When SuperC compares files, it determines matching and missing lines or words based only on the data
content of the input files. It does not use any synchronization data, such as column or sequence numbers,
to find matching file sections. It does not use the common “start at the top”, then look-ahead or look-back
method to determine large sections of matching data. Neither does it sort the data before comparing.
SuperC is unique in that, except for files that are identical, it does not determine matching sections until it
has completely read both files. Missing data units are units that are out of sequence, as opposed to units
that have been deleted from a file. During a comparison, SuperC finds all matches, locates the largest set
of matching data units, and recursively allows this compare set to divide the file into additional partitioned
subsections. All new subsections are processed for corresponding matches. The subprocess ends when
no more matches can be found within corresponding new and old file partitioned subsections. Sections
classified as inserted or deleted are corresponding areas for which SuperC could not find a match.
SuperC/search-for technical overview
470  z/OS: z/OS ISPF User's Guide Vol II

## Page 509

Figure 253 on page 471 shows an example of a comparison of two files that are identified as having lines
represented by A, B, C, ... F. The SuperC algorithm attempts to find the best match set from the input
lines. Notice how the match set requires consideration of duplicate lines.
      New File Lines                              Old File Lines
         ───A───  ────────────Matched Line─────────  ───A───
Inserted ───B───                                     ───I─── Deleted
Inserted ───C───    ┌─────────Matched Line─────────  ───D─── Largest  ──┐
                    │                                                   │
         ───D───  ──┘  ┌──────Matched Line─────────  ───E─── Set        │
                       │                                                │
         ───E───  ─────┘  ┌───Matched Line─────────  ───F─── Unchanged──┘
                          │
         ───F───  ────────┘                          ───B─── Deleted
Inserted ───A───                                     ───C─── Deleted
         ───H───  ────────────Matched Line─────────  ───H───
         ───A───  ────────────Matched Line─────────  ───A───
                                                     ───A─── Deleted
       Sequence                 Left Side               Right Side
       Largest Set           ─    D E F    Divides Set    D E F
       Top Set               ─    A        Matches        A
       Leftover Top Set      ─    B C      Mismatches     I
       Largest Bottom Match  ─    H A      Matches        H A
       Leftover Bottom Set   ─    A        Mismatches     B C A
  Note:  The inserted &odq.A&cdq. on the lower left cannot connect with the
         deleted &odq.A&cdq. on the bottom right due to H A barrier.
Figure 253. Find match example
How SuperC corrects false matches
Occasionally, SuperC reports that it has detected a false line or word match and has corrected the results
in the listing and summary report. Any affected matched pair has been reclassified as an insert/delete
pair. Any resulting error might be in the masking of potential matches that would be overlooked due to the
early false match coupling. That is an equivalent yet undiscovered match may be overlooked due to the
premature false matching. The condition should be of minor importance since it happens so rarely and the
masking effect has a low probability of affecting the final results.
An equally important SuperC concern would be whether it finds the best match set and whether it finds
all matches. Unfortunately, the match-finding algorithm is not perfect. Ignoring the false match masking
problem, and the large number of duplicate source lines obscuring the match set possibilities, occasional
matches can be overlooked. Comparison of files with small CMPCOLM values can sometimes lead to
false matches. Depending on the data, increasing CMPCOLM can sometimes alleviate the number of false
matches reported.
Many Artificial Intelligence (AI) computer programs, like SuperC, only approximate the human intellect
that can sometimes make a better match set selection. Furthermore, these same computer programs are
designed for speed and efficiency. They necessarily make certain simplifying assumptions and contain
additional operational weaknesses. SuperC , however, does not fail to correctly classify mismatches and
does not incorrectly classify a mismatch as a match.
How SuperC partitions and processes large files
In SuperC, there is no limit on the size of files processed in terms of lines, words or bytes. Yet it had an
internal methodology based upon a maximum field size for each work area storage structure (for example,
array size and precision of variables). A method was developed to do the overall comparison process by
breaking very large files into smaller comparison partitions and combining the intermediate results into
one overall result. The process had to be done carefully so that it did not look as if the file partitioning
SuperC/search-for technical overview
Appendix A. SuperC reference  471

## Page 510

was determined after some arbitrary limit was reached. That could affect the results on either side of the
break point. The partitioning had to be done heuristically based upon the comparison results from the
previously inspected intermediate process.
A fixed partitioning size of 32K lines/words/bytes was selected that was based on some test studies. The
compare processes up to this limit and iteratively adjusts the intermediate ending break point of the pass
by an adaptive method. Continuation from the adjusted end point is the basis for the next pass. That end
point might even be adjusted to some previous records that had already been processed. The objective is
to achieve the next best compare set for future unprocessed records.
The overall process ends when both files reach the End-of-File during a pass. The results from the
intermediate passes are combined into one user end result. Most large compares are never suspected to
have been partitioned and recombined.
The unlimited file size solution may appear, at first, unnecessary for Line compare using a virtual address
space that is nearly unlimited. Yet there often has to be some limit—even if it is a high value. Programs
need to store data with predetermined precision limits and programs work better with limits that are
reasonable. Word compare and Byte compare eventually needed a partitioning limit for the compare as
the number of words and bytes become large even for small file sizes.
Because of this partitioning process, comparisons of large files may take a long time.
Comparing and searching alias members
When you compare or search all of the members of two partitioned data sets using the command S
* (to select all entries in the directory) on the member list, any members that have alias entries are
processed once for the real name, and once for each alias entry. For compare, unpaired aliases appear
in the same list as unpaired real members as "NON-PAIRED NEW FILE MEMBERS" or "NON-PAIRED OLD
FILE MEMBERS".
When you compare or search entire data sets by using an asterisk (*) for a member name pattern, only
real members, not aliases, are processed. For compare, all directory entries (for both real members and
aliases) are analyzed. Messages appear at the end of the SuperC output listing that give information about
unpaired alias entries for paired real members as follows:
"NEW" PAIRED MEMBERS WITH "NEW" ALIAS MEMBERS NOT PAIRED FOLLOW:
         MEMBER1/ALIAS1  MEMBER1/ALIAS2  MEMBER2/ALIAS1 ...
          or
"OLD" PAIRED MEMBERS WITH "OLD" ALIAS MEMBERS NOT PAIRED FOLLOW:
         MEMBER1/ALIAS1  MEMBER1/ALIAS2  MEMBER2/ALIAS1 ...
 
followed by a listing of the unpaired member/alias entries.
Note:
1. Consider the ALLMEMS process option if you want to compare all directory entries, including aliases,
but do not want to select them from a member list. This is useful for batch comparisons of entire load
modules.
2. This listing section is not created for non-load module data sets containing alias entries.
Comparing load modules
SuperC compare of load module data might show unexpected differences. This is because SuperC
compares all the data in the load module as it is found on DASD, and does not attempt to decode which
portions are executable, and which might contain uninitialized storage.
The complex data format on DASD is dependent on the load module data set block size, and defined
storage definitions which are controlled by the linkage editor. The size stored by the linkage editor in the
PDS directory may differ from the DASD data byte count reported by SuperC and Browse depending on
the characteristics of the load module.
SuperC/search-for technical overview
472  z/OS: z/OS ISPF User's Guide Vol II

## Page 511

If load modules are exact copies of each other, SuperC should find no differences. If load modules have
been link-edited from the same object but with different block sizes, SuperC will probably report they are
different.
Because of the relative DASD addresses (TTRs) in load modules, the recommended procedure for
comparing load modules which have not been reblocked is to use the AMBLIST utility with LISTLOAD
OUTPUT=MODLIST against both load modules, then use SuperC to compare the two AMBLIST outputs.
There is no easy way to compare load modules with different internal record sizes such as occurs when
COPYMOD or LINKEDIT processes them.
Comparing CSECTs
SuperC compare of PDS Load Module Csects (using the LMCSFC Process Option) can return unexpected
differences. SuperC looks at the length of the Csect from the control record immediately preceding the
Csect data record in the load module. This physical data length can differ from the logical Csect data
length in the load module header that the AMBLIST utility uses to report the length of the Csect.
SuperC always compares all of the physical data in each Csect. You can use SuperC Byte compare to
examine the Csect data content in detail.
Note: This option is only valid for PDS load modules.
How to directly invoke SuperC and search-for
You can run the SuperC and Search-For programs directly without using the ISPF-provided utilities
(Options 3.12, 3.13, 3.14, or 3.15). This requires an installation (or system programmer) to customize
a CLIST (for interactive usage) or a PROCLIB procedure (for batch execution of a catalog procedure).
Although these methods are not warranted by the ISPF product, a sample CLIST and a sample PROCLIB
procedure are distributed as an aid in the SAMPLIB data set as members ISRSCLST and ISRSPROC.
The sample CLIST allows a TSO user to enter a line command to communicate the operational parameters
directly to the SuperC program without displaying the ISPF panels. The sample CLIST will request entry of
a search pattern or string. A sample SuperC call as entered on the terminal might look like:
    superc newfile(.newdata.file) oldfile(ludlow.olddata.file)
 
or
    exec clist(superc) 'new(.newdata.file) old(ludlow.olddata.file)'
where superc is the command and newfile  and oldfile  are the keywords for the input files.
The SuperC load module may be supported using a private library or a concatenated system library. The
installation is responsible for making the corresponding changes to the sample CLIST.
The sample CLIST uses this format:
SUPERC NEW(dsn) OLD(dsn) {keyword(parameter) .... }
Note: Avoid using uninitialized data sets (that is, empty sequential data data sets with no end-of-file
marker) in the concatenation of data sets to be compared. Including these data sets in the search can
lead to unpredictable results.
The keywords and parameters are:
CTYPE
Specifies the compare type. The parameter can be one of the SuperC compare types (File, Line, Word,
or Byte). To call the Search-For program, use CTYPE(SRCH).
LISTING
Specifies the listing type. The parameter can be one of the SuperC listing types.
SuperC/search-for technical overview
Appendix A. SuperC reference  473

## Page 512

OUTDD
Specifies the name of the Listing Data Set. Use a fully qualified dsn or use a period (.) to precede the
dsn with SYSPREF. The use of the period is a compromise because fully qualified names enclosed in
quotes are difficult to pass in CLISTs.
BROWSE
Specifies the auto display program.
SYSIN
Specifies whether SuperC prompts the user for the process statements or uses a statements data set.
The parameters can be PROMPT or the name of the statements data set.
DELDD
Specifies the name of the update data set.
PROCESS
Specifies the process options. The parameter can be a SuperC or Search-For process option. Not all
options are allowed with each compare type (for example, GWCBL is valid only with Line and Word
compare) or with other options (for example, you cannot use SEQ with COBOL). See “Process options”
on page 434 for more information.
When coding the JCL yourself, the following options are specified in the PARM field. Each may be
separated by either a space or a comma.
compare_type
The type of comparison you want performed: FILE, LINE, WORD, or BYTE. When specifying
the compare type in the PARM parameter, add the suffix "CMP" (for example, WORD becomes
WORDCMP).
listing_type
The type of listing you want from the comparison: OVSUM, DELTA, CHNG, LONG, or NOLIST. When
specifying the listing type in the PARM parameter, add the suffix "L" (for example, CHNG becomes
CHNGL).
process_options
Process options are keywords that direct SuperC how to perform the comparison or format the listing.
Process options can be separated by spaces or commas.
Examples
This example shows a SuperC compare JCL sequence:
    //COMPARE  EXEC PGM=ISRSUPC,PARM=('LINECMP,CHNGL,UPDCNTL')
    //STEPLIB  DD   DSN=ISPF.LOAD,DISP=SHR
    //NEWDD    DD   DSN=DLUDLOW.PDS(TEST1),DISP=SHR
    //OLDDD    DD   DSN=DLUDLOW.PDS(TEST2),DISP=SHR
    //OUTDD    DD   SYSOUT=A
    //DELDD    DD   DSN=DLUDLOW.UCTL1,DISP=OLD
    //SYSIN    DD   *
       CMPCOLM 2:72
    /*
The sequence allows the SuperC program to compare two input data sets and generates a line compare
CHANGE type listing to the spool output queue and a separate UPDCNTL update control data set output
using source columns 2 through 72.
A catalog procedure is a set of “canned” JCL statements that you can invoke as an extension of your own
JCL. Here is a simplified JCL sequence:
    //SUPERC JOB
    //       EXEC  SUPERC,
    //       NEWFILE='DLUDLOW.GROUP.DATA1',
    //       OLDFILE='MFRAME.GROUP.DATA2',
    //       LISTING=DELTA
 
The keywords NEWFILE, OLDFILE, and LISTING cause symbolic substitution before the job submittal.
SuperC/search-for technical overview
474  z/OS: z/OS ISPF User's Guide Vol II

## Page 513

Note: A sample catalog procedure is contained in the SAMPLIB member ISRSPROC.
A simplified Search-For JCL sequence follows. The SRCHFOR process statement used in the search is part
of the JCL instead of a separate SYSIN data set. Concatenated data sets are also shown as part of the JCL.
    //         JOB
    //SEARCH   EXEC PGM=ISRSUPC,PARM=('SRCHCMP,ANYC')
    //STEPLIB  DD   DSN=ISPF330.LOAD,DISP=SHR
    //NEWDD    DD   DSN=USERID.PDS,DISP=SHR
    //         DD   DSN=USERID.PDS2,DISP=SHR
    //OUTDD    DD   SYSOUT=*
    //SYSIN    DD     *
   SRCHFOR 'NEEDLE',W,10:20
    /*
    //
 
A very simplified Search-For sample CLIST follows:
   PROC 0
   FREE   FI(NEWDD,SYSIN,OUTDD,SYSIN2)
   ALLOC  FI(NEWDD) DA('USERID.PDS(TEST1)')        REUSE SHR
   ALLOC  FI(SYSIN) DA('USERID.SYSIN.DATA(STMTS)') REUSE SHR
   DELETE              'USERID.USER.PDS'
   ALLOC  FI(OUTDD) DA('USERID.USER.LIST') SPACE(10,20) RECFM(F B) +
               REUSE TRACKS RELEASE
   CALL 'USERID.ISPF.LOAD(ISRSUPC)' 'SRCHCMP,ANYC'
  /******************************************************/
  /* SIMPLE CLIST WITH MINIMUM STATEMENTS.              */
  /* “USERID.SYSIN.DATA(STMTS)” MUST CONTAIN SRCH STMTS.*/
  /******************************************************/
 
SuperC/search-for technical overview
Appendix A. SuperC reference  475

## Page 514

SuperC/search-for technical overview
476  z/OS: z/OS ISPF User's Guide Vol II
