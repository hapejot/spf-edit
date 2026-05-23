# Chapter 3. Introduction to writing dialogs

Source file: f54dg00_v3r1.md
Start page: 65
Page span: 65-106

## Page 65

Chapter 3. Introduction to writing dialogs
This topic introduces you to how to write dialogs using the ISPF display, variable, table, file-tailoring, PDF,
and other miscellaneous services. For more detailed information on using these services, refer to the z/OS
ISPF Services Guide.
You can use the ISPDPTRC command to trace both the execution of panel service calls (DISPLAY,
TBDISPL, and TBQUERY) and the processing that occurs within the Dialog Manager panel code. For more
information, refer to “Panel trace command (ISPDPTRC)” on page 317.
Using the display services
The display services allow a dialog to display information and interpret responses from users. The display
services are:
ADDPOP
Start pop-up window mode. The ADDPOP service specifies that the listed panel displays are to be in
a pop-up window. It also identifies the location of the pop-up window on the screen in relation to the
underlying panel or window.
DISPLAY
Display a panel. The DISPLAY service reads a panel definition from the panel files, initializes
variable information in the panel from the corresponding dialog variables in the function, shared,
or profile variable pools, and displays the panel on the screen. Optionally, the DISPLAY service might
superimpose a message on the display.
After the user has entered information on the panel, the information is stored in the corresponding
dialog variables in the function, shared, or profile variable pools, and the DISPLAY service returns
control to the calling function.
The COMMAND option on the DISPLAY service allows a dialog to pass a chain of commands to ISPF
for execution. This option is explained fully in the z/OS ISPF Services Guide. Use of the DISPLAY
service is illustrated in a function example later.
LIBDEF
Define optional search libraries. The LIBDEF service allows users to define an optional, application-
level set of libraries containing, for example, messages or panels, to be searched before the IBM-
supplied ISPF libraries. See the z/OS ISPF Services Guide for more information.
REMPOP
Remove a pop-up window. The REMPOP service call removes a pop-up window from the screen.
SELECT
Select a panel or function. The SELECT service is used to display a hierarchy of selection panels or
invoke a function.
SETMSG
Display a message on the next panel. The SETMSG service constructs a specified message from the
message file in an ISPF system save area. The message will be superimposed on the next panel
displayed by any DM service. The optional COND parameter allows you to specify that the message is
to be displayed on the next panel only if there is no SETMSG request pending.
TBDISPL
Display a table. The TBDISPL service combines information from panel definitions with information
stored in ISPF tables. It displays selected rows from a table, and allows the user to identify rows for
processing.
Panel definitions used by the TBDISPL service contain nonscrollable text, including column headings,
followed by one or more "model lines" that specify how each row from the table is to be formatted
in the scrollable area of the display. For more information about TBDISPL, see “Defining table display
panels” on page 112 and the description of the TBDISPL service in z/OS ISPF Services Guide.
Display Services
© Copyright IBM Corp. 1980, 2025 37

## Page 66

Example: creating a display with TBDISPL
The TBDISPL service displays information from an ISPF table on a panel formatted by information on a
panel definition. Table 1 on page 38 shows an ISPF table named TAB1.
Table 1. TBDISPL – ISPF table
RANK ID CITY STATE POPCH ROW
1 FLO621 Fort Myers fl +95.1 r1
2 NV1235 Las Vegas nv +69.0 r2
3 FL1972 Sarasota fl +68.0 r3
4 COO649 Fort Collins co +66.0 r4
5 FL2337 West Palm Beach fl +64.3 r5
6 FLO608 Fort Lauderdale fl +63.6 r6
7 TXO231 Bryan tx +61.5 r7
8 NV1833 Reno nv +60.0 r8
9 UT1656 Provo ut +58.4 r9
10 TX1321 McAllen tx +56.1 r10
Here is a panel definition named PAN1.
************************************************************
* )Attr                                                    *
*   @ Type(output) Intens(low) Just(asis)   Caps(off)      *
* )Body                                                    *
* -------------------- Population Change ----------------- * ---┐
* +Command ==>Cmdfld                    +Scroll ==>_samt+  *    |
* +                                                        *    |
* This table shows selected metropolitan areas which had a *    |---> (See Note 1)
*                                                          *    |
* large relative increase in population from 1970 to 1980. *    |
*                                                          *    |
* +Metro area      State     Change                        *    |
* +                         (Percent)                      * ---┘
* )Model                                                   *
* @City            @State  @popchg+                        * -------> (See Note 2)
*                                                          *
* )Init                                                    *
*   &samt=page                                             *
* )Proc                                                    *
* )End                                                     *
************************************************************
Figure 14. TBDISPL panel definition 
Note:
1. See "A" in Figure 15 on page 39.
2. See "B" in Figure 15 on page 39.
The )BODY section of PAN1 defines the fixed portion of the display, area "A" in Figure 14 on page 38.
The )MODEL section of PAN1 produces the scrollable portion of the display, area "B" in Figure 14 on page
38.
There can be up to eight model lines. Panel PAN1 has only one. The scrollable portion of the display is
formed by replicating the model lines to fill the screen. Each of these replications, as well as the original,
is known as a model set. Each model set corresponds to a table row. Table rows are then read to fill in the
appropriate fields in the model set replications.
Display Services
38  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 67

PAN1 displays only three (city, state, and popchg) of the five columns of table TAB1. The model lines can
include any number of the KEYS, NAMES, and extension variables of the table. They can also include fields
that are not variables in the table. Figure 15 on page 39 shows the effect of displaying information from
TAB1 on panel PAN1.
               +---------------------------------------------------------+
       +-------|------------------- Population Change ------ ROW 4 OF 10 |
       |       | Command ==>                           Scroll ==> Page   |
       |       |                                                         |
  --(A)|       |This table shows selected metropolitan areas which had a |
       |       |large relative increase in population from 1970 to 1980. |
       |       |                                                         |
       |       |  Metro area       State        Change                   |
       --------|                               (Percent)                 |
       +--r4-- |  Fort Collins      co          +66.0                    |
       |  r5-- |  West Palm Beach   fl          +64.3                    |
       |  r6-- |  Fort Lauderdale   fl          +63.6                    |
  --(B)|  r7-- |  Bryan             tx          +61.5                    |
       |  r8-- |  Reno              nv          +60.0                    |
       |  r9-- |  Provo             ut          +58.4                    |
       |  r10- |  McAllen           tx          +56.1                    |
       +-------|********************** BOTTOM OF DATA ****************** |
               |                                                         |
               +---------------------------------------------------------+
Figure 15. TBDISPL display
When the TBDISPL service is invoked with the panel name specified, the scrollable portion begins with
the current row. That is, the current row is the top row displayed. In this example, the current row
pointer (CRP) for table TAB1 has been set to row 4. Table rows are read starting with row 4 to fill in the
appropriate fields in the model set replications. If there were any non-table variables in the model line,
they would be filled in with their current values. Because there aren't enough rows in the table to fill the
screen, the bottom-of-data marker is placed in the display after the last row. The "empty" model sets
beyond this marker are not displayed.
In Table 1 on page 38, the symbols r1 through r10 label the 10 rows in the table TAB1. The highlighted
rows, r4 through r10, indicate that these rows provide the information for the scrollable portion of the
display (marked as area B in Figure 15 on page 39).
Figure 15 on page 39 is the result of using the TBDISPL service with panel definition PAN1 (Figure 14 on
page 38) and ISPF table TAB1 (Table 1 on page 38). Portion A is the fixed portion defined by the )BODY
section of PAN1. Portion B is the scrollable portion defined by the )MODEL section of PAN1. The table
information in the display is the specified columns from row 4 to row 10.
Processing selected rows
When a user changes data in a model set, the corresponding table row is said to be selected for
processing. More than one row can be selected in a single interaction. Before the TBDISPL service returns
control to the dialog function, the CRP is positioned to the first of the selected rows. First means the row
closest to the top of the table, not the row that was selected first. The other selected rows are called
pending selected rows.
Note: System command ZCLRSFLD causes a row to be selected if it is used on a scrollable input field.
When the CRP is positioned at a selected row, the row is retrieved, meaning the values from that row are
stored in the appropriate dialog variables. Then, all input fields in the selected model set on the display
are stored in the corresponding dialog variables. The dialog function can then process the row in any
manner it chooses. For example, the function can invoke the TBPUT service to update the row, or it can
invoke the BROWSE service to examine a file specified in that row.
A call of the TBDISPL service is required to position the CRP to each pending selected row. For these calls,
neither the PANEL nor MSG parameter should be specified.
Display Services
Chapter 3. Introduction to writing dialogs  39

## Page 68

The system variable ZTDSELS contains the number of selected rows. It can be tested by the dialog
function or in the )PROC section of the table display panel to determine if any rows were selected. For
example:
)PROC
  . . .                    /* Process fixed portion fields   */
  IF (&ZTDSELS ¬= 0000)    /* Any selected rows?             */
    . . .                  /* Process scrollable portion flds*/
)END
The interpretation of this variable is as follows:
0000
No selected rows
0001
One selected row (now the current row)
0002
Two selected rows, consisting of the current row and a pending selected row
0003
Three selected rows, consisting of the current row and two pending selected rows
⋮
n
"n" selected rows, consisting of the current row and "n-1" pending selected rows.
As TBDISPL is reinvoked without the PANEL and MSG parameters (to process any pending selected rows),
ZTDSELS is decremented by one. An example is shown in Table 2 on page 40.
Table 2. ZTDSELS decrementation
DM Service User Action Value of ZTDSELS
TBDISPL TAB1
PANEL(PAN1)
Selects 3 rows 0003 (current row plus two pending selected rows)
TBDISPL TAB1 None 0002 (current row plus one pending selected row)
TBDISPL TAB1 None 0001 (current row; no pending selected rows)
Adding table rows dynamically during table display scrolling
Assume that you have access to a large amount of related data that might be built into a single table.
However, you need to interface with only a subset of that data during an ISPF session, but you are not
sure just how extensive that subset is. Normally, you would have to initially construct a table that included
all possible data that you might wish to access during a session before you began scrolling and update
activity on the table. This could lead to a great deal of unnecessary overhead because you might include a
lot of data in your table that you never access.
By interacting with a set of function system variables, an ISPF function can dynamically expand the table
as you scroll through it during a session. The function can specify that the table is to be expanded upward
when the user has scrolled past the top, expanded downward when the user has scrolled past the bottom,
or both. In this way, the function adds only the table rows that satisfy your needs as you need them.
System variables are the ISPF-function interface
Eight system variables in the function pool are the vehicle for passing, between ISPF and the function,
values that control table expansion. These variables and the functions they perform are:
ZTDRET (input; length 8)
The function sets variable ZTDRET in the function pool to a value (UP, DOWN, or VERTICAL) that
indicates to ISPF when control is to return to the function so that more rows can be added to the table
being processed.
Display Services
40  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 69

ZTDADD (output; length 3)
ISPF sets this variable to either YES or NO before returning control to the function. A value of YES
indicates that the function needs to add more rows to the table being processed. ZTDADD is normally
set to NO, indicating that no more rows need to be added to the table.
ZTDSCRP (input/output; length 6)
This variable is set to the row pointer (number of the row relative to the top of the table) of the row
that is to be at the top of the panel's scrollable area after the scroll request is processed. If ISPF
cannot determine this value, this variable is set to zero.
ZTDSRID (output; length 6)
ISPF sets this variable to the row ID of the row pointed to by the value in variable ZTDSCRP. During
table processing, the row pointer value for a given row can change. However, the row ID of that row
does not change.
ZTDAMT (output; length 4)
When ISPF returns control to the function with the value of variable ZTDADD set to YES, the value
that ISPF has set in variable ZTDAMT tells the function how many rows, based on the information
available, ISPF calculates should be added to the table to satisfy the current scroll request. If the
number of rows is calculated to be greater than 9999 then ZTDAMT is set to 9999. ZTDAMTL always
holds the number of rows.
ZTDAMTL (output; length 8)
When ISPF returns control to the function with the value of variable ZTDADD set to YES, the value
that ISPF has set in variable ZTDAMTL tells the function how many rows, based on the information
available, ISPF calculates should be added to the table to satisfy the current scroll request. If the
value is less than 10000, then ZTDAMT also holds the number of rows. ZTDAMTL always holds the
correct number of rows.
ZTDSIZE (output; length 4)
ISPF sets the value of ZTDSIZE to the total number of model sets; that is, the number of table rows
that fill the scrollable area of the panel. This is not necessarily the same as the number of lines
displayed in the panel's scrollable area.
ZTDLTOP (input; length 6)
The function can optionally set this variable to a value for ISPF to use in calculating the value x
(top-row-displayed) in the indicator 'ROW x OF y', which ISPF displays on a TBDISPL screen.
ZTDLROWS (input; length 6)
The function can optionally set this variable to a value for ISPF to use as the value y (total rows in the
logical table) in the indicator 'ROW x OF y'.
You can define variables ZTDAMT, ZTDSCRP, ZTDSRID, ZTDSIZE, ZTDLTOP, and ZTDLROWS as fullword
fixed binary in a program function. If you do not, the default for each of these variables is character with
lengths as specified in the system variable charts in the Appendix E, “System variables,” on page 359.
Dynamic table building
To put the dynamic table building concept into practice, a function first builds a basic table structure.
The initial size of this table is determined by balancing the minimum amount of table data that would
satisfy most anticipated user needs against the overhead of including a large amount of table data to
cover more contingencies. As more table rows are needed to satisfy scroll requests, ISPF returns control
to the function so that it can add those rows.
When a user issues a scroll request, there might be input fields in a panel that have been typed into
(selected for processing). In that case, the dialog first processes all selected rows and then issues a
TBDISPL request, without panel name, to cause the panel to redisplay. If no table rows are needed to fill
the scroll request, ISPF completes the scroll and redisplays the panel. If more table rows are needed to
fill the scroll request, ISPF returns control to the function to add table rows. Keep in mind that each time
control returns to the function, the )PROC section of the panel from which the table display was requested
is executed. After adding the table rows, the function issues a TBDISPL without a panel name to complete
the scroll and redisplay. Remember, specifying a panel name on a TBDISPL request nullifies any pending
selected rows or request for scrolling.
Display Services
Chapter 3. Introduction to writing dialogs  41

## Page 70

The values of a set of system variables in the function pool are the parameters used in the interchange
between ISPF and a function when dynamically increasing the table size.
Using variable ZTDRET
The need for expanding a table occurs when a user scrolls beyond the top or bottom of the table while
using the TBDISPL service. The function must set variable ZTDRET to a value that tells ISPF when to
return control so the function can expand the table. The function sets ZTDRET to one of three possible
values:
UP
Control returns to the function when the top of the scrollable data is reached. This applies when you
are building the table upward from the bottom. The value UP has no effect when the bottom of the
scrollable data is reached.
DOWN
Control returns to the function when the bottom of scrollable data is reached. This applies when you
are building the table downward from the top. The value DOWN has no effect when the top of the
scrollable data is reached.
VERTICAL
Control returns to the function when the top or bottom of the scrollable data is reached.
The value in ZTDRET must be left-justified (no leading blanks). ISPF evaluates the value of ZTDRET only
when the function issues a TBDISPL request with a panel name specified . This is true, even though in
the interim, the function might change the value of ZTDRET and issue TBDISPL requests without a panel
name specified. A TBDISPL request with a panel name specified also nullifies processing of any pending
selected table rows and any pending scroll request.
When a scroll request is pending, a TBDISPL request with a message ID specified (but without a panel
name specified) causes the panel to be redisplayed with the message, but the scroll request is nullified.
Using variable ZTDADD
Before returning control to a function from a TBDISPL request, ISPF sets function variable ZTDADD to
YES or NO, indicating to the function whether rows are to be added to the table. The function normally
receives a return code of 0 from the TBDISPL service. It can then interrogate variable ZTDADD. If its value
is 'YES', then ZTDSCRP, ZTDSRID, ZTDAMT, and ZTDSIZE contain valid values.
ISPF normally returns control to the function for reasons other than to add table rows. In those cases,
ISPF sets the value of ZTDADD to NO. For example, the function might need to interact with table rows
that have been selected for processing during a table display.
Using variables ZTDAMT and ZTDAMTL
When ISPF returns control to a function with variable ZTDADD set to YES, the function must add rows to
the table. If rows must be added to the table to satisfy a scroll request, ISPF calculates, when possible,
the number of rows that need to be added to the table and returns that value to the function in variables
ZTDAMT and ZTDAMTL.
ZTDAMT is limited to values up to four digits. If the value is larger than four digits ZTDAMT is 9999. The
value is always returned in ZTDAMTL (an 8 digit value).
The function should use this value for determining the number of rows to add.
For some scroll requests, such as UP MAX or DOWN MAX, ISPF cannot determine the number of rows to
be added to the table. In those cases, ISPF returns a value of 0 to the function in ZTDAMT and ZTDAMTL.
Using variables ZTDSCRP and ZTDSRID
When ZTDSCRP contains a value other than 0, that value is the number of the table row that is to be at the
top of the panel's scrollable area when the panel is redisplayed. ISPF sets ZTDSCRP to a nonzero value
Display Services
42  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 71

if a user has requested a downward scroll such that, when ISPF redisplays the panel following the scroll,
the top row displayed in the scrollable area existed in the table at the time of the scroll request.
When the user requests an UP MAX or DOWN MAX, ISPF does not require the ZTDSCRP value to position
the table when it is redisplayed following the scroll. It simply positions the table in the scrollable display
area relative to the top table row (UP MAX) or the bottom-of-data marker (DOWN MAX).
For other scroll requests that require that rows be added to the table, ISPF may not be able to determine
what the value of ZTDSCRP should be. In other words, one of the table rows to be added by the function
will be the new top row displayed. ISPF has no way of knowing what the number of that row will be. In
those cases, ISPF returns a value of 0 to the function.
If a function receives a value of 0 in ZTDSCRP (other than for UP MAX or DOWN MAX), it must set the
variable's value to the number of the new table row that should display at the top of the panel's scrollable
area. When the function sets the value of ZTDSCRP, the developer must take into account that the number
specified is the number of the top displayed table row relative to the top of the table as the user who
issued the scroll requests will see it. The developer must also take into account any processing that takes
place from the time the user requests a scroll to the time the scroll is processed. For example, assume
that variable ZTDRET is set to UP. A user issues:
UP 10
but there are only eight table rows above the top one currently displayed. ISPF returns control to the
function with variable ZTDAMT having a value of 2, indicating that two lines must be added to the table
to satisfy the current scroll request. ISPF has set variable ZTDSCRP to 0 because the new top displayed
row did not exist in the table when the scroll was requested. Assume that, instead of adding only the two
required table rows at the top of the table to satisfy this scroll request, the function adds 20 rows as a
cushion against additional scrolling. Therefore, the function must set ZTDSCRP to 19 so that ISPF will
redisplay the panel with the table positioned as the user wants it.
In addition to the row pointer in variable ZTDSCRP, ISPF returns to the function in variable ZTDSRID the
identification (rowid) of the row that is to be displayed at the top of the scrollable area. As just described
for ZTDSCRP, if ISPF cannot determine which is to be the top row displayed, it returns a value of 0 in
ZTDSRID.
Using variable ZTDSIZE
When ISPF returns control to the function to add more rows to a table, variable ZTDSIZE contains the
total number of table rows that can fit into the entire panel scrollable area. Changes made to the panel
structure, such as by PFSHOW ON or split-screen mode, do not affect this value. The value is the total
number of scrollable area rows.
Using variables ZTDLTOP and ZTDLROWS
ISPF displays in the upper-right corner of a TBDISPL panel a default top-row-displayed indicator, 'ROW
x OF y', where x is the current row pointer of the top row displayed, and y is the total number of rows
in the physical table being displayed. By assigning a message ID to system variable ZTDMSG, a function
can specify a message whose short message text is to replace the top-row-displayed indicator. However,
keep in mind that in the text shown, all references to the top-row-displayed indicator refer to the default
supplied by ISPF, not an alternate indicator specified by the application.
Because the dimensions of only the physical table are available, ISPF has no way of assuring what the x
and y values for the top-row-displayed indicator should be. Therefore, it is the application's responsibility
to pass to ISPF the logical table positioning in variables ZTDLTOP and ZTDLROWS, respectively, any time
control returns to the function to add table rows. If the function does not set these variables to a value,
ISPF calculates the x and y values according to the size and position of the table being displayed.
For example, assume that, to satisfy scroll requests, an application adds records dynamically to a table
from a 1000-record file. The application initially builds the table with records 500 through 520. To pass
these values to ISPF for use as the x and y values in the top-row-displayed indicator, the application
function sets ZTDLTOP to 500 and sets ZTDLROWS to 1000. This causes the indicator text 'ROW 500 OF
Display Services
Chapter 3. Introduction to writing dialogs  43

## Page 72

1000' to be displayed initially on the TBDISPL panel. Then assume that the user scrolls down 10 rows.
ISPF, using the value in ZTDLTOP plus the 10 rows scrolled, changes the indicator to 'ROW 510 OF 1000'.
In the example just described, assume that the user first scrolled up 10 rows instead of down 10 rows.
Because the top row displayed was the top table row, control returns to the application function to add
rows to the top of the table so the scroll request can be completed. As mentioned, it is the application's
responsibility to change the values of ZTDLTOP and ZTDLROWS as needed to provide ISPF an accurate
base for generating the top-row-displayed indicator. Therefore, after adding rows to the top of the table,
the function sets variable ZTDLTOP to 490 before issuing the TBDISPL request to redisplay the table. The
text of the top-row-displayed indicator on the displayed panel is now 'ROW 490 OF 1000'.
Example: dynamic table expansion
This example illustrates how you can use dynamic expansion to reduce the initial overhead of creating a
large table for display.
Assume that you are given the task of creating an ISPF dialog that allows a user to browse through a list of
invoices for a given year. The list is maintained in a sequential file. It contains information (such as invoice
number, transaction date, part number, quantity, and customer name) for each transaction made during
the year.
The file is fixed-block with a logical record length of 80 and a block size of 6160. The first record in the file
contains the year and the number of invoices that follow in the file.
The format of this record is as follows:
Positions
Format
1-4
Year
5-10
Number of invoices
11-80
Reserved
The format of each of the invoice records is as follows:
Positions
Format
1-6
Invoice number
7-14
Transaction date (format mm/dd/yy)
15-18
Part number
19-21
Quantity (right justified)
22-46
Customer name (left justified)
47-80
Reserved
For example, the file might look something like this:
1986010000
00000101/06/867071100Acme Auto
00000201/06/860015 15Parts City
00000301/07/861023340XYZ Auto Center
00000401/08/860231  1Parts Unlimited
00000501/08/863423805Bosworth's Parts
00000601/08/862341165Acme Parts
Display Services
44  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 73

00000701/08/867653 20Acme Parts
00000801/08/863353100Bosworth's Parts
00000901/08/860003325Bosworth's Parts
00001001/08/863322  1Bosworth's Parts
⋮
00999912/15/860325 43ABC Parts
01000012/18/864234340ACME Parts
As you can see, the file is in no form to be browsed as it is. One way to implement the dialog is to transfer
the invoice file to a temporary ISPF table, and then display the table with the TBDISPL service. However,
since the number of invoices can be relatively high (in this example, there are 10 000 invoices), the initial
overhead of reading every record and adding it to the table is unacceptable. As an alternative, the dialog
uses dynamic table expansion instead. Using this method, it adds only the first 60 invoices to the table
initially. Other invoices are added on an as-needed basis as the user scrolls through the table. The user
sees no evidence that only a portion of the invoices are in the table.
Figure 16 on page 45 shows the definition for panel INVPANEL, which the dialog uses to display table
rows.
)Attr
 @ Type(Output) Intens(Low)
)Body Expand(//)
+-/-/-%&year TRANSACTIONS+-/-/-
%Command ====>_cmd                                   %Scroll ===>_amt +
+
+
%Invoice    Transaction    Part
%Number        Date        Number    Quantity    Customer
%-------    -----------    ------    --------    --------
)Model
@inv        @date         @part       @qty      @cust                +
)Init
 &amt = PAGE
)End
Figure 16. Panel definition  dynamic table expansion
The PL/I dialog function INVOICE requires that the invoice file be allocated to ddname INVFILE before the
dialog is executed. The intent of this example is to illustrate the dynamic expansion function. Normal error
checking and error processing is not shown, but should be included in all dialogs.
 INVOICE: PROC OPTIONS(MAIN);
  /*******************************************************************/
  /*  THIS PROGRAM ILLUSTRATES THE USE OF DYNAMIC EXPANSION WITH     */
  /*  THE TABLE DISPLAY SERVICE.  THE PROGRAM READS RECORDS FROM A   */
  /*  SEQUENTIAL FILE CONTAINING A LIST OF INVOICES AND ADDS THE     */
  /*  INVOICE INFORMATION TO A TEMPORARY ISPF TABLE (INVTABLE).      */
  /*  THE TABLE IS THEN DISPLAYED SO THAT THE USER CAN BROWSE        */
  /*  THROUGH THE INVOICES.  THE FOLLOWING STEPS ARE PERFORMED BY    */
  /*  THE PROGRAM:                                                   */
  /*                                                                 */
  /*    1. DEFINE THE FUNCTION POOL VARIABLES FOR THE TEMPORARY      */
  /*       TABLE, THE TBDISPL SYSTEM VARIABLES, AND MISCELLANEOUS    */
  /*       VARIABLES.                                                */
  /*                                                                 */
  /*    2. ISSUE A TBCREATE SERVICE CALL FOR TEMPORARY TABLE,        */
  /*       INVTABLE.                                                 */
  /*                                                                 */
  /*    3. OPEN FILE INVFILE AND READ THE HEADER RECORD INTO THE     */
  /*       HEADER_RECORD STRUCTURE.                                  */
  /*                                                                 */
  /*    4. READ EACH OF THE FIRST 60 INVOICE RECORDS FROM INVFILE    */
  /*       INTO THE INVOICE_RECORD STRUCTURE AND ADD THEM TO TABLE   */
  /*       INVTABLE.  USE THE TBADD MULT PARAMETER TO OPTIMIZE       */
  /*       TBADD ROW STORAGE MANAGEMENT.                             */
  /*    5. ISSUE A TBTOP SERVICE CALL TO POSITION THE CRP AT THE     */
  /*       TOP OF INVTABLE.                                          */
  /*    6. INITIALIZE SYSTEM VARIABLE ZTDRET TO "DOWN"               */
  /*       AND SYSTEM VARIABLE ZTDLROWS TO THE NUMBER OF INVOICES    */
  /*       IN THE FILE.                                              */
  /*    7. ISSUE A TBDISPL SERVICE CALL THAT REFERS TO TABLE         */
  /*       INVTABLE AND PANEL INVPANEL.                              */
  /*    8. LOOP WHILE THE TBDISPL SERVICE RETURN CODE IS LESS THAN   */
  /*       8 (WHILE THE USER HAS NOT ISSUED THE END COMMAND AND      */
  /*       WHILE THERE HAVE BEEN NO SEVERE ERRORS).  ON RETURN       */
Display Services
Chapter 3. Introduction to writing dialogs  45

## Page 74

/*       FROM THE TBDISPL SERVICE, DO THE FOLLOWING:               */
  /*                                                                 */
  /*        - CHECK TO SEE IF ADDITIONAL ROWS ARE NEEDED TO          */
  /*          SATISFY A SCROLL REQUEST.                              */
  /*        - IF ADDITIONAL ROWS ARE NEEDED, READ THE APPROPRIATE    */
  /*          NUMBER OF INVOICES FROM INVFILE AND ADD THEM TO        */
  /*          INVTABLE AGAIN USING THE TBADD MULT PARAMETER.         */
  /*        - IF NECESSARY, SET THE SYSTEM VARIABLE ZTDSCRP TO       */
  /*          THE CRP OF THE NEW TOP ROW.                            */
  /*        - FINALLY, ISSUE A TBDISPL SERVICE CALL (WITHOUT A       */
  /*          PANEL NAME) TO REDISPLAY INVTABLE.                     */
  /*    9. PERFORM SOME FINAL CLEANUP BEFORE EXITING THE DIALOG:     */
  /*                                                                 */
  /*        - ISSUE A TBEND SERVICE CALL TO CLOSE AND DELETE         */
  /*          INVTABLE.                                              */
  /*        - CLOSE INVFILE.                                         */
  /*        - ISSUE A VDELETE SERVICE CALL TO DELETE ALL FUNCTION    */
  /*          POOL VARIABLES CREATED BY THE DIALOG.                  */
  /*******************************************************************/
  DECLARE                                   /*                       */
    1 HEADER_RECORD,                        /* HEADER RECORD FIELDS  */
      3 YEAR      CHAR(4),                  /*   YEAR OF INVOICES    */
      3 NUM_RECS  CHAR(6),                  /*   NUMBER OF INVOICES  */
      3 FILLER    CHAR(70);                 /*   ** RESERVED **      */
                                            /*                       */
  DECLARE                                   /*                       */
    1 INVOICE_RECORD,                       /* INVOICE RECORD FIELDS */
      3 INV       CHAR(6),                  /*   INVOICE NUMBER      */
      3 DATE      CHAR(8),                  /*   TRANSACTION DATE    */
      3 PART      CHAR(4),                  /*   PART NUMBER         */
      3 QTY       CHAR(3),                  /*   QUANTITY            */
      3 CUST      CHAR(25),                 /*   CUSTOMER NAME       */
      3 FILLER    CHAR(34),                 /*   ** RESERVED **      */
    INVOICE_FORMAT  (5) CHAR(8)             /* FORMAT ARRAY FOR      */
                  INIT((5) (1)'CHAR    '),  /*  INVOICE_RECORD VDEF  */
    INVOICE_LENGTH   (5) FIXED BIN(31,0)    /* LENGTH ARRAY FOR      */
                  INIT(6,8,4,3,25);         /*  INVOICE_RECORD VDEF  */
  DECLARE                                   /*                       */
    1 SCROLL_VARS,                          /* TBDISPL SCROLL FIELDS */
      3 ZSCROLLA  CHAR(4),                  /*   SCROLL AMOUNT       */
      3 ZTDRET    CHAR(8),                  /*   RETURN ON EOD       */
      3 ZTDSCRP   FIXED BIN(31,0),          /*   TOP ROW CRP         */
      3 ZTDAMT    FIXED BIN(31,0),          /*   #ROWS TO ADD        */
      3 ZTDSIZE   FIXED BIN(31,0),          /*   SCROLLABLE AREA SIZE*/
      3 ZTDLROWS  FIXED BIN(31,0),          /*   #ROWS IN LOGICAL TBL*/
      3 ZTDADD    CHAR(3),                  /*   NEED TO ADD ROWS?   */
    SCROLL_FORMAT (7) CHAR(8)               /* FORMAT ARRAY FOR      */
                  INIT((2) (1)'CHAR    ',   /*  SCROLL_VARS VDEFINE  */
                       (4) (1)'FIXED   ',   /*                       */
                       'CHAR    '),         /*                       */
    SCROLL_LENGTH (7) FIXED BIN(31,0)       /* LENGTH ARRAY FOR      */
                  INIT(4,8,4,4,4,4,3);      /*  SCROLL_VARS VDEFINE  */
                                            /*                       */
  DECLARE                                   /*                       */
    I             FIXED BIN(31,0),          /* WORK INDEX            */
    L4            FIXED BIN(31,0),          /* VDEFINE LENGTH PARM   */
    TBDISPL_RC    FIXED BIN(31,0),          /* TBDISPL RETURN CODE   */
    BOTTOM        FIXED BIN(31,0),          /* CRP OF BOTTOM ROW     */
    NEW_BOTTOM    FIXED BIN(31,0),          /* CRP OF NEW BOTTOM ROW */
    REQUESTED_TOP FIXED BIN(31,0),          /* TOP ROW REQUESTED BY  */
                                            /*  END USER SCROLL      */
    ADD_NUMBER    FIXED BIN(31,0);          /* #ROWS TO ADD          */
                                            /*                       */
  DECLARE                                   /*                       */
    MIN     BUILTIN,                        /* PL/I BUILTIN          */
    PLIRETV BUILTIN,                        /*  FUNCTIONS            */
    ISPLINK EXTERNAL ENTRY                  /* ISPF SERVICE          */
            OPTIONS(ASM INTER RETCODE);     /*  INTERFACE            */
                                            /*                       */
  DECLARE                                   /*                       */
    INVFILE  FILE INPUT RECORD SEQUENTIAL   /* INVOICE FILE          */
       ENV(FB BLKSIZE(6160) RECSIZE(80));   /*                       */
                                            /*                       */
  /*******************************************************************/
  /*                                                                 */
  /*  ISSUE VDEFINE SERVICE CALLS TO DEFINE THE TABLE VARIABLES,     */
  /*  SCROLL SYSTEM VARIABLES, AND OTHER MISCELLANEOUS FIELDS TO     */
  /*  ISPF.                                                          */
  /*                                                                 */
  /*******************************************************************/
                                            /*                       */
  CALL ISPLINK('VDEFINE ',                  /* DEFINE TABLE VARS     */
Display Services
46  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 75

'(INV DATE PART QTY CUST)',  /*                       */
               INVOICE_RECORD,              /*                       */
               INVOICE_FORMAT,              /*                       */
               INVOICE_LENGTH,              /*                       */
               'LIST    ');                 /*                       */
                                            /*                       */
  CALL ISPLINK('VDEFINE ',                  /* DEFINE SCROLL VARS    */
            '(ZSCROLLA ZTDRET ZTDSCRP ZTDAMT ZTDSIZE ZTDLROWS ZTDADD)',
               SCROLL_VARS,                 /*                       */
               SCROLL_FORMAT,               /*                       */
               SCROLL_LENGTH,               /*                       */
               'LIST    ');                 /*                       */
  L4 = 4;                                   /*                       */
  CALL ISPLINK('VDEFINE ',                  /* DEFINE BOTTOM ROW CRP */
               '(BOTTOM)',                  /*                       */
               BOTTOM,                      /*                       */
               'FIXED   ',                  /*                       */
               L4);                         /*                       */
                                            /*                       */
  CALL ISPLINK('VDEFINE ',                  /* DEFINE PANEL VAR YEAR */
               '(YEAR)',                    /*                       */
               YEAR,                        /*                       */
               'CHAR    ',                  /*                       */
               L4);                         /*                       */
                                            /*                       */
  /*******************************************************************/
  /*                                                                 */
  /*  ISSUE TBCREATE SERVICE CALL TO CREATE TEMPORARY TABLE          */
  /*  INVTABLE.  MAKE EACH OF THE TABLE VARIABLES NAME VARIABLES.    */
  /*                                                                 */
  /*******************************************************************/
                                            /*                       */
  CALL ISPLINK('TBCREATE',                  /*                       */
               'INVTABLE',                  /*                       */
               ' ',                         /*                       */
               '(INV DATE PART QTY CUST)'); /*                       */
                                            /*                       */
  /*******************************************************************/
  /*                                                                 */
  /*  OPEN FILE INVFILE AND READ THE HEADER RECORD.                  */
  /*                                                                 */
  /*******************************************************************/
                                            /*                       */
  OPEN FILE(INVFILE);                       /* OPEN INVOICE FILE     */
  READ FILE(INVFILE)                        /* READ HEADER RECORD    */
    INTO(HEADER_RECORD);                    /*                       */
                                            /*                       */
  /*******************************************************************/
  /*                                                                 */
  /*  READ THE FIRST 60 RECORDS FROM INVFILE, ADDING EACH TO THE     */
  /*  TABLE.                                                         */
  /*                                                                 */
  /*******************************************************************/
                                            /*                       */
  ADD_NUMBER = 60;                          /*                       */
  DO I = 1 TO ADD_NUMBER;                   /*                       */
    READ FILE(INVFILE)                      /* READ NEXT RECORD      */
      INTO(INVOICE_RECORD);                 /*                       */
    CALL ISPLINK('TBADD   ',                /* ADD INVOICE TO TABLE  */
                 'INVTABLE',                /*                       */
                 ' ',                       /*                       */
                 ' ',                       /*                       */
                 ADD_NUMBER);               /*                       */
  END;                                      /*                       */
                                            /*                       */
  /*******************************************************************/
  /*                                                                 */
  /*  SKIP BACK TO THE TABLE TOP, INITIALIZE THE ZTDRET AND          */
  /*  ZTDLROWS SYSTEM VARIABLES, AND ISSUE A TBDISPL SERVICE CALL    */
  /*  TO DISPLAY THE TABLE.                                          */
  /*                                                                 */
  /*******************************************************************/
                                            /*                       */
  CALL ISPLINK('TBTOP   ',                  /* SKIP TO TABLE TOP     */
               'INVTABLE');                 /*                       */
  ZTDRET = 'DOWN    ';                      /* RETURN ON BOTTOM OF   */
                                            /*  DATA                 */
  ZTDLROWS = NUM_RECS;                      /* SET LOGICAL #ROWS     */
  CALL ISPLINK('TBDISPL ',                  /* PUT UP TABLE          */
               'INVTABLE',                  /*                       */
               'INVPANEL');                 /*                       */
  TBDISPL_RC = PLIRETV();                   /*                       */
Display Services
Chapter 3. Introduction to writing dialogs  47

## Page 76

/*                       */
  /*******************************************************************/
  /*                                                                 */
  /*  LOOP WHILE USER HAS NOT ISSUED THE END COMMAND, CHECK TO       */
  /*  SEE IF ADDITIONAL ROWS ARE NEEDED TO SATISFY SCROLL, ADD ROWS  */
  /*  IF APPROPRIATE, AND THEN REDISPLAY TABLE.                      */
  /*                                                                 */
  /*******************************************************************/
                                            /*                       */
  DO WHILE(TBDISPL_RC < 8);                 /* LOOP WHILE NOT END    */
    IF ZTDADD = 'YES' THEN                  /* NEED TO ADD ROWS?     */
      DO;                                   /*                       */
                                            /*                       */
        CALL ISPLINK('VGET    ',            /*  CHECK TO SEE IF MAX  */
                     '(ZSCROLLA)',          /*   SCROLL              */
                     'SHARED  ');           /*                       */
        IF ZSCROLLA = 'MAX' THEN            /*  IF SO, ADD ALL       */
          ZTDAMT = 999999;                  /*    REMAINING INVOICES */
        ELSE;                               /*  ELSE, ADD ZTDAMT ROWS*/
                                            /*                       */
        CALL ISPLINK('TBBOTTOM',            /*  SKIP TO TABLE BOTTOM */
                     'INVTABLE',            /*  TO ADD ROWS          */
                     ' ',                   /*                       */
                     ' ',                   /*                       */
                     ' ',                   /*                       */
                     'BOTTOM  ');           /*  SAVE CRP OF BOTTOM   */
                                            /*                       */
        ADD_NUMBER = MIN(ZTDAMT,            /*  ADD ZTDAMT ROWS OR   */
                         ZTDLROWS-BOTTOM);  /*  UNTIL INVFILE EOF    */
        DO I = 1 TO ADD_NUMBER;             /*                       */
                                            /*                       */
          READ FILE(INVFILE)                /*  READ RECORD          */
            INTO(INVOICE_RECORD);           /*                       */
                                            /*                       */
          CALL ISPLINK('TBADD   ',          /*  ADD IT TO TABLE      */
                       'INVTABLE',          /*                       */
                       ' ',                 /*                       */
                       ' ',                 /*                       */
                       ADD_NUMBER);         /*                       */
        END;                                /*                       */
        IF ZSCROLLA ¬= 'MAX' THEN           /*  IF NOT MAX SCROLL,   */
          IF ZTDSCRP = 0 THEN               /*   MAY NEED TO SET     */
            DO;                             /*   ZTDSCRP             */
                                            /*                       */
              NEW_BOTTOM = BOTTOM +         /*  CALCULATE NEW BOTTOM */
                ADD_NUMBER;                 /*                       */
              REQUESTED_TOP = BOTTOM +      /*  CALCULATE TOP ROW    */
                ZTDAMT - ZTDSIZE + 1;       /*   REQUESTED BY SCROLL */
                                            /*                       */
              IF NEW_BOTTOM <               /*  IF REACH EOF BEFORE  */
                 REQUESTED_TOP THEN         /*    REACHING TOP ROW   */
                                            /*    REQUESTED, DISPLAY */
                ZTDSCRP = NEW_BOTTOM + 1;   /*    ONLY BOTTOM OF     */
                                            /*    DATA MARKER        */
              ELSE                          /*  ELSE                 */
                ZTDSCRP = REQUESTED_TOP;    /*    ADDED REQUESTED    */
                                            /*    TOP, SET ZTDSCRP   */
                                            /*    TO NEW TOP ROW     */
            END;                            /*                       */
          ELSE;                             /* NO NEED TO SET        */
        ELSE;                               /*  ZTDSCRP              */
                                            /*                       */
      END;                                  /*                       */
    ELSE;                                   /* DON'T NEED TO ADD ROWS*/
                                            /*                       */
    CALL ISPLINK('TBDISPL ',                /* REDISPLAY TABLE       */
                 'INVTABLE');               /*                       */
    TBDISPL_RC = PLIRETV();                 /*                       */
  END;                                      /*                       */
                                            /*                       */
  /*******************************************************************/
  /*                                                                 */
  /*  PERFORM FINAL CLEANUP.                                         */
  /*                                                                 */
  /*******************************************************************/
                                            /*                       */
  CALL ISPLINK('TBEND   ',                  /* CLOSE AND DELETE      */
               'INVTABLE');                 /*  TABLE                */
  CLOSE FILE(INVFILE);                      /* CLOSE INVOICE FILE    */
  CALL ISPLINK('VDELETE ',                  /* DELETE FUNCTION POOL  */
               '*       ');                 /*  VARIABLES            */
                                            /*                       */
Display Services
48  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 77

RETURN (0);                               /*                       */
  END INVOICE;                              /*                       */
Now, assume that a user is running the invoice dialog on a terminal with 24 lines. The initial display of the
table is shown in Figure 17 on page 49.
 ------------------------------ 1986 TRANSACTIONS ------ ROW 1 OF 10000
 Command ====>                                        Scroll ===> PAGE
 Invoice    Transaction    Part      Quantity   Customer
  Number        Date        Number
 -------    -----------    ------    --------   --------
 0000001     01/06/86      7071        100       Acme Parts
 0000002     01/06/86      0015         15       Parts City
 0000003     01/07/86      1023        340       XYZ Auto Center
 0000004     01/08/86      0231          1       Parts Unlimited
 0000005     01/08/86      3423        805       Bosworth's Parts
 0000006     01/08/86      2341        165       Acme Parts
 0000007     01/08/86      7653         20       Acme Parts
 0000008     01/08/86      3353        100       Bosworth's Parts
 0000009     01/08/86      0003        325       Bosworth's Parts
 0000010     01/08/86      3322          1       Bosworth's Parts
 0000011     01/10/86      2344         23       Parts Unlimited
 0000012     01/10/86      4333         55       XYZ Auto Center
 0000013     01/10/86      3079         65       Parts Company of NC
 0000014     01/10/86      4763        340       XYZ Auto Center
 0000015     01/10/86      0956         70       XYZ Auto Center
 0000016     01/10/86      4536         52       ABC Parts
 0000017     01/10/86      0973        330       ABC Parts
Figure 17. Initial display for dynamic table expansion example
Notice that even though the table actually contains only 60 rows, the top row displayed indicator shows
"ROW 1 OF 10000". This was accomplished by setting the ZTDLROWS variable in the function pool to a
value of 10 000. TBDISPL will pick up this value and use it when ZTDRET has been properly set.
Assume that the user enters the command "DOWN 50" on the command line. This should result in rows
51-67 being displayed. Remember though that only rows 1-60 are currently in the table. Because there
are not enough rows in the table to fill the screen, control will return to function INVOICE. Upon return
from TBDISPL, the system variables used by the dialog have these values:
ZSCROLLA
0050
ZTDADD
YES
ZTDSCRP
51
ZTDAMT
7
ZTDSIZE
17
ZTDAMT contains the number of rows that must be added to satisfy the scroll request and fill a full
screen. ZTDSCRP has the CRP of the row that will be at the top of the screen after the scroll. Because it
is nonzero, function INVOICE does not need to set it. In fact, all that the function has to do is skip to the
table bottom, read and add the next 7 invoices to the table, and then issue a TBDISPL service request to
redisplay the table. When the table is displayed again, it appears as shown in Figure 18 on page 50.
Display Services
Chapter 3. Introduction to writing dialogs  49

## Page 78

------------------------------ 1986 TRANSACTIONS ------ ROW 51 OF 10000
 Command ====>                                        Scroll ===> PAGE
 Invoice    Transaction    Part      Quantity   Customer
 Number        Date        Number
 -------    -----------    ------    --------   --------
 0000051     01/15/86      7536          6      Parts Unlimited
 0000052     01/15/86      0546         54      ABC Parts
 0000053     01/15/86      3349         65      Parts Company of NC
 0000054     01/15/86      4234        340      XYZ Auto Center
 0000055     01/15/86      0342         70      XYZ Auto Center
 0000056     01/18/86      4544         52      ABC Parts
 0000057     01/19/86      0763        330      XYZ Auto Parts
 0000058     01/19/86      0841        540      Bosworth's Parts
 0000059     01/19/86      0445        560      ABC Parts
 0000060     01/19/86      4542        450      ACME Parts
 0000061     01/25/86      7071        100      Acme Parts
 0000062     01/25/86      0015         15      Parts City
 0000063     02/27/86      1023        340      XYZ Auto Center
 0000064     02/04/86      0231          1      Parts Unlimited
 0000065     02/04/86      3423        805      Bosworth's Parts
 0000066     02/04/86      2341        165      Acme Parts
 0000067     02/04/86      7653         20      Acme Parts
Figure 18. Second display for dynamic table expansion example
Now assume that the user runs the command DOWN 5000:
This should result in rows 5051-5067 being displayed. As before, there are not enough rows in the table
to handle the scroll request, so control returns to function INVOICE with this information in the system
variables:
ZSCROLLA
5000
ZTDADD
YES
ZTDSCRP
0
ZTDAMT
5000
ZTDSIZE
17
Notice that this time ZTDSCRP has a value of 0. This indicates that the new top row, as requested by
the user scroll, is not in the physical table. After adding the 5000 rows indicated by the ZTDAMT system
variable, function INVOICE must set ZTDSCRP to the CRP of the row that should be displayed at the top
after the scroll (row 5051). This is accomplished in the dialog by adding ZTDAMT to the number of rows
in the current table, and then subtracting out the size of the scrollable area (ZTDSIZE). When the table is
redisplayed, it appears as shown in Figure 19 on page 51.
Display Services
50  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 79

------------------------------ 1986 TRANSACTIONS ------ ROW 5051 OF 10000
 Command ====>                                        Scroll ===> PAGE
 Invoice    Transaction    Part      Quantity   Customer
 Number        Date        Number
 -------    -----------    ------    --------   --------
 0005051     07/12/86      7326        436      Parts Unlimited
 0005052     07/12/86      0516         54      ABC Parts
 0005053     07/21/86      3549          5      Parts Company of NC
 0005054     07/24/86      4243        350      XYZ Auto Center
 0005055     07/25/86      0342        540      XYZ Auto Center
 0005056     07/31/86      4544        444      ABC Parts
 0005057     07/11/86      0653         30      XYZ Auto Parts
 0005058     08/29/86      0821        450      Bosworth's Parts
 0005059     08/01/86      6445        460      ABC Parts
 0005060     08/01/86      4942        850      ACME Parts
 0005061     08/01/86      7021        180      Acme Parts
 0005062     08/01/86      6026        945      Parts City
 0005063     08/07/86      1523         30      XYZ Auto Center
 0005064     08/07/86      0531        451      Parts Unlimited
 0000065     08/07/86      3263        455      Bosworth's Parts
 0005066     08/07/86      2771          5      Acme Parts
 0005067     08/07/86      7453        576      Acme Parts
Figure 19. Third display for dynamic table expansion example
Finally, assume that the user runs the command DOWN 5000: A scroll of 5000 would display rows
10051-10067, if there were that many invoices in the file. However, because there are only 10 000
invoices, function INVOICE can add only rows 5068-10000 to the table and then redisplay the table. On
return from TBDISPL, the system variables again contain this information:
ZSCROLLA
5000
ZTDADD
YES
ZTDSCRP
0
ZTDAMT
5000
ZTDSIZE
17
After adding all of the invoices to the table (end of file is reached), the dialog must set system variable
ZTDSCRP. Because the scroll amount has caused the user to scroll past the end of data, the dialog sets
ZTDSCRP to a value that will cause only the bottom of data marker to be displayed. That is, ZTDSCRP is
set to a value greater than the number of rows in the table. When the table is redisplayed it appears as
shown in Figure 20 on page 52.
Display Services
Chapter 3. Introduction to writing dialogs  51

## Page 80

------------------------------ 1986 TRANSACTIONS ---------------------
 Command ====>                                        Scroll ===> PAGE
 Invoice    Transaction    Part      Quantity   Customer
 Number        Date        Number
 -------    -----------    ------    --------   --------
 ****************************** BOTTOM OF DATA *****************************
 
Figure 20. Fourth display for dynamic table expansion example
One case not illustrated is that of the user issuing a DOWN MAX scroll request. In this case ZTDAMT and
ZTDSCRP would each have a value of 0 when control returns to the dialog. ZSCROLLA would have a value
of MAX. The dialog would add all remaining invoices to the table and then redisplay the table. It is not
necessary in a MAX scroll case to set ZTDSCRP before redisplaying the table because ISPF automatically
positions the table so that a full screen plus the bottom of data marker are displayed.
In this example the program has been written so that control continues to return to the dialog after
all of the invoice file records have been added to the table. To further improve performance, it may be
desirable for the dialog to disable the return after the end of file has been reached. This can be done by
setting the ZTDRET function pool variable to some value other than DOWN, UP, or VERTICAL, and then
issuing a TBDISPL service request with the panel name specified. Be aware that when a panel name is
specified, ISPF clears any pending scroll requests. So it is up to the dialog to position the table CRP to the
appropriate row to simulate the scroll. For example, assume that a DOWN MAX scroll request has been
issued and the dialog has added all remaining invoices to the table. The dialog then sets ZTDRET to blank
and prepares to issue the TBDISPL service request, with a panel name, to display the table. To simulate
the user scroll the dialog issues a TBSKIP service request to position the CRP to the row that will cause
a full screen plus the bottom of data marker to be displayed. When the TBDISPL request is subsequently
issued, ISPF will position the table based on the CRP, thereby simulating the scroll.
Using the variable services
Dialog variables are the main communication vehicle between the components of a dialog and ISPF
services. Program modules, command procedures, panels, messages, tables, and skeletons can all refer
to the same data through the use of dialog variables. Variable services allow you to define and use dialog
variables.
Some variable services require that ISPF search through the variable pools to locate requested variables.
ISPF searches the pools in this order:
1. Function pool (defined variables)
2. Function pool (implicit variables)
3. Shared pool
4. Application profile pool (profile pool).
Note: ISPF uses variable names that start with Z. Therefore, Z variables are reserved for ISPF system-
related uses. User written dialogs must avoid creating or manipulating variable names that start with Z
unless their use has been clearly documented.
Variable Services
52  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 81

Searching variable pools
Dialog variables are organized into groups, or pools, according to the dialog and application with which
they are associated. An application is one or more dialogs, each of which has been started using the same
application ID.
A pool can be thought of as a list of variable names that enables ISPF to access the associated values.
When a DM service encounters a dialog variable name in a panel, message, table, or skeleton, it searches
these pools to access the dialog variable's value. The pools and the types of dialog variables that reside in
them are:
Function pool
Contains variables accessible only by that function. A variable that resides in the function pool of the
function currently in control is called a function variable.
Shared pool
Contains variables accessible only by dialogs belonging to the same application. A variable that
resides in the shared pool of the current application is called a shared variable.
Profile pool
Contains variables that are automatically retained for the user from one session to another. A variable
that resides in the profile pool is called an application profile variable or profile variable. Profile
variables are automatically available when an application begins and are automatically saved when it
ends.
The number of shared, function, and profile variables that can exist at any one time depends on the
amount of storage available.
SELECT service and variable access
Figure 21 on page 54 shows how the SELECT service can be used to pass control within a dialog and
illustrates the resulting pool structures. Menus A and B access variables from the shared and profile
pools, because menus are not part of any function. The dialog invokes Function X, which uses the VPUT
service to copy one of the variables from its function pool into the shared pool. Next, the dialog invokes
Function Y, which uses the VGET service to copy a dialog variable from the shared pool to its function
pool. Then it uses the SELECT service for further menu processing.
Variable Services
Chapter 3. Introduction to writing dialogs  53

## Page 82

Figure 21. Control and data flow in a dialog
Figure 21 on page 54 also shows how the SELECT service controls access to dialog variable pools from
both functions and menus.
When you define a variable as an input variable on a selection panel, these actions occur during
processing of the menu:
• If the variable does not exist in either the shared pool or the profile pool, it is created in the shared pool.
• If the variable exists in the shared pool, it is accessed from, and is updated in, the shared pool.
• If the variable exists in the profile pool and not in the shared pool, it is accessed from, and is updated in,
the profile pool.
Function pools and dialog functions
Each function has its own unique pool of dialog variables. This is illustrated in Figure 21 on page 54.
These function pools are maintained by ISPF on behalf of each respective function. A function uses
these dialog variables to communicate with the various DM services. A function pool's variables can be
accessed only by the function for which the pool was created. To make these variables available to other
functions, you must use variable services to copy any variables to be shared into the shared pool.
Dialog variables associated with one function can have the same names as dialog variables associated
with another function, but they reside in different function pools, and therefore, are not the same
variables.
When a new function begins, ISPF creates a function pool for it. Variables can then be created in the
function pool and accessed from it. When the function ends, its function pool, along with any variables in
it, is deleted.
Variable Services
54  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 83

Command procedures, program functions, and function pools
When the function in control is a command procedure, the list of variable names kept by the command
language processor and the list of function variables kept by ISPF is the same list. Thus, a variable created
by the command procedure during its execution is automatically a dialog variable. Likewise, the command
procedure can automatically access a dialog variable entered in the function pool by ISPF. However, ISPF
variable names cannot exceed 8 characters.
Any CLIST or REXX variable such as SYSDATE and SYSTIME, which are dynamically evaluated when
referred to, can be used in a CLIST or REXX exec running under ISPF; however, it cannot be used in
panels, messages, skeletons, or tables. For SYSDATE and SYSTIME, use ISPF system variables ZDATE and
ZTIME, respectively, which contain similar information.
ISPF makes available two other system variables, ZDATEF and ZDATEFD, to support date representation
in various national languages. ZDATEF contains the date represented by the characters YY, MM, and DD
plus delimiters. These characters are never translated; however, they can be in any order. For example,
the date could be expressed MM/DD/YY, YY/MM/DD, and so on, depending on how a date is expressed in
a given national language. ZDATEFD contains the same date format, translated into the session national
language.
TSO global variables, in effect when ISPF is started, are not available to CLISTs running under ISPF. These
global variables are restored when ISPF terminates. Any global variables put into effect from within ISPF
are lost when ISPF terminates.
This CLIST command procedure example illustrates that ISPF treats command procedure variables as
dialog variables.
Assume that the definition for panel XYZ contains two dialog variable input fields, AAA and BBB. In the
panel definition, they might appear as follows:
+  INITIAL VALUE %===>_AAA     + 
+  INCREMENT     %===>_BBB     + 
where the underscore indicates the start of an input field, followed by the name of the variable.
When the procedure:
SET &AAA = 1
ISPEXEC DISPLAY PANEL(XYZ)
SET &CCC = &AAA + &BBB
is executed, variable AAA is set to the value 1. The procedure then invokes the DISPLAY service to display
panel XYZ. The value of AAA is 1 on the displayed panel. ISPF creates the variable BBB in the function
pool and displays it as a blank.
Now, in response to the panel display, you type 100 in the first field (AAA) and 20 in the second field
(BBB). When you press Enter, the value 100 is automatically stored in AAA and the value of 20 is
automatically stored into BBB. The DISPLAY service then returns control to the command procedure.
When the next statement executes, it creates variable CCC and sets it to 120, the sum of AAA and BBB.
When the function in control is a program, the associated function pool is not shared with ISPF. This is
because a program is compiled, not interpreted as command procedures are. ISPF maintains a list of
variables that belong to the function so that DM services can use dialog variables for communication of
data.
ISPF makes two types of entries in the program function pool, defined and implicit.
Use a variable service to create or delete defined variables
Use the VDEFINE service to create a defined dialog variable name in the function pool and associate it
with the corresponding program variable. This association enables ISPF to directly access and modify that
program variable. Otherwise, the program's variables are not available to ISPF. Use the VDELETE service
to end this association and remove ISPF's ability to access that program variable.
Variable Services
Chapter 3. Introduction to writing dialogs  55

## Page 84

The program shown, coded in PL/I, specifies that field PA of the program can be accessed by ISPF by
using a dialog variable named FA. Then, the DISPLAY service is called to display panel XYZ.
DECLARE PA CHAR(8);
DECLARE LENGTHPA FIXED BIN(31) INIT(LENGTH(PA));
PA = 'OLD DATA';
CALL ISPLINK ('VDEFINE ', 'FA ', PA, 'CHAR ', LENGTHPA);
CALL ISPLINK ('DISPLAY ', 'XYZ ');
PA is declared as a program variable (character string, length 8). The program calls the VDEFINE service
to make PA accessible to ISPF through dialog variable FA. If dialog variable FA is specified as an input
field on panel XYZ, then "OLD DATA" displays in field FA, and ISPF stores any data entered in that field
into the program variable PA.
Creating implicit variables
ISPF places implicit variables in the function pool when an ISPF service:
• Refers to a dialog variable name that is not found in the standard search reference
• Must store data in a dialog variable that does not already exist in the function pool.
Here is an illustration of how ISPF creates an implicit variable. Assume that panel XYZ, in the preceding
example, allows the user to enter a second value and that this value is to be stored in dialog variable IA.
This is the first reference to IA; therefore, it does not yet exist in the function pool. Because variable IA
does not exist when it is referred to, ISPF creates it in the function pool. ISPF then stores into IA the value
entered on the panel. Thus, IA is an implicit dialog variable.
Any DM service invoked by a program function can access an implicit variable directly by referencing
the variable name. However, implicit variables cannot be accessed directly from a program function.
Programs access implicit variables only through the use of the VCOPY and VREPLACE services.
When you are using APL2, variables in the current APL2 workspace that follow APL2 and ISPF naming
rules become function pool variables. ISPF treats these as implicit variables. The VDEFINE service is not
used with APL2 dialogs.
Naming defined and implicit variables
A defined variable and an implicit variable can have the same name. This occurs when, using the VDEFINE
service, a defined variable is created that uses the same name as an existing implicit variable. When the
same name exists in both the defined and the implicit areas of a function pool, only the defined entry
can be accessed. You can make the implicit entry accessible by using the VDELETE service to remove any
defined entries for that variable name made through the VDEFINE service. The implicit entries are not
affected.
You can define a given dialog variable name many times within a given function. Each definition can
associate a different program variable with the dialog variable name. This is referred to as stacking. Only
the most recent definition of that dialog variable is accessible. A previous definition of that variable can be
made accessible by using the VDELETE service to delete the more recent definitions of that name.
For example, the main routine of a program can define a dialog variable to be associated with one
program variable. A subroutine is called and can define the same dialog variable name to be associated
with a different program variable. Any ISPF services invoked after the second VDEFINE would have access
to only the subroutine's program variable. The subroutine would use the VDELETE service to delete that
dialog variable before returning, thereby uncovering the earlier definition set up in the main routine. To
avoid a possible program error, each VDEFINE processed within a function for a given dialog variable
name should have a VDELETE using the same name or an asterisk (*) as the operand. When an asterisk
is used as the operand, the VDELETE service removes all dialog variable names previously defined by the
program module from the function pool.
The VRESET service allows a program to remove its function pool variables as though VDELETEs had been
done. Any implicit variables are also deleted.
Variable Services
56  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 85

Sharing variables among dialogs
The shared pool allows dialog functions and selection panels to share access to dialog variables.
The SELECT service creates shared pools when it processes the ISPSTART or ISPF command, and when
you specify the NEWAPPL or NEWPOOL keywords with the SELECT service. When SELECT returns, it
deletes the shared pool and reinstates any previous shared pool.
A function can copy dialog variables from its function pool to the shared pool by using the VPUT service.
In addition, another function can directly copy these variables to its function pool by means of the VGET
service. Because a panel displayed by the SELECT service does not belong to any function, any dialog
variables used in the panel are read from and stored into the shared or profile pool.
Saving variables across ISPF sessions
Like the shared pool, the application profile pool contains variables that are accessible to dialogs within
an application. But, unlike the shared pool, the profile variables are saved across sessions.
When a new application is started, it has access to a profile pool. If an application is restarted by split
screen, for example, both calls of the application access exactly the same profile pool. The profile pool is
maintained as an ISPF table whose name is xxxxPROF, where xxxx is the application ID. If the application
is already active, then the current profile pool is used.
When accessing an application profile pool that is not currently active, ISPF first searches the user's
profile files for a profile named xxxxPROF. ISPF finds the profile if the user previously ran the application,
and thus, had a copy of the profile pool.
If ISPF cannot find the profile, it searches the table input file. The application developer can provide
a profile pool with the table files. A profile pool contains variable names and values initialized for the
application.
If ISPF cannot find the member in either the user's profile pool or table input library, it initializes the
application profile pool with the contents of the default profile pool, ISPPROF, which is read from the
table input library. If the dialog manager application ID "ISP" is active, the currently active copy of
ISPPROF is used as the default, rather than reading ISPPROF from ISPTLIB. ISPPROF is distributed with
ISPF. It contains a set of default Function key values. An installation can modify this table to change these
settings or to include other variables that will be copied to initialize new profile pools.
Upon completion of the application, ISPF saves the contents of the application profile pool, under the
name xxxxPROF, in the user's profile library. ISPF deletes the profile pool from storage when the last call
of the application terminates.
You must use the VPUT service to enter variables in the profile pool. Functions can copy variables from
the profile pool into function pools by using the VGET variable services. Selection panels automatically
update existing profile variables.
Removing variables from the shared or profile pool
You can use the VDELETE or VRESET service to remove variables only from the function pool. However,
if you wish to do some housekeeping in the other variable pools, you can use the VERASE service. The
VERASE service allows you to remove variable names and values from the shared pool, the profile pool, or
both. You can specify on the VERASE service request a list of one or more variable names to be removed
from the shared pools or both. For example:
ISPEXEC VERASE (AGE ADDRESS SOCSEC) PROFILE
might be used to remove variable values for age, address, and social security number from the profile
pool.
For detailed information about VERASE and other services, refer to the z/OS ISPF Services Guide.
Variable Services
Chapter 3. Introduction to writing dialogs  57

## Page 86

Read-only profile pool extension variables
ISPF provides for a read-only extension of the application profile variable pool. This allows installations
to maintain better control over application default profile variables. It also results in conservation of disk
storage because a copy of these variables need not exist in the application profile of every application
user.
To use the read-only extension, you do two things:
1. First you must define the read-only extension. The read-only extension is actually a table, which you
can create by using the ISPF TBCREATE table service. You add variables to this table as extension
variables; that is, variables not specified when the table is created. This is illustrated in the CLIST
procedure shown, using the SAVE keyword on the TBADD table service.
You need to create the extension table only once. After the table is saved, you must define it to ISPF by
using an ALLOCATE command or a LIBDEF service request.
2. You then use DM variable services to put the name of the read-only extension table into system
variable ZPROFAPP in the profile variable pool.
An example of a CLIST to create a read-only extension table named ROTABLE is shown in Figure 22
on page 58. The table is to contain variables RDONLY1, RDONLY2, and RDONLY3 set to values of
LKHFC, FLIST, and SPOOLFUL, respectively. After the procedure closes the table, it sets system variable
ZPROFAPP to the table name, ROTABLE. The procedure then puts ZPROFAPP into the profile variable
pool.
/* Example of creating a read-only extension table */
SET ROV1 = LKHFC
SET ROV2 = FLIST
SET ROV3 = SPOOLFUL
SET ROVLIST = &STR(ROV1 ROV2 ROV3)
ISPEXEC TBCREATE ROTABLE
ISPEXEC TBADD ROTABLE SAVE(&ROVLIST)
ISPEXEC TBCLOSE ROTABLE
SET &RC = &LASTCC
IF &RC = 0 THEN -
DO
  /* Put extension table name into system variable ZPROFAPP. */
  SET ZPROFAPP = ROTABLE
  ISPEXEC VPUT ZPROFAPP PROFILE
END
Figure 22. CLIST to create a read-only extension table
When a new application that uses the NEWAPPL keyword on the SELECT service is specified, ISPF
interrogates variable ZPROFAPP in the new application's profile pool. If the variable value is not null, it is
assumed to be the name of the profile extension table. ISPF searches the table input files for a table with
the name specified by ZPROFAPP. The set of variables in this table becomes the read-only extension for
the profile pool of the application just selected.
Although variable services are not effective for updating the read-only extension, you can create or
update the table used as the extension by using DM table services. Updating the extension may be done
only when the application is not active, because the table is open in nowrite mode while the application is
active.
If a variable name is referred to and exists in both the profile pool and the read-only extension table, ISPF
uses the variable from the user's profile pool. In other words, the search order is: first the profile pool,
and then the read-only extension.
If a VPUT PROFILE is issued for a variable in the read-only extension, the update for that variable is made
to the user area of the profile pool, not to the read-only extension. Only the profile pool variable update is
saved and accessed, not the extension variable value.
Variables owned by ISPF
A second level of profile pool, the system profile pool (ISPSPROF), is always active. The dialog manager
owns the dialog variables within the system profile pool, and the variables cannot be modified by an
Variable Services
58  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 87

application. They can be read, however, because the system profile pool is included in the standard
search sequence after the profile pool. All system variable names begin with "Z", such as ZTERM, and
supply information such as terminal type and list and log defaults.
If a system profile pool variable is used on a selection panel, a corresponding field is created in the profile
pool (ISPPROF). Subsequently, when that variable is referred to by the dialog, the profile pool value is
used rather than the system profile pool value. The dialog can use the VERASE service to delete variables
from the profile (ISPPROF) pool.
Note: ISPF uses variable names that start with Z. Therefore, Z variables are reserved for ISPF system-
related uses. User written dialogs must avoid creating or manipulating variable names that start with Z
unless their use has been clearly documented.
Variable formats
Information entered on a panel is in character string format. All dialog variables remain in character string
format when stored:
• As implicit variables in a function pool
• In the shared pool
• In the profile pool
• In ISPF tables.
Defined variables, however, can be translated to a fixed binary, bit, hexadecimal, float, packed, or binary
string, or to a user-defined format when stored internally in a program module. The translation occurs
automatically when the variable is stored by an ISPF service. A translation back to character string format
occurs automatically when the variable is accessed.
The VMASK service is used to validate input into a VDEFINEd dialog variable. See the z/OS ISPF Services
Guide for more information.
When a defined variable is stored, either of two errors can occur:
Truncation
If the current length of the variable is greater than the defined length within the module, the
remaining data is lost.
Translation
If the variable is defined as something other than a character string, and the external representation
has invalid characters, the contents of the defined variable are lost.
In either case, the ISPF service issues a return code of 16.
System variables communicate between dialogs and ISPF
System variables are used to communicate special information between the dialog and the dialog
manager (ISPF). System variable names are reserved for use by the system. They begin with the letter
"Z". Therefore, avoid names that begin with "Z" when choosing dialog variable names.
The types of system variables are input, output, non-modifiable, and input-output. Their type depends on
their usage.
To access and update system variables, use variable services according to which pool the variables are in.
System variables in the function pool can be accessed and updated directly from a command procedure.
Those in the shared or profile pools can be accessed by using the VGET service, and updated by using the
VPUT service.
A program function can access and update system variables in the function pool using the VDEFINE
service. Dialog variables can be accessed by using the VCOPY service and updated by using the
VREPLACE service.
Variable Services
Chapter 3. Introduction to writing dialogs  59

## Page 88

The system variables in the shared or profile pools can be accessed by using the VCOPY service. They can
be updated by first updating the variable in the function pool by using the VDEFINE or VREPLACE service
and then moving that value to the shared or profile pool by using the VPUT service.
Using VDEFINE, VDELETE, VRESET, VCOPY, VMASK, and VREPLACE
For functions coded in a programming language other than APL2, you can manage the availability to
ISPF of the internal program variables that are to be used as dialog variables through the ISPF VDEFINE,
VDELETE, and VRESET services.
Variables used in a program function are not automatically put into that function's variable pool.
Therefore, those variables are not initially available to ISPF for processing function requests. A function
can use the VDEFINE service to make function variable names available to ISPF through the function pool.
The VDELETE and VRESET services are used to cancel the effect of using VDEFINE service requests.
VDELETE can be used to delete access by ISPF to selected defined variables by removing them from the
function pool. VRESET removes all defined and implicit variables from the function pool.
A program function can obtain a copy of dialog variables by using the VCOPY service. The service request
can specify that either the variable data address or the data itself be returned.
The VMASK service is used to validate the data of a variable defined with the VDEFINE service. VMASK
associates a specified user or predefined mask with a variable previously defined with VDEFINE. The
VEDIT statement must be used to indicate VMASKed variables on a panel.
A program function can update the contents of dialog-defined or implicit variables in the function pool
by using the VREPLACE service. The names of the variables to be updated and the new contents are
specified with the VREPLACE service request.
The VDEFINE, VDELETE, VRESET, VCOPY, VMASK, and VREPLACE variable services are not used with
functions coded as procedures. For a function coded as a CLIST or APL2 procedure, variables used in the
procedure are automatically treated as dialog variables. No special action is required to define them to
ISPF. Any trailing blanks in CLIST variables are not truncated; they remain as part of the variables.
Using the VGET, VPUT, and VERASE services
The VGET, VPUT, and VERASE services can be used by both program and procedure functions. Functions
use the VGET and VPUT services to control movement of variables between function pools and shared
or profile pools. Functions can also obtain the values of system symbolic variables by using the SYMDEF
parameter on the VGET service.
Each function has its own function variable pool. The variables in a given function's pool are not available
to other functions, and vice versa. To overcome this, a function can use the VGET service to copy into its
function pool variables from the shared or profile pools. The function can make variables in its function
pool available to other functions in the same application by copying them to the shared or profile pool by
using the VPUT service.
You can use the VERASE service to remove variable names and values from the shared pool and profile
pool. The VDELETE and VRESET services are available for removing function pool variables.
Summary of variable services
The variable services are:
All Functions
VERASE
Remove variables from the shared pool or profile pool
VGET
Retrieve variables from the shared pool or profile pool or retrieve the value of a system symbolic
variable
Variable Services
60  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 89

VPUT
Update variables in the shared pool or profile pool
Program Functions Only
VCOPY
Copy data from a dialog variable to the program
VDEFINE
Define function program variables to ISPF
VDELETE
Remove definition of function variables
VMASK
Associate a mask with a dialog variable
VREPLACE
Update a dialog variable with program data specified in the service request
VRESET
Reset function variables
Using the table services
Table services let you use and maintain sets of dialog variables. A table is a two-dimensional array of
information in which each column corresponds to a dialog variable, and each row contains a set of values
for those variables.
Contents for a table are shown in Table 6 on page 67. In that example, the variables that define the
columns are as follows:
EMPSER
Employee Serial Number
LNAME
Last Name
FNAME
First Name
I
Middle Initial
PHA
Home Phone: Area Code
PHNUM
Home Phone: Local Number
Where tables reside
A table can be either temporary or permanent. A temporary table exists only in virtual storage. It cannot
be written to disk storage.
Permanent tables are maintained in one or more table libraries. A permanent table, while created in
virtual storage, can be saved on direct access storage. It can be opened for update or for read-only
access, at which time the entire table is read into virtual storage. When a table is being updated in virtual
storage, the copy of the table on direct access storage cannot be accessed until the update is complete.
For both temporary and permanent tables, rows are accessed and updated from the in-storage copy. A
permanent table that has been accessed as read-only can be modified in virtual storage, but cannot be
written back to disk storage.
When a permanent table is opened for processing, it is read from a table input library. A table to be
saved can be written to a table output library that is different from the input library. The input and output
libraries should be the same if the updated version of the table is to be reopened for further processing by
the same dialog.
Table Services
Chapter 3. Introduction to writing dialogs  61

## Page 90

Accessing data
You specify the variable names that define table columns when the table is created. Specify each variable
as either a KEY field or a NAME (non-key) field. You can specify one or more columns (variable names) as
keys for accessing the table. For the table shown in Table 6 on page 67, EMPSER might be defined as the
key variable. Or EMPSER and LNAME might both be defined as keys, in which case, a row would be found
only if EMPSER and LNAME both match the current values of those variables. A table can also be accessed
by one or more "argument" variables that need not be key variables. You can define the variables that
constitute the search argument dynamically by using the TBSARG and TBSCAN services.
In addition, a table can be accessed by use of the current row pointer (CRP). The table can be scanned by
moving the CRP forward or backward. A row can be retrieved each time the CRP is moved. When a table is
opened, the CRP is automatically positioned at TOP, ahead of the first row. Table services, such as TBTOP,
TBBOTTOM, and TBSKIP are available for positioning the CRP.
When a row is retrieved from a table, the contents of the row are stored in the corresponding dialog
variables. When a row is updated or added, the contents of the dialog variables are saved in that row.
When a row is stored, a list of "extension" variables can be specified by name. These extension variables,
and their values, are added to the row. Thus, variables that were not specified when the table was created
can be stored in the row. A list of extension variable names for a row can be obtained when the row is
read. If the list of extension variables is not specified again when the row is rewritten, the extensions are
deleted.
ISPF Table Services treat blank data and NULL (zero-length) data as equal. For example, these VDEFINES
are executed:
"ISPLINK('VDEFINE ','(V1)',VAL1,'CHAR ',L8,' NOBSCAN ')"
"ISPLINK('VDEFINE ','(V2)',VAL2,'CHAR ',L8)"
If L8 = 8, VAL1 = 'ABCD    ' and VAL2 = 'ABCD    ', V1 will have a length of 8 and a value of 'ABCD    ', and
V2 will have a length of 4 and a value of 'ABCD'. To ISPF, V1 and V2 will be equal because before ISPF
compares two values, it pads the shorter value with blanks so that the lengths are equal.
If the same VDEFINES are done with VAL1 = '        ' and VAL2 = '        ', V1 will have a length of 8 and a value
of '        ' (8 blanks), and V2 will have a length of 0 (NULL value). To ISPF, V1 is EQUAL to V2, because ISPF
will pad V2 with 8 blanks before doing the comparison to V1.
Services that affect an entire table
These services operate on an entire table:
TBCLOSE
Closes a table and saves a permanent copy if the table was opened
TBCREATE
Creates a new table and opens it for processing
TBEND
Closes a table without saving
TBERASE
Deletes a permanent table from the table output file
TBOPEN
Opens an existing permanent table for processing
TBQUERY
Obtains information about a table
TBSAVE
Saves a permanent copy of a table without closing
TBSORT
Sorts a table
Table Services
62  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 91

TBSTATS
Provides access to statistics for a table
Temporary tables are created by the TBCREATE service (NOWRITE mode) and deleted by either the
TBEND or TBCLOSE service. A new permanent table is created in virtual storage by the TBCREATE service
(write mode). The table does not become permanent until it is stored on direct access storage by either
the TBSAVE or TBCLOSE service.
An existing permanent table is opened and read into virtual storage by the TBOPEN service. If the table
is to be updated (WRITE mode), the new copy is saved by either the TBSAVE or TBCLOSE service. If it is
not to be updated (NOWRITE mode), the virtual storage copy is deleted by either the TBEND or TBCLOSE
service.
Services that affect table rows
These services operate on a row of the table:
TBADD
Adds a new row to the table.
TBBOTTOM
Sets CRP to the last row and retrieves the row.
TBDELETE
Deletes a row from the table.
TBEXIST
Tests for the existence of a row (by key).
TBGET
Retrieves a row from the table.
TBMOD
Updates an existing row in the table. Otherwise, adds a new row to the table.
TBPUT
Updates a row in the table if it exists and if the keys match.
TBSARG
Establishes a search argument for use with TBSCAN. Can also be used in conjunction with TBDISPL.
TBSCAN
Searches a table for a row that matches a list of "argument" variables, and retrieves the row.
TBSKIP
Moves the CRP forward or back by a specified number of rows, and then retrieves the row at which the
CRP is positioned.
TBTOP
Sets CRP to TOP, ahead of the first row.
TBVCLEAR
Sets to null dialog variables that correspond to variables in the table.
Protecting table resources
Table services provide a resource protection mechanism designed to prevent concurrent updating of the
same table by more than one user. This protection mechanism assumes that for all users having update
access to a given table, the same library name is used in the first statement defining the table for the table
library. This can be ISPTLIB or another specified library. Other libraries can be specified by the use of the
LIBRARY keyword or the LIBDEF service.
When a table is opened or created in write mode, an exclusive enqueue is requested for a resource name
consisting of the first library name defined in the ISPTLIB, or the first library name defined in the LIBRARY
DD or the top file specified in the LIBDEF Service stack, concatenated with the table name. The TBOPEN
or TBCREATE service fails with a return code of 12 if this enqueue or lock is unsuccessful. A successful
enqueue or lock stays in effect until the completion of a TBEND or TBCLOSE service for the table. If the
Table Services
Chapter 3. Introduction to writing dialogs  63

## Page 92

NAME parameter is specified on the TBSAVE or TBCLOSE service, an additional exclusive enqueue or lock
is issued. The resource name consists of the first library name defined in the ISPTLIB, or the first library
name defined in the LIBRARY DD or the top file specified in the LIBDEF Service stack, concatenated with
the name specified in the NAME parameter. If this enqueue or lock fails, the service terminates with a
return code of 12 and the table is not written.
The table output library represented by the ISPTABL definition or specified library name is protected from
concurrent output operations from any ISPF function through a separate mechanism not specific to table
services.
The first data set in the ISPTLIB concatenation should be the same as the data set used for ISPTABL. This
ensures predictable behavior of dialogs that use table services without specifying the LIBRARY keyword.
Example: create and update a simple table
These series of commands demonstrates the use of table services:
1. Create a permanent table, named DALPHA, to consist of dialog variables AA, BB, and CC. AA is the key
field. BB and CC are name fields.
ISPEXEC TBCREATE DALPHA KEYS(AA) NAMES(BB CC) WRITE
Table 3. Sample table
AA BB CC
     
2. Display a panel named XYZ. This panel prompts a user to enter values for dialog variables AA, BB, and
CC, which are used in the steps of this example.
ISPEXEC DISPLAY PANEL (XYZ)
3. Assume the user enters these values on panel XYZ:
AA = Pauly John
BB = W590
CC = Jones Beach
ISPF automatically updates dialog variables AA, BB, and CC, in the function variable pool, with the
user-entered values.
Record these values in the table DALPHA.
ISPEXEC TBADD DALPHA
Table 4. Sample table with dialog variables
AA BB CC
Pauly John W590 Jones Beach
4. Assume these values for dialog variables AA, BB, and CC are entered by a user, as in step 3, through a
panel display operation:
AA = Clark Joan
BB = Y200
CC = Bar Harbor
Record these values in the table DALPHA.
ISPEXEC TBADD DALPHA
Table Services
64  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 93

Table 5. Sample table with user dialog variables
AA BB CC
Pauly John
Clark Joan
W590
Y200
Jones Beach
Bar Harbour
Table services adds a row to table DALPHA immediately following the row added by the previous
TBADD. Following the TBADD, the current row pointer (CRP) is positioned at the newly added row.
Before a row is added by the TBADD service, table service scans the table to determine if the KEY field
of the new row to be added duplicates the KEY field of an existing row. If it does, the TBADD is not
performed.
5. Save table DALPHA for later use by writing it to the table output library.
ISPEXEC TBCLOSE DALPHA
The table DALPHA is written from virtual storage to the file specified by ISPTABL.
Determining table size
The length of any row in a table cannot exceed 65␠536 bytes. The length can be computed as follows:
Row size = 22 + 4a + b + 9c
where:
a
= total number of variables in the row, including extensions
b
= total length of variable data in the row
c
= total number of extension variables in the row
The maximum number of rows allowed in a table is 16␠777␠215. However, dialog variables later used
in processing can only keep a value of 999␠999 as the maximum number of table rows. The total table
size is the sum of the row lengths, plus the length of the data table control block (DTCB), plus the sort
information record for sorted tables. The length of the DTCB can be computed as follows:
DTCB length = 152 + 16d
where:
d
= total number of columns in the table, not including extension variables
The length of the sort information record can be computed as follows:
sort-information length = 12 + 8e
where:
e
= number of sort fields
The number of tables that can be processed at one time is limited only by the amount of available virtual
storage.
Table Services
Chapter 3. Introduction to writing dialogs  65

## Page 94

Example: function using the DISPLAY, TBGET, and TBADD services
This topic describes the use of the DISPLAY, TBGET, and TBADD services in a dialog function that allows a
user to add data to a table. A user can start the function by using the ISPSTART command. If the user has
already started ISPF, the function can be started from:
• A menu
• The command field in any display with an application command that is defined in the current command
table to have the SELECT action
• Another function by using the SELECT service
During function processing, the DISPLAY service controls displays requesting the user to enter data about
new employees. The data consists of:
• Employee serial number, entered on panel SER
• Name and phone number, entered on panel DATA.
Entered information is added to the table, as a row, through the TBADD service.
If the user enters an employee serial number for which an employee record already exists in the table,
a DUPLICATE NUMBER short message displays on line 1 of panel SER. If the user enters the HELP
command or presses the HELP Function key to get further explanation of this short message, this long
message is displayed on line 3 of the panel:
EMPLOYEE RECORD ALREADY EXISTS FOR THIS NUMBER. ENTER ANOTHER
When the user successfully enters data for an employee, the short message NEW RECORD INSERTED is
displayed on line 1 of panel SER. Then the user can enter the serial number of the next employee for
which table data is to be added.
The user ends function processing by entering the END or RETURN command on any displayed panel or
by pressing the END Function key or RETURN Function key.
“Command procedure function” on page 66 lists the complete function, followed by each statement with
supporting text and figures.
Command procedure function
1. CONTROL ERRORS CANCEL
2. TBOPEN TAB1 WRITE
3. DISPLAY PANEL(SER)
4. if return code = 0, go to 6
5. if return code = 8, go to 21
6. TBGET TAB1
7. if return code = 0, go to 9
8. if return code = 8, go to 12
9. DISPLAY PANEL(SER) MSG(EMPX210)
10. if return code = 0, go to 6
11. if return code = 8, go to 21
12. Set dialog variables to blanks
13. DISPLAY PANEL(DATA)
14. if return code = 0, go to 16
15. if return code = 8, go to 21
16. TBADD TAB1
17. if return code = 0, go to 18
Table Services
66  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 95

18. DISPLAY PANEL(SER) MSG(EMPX211)
19. if return code = 0, go to 6
20. if return code = 8, go to 21
21. TBCLOSE TAB1
22. End the function
Description of function steps
1. CONTROL ERRORS CANCEL
This DM service request specifies that the function is to be terminated for a return code of 12 or
higher from a DM service request.
2. TBOPEN TAB1 WRITE
Open table TAB1 in update (WRITE) mode. Read table contents, shown in Table 6 on page 67, into
virtual storage. TAB1 is referred to by Steps 2, 6, 16, and 21. 
Table 6. Five rows in table TAB1
EMPSER LNAME FNAME I PHA PHNUM
598304 Robert Richard P 301 555-1224
172397 Smith Susan A 301 555-8465
813058 Lowe Charles L 202 555-9557
395733 Adams John Q 202 555-1776
502774 Hsu Ann A 914 555-4156
3. DISPLAY PANEL(SER)
This DISPLAY operation uses the panel definition SER, shown in Figure 23 on page 67, to control the
format and content of the panel display, shown in Figure 24 on page 68.
)BODY
%--------------------- EMPLOYEE SERIAL ---------------------------------%
%COMMAND ===>_ZCMD                                                     %
+ 
%ENTER EMPLOYEE SERIAL BELOW:
+ 
+ 
+   EMPLOYEE SERIAL%===>_EMPSER+   (MUST BE 6 NUMERIC DIGITS)
+ 
+ 
+ 
+PRESS%ENTER+TO DISPLAY NEXT SCREEN FOR ENTRY OF EMPLOYEE DATA.
+ 
+PRESS%END KEY+(PF3) TO END THIS SESSION.
)PROC
  VER (&EMPSER,NONBLANK,PICT,NNNNNN)
)END
Figure 23. Panel definition  SER
Table Services
Chapter 3. Introduction to writing dialogs  67

## Page 96

--------------------------  EMPLOYEE SERIAL  ------------------------
 COMMAND ===>
 ENTER EMPLOYEE SERIAL BELOW:
    EMPLOYEE SERIAL ===>             (MUST BE 6 NUMERIC DIGITS)
 PRESS ENTER TO DISPLAY NEXT SCREEN FOR ENTRY OF EMPLOYEE DATA.
 PRESS END KEY (PF3) TO END THIS SESSION.
 
Figure 24. Panel display SER
Both the panel definition and the display are referred to in Steps 3, 9, and 18. The display requests
that a serial number be entered for an employee. The user enters the serial number in the field
labeled EMPLOYEE SERIAL NUMBER. The DISPLAY service then stores it in function pool variable
EMPSER, and verifies it as specified on the panel definition. The verification is specified in a VER
statement in the )PROC section of the panel definition, as shown in Figure 23 on page 67:
VER (&EMPSER,NONBLANK,PICT,NNNNNN)
This statement specifies that EMPSER must be nonblank and must consist of six digits, each in the
range of 0-9.
When the input passes the verification, the DISPLAY service returns control to the function.
If the input fails the verification, the panel is automatically displayed again, but with an appropriate
ISPF-supplied message displayed, right-justified, on line 1. For example, if the user fails to enter the
required employee serial number, the ENTER REQUIRED FIELD message is displayed, as shown in
Figure 25 on page 68, and referred to in Steps 3 and 18.
 ---------------------  EMPLOYEE SERIAL  -------------ENTER REQUIRED FIELD
 COMMAND ===>
 ENTER EMPLOYEE SERIAL BELOW:
    EMPLOYEE SERIAL ===>             (MUST BE 6 NUMERIC DIGITS)
 PRESS ENTER TO DISPLAY NEXT SCREEN FOR ENTRY OF EMPLOYEE DATA.
 PRESS END KEY (PF3) TO END THIS SESSION.
 
Figure 25. Panel display SER with an ISPF-provided message superimposed on line 1
Table Services
68  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 97

After the user re-enters the information, it is stored again in function pool variable EMPSER and
reverified. The process is repeated until the information passes the verification tests.
4. if return code = 0, go to 6
If the return code is 0, the display operation is successfully completed. Go to step 6 to verify that no
record exists for this employee number.
5. if return code = 8, go to 21
If the return code is 8, the END or RETURN command was entered on the display by the user. Go to
step 21 to end processing.
6. TBGET TAB1
This TBGET uses the employee serial number, stored in EMPSER in step 3 or 18, to attempt retrieval
of an employee record from the TAB1 table. The table is a keyed table and has been created in
another dialog by the service request:
TBCREATE TAB1 KEYS(EMPSER) NAMES(LNAME FNAME I PHA PHNUM)
7. if return code = 0, go to 9
A return code of 0 means that the record is found. Therefore, a record already exists for the employee
serial number entered by the user. Go to step 9 to display the DUPLICATE NUMBER message.
8. if return code = 8, go to 12
A return code of 8 means that no record is found. Go to step 12 to request the user to enter employee
data.
9. DISPLAY PANEL(SER) MSG(EMPX210)
This DISPLAY operation uses panel definition SER (Figure 23 on page 67) and message EMPX210,
shown in Figure 26 on page 69 to control the format and content of the display. Figure 26 on page
69 is referred to by steps 9, 13, and 18.
EMPX210  'DUPLICATE NUMBER'       .ALARM=YES
'EMPLOYEE RECORD ALREADY EXISTS FOR THIS NUMBER. ENTER ANOTHER.'
EMPX211  'NEW RECORD INSERTED'
'ENTER SERIAL NUMBER FOR NEXT EMPLOYEE RECORD TO BE INSERTED.'
EMPX212  'ENTER PHONE NUMBER'
'IF THE EMPLOYEE HAS NO PHONE, ENTER 000-000'
EMPX213  'ENTER FIRST NAME'
'A FIRST NAME OR FIRST INITIAL IS REQUIRED.'
EMPX214  'ENTER LAST NAME'
'A LAST NAME IS REQUIRED.'
Figure 26. Message EMPX21
This DISPLAY request, omitting the PANEL(SER) parameter, could have been used in this step:
   DISPLAY MSG(EMPX210)
When the PANEL parameter is omitted, the specified message is superimposed on the panel currently
being displayed, which, in this case, is the panel SER.
The short form of the message EMPX210, DUPLICATE NUMBER, is superimposed on line 1 of the
panel display, shown in Figure 27 on page 70.
Table Services
Chapter 3. Introduction to writing dialogs  69

## Page 98

---------------------  EMPLOYEE SERIAL  -------------DUPLICATE NUMBER
 COMMAND ===>
 ENTER EMPLOYEE SERIAL BELOW:
    EMPLOYEE SERIAL ===> 598304      (MUST BE 6 NUMERIC DIGITS)
 PRESS ENTER TO DISPLAY NEXT SCREEN FOR ENTRY OF EMPLOYEE DATA.
 PRESS END KEY (PF3) TO END THIS SESSION.
 
Figure 27. Panel display SER—short form of message EMPX210 superimpose line 1
While viewing this message, the user can request the long form of the message by pressing the HELP
Function key. The long form of the message
EMPLOYEE RECORD ALREADY EXISTS FOR THIS NUMBER. ENTER ANOTHER.
is superimposed on line 3 of the display. See Figure 28 on page 70.
 ---------------------  EMPLOYEE SERIAL  -------------DUPLICATE NUMBER
 COMMAND ===>
 EMPLOYEE RECORD ALREADY EXISTS FOR THIS NUMBER. ENTER ANOTHER.
 ENTER EMPLOYEE SERIAL BELOW:
    EMPLOYEE SERIAL ===> 598304      (MUST BE 6 NUMERIC DIGITS)
 PRESS ENTER TO DISPLAY NEXT SCREEN FOR ENTRY OF EMPLOYEE DATA.
 PRESS END KEY (PF3) TO END THIS SESSION.
 
Figure 28. Panel display SER—long form of message EMPX210 superimposed on line 3
After the user enters the requested serial number, the DISPLAY service stores it in function pool
variable EMPSER and verifies it as described for step 3. After the input passes verification, the
DISPLAY service returns control to the function.
10. if return code = 0, go to 6
If the return code is 0, the display operation is successfully completed. Go to step 6 to verify that no
record already exists for this employee number.
11. if return code = 8, go to 21
If the return code is 8, the END or RETURN command was entered on the display by the user. Go to
step 21 to end processing.
Table Services
70  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 99

12. Set dialog variables to blanks
These function pool variables are set to blank to prepare to receive data for a new employee record.
13. DISPLAY PANEL(DATA)
The DISPLAY operation uses panel definition DATA, shown in Figure 29 on page 71, to control the
format and content of the display shown in Figure 30 on page 71.
)BODY
%----------------------------  EMPLOYEE RECORDS  ----------------------%
%COMMAND ===>_ZCMD
+ 
%   EMPLOYEE SERIAL: &EMPSER
+ 
+   EMPLOYEE NAME:
+     LAST   %===>_LNAME          + 
+     FIRST  %===>_FNAME          + 
+     INITIAL%===>_I+ 
+ 
+   HOME PHONE:
+     AREA CODE   %===>_PHA+ 
+     LOCAL NUMBER%===>_PHNUM   + 
+ 
+ 
+PRESS%ENTER+TO STORE EMPLOYEE DATA AS ENTERED ABOVE.
+ 
+PRESS%END KEY+(PF3) TO END THIS SESSION.
)INIT
  .CURSOR = LNAME
  IF (&PHA = ' ')
    &PHA = 914
)PROC
  VER (&LNAME,ALPHA)
  VER (&FNAME,ALPHA)
  VER (&I,ALPHA)
  VER (&PHA,NONBLANK,PICT,NNN)
  VER (&PHNUM,PICT,'NNN-NNNN')
  VER (&LNAME,NONBLANK,MSG=EMPX214)
  VER (&FNAME,NONBLANK,MSG=EMPX213)
  VER (&PHNUM,NONBLANK,MSG=EMPX212)
)END
Figure 29. Panel definition  DATA
 ---------------------  EMPLOYEE RECORDS ------------------------------------
 COMMAND ===>
    EMPLOYEE SERIAL: 106085
    EMPLOYEE NAME:
      LAST    ===> __
      FIRST   ===>
      INITIAL ===>
    HOME PHONE:
      AREA CODE    ===>
      LOCAL NUMBER ===>
 PRESS ENTER TO STORE EMPLOYEE DATA AS ENTERED ABOVE.
 PRESS END KEY (PF3) TO END THIS SESSION.
 
Figure 30. Panel display DATA
Table Services
Chapter 3. Introduction to writing dialogs  71

## Page 100

The variables set to blank in step 12 are displayed, along with the new employee serial number
entered in step 3 or 18. The user is asked to enter, in the blank fields displayed on the screen, the
name and phone number for the employee.
After the user enters these fields, the DISPLAY service stores the input in function pool variables
LNAME, FNAME, I, PHA, and PHNUM. Then, verification of the input is performed as specified in VER
statements in the )PROC section of the panel definition (Figure 29 on page 71).
If the input fields pass the verification tests, the DISPLAY service returns control to the function.
If the input fields fail the verification tests, a short-form message is displayed on line 1.
The message can be provided by ISPF, or the number of the message displayed may have been
specified in the VER statement that defined the verification test. See VER statements containing
message IDs EMPX212, EMPX213, and EMPX214 in Figure 29 on page 71. When a message ID is
specified, this message is displayed instead of an ISPF-provided message. In either case, if the user
enters the HELP command, the long form of the message is displayed on line 3.
The messages request that information be re-entered. When this information is re-entered, it is
stored again in function pool variables and reverified. The process is repeated until the verification
tests are passed.
14. if return code = 0, go to 16
If the return code is 0, the display operation is successfully completed. Go to step 16 to add the
record to the table.
15. if return code = 8, go to 21
If the return code is 8, the END or RETURN command was entered on the display by the user. Go to
step 21 to end processing.
16. TBADD TAB1
This TBADD adds a row to table TAB1 by copying values from function pool variables to the table
row. The values copied are employee serial number, stored in the function pool variable EMPSER
by step 3 or 18, and employee name and phone number, stored in function pool variables LNAME,
FNAME, I, PHA, and PHNUM by step 13. Function pool variables must have the same names as the
table variables to which they are to be copied by the TBADD operation. Therefore, the names used
in the TBCREATE request are the same as the names used in the definitions for panels on which the
DISPLAY service accepts user input.
17. if return code = 0, go to 18
If the return code is 0, the TBADD operation is successfully completed. Go to step 18 to display the
NEW RECORD INSERTED message.
18. DISPLAY PANEL(SER) MSG(EMPX211)
This DISPLAY operation uses panel definition SER (Figure 23 on page 67) and message EMPX211
(Figure 26 on page 69) to control the format and content of the display. The short form of message
EMPX211, NEW RECORD INSERTED, is displayed on line 1. If the user enters the HELP command
while this message is being displayed, the long form of the message (Figure 26 on page 69):
ENTER SERIAL NUMBER FOR NEXT EMPLOYEE RECORD TO BE INSERTED
is displayed on line 3.
The user enters another serial number. The DISPLAY service verifies it as described in step 3. When
the serial number passes the verification tests, the DISPLAY service returns control to the function.
19. if return code = 0, go to 6
If the return code is 0, the display operation is successfully completed. Go to step 6 to verify that no
record already exists for this employee number.
20. if return code = 8, go to 21
Table Services
72  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 101

If the return code is 8, the END or RETURN command was entered on the display by the user. Go to
step 21 to end processing.
21. TBCLOSE TAB1
Close the table TAB1. Write it from virtual storage to permanent storage.
22. End the function.
Specifying dbcs search argument format for table services
For table services, you can specify either a DBCS or MIX (DBCS and EBCDIC) format string as a search
argument. If either is used as a generic search argument, such as xxx* (any argument whose first three
characters are ‘xxx’), the argument must be specified as follows:
• DBCS format string
DBDBDBDB**
where DBDBDBDB represents a 4-character DBCS string and ** is a single DBCS character representing
the asterisk (*).
• MIX (DBCS and EBCDIC) format string
eeee[DBDBDBDBDB]*
where eeee represents a 4-character EBCDIC string, DBDBDBDBDB represents a 5-character DBCS
string, [ and ] represent shift-out and shift-in characters, and * is an asterisk in single-byte EBCDIC
format.
Using the file-tailoring services
The file-tailoring services, listed in the order they are normally invoked, are:
FTOPEN
Prepares the file-tailoring process and specifies whether the temporary file is to be used for output
FTINCL
Specifies the skeleton to be used and starts the tailoring process
FTCLOSE
Ends the file-tailoring process
FTERASE
Erases an output file created by file tailoring.
File-tailoring services read skeleton files and write tailored output that can be used to drive other
functions. Frequently, file tailoring is used to generate job files for batch execution.
The file-tailoring output can be directed to a file specified by the function, or it can be directed to a
temporary sequential file provided by ISPF. The file name of the temporary file is available in system
variable ZTEMPF. In MVS, ZTEMPF contains a data set name. The ddname of the temporary file is
available in system variable ZTEMPN.
You can use the ISPFTTRC command to trace both the execution of file tailoring service calls (FTOPEN,
FTINCL, FTCLOSE, and FTERASE) and the processing that occurs within the file tailoring code and
processing of each statement. For more information, refer to “File tailoring trace command (ISPFTTRC)”
on page 324.
Skeleton files
Each skeleton file is read record-by-record. Each record is scanned to find any dialog variable names,
which are names preceded by an ampersand. When a variable name is found, its current value is
substituted from a variable pool.
File—Tailoring Services
Chapter 3. Introduction to writing dialogs  73

## Page 102

Skeleton file records can also contain statements that control processing. These statements provide the
ability to:
• Set dialog variables
• Imbed other skeleton files
• Conditionally include records
• Iteratively process records in which variables from each row of a table are substituted.
When iteratively processing records, file-tailoring services read each row from a specified table. If the
table was already open before processing the skeleton, it remains open with the CRP positioned at TOP.
If the table was not already open, file tailoring opens it automatically and closes it upon completion of
processing.
Problems can occur when using file-tailoring services in conjunction with other services (EDIT, COPY, ...)
that result in modifying the data set members in the ISPSLIB concatenation. ISPSLIB is the input skeleton
library, and it is assumed to be a static library. FTINCL obtains existing DCB/DEB information based on the
last OPEN done against ISPSLIB by ISPF. It is recommended that applications that use file tailoring and
modify members of ISPSLIB, use the LIBDEF service for ISPSLIB to point to the application's skeleton
library.
The application should also check for any changes to the data set information DCB/DEB before invoking
file-tailoring services. If there has been a change, then the application should issue a NULL LIBDEF for
ISPSLIB and then reissue the original LIBDEF for ISPSLIB. This will force a close and re-open of the
ISPSLIB library.
Example of a skeleton file
A sample skeleton file is shown in Figure 31 on page 74. It contains job control language (JCL) for
an assembly and optional load-and-go. The tailored output could be submitted to the background for
submission.
//ASM  EXEC            PGM=IFOXOO,REGION=128K
//                     PARM=(&ASMPARMS)
//SYSIN    DD          DSN=&ASMIN:(&MEMBER),DISP=SHR
//SYSLIB   DD          DSN=SYS1.MACLIB,DISP=SHR
)SEL     &ASMMAC1       ^=&Z
//         DD          DSN=&ASMMAC1,DISP=SHR
)SEL     &ASMMAC2       ^=&Z
//         DD          DSN=&ASMMAC2,DISP=SHR
)ENDSEL
)ENDSEL
//SYSUT1   DD          UNIT=SYSDA,SPACE=(CYL,(5,2))
//SYSUT2   DD          UNIT=SYSDA,SPACE=(CYL,(2,1))
//SYSUT3   DD          UNIT=SYSDA,SPACE=(CYL,(2,1))
//SYSPRINT DD          SYSOUT=(&ASMPRT)
)CM     IF USER SPECIFIED "GO", WRITE OUTPUT IN TEMP DATA SET
)CM     THEN IMBED "LINK AND GO" SKELETON
)SEL    &GOSTEP=YES
//SYSGO    DD    DSN=&&&&OBJSET,UNIT=SYSDA,SPACE=(CYL,(2,1)),
//               DISP=(MOD,PASS)
)IM     LINKGO
)ENDSEL
)CM     ELSE (NOGO), WRITE OUTPUT TO USER DATA SET
)SEL    &GOSTEP=NO
//SYSGO    DD   DSN=&ASMOUT(&MEMBER),DISP=OLD
)ENDSEL
//*
Figure 31. Sample skeleton file 
The sample skeleton refers to several dialog variables (ASMPARMS, ASMIN, MEMBER, and so on)
highlighted in the figure. It also illustrates use of select statements ")SEL" and ")ENDSEL" to conditionally
include records. The first part of the example has nested selects to include concatenated macro libraries
if the library names have been specified by the user (that is, if variables ASMMAC1 and ASMMAC2 are not
equal to the null variable Z).
File—Tailoring Services
74  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 103

In the second part of the example, select statements are used to conditionally execute a load-and-go
step. An imbed statement, ")IM", is used to bring in a separate skeleton for the load-and-go step.
Example of using file-tailoring services
The example shown illustrates file-tailoring services. For this example, assume that:
• LABLSKEL is a member in the file tailoring library. It contains these statements:
    )DOT DALPHA
         NAME:   &AA
    APARTMENT:   &BB
         CITY:   &CC
         YEAR:   &ZYEAR
    )ENDDOT
ZYEAR is the name of an ISPF system variable that contains the current year.
• DALPHA is a member of the table library. It contains these records: 
Table 7. Records contained in DALPHA table
AA BB CC
Pauly John
Clark Joan
W590
Y200
Jones Beach
Bar Harbour
This example creates a name and address list. The file-tailoring service requests are:
• ISPEXEC FTOPEN
ISPEXEC FTINCL LABLSKEL
Issue ISPF commands to process skeleton LABLSKEL. Obtain values for dialog variables AA, BB, and
CC from table DALPHA. The resulting file-tailoring output consists of one address label for each row of
information in table DALPHA.
FTOPEN opens both the file-tailoring skeleton and file-tailoring output files. These files must be defined
to ISPF before starting the ISPF session.
FTINCL performs the file-tailoring process by using the file-tailoring skeleton named LABLSKEL.
LABLSKEL contains the file-tailoring controls, )DOT and )ENDDOT, which specify the use of table
DALPHA.
You can issue multiple FTINCL commands to pull in more than one skeleton.
• ISPEXEC FTCLOSE NAME (LABLOUT)
Write the resulting file-tailoring output to a member named LABLOUT SKELETON.
After the previous commands have been processed, the file-tailoring output file LABLOUT SKELETON
contains these records:
       ┌──────────────────────────────────┐
       │                             │
       │       NAME:   Pauly John    │
       │  APARTMENT:   W590          │
       │       CITY:   Jones Beach   │
       │       YEAR:   84            │
       │       NAME:   Clark Joan    │
       │  APARTMENT:   Y200          │
       │       CITY:   Bar Harbour   │
       │       YEAR:   84            │
       │                             │
       ⋘──────────────────────────────────┘
File—Tailoring Services
Chapter 3. Introduction to writing dialogs  75

## Page 104

Using the PDF services
PDF services consist of the BRIF (Browse Interface), BROWSE, EDIF (Edit Interface), EDIREC (edit
recovery for EDIF), EDIT, EDREC (edit recovery for EDIT), VIEW, and VIIF (View Interface) services and a
set of library access services.
BROWSE, EDIT, EDREC, and VIEW
The BROWSE, EDIT, and VIEW services allow you to create, read, or change MVS data sets or members of
an ISPF library. An ISPF library is a cataloged partitioned data set with a three-level name made up of a
project, a group, and a type. The ISPF library can be private (available only to you) or can be shared by a
group of users. The BROWSE, EDIT, and VIEW services provide direct access to the Browse, Edit, and View
options of PDF, bypassing the Browse mode on the View Entry panel (with or without Browse mode) and
Edit Entry panels.
The EDREC service, which you usually invoke before calling EDIT or VIEW, helps you recover work that
would otherwise be lost if ISPF ended abnormally, such as after a power loss.
See the z/OS ISPF Services Guide for complete descriptions, including examples, of the BROWSE, EDIT,
EDREC, and VIEW services.
BRIF, EDIF, EDIREC, and VIIF
Three services, the Browse Interface (BRIF) service, the Edit Interface (EDIF) service, and the VIEW
Interface (VIIF) service, allow dialogs to provide their own I/O for PDF Browse, Edit, and View. These
services provide browse, edit, and view functions for data accessed through dialog-supplied I/O routines.
BRIF, EDIF, and VIIF require that the invoking dialog perform all environment-dependent functions (such
as allocating, opening, reading, writing, closing, and freeing files).
Use of the BRIF, EDIF, and VIIF services allows the type of data and data access methods being employed
by a dialog to be transparent to Browse, Edit, and View. The Edit Interface Recovery (EDIREC) service
performs edit recovery for EDIF.
These services make it possible to implement functions such as:
• Edit/browse of data other than partitioned data sets or sequential files
• Edit/browse of in-storage data
• Pre- and post-processing of edited or browsed data.
See the z/OS ISPF Services Guide for descriptions and examples of BRIF, EDIF, and EDIREC.
Library access services
The library access services can interact with the BROWSE and EDIT services and can also give you access
to ISPF libraries and to certain system data sets. These services carry out functions such as opening a
library, copying a library or library member, and displaying a library's members.
You can use the library access services with four types of libraries or data sets:
• An ISPF library known by project, group, and type
• A concatenated set of up to four ISPF libraries
• A single existing TSO or MVS partitioned or sequential data set
• A concatenated set of up to four MVS partitioned data sets.
The library access services only support data sets with these attributes:
• The data set is stored on a single DASD volume
• The record format is F, FB, V, VB, or U
• The data set organization is either partitioned or sequential
z/OS ISPF User's Guide Vol I contains an explanation of the ISPF library structure.
PDF Services
76  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 105

See the z/OS ISPF Services Guide for complete descriptions, including examples, of the library access
services.
Another way you can maintain different levels or versions of a library member is to use the software
configuration and library manager (SCLM) utilities. SCLM is a software tool that helps you develop
complex software applications. Throughout the development cycle, SCLM automatically controls,
maintains, and tracks all of the software components of the application. And, you can lock the version
being edited in a private library and then promote it to another group within the library for further
development or testing. See z/OS ISPF Software Config ur ation  and Library Manager Guide and Reference
for more information about SCLM.
Using the miscellaneous services
ISPF provides the CONTROL, GRINIT, GRTERM, GRERROR, GETMSG, LIBDEF, LIST, LOG, and PQUERY
services. You can find more information about these services in the z/OS ISPF Services Guide.
CONTROL service
The CONTROL service defines processing options for the dialog environment as follows:
DISPLAY
Specifies that a display mode is to be set. The valid modes are LOCK, LINE, REFRESH, SAVE,
RESTORE, SM, and ALLVALID.
NONDISPL
Specifies that no display output is to be issued to the terminal when processing the next panel
definition. The valid options are ENTER and END.
ERRORS
Specifies that an error mode is to be set. The valid options are CANCEL and RETURN.
SPLIT
Specifies the user's ability to enter split-screen mode. The valid options are ENABLE and DISABLE.
NOCMD
Specifies that for the next displayed panel only, any command entered on the command line or
through use of a function key is not to be accepted.
SUBTASK
This option pertains to multi-task program dialogs that are invoked as TSO commands by the CMD
interface of the SELECT service. The valid options are PROTECT and CLEAR.
TSOGUI
The valid options are ON, OFF and QUERY. Options ON and OFF are ignored if specified. QUERY always
returns a return code of 0, indicating that all TSO input and output is directed to the 3270 session.
REFLIST
Specifies whether ISPF allocations are allowed to add entries to the data set and library reference
lists. The valid options are UPDATE and NOUPDATE.
LE
Must be used before and after each BRIF or EDIF call where the application has provided Language
Environment®-enabled command, read, or write routines. The valid options are ON and OFF.
PASSTHRU
Controls whether the specified commands are to be passed to the dialog program for processing. The
current status can also be queried. The valid options are LRSCROLL, to specify the LEFT and RIGHT
scroll commands, and actions PASON, PASOFF, and PASQUERY.
For more information on the CONTROL service options, refer to z/OS ISPF Services Guide.
GDDM services (GRINIT, GRTERM, and GRERROR)
The graphics initialization (GRINIT) service initializes the ISPF/GDDM interface and optionally requests
that ISPF define a panel's graphic area as a GDDM graphics field. The graphics termination (GRTERM)
Miscellaneous Services
Chapter 3. Introduction to writing dialogs  77

## Page 106

service terminates a previously established GDDM interface. The graphics error block (GRERROR) service
provides access to the address of the GDDM error record and the address of the GDDM call format
descriptor module.
GETMSG service
The GETMSG service obtains a message and related information and stores them in variables specified in
the service request.
LIBDEF service
The LIBDEF service provides applications with a method of dynamically defining application data element
files while in an active ISPF session.
LIST service
The LIST service allows a dialog to write data lines directly (without using print commands or utilities) to
the ISPF list data set. You specify the name of the dialog variable containing the data to be written on the
LIST service request.
LOG Service
The LOG service allows a function to write a message to the ISPF log file. The user can specify whether
the log is to be printed, kept, or deleted when ISPF is terminated.
PQUERY Service
The PQUERY service returns information for a specific area on a specific panel. The type, size, and
position characteristics associated with the area are returned in variables.
Miscellaneous Services
78  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
