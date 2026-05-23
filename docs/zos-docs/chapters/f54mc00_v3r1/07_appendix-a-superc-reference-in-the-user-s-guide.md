# Appendix A. SuperC Reference in the User's Guide

Source file: f54mc00_v3r1.md
Start page: 922
Page span: 922-928

## Page 922

Programmer response
Ensure the first "search" statement is a SRCHFOR
followed, if necessary, by a SRCHFORC statement
containing "continuation" information.
Refer:
SuperC Search-For Process Statements in
Appendix A. SuperC Reference in the User's Guide
Vol II
ISRS040W SOME LINES OVERFLOW WITH
CHANGE TEXT SUBSTITUTION.
RESULTS MAY BE AFFECTED.
Explanation
Change text (NCHGT/OCHGT process statement) has a
different length than search text. The result could run
past the end of the record.
System action
The SuperC run continues.
Programmer response
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS041W UPDLDEL OPTION INVALID DUE
TO INCONSISTENT LRECL OR
RECFM ATTRIBUTES.
Explanation
If input is fixed, then both files must be the same
record length. The UPDLDEL option is ignored.
System action
The SuperC run continues (without UPDLDEL process
option).
Programmer response
Refer:
SuperC Compare Process Options in Appendix A.
SuperC Reference in the User's Guide Vol II
ISRS042W NCHGT AND OCHGT MIXED DBCS
PATTERNS MUST BE THE SAME
LENGTH. STATEMENT REJECTED.
Explanation
The lengths of the search text and change text must be
equal length in DBCS.
System action
The SuperC run continues.
Programmer response
Correct NCHGT or OCHGT process statement.
ISRS043W CMPCOLM NOT VALID FOR
MIXED DATA AND SRCHCMP
OR WORDCMP OPERATIONS.
STATEMENT REJECTED.
Explanation
CMPCOLM process statement cannot be used with
search or WORD compare type when the input
contains a mixture of DBCS and non-DBCS data.
System action
The SuperC run continues.
Programmer response
Correct process statement or change to a line
compare.
ISRS044W MIXING CMPLINE, CMPSECT,
AND CMPBOFS STMTS IS NOT
ALLOWED. STATEMENT REJECTED.
Explanation
Invalid combination of process statements.
System action
The SuperC run continues.
Programmer response
Use only one of these type of process statements at a
time.
ISRS045W statement-type STATEMENT(S)
ONLY ALLOWED WITH SINGLE
MEMBERS OR SEQUENTIAL
FILES/DATA SETS. STATEMENT
REJECTED.
SuperC messages
902  z/OS: z/OS ISPF Messages and Codes

## Page 923

Explanation
An NTITLE, OTITLE or CMPSECT process statement
has been used for a "group" of files or members. These
statements are only valid for single members or files.
System action
The SuperC run continues.
Programmer response
Specify a single member/file.
ISRS051W CONFLICTING FOCUS/EXCLUDE
STATEMENTS DEFINED.
Explanation
NEXCLUDE/OEXCLUDE process statements are
mutually exclusive to NFOCUS/OFOCUS respectively if
using the same operand keyword (ROWS or COLS).
System action
The SuperC run continues.
Programmer response
Check that the NEXCLUDE/OEXCLUDE and NFOCUS/
OFOCUS process statements will "exclude" and
"focus" on the data you want without conflicting with
each other.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS052W WRONG DATE FORMAT IN NEW
FILE
Explanation
Date definition format in NY2C/NY2Z/NY2D/NY2P
statement is invalid. Date is ignored.
System action
The SuperC run continues.
Programmer response
Correct process statement.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS053W WRONG DATE FORMAT IN OLD
FILE
Explanation
Date definition format in OY2C/OY2Z/OY2D/OY2P
statement is invalid. Date is ignored.
System action
The SuperC run continues.
Programmer response
Correct process statement.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS054E "NEW" FILE/DATA SET NAME/
MEMBER IS INVALID OR AN
ERROR WAS ENCOUNTERED
DURING OPEN. OPERATION
TERMINATED.
Explanation
"New" input file could not be found or a problem was
encountered during the open process.
System action
The SuperC run terminates.
Programmer response
Check that the "new" file name has been specified
correctly
ISRS055E "OLD" FILE/DATA SET NAME/
MEMBER IS INVALID OR AN
ERROR WAS ENCOUNTERED
DURING OPEN. OPERATION
TERMINATED.
Explanation
"Old" input file could not be found or a problem was
encountered during the open process.
System action
The SuperC run terminates.
Programmer response
Check that the "old" file name has been specified
correctly
SuperC messages
Chapter 5. SuperC messages  903

## Page 924

ISRS056E "SRH" FILE/DATA SET NAME/
MEMBER IS INVALID OR AN
ERROR WAS ENCOUNTERED
DURING OPEN. OPERATION
TERMINATED.
Explanation
New file could not be opened
System action
The SuperC run terminates.
Programmer response
Check that the data set/file has been assigned
correctly.
ISRS057E THE INPUT FILES/DATA SETS
COULD NOT BE PROCESSED. BOTH
MUST BE SEQUENTIAL OR A
WHOLE PDS/MACLIB.
Explanation
Cannot compare a PDS/MACLIB/TXTLIB/Librarian with
a sequential file/data set.
System action
The SuperC run terminates.
Programmer response
Ensure input files are comparable.
ISRS058E MEMORY AVAILABLE WAS
INSUFFICIENT. OPERATION
TERMINATED.
Explanation
There was insufficient memory available for SuperC to
run.
System action
The SuperC run terminates.
Programmer response
Increase amount of memory available.
ISRS059E A SYNAD ERROR INTERCEPT
ON THE NEW-FILE/DATA SET IS
AN I/O ERROR, CONCATENATION
ORDERING OR ATTRIBUTE
CONFLICT.
Explanation
New file/data set I/O error.
System action
The SuperC run terminates.
Programmer response
Ensure that no unitialized data sets are included in the
input file concatenation, then refer to your systems
programmer. In this context, an unitialized data set
is an empty sequential data set with no end-of-file
marker.
ISRS060E A SYNAD ERROR INTERCEPT
ON THE OLD-FILE/DATA SET IS
AN I/O ERROR, CONCATENATION
ORDERING OR ATTRIBUTE
CONFLICT.
Explanation
Old file/data set I/O error.
System action
The SuperC run terminates.
Programmer response
Ensure that no unitialized data sets are included in the
input file concatenation, then refer to your systems
programmer. In this context, an unitialized data set
is an empty sequential data set with no end-of-file
marker.
ISRS061E A SYNAD ERROR INTERCEPT ON
THE UPD-FILE/DATA SET WAS
DETECTED. THE OUTPUT MAY BE
INCOMPLETE.
Explanation
Update file/data set I/O error.
System action
The SuperC run terminates.
Programmer response
Refer to your system programmer.
ISRS062E UPDATE FILE/DATA SET, DELDD,
MISSING OR INCOMPATIBLE
ATTRIBUTES/LRECL FOR PDS/
SuperC messages
904  z/OS: z/OS ISPF Messages and Codes

## Page 925

MACLIB. UPDATE OPTIONS
CANCELED.
Explanation
Update/delta file requested but there is no assignment
for it.
System action
The SuperC run terminates.
Programmer response
Refer to your system programmer.
ISRS063E member-name - SYNAD
ERROR INTERCEPT OCCURRED
PROCESSING NAMED MEMBER.
Explanation
I/O error on processing member.
System action
The SuperC run terminates.
Programmer response
Refer to your system programmer.
ISRS064E data-set-name COULD NOT BE
OPENED
Explanation
Problem encountered when trying to open data set.
System action
The SuperC run terminates.
Programmer response
Correct either the data-set-name process statement or
the dataset-name JCL statement.
ISRS065E LABEL INFORMATION NOT
AVAILABLE FOR data-set-name.
Explanation
Label details for data set missing.
System action
The SuperC run terminates.
Programmer response
Correct either the data-set-name process statement or
the data-set-name JCL statement.
ISRS066E data-set-name IS A NATIVE VSAM
FILE. NATIVE VSAM IS NOT
SUPPORTED.
Explanation
VSAM catalog indicates that this file is not a VSAM-
managed SAM file.
System action
The SuperC run terminates.
Programmer response
Refer to your system programmer.
ISRS067E data-set-name SHOWCAT
FAILURE.
Explanation
Error in accessing VSAM catalog.
System action
The SuperC run terminates.
Programmer response
Make sure the data-set-name is assigned correctly.
ISRS068E data-set-name DEVICE TYPE NOT
SUPPORTED.
Explanation
data-set-name is supported for disk and tape only.
System action
The SuperC run terminates.
Programmer response
Correct the data-set-name to ensure it is assigned to
disk or tape.
ISRS069W LIBRARY MEMBER IN data-set-
name NOT FOUND.
Explanation
Member could not be found in library.
SuperC messages
Chapter 5. SuperC messages  905

## Page 926

System action
The SuperC run continues (without this member).
Programmer response
Inspect output listing for further details.
ISRS070W REQUEST FOR WIDE OPTION NOT
SUPPORTED BY SYSLST. NARROW
OPTION WILL BE SUBSTITUTED.
Explanation
The WIDE process option requires a printing device
capable of printing lines up to 202 characters long.
The 55-character side-by-side NARROW option has
been used instead.
System action
The SuperC run continues.
Programmer response
Refer to your system programmer.
ISRS071W SIDE BY SIDE LISTINGS NOT
ALLOWED WHEN USING COLHEAD
PROCESS STATEMENT.
Explanation
The NARROW process option cannot be used with the
COLHEAD process statement.
System action
The COLHEAD statements are accepted and the
NARROW (side-by-side) process option is ignored. The
SuperC run continues.
Programmer response
Check that you are using the correct process options
and statements.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Compare Process Statements
ISRS072W UPDATE PROCESS OPTIONS
INCOMPATIBLE WITH Y2DTONLY
PROCESS OPTION.
Explanation
Update process options cannot be used with the "Year
2000 Compare Dates Only" process option.
System action
The UPD… process option is ignored. The SuperC run
continues.
Programmer response
Check that you are using the correct process options
and statements.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Compare Process Statements
ISRS073W Y2PAST PROCESS STATEMENT
SPECIFIED WITHOUT ANY
DATE DEFINITION PROCESS
STATEMENTS.
Explanation
A Y2PAST process statement has been used but
there are no accompanying Year 2000 Date Definition
process statements.
System action
The Y2PAST process option is ignored. The SuperC run
continues.
Programmer response
Check that you are using the Year 2000 process
statements correctly. Either the Y2PAST process
statement should be removed—or one or more date
definition process statements should be included.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS074W FOCUS/EXCLUDE PROCESS
STATEMENTS ARE IGNORED
WHEN USING THE Y2DTONLY
PROCESS OPTION.
Explanation
NFOCUS, OFOCUS, NEXCLUDE, and OEXCLUDE
process statements have no effect when the
Y2DTONLY process option is used.
SuperC messages
906  z/OS: z/OS ISPF Messages and Codes

## Page 927

System action
The FOCUS/EXCLUDE process statements are ignored.
The SuperC run continues.
Programmer response
Check that you are using the correct process options
and statements.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Compare Process Statements
ISRS075I DATE DEFINITION PROCESS
STATEMENTS ARE IGNORED
WHEN USING THE COLHEAD
PROCESS STATEMENT.
Explanation
Year 2000 Date Definition process statements cannot
be used with the COLHEAD process statement. (The
Year 2000 process statements generate their own
information line for which column headings are not
appropriate.)
System action
The Year 2000 Date Definition process statements are
ignored. The SuperC run continues.
Programmer response
Check that you are using the correct process
statements.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS076I FOCUS/EXCLUDE OF ROWS USED
FOR ONLY ONE FILE. ALL ROWS
PROCESSED IN THE OTHER FILE.
Explanation
A "focus" (NFOCUS or OFOCUS) or an "exclude"
(NEXCLUDE or OEXCLUDE) process statement has
been specified for one file but not for the other file.
System action
All rows (records) of the file for which no "focus"
or "exclude" statement exists are included in the
comparison process.
Programmer response
Check that you are using the "focus" or "exclude"
process statements correctly.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS078I FALSE MATCHES CORRECTED.
RESULTS MAY NOT REFLECT
ALL MATCHES. SEE "ADDITIONAL
NOTES" IN SUPERC TUTORIAL.
Explanation
Occasionally, SuperC reports that it has detected a
false line or word match and has corrected the results
in the listing and summary report.
System action
The SuperC run continues.
Programmer response
None.
Refer:
How SuperC Corrects False Matches in Appendix A.
SuperC Reference in the User's Guide Vol II
ISRS079W FMSTOP OPTION ONLY VALID
WITH FILE COMPARE OR SEARCH.
Explanation
The FMSTOP option is set for a compare that is not a
file compare.
System action
The FMSTOP option is ignored.
Programmer response
Remove the FMSTOP option, or change the compare to
a FILE compare.
ISRS080W FOCUS COLS AND EXCLUDE
COLS ARE NOT ALLOWED WITH
CMPCOLM. STATEMENT IGNORED.
Explanation
A CMPCOLM, CMPCOLMN or CMPCOLMO statement
has preceded this FOCUS/EXCLUDE COLS statement.
SuperC messages
Chapter 5. SuperC messages  907

## Page 928

System action
The CMPCOLM, CMPCOLMN, CMPCOLMO statements
will be processed and the FOCUS/EXCLUDE COLS
statements will be ignored.
Programmer response
To limit a compare or search within column
boundaries, either the FOCUS/EXCLUDE COLS or
the CMPCOLM, CMPCOLMN, CMPCOLMO process
statements should be used but not a combination of
both.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS081W CMPCOLM STATEMENTS ARE
IGNORED WHEN USING THE
Y2DTONLY PROCESS OPTION.
Explanation
The Y2DTONLY process option is used when only
comparing dates. Using a CMPCOLM type of process
statement is not allowed with it.
System action
The CMPCOLM, CMPCOLMN, CMPCOLMO statements
will be ignored.
Check that you are using the correct process options
and statements.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Compare Process Statements
Programmer response
Determine which type of compare is required and
either use Y2DTONLY or CMPCOLM, not both.
ISRS082W CMPCOLM IS NOT ALLOWED WITH
EITHER FOCUS COLS OR EXCLUDE
COLS. STATEMENT IGNORED.
Explanation
A FOCUS/EXCLUDE COLS statement has preceded this
CMPCOLM, CMPCOLMN, CMPCOLMO statement.
System action
The FOCUS/EXCLUDE COLS statements will
be processed and the CMPCOLM, CMPCOLMN,
CMPCOLMO statements will be ignored.
Programmer response
To limit a compare or search within column
boundaries, either the FOCUS/EXCLUDE COLS or
the CMPCOLM, CMPCOLMN, CMPCOLMO process
statements should be used but not a combination of
both.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
SuperC messages
908  z/OS: z/OS ISPF Messages and Codes
