# Chapter 5. SuperC messages

Source file: f54mc00_v3r1.md
Start page: 915
Page span: 915-921

## Page 915

Chapter 5. SuperC messages
There are three levels of SuperC messages:
• Informational messages do not affect the return code. SuperC completes normally.
• Warning messages return a code of 4 to 7. Processing is completed, but some user option/operation
may not be completely performed.
• Error messages are accompanied by a return code of 8 or greater. Processing is prematurely terminated.
This topic explains the SuperC message format and the messages you may receive.
Each of the messages issued by SuperC is of the form:
ISRSnnns
where:
ISRS
is the program identifier for SuperC
nnn
represents a particular message number
s
is the message severity level:
I
Informational message
W
Warning message
E
Error message
ISRS001I EMPTY COMPARE SET, INVALID
NAMES, NO COMMON NAMED
EMPTY FILES/DATA SETS, OR
ZERO COMPARE AFTER FILTERED.
Explanation
No data has been found to be compared.
System action
The SuperC run continues.
Programmer response
Check that the file/member name(s) have been
entered correctly. Also, check that the parameters for
any select, focus/exclude options are correct.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Compare Process Statements
ISRS002I NO UPDATE FILE/DATA SET
GENERATED FOR UPDCMS8 OR
UPDMVS8 OPTIONS WHEN NO
INPUT DIFFERENCES ARE FOUND.
Explanation
No differences in the input have been found. The
update process option specified does not create an
output update file in this situation.
System action
The SuperC run continues.
Programmer response
None.
ISRS003I THE COMPARISON OPERATION
WAS EXECUTED UNDER STORAGE
CONSTRAINTS THAT MAY AFFECT
RESULTS/THROUGHPUT.
SuperC messages
© Copyright IBM Corp. 1980, 2024 895

## Page 916

Explanation
Insufficient storage available for normal processing.
Results are unpredictable. Output may be formatted
incorrectly.
System action
The SuperC run continues.
Programmer response
Specify a larger region parameter on the JCL and
resubmit the job.
ISRS004I LISTING LINES MAY BE
TRUNCATED DUE TO LIMITING
OUTPUT LINE WIDTH.
Explanation
The length of the data being printed is less than the
length of one of the records. This would be normal for
a NARROW listing of 80 character records.
System action
The SuperC run continues.
Programmer response
The maximum listing length is 80 characters. If the
data has records greater than 80, the part after the
80th character will not be displayed. If the length of
the data is between 56 and 80 characters, the WIDE
option will give a side-by-side listing of 80 characters
from each file.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Search-For Process Options
ISRS005I NO DATA SEARCHED INVALID
NAME(S), EMPTY MEMBERS
PROCESSED OR ZERO SEARCH
SET AFTER INPUT FILTERING.
Explanation
No data has been found to be searched.
System action
The SuperC run continues.
Programmer response
Check that the file/member name(s) have been
entered correctly. Also, check that the parameters
for any SELECT, FOCUS/EXCLUDE process options are
correct.
Refer:
SuperC Search-For Process Options in Appendix A.
SuperC Reference in the User's Guide Vol II
ISRS006I UPDATE PROCESSING DETECTED
SEQUENCE NUMBERING ERRORS.
Explanation
The sequence numbers on one or both input files have
found to be incorrect.
System action
The SuperC run continues.
Programmer response
Check sequence numbering on input.
ISRS007I MOVED LINE FLAGGING ONLY
VALID FOR FIRST 32K
LINES PORTION OF COMPARE
OPERATION PER DATA SET (OR
FILE).
Explanation
Process option FMVLNS (Flag Moved Lines) restricted
to a maximum of 32K "blocks" of moved lines.
System action
The SuperC run continues.
Programmer response
None.
ISRS009W GWCBL OPTION AND Y2DTONLY
MUTUALLY EXCLUSIVE. GWCBL IS
IGNORED.
Explanation
GWCBL and Y2DTONLY process options cannot be
used together.
System action
The SuperC run continues (without GWCBL process
option).
SuperC messages
896  z/OS: z/OS ISPF Messages and Codes

## Page 917

Programmer response
None.
ISRS010W process-option PROCESS OPTION
PARAMETER IS NOT A VALID
PROCESS OPTION. IT IS
IGNORED.
Explanation
process-option is not a valid process option keyword
and has been ignored.
System action
The SuperC run continues.
Programmer response
Check that the process option(s) have been entered
correctly.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Search-For Process Options
ISRS011W start-value SPECIFIED START
VALUE GREATER THAN STOP
VALUE. STOP VALUE CHANGED TO
MAXIMUM VALUE.
Explanation
When nominating a range, the start value for the
range has been specified with value greater than the
stop value for the range. SuperC has attempted to
accommodate the range by extending the stop value
to the maximum value for the line or file concerned.
System action
The SuperC run continues.
Programmer response
Check start and stop values for range(s).
ISRS012W SRCHFOR STATEMENT(S)
MISSING FOR SEARCH-FOR
COMPARE TYPE REQUEST. ZERO
LINES WILL BE INSPECTED.
Explanation
SuperC expected 1 or more SRCHFOR process
statements to be present (specifying the "string(s)"
to be searched for) but none were found. No records
searched.
System action
The SuperC run continues.
Programmer response
Check that "search string" is being supplied to SuperC
correctly.
Refer:
SRCHFOR, SRCHFORC - Search for Strings in the
Input Files in Appendix A. SuperC Reference in the
User's Guide Vol II
ISRS013W CERTAIN "DO NOT PROCESS"
OPTIONS ARE REJECTED DUE TO
LINE LENGTHS > 256. OPTIONS
RESERVED FOR PROGRAM
SOURCE DATA.
Explanation
"Do not process" options are not allowed if line > 256
characters. These options are primarily for source text.
The DPLINE process statement is allowed in these
cases.
System action
The SuperC run continues.
Programmer response
Either use the DPLINE statement or modify the data
before comparing.
ISRS014W UPDATE OPTION CONFLICTS
WITH "DO NOT PROCESS" OPTION
SELECTION. "DO NOT PROCESS"
OPTIONS IGNORED.
Explanation
The update process option specified is incompatible
with the "Do not process" (DP…) process option(s)
specified.
System action
The SuperC run continues.
Programmer response
Check process options used.
Refer:
SuperC messages
Chapter 5. SuperC messages  897

## Page 918

SuperC Compare Process Options in Appendix A.
SuperC Reference in the User's Guide Vol II
ISRS015W UPDMVS8 AND UPDCMS8
PROCESS OPTIONS ARE ONLY
ALLOWED WITH FIXED 80
RECORDS.
System action
The SuperC run continues. No update file is created.
Programmer response
Check that the appropriate update process option is
being used for the input file.
Refer:
SuperC Compare Process Options in Appendix A.
SuperC Reference in the User's Guide Vol II
ISRS016W MOVE LINE DETECTION
RESTRICTED TO LINES <= 256
LRECL. OPTION IS IGNORED.
Explanation
Process option FMVLNS is restricted to lines <= 256
characters.
System action
The SuperC run continues.
Programmer response
None.
ISRS017W file -name  - SELECT MEMBER WAS
NOT FOUND.
Explanation
The member or file in the SELECT process statement
could not be found.
System action
The SuperC run continues.
Programmer response
Check that the member/file name in the SELECT
process statement is correct. Also, check that the
"group" from which the member/file is to be selected
has been specified correctly.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS018W file -name1:file -name2  SELECT
MEMBER-PAIR WAS NOT FOUND.
Explanation
One or both of the members or files in the SELECT
process statement could not be found.
System action
The SuperC run continues.
Programmer response
Check that both member/file names have been
specified correctly.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS019W YEAR 2000 AGING PARAMETER IS
INVALID
Explanation
Aging parameter in NY2AGE/OY2AGE is not numeric. It
should be a value between 1 and 999.
System action
The SuperC run continues.
Programmer response
Change NY2AGE/OY2AGE aging parameter to a valid
value.
ISRS020W Y2DTONLY OPTION IGNORED AS
THERE ARE NO VALID DATE
DEFINITIONS.
Explanation
A Year 2000 Compare Dates Only (Y2DTONLY) process
option has been specified but no dates have been
defined by Year 2000 Date Definition (NY2C, NY2Z,
NY2D, NY2P, OY2C, OY2Z, OY2D, OY2P) process
statements.
System action
The SuperC run continues.
SuperC messages
898  z/OS: z/OS ISPF Messages and Codes

## Page 919

Programmer response
Use appropriate Year 2000 Date Definition process
statements to define the date(s) to be compared.
Refer:
SuperC Compare Process Options in Appendix A.
SuperC Reference in the User's Guide Vol II
ISRS022W compare-type COMPARE TYPE
AND THIS PROCESS STATEMENT
ARE INCOMPATIBLE. STATEMENT
IGNORED.
Explanation
The compare type specified (FILE, LINE, WORD, or
BYTE) is not valid for the process statement that has
been specified.
System action
The SuperC run continues.
Programmer response
Change compare type to one that is valid for the
process statement involved.
Refer:
SuperC Compare Process Statements in Appendix
A. SuperC Reference in the User's Guide Vol II
ISRS023W UNRECOGNIZED OR INVALID
PROCESS STATEMENT KEYWORD.
Explanation
Keyword not valid for the process statement specified
System action
The SuperC run continues.
Programmer response
Check if the process statement involved requires a
keyword. If so, ensure a valid keyword is used.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS024W EXTRA DATA DETECTED AFTER
NORMAL STATEMENT END.
STATEMENT ACCEPTED WITH
WARNING NOTIFICATION.
Explanation
Extraneous data or incorrect syntax.
System action
The SuperC run continues.
Programmer response
Check format of statement.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Compare Process Statements
SuperC Search-For Process Options
SuperC Search-For Process Statements
ISRS025W INVALID PROCESS STATEMENT
DATA-VALUE/OPERAND, EXTRA
DATA OR EXCEEDS COLUMN 72.
STMT/OPERAND IGNORED.
Explanation
Incorrect syntax for process statement.
System action
The SuperC run continues.
Programmer response
Check required syntax for process statement.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS026W THE CMPBOFS STATEMENT AND
UPDCNTL CONFLICT. STATEMENT
IGNORED.
Explanation
Cannot use a CMPBOFS process statement with
UPDCNTL process option.
System action
The SuperC run continues. CMPBOFS process
statement ignored.
Programmer response
Change process option(s) or process statement(s) as
necessary.
SuperC messages
Chapter 5. SuperC messages  899

## Page 920

ISRS028W statement-type STATEMENT
CONFLICTS WITH SPECIFIED
UPDATE OPTIONS. STATEMENT
IGNORED.
Explanation
The type of statement specified is not compatible with
one or more of the update process options specified.
System action
The SuperC run continues.
Programmer response
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Options
SuperC Compare Process Statements
ISRS029W A SELECT PROCESS STATEMENT
IS INVALID WITH SEQUENTIAL
FILES/DATA SETS. STATEMENT
IGNORED.
Explanation
SELECT process statements can only be used to select
members/files from a "group".
System action
The SuperC run continues.
Programmer response
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS030W THE SELECT STATEMENT HAS
AN INVALID MEMBER NAME OR
IMPROPER OPERAND FORMAT.
STMT/MEMBER IGNORED.
Explanation
Incorrect content or syntax.
System action
The SuperC run continues.
Programmer response
Check that the member/file name(s) have been
entered correctly in the SELECT process statement.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS031W AN INVALID START COLUMN
VALUE WAS SPECIFIED.
Explanation
Missing, nonnumeric, or otherwise invalid "start
column" parameter specified.
System action
The SuperC run continues.
Programmer response
Check that details have been entered correctly and in
accordance with the required syntax.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS032W COLUMN VALUES MUST BE
IN ASCENDING SEQUENCE.
STATEMENT IGNORED.
Explanation
Column values not in ascending sequence or, possibly,
statements out of sequence.
System action
The SuperC run continues.
Programmer response
Check that SuperC receives column numbers/ranges
in ascending sequence such that a record can be
scanned sequentially from "left to right".
ISRS033W CMPCOLM RANGE STARTS WITH
A VALUE EXCEEDING THE
MAXIMUM PROCESSING LENGTH.
STATEMENT TERMINATED.
SuperC messages
900  z/OS: z/OS ISPF Messages and Codes

## Page 921

Explanation
The "start-column" specified in the CMPCOLM process
statement is greater than the logical record length of
the file.
System action
The SuperC run continues.
Programmer response
Correct the column/range specified in the CMPCOLM
process statement.
ISRS034W CMPCOLM STMT(S) HAS TOO
MANY RANGES. ONLY FIRST 15
RANGES WILL BE USED.
Explanation
More than the permitted maximum of 15 ranges/
individual columns specified for the CMPCOLM process
statement. Extraneous information ignored.
System action
The SuperC run continues.
Programmer response
Limit ranges/individual columns to a maximum of 15
for each run of SuperC. Additional ranges/individual
columns can be specified in a separate run.
ISRS035W INVALID CHANGE TEXT
COMBINATION OF NEW TEXT >
OLD TEXT AND LINE LENGTHS >
256 ATTRIBUTE.
Explanation
The length of the search text in a NCHGT or OCHGT
process statement can not be greater than the length
of the change text when a record is greater than 256
characters.
System action
The SuperC run continues.
Programmer response
Correct process statement.
ISRS036W SELECT STATEMENTS VALID ONLY
WITH /PDS/MACLIBS/TXTLIBS OR
* FILE NAMES. STATEMENT
IGNORED.
Explanation
SELECT process statements can only be used to select
members/files from a "group".
System action
The SuperC run continues.
Programmer response
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS037W DPLINEC MUST BE PRECEDED
BY A VALID DPLINE/
DPLINEC STATEMENT. STATEMENT
REJECTED.
Explanation
The DPLINEC process statement is a continuation of
the preceding DPLINE (or DPLINEC) statement and
therefore must always be preceded by one of those
statements.
System action
The SuperC run continues.
Programmer response
Ensure the first "Do not process" statement is
a DPLINE followed, if necessary, by a DPLINEC
statement containing "continuation" information.
See these topics in Appendix A. SuperC Reference in
the User's Guide Vol II:
SuperC Compare Process Statements
SuperC Search-For Process Statements
ISRS038W SRCHFORC MUST BE PRECEDED
BY A VALID SRCHFOR/SRCHFORC
STATEMENT. STATEMENT
REJECTED.
Explanation
The SRCHFORC process statement is a continuation
of the preceding SRCHFOR (or SRCHFORC) statement
and therefore must always be preceded by one of
those statements.
System action
The SuperC run continues.
SuperC messages
Chapter 5. SuperC messages  901
