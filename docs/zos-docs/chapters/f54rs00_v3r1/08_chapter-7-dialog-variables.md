# Chapter 7. Dialog variables

Source file: f54rs00_v3r1.md
Start page: 263
Page span: 263-272

## Page 263

Chapter 7. Dialog variables
This topic describes the ISPF dialog variables.
The following table lists the dialog function pool variables that are both read from and written to by
several of the PDF library access services. For details of function pool variables written by other services,
refer to the z/OS ISPF Services Guide.
The variables are listed in alphabetical order. The first column lists the variable name. The second column
indicates the variable's type, which corresponds to the format parameter of the ISPF VDEFINE service.
The third column specifies the variable's length, which corresponds to the length parameter of the
VDEFINE service.
The fourth column lists the PDF services that either read from or write to the variable. An R in parentheses
(R) after a service name indicates that the service, when called, reads from the given variable. A W in
parentheses (W) after a service name indicates that the service, when called, writes to the given variable.
All variables are available to a dialog unless otherwise indicated.
The last column contains a brief description of the contents of the variable and any restrictions on the
value of the variable.
Table 19. Dialog function pool variables
Variable
Name Format Length Service (Access) Description
ZCMD Char 256 LMMDISP(W) Primary Command field from member list panel if
the command is not a valid ISPF or PDF primary
command.
ZDLBLKSZ Char 5 LMDLIST(W) Block size.
ZDLCATNM Char 44 LMDLIST(W) Name of the catalog in which the data set was
located.
ZDLCDATE Char 10 LMDLIST(W) Creation date.
ZDLDEV Char 8 LMDLIST(W) Device type.
ZDLDSNTP Char 8 LMDLIST(W) DS name type (‘PDS’, ‘LIBRARY’, or ‘ ’).
ZDLDSORG Char 4 LMDLIST(W) Data set organization.
ZDLEDATE Char 10 LMDLIST(W) Expiration date.
ZDLEXT Char 3 LMDLIST(W) Number of extents used.
ZDLEXTX Char 5 LMDLIST(W) Number of extents used (long format).
ZDLLRECL Char 5 LMDLIST(W) Logical record length.
ZDLMIGR Char 3 LMDLIST(W) Whether the data set is migrated (YES or NO).
ZDLMVOL Char 1 LMDLIST(W) Multivolume indicator (Y or N).
ZDLOVF Char 3 LMDLIST(W) Whether variables ZDLEXTX and ZDLSIZEX should be
used to obtain the 'number of extents used' and 'data
set size in tracks' values (YES or NO). The value is YES
when the 'number of extents used' value exceeds the
size of variable ZDLEXT or the 'data set size in tracks'
value exceeds the size of variable ZDLSIZE.
ZDLRDATE Char 10 LMDLIST(W) Date last referenced.
Dialog variables
© Copyright IBM Corp. 1989, 2024 237

## Page 264

Table 19. Dialog function pool variables (continued)
Variable
Name Format Length Service (Access) Description
ZDLRECFM Char 5 LMDLIST(W) Record format.
ZDLSIZE Char 6 LMDLIST(W) Data set size in tracks.
ZDLSIZEX Char 12 LMDLIST(W) Data set size in tracks (long format).
ZDLSPACU Char 10 LMDLIST(W) Space units, one of the following values: CYLINDERS,
MEGABYTES, KILOBYTES, BYTES, BLOCKS or
TRACKS.
ZDLUSED Char 3 LMDLIST(W) Percentage of used tracks or pages (PDSE).
ZDLVOL Char 6 LMDLIST(W) Volume serial.
ZDSN Char 44 LMMDISP(W) Name of the first or only data set in the concatenation
of the member list being displayed. This variable is
only available for member list panels.
ZDST Char 54 BRIF (W) EDIF (W) Title line data name for EDIF and BRIF.
ZEDBDSN Char 44 EDIT (R)
EDREC(W)
Backup data set name for standard edit recovery.
ZEDILMSG Char 240 Any Edit macro Long message text. Corresponds to the first 240
bytes of the message that would be displayed if
the command were entered from the command line
instead of within an edit macro.
ZEDISMSG Char 24 Any Edit macro Short message text. Corresponds to the short
message that would be displayed if the command
were entered from the command line instead of
within an edit macro.
ZEDITCMD Char 8 Any Edit macro The last primary command entered in Edit.
ZEDMSGNO Char 8 Any Edit macro Message ID. Corresponds to the message that would
be displayed if the command were entered from the
command line instead of within an edit macro.
ZEDROW Fixed 4 EDIT (R)
EDREC(W)
Row number of entry in standard edit recovery table.
ZEDSAVE Char 8 Data_changed
EDIT macro
command
END command will save data (SAVE or NOSAVE).
ZEDTDSN Char 44 EDIT (R)
EDREC(W)
Target data set name for standard edit recovery.
ZEDTMCMD Char 8 Any Edit macro The edit command entered that caused an edit macro
to run. Can be the macro name or other name is the
edit DEFINE command was used to define an alias.
ZEDTMEM Char 8 EDIT (R)
EDREC(W)
Target member name (if applicable) for standard edit
recovery.
ZEDTRD Char 6 EDIT (R)
EDREC(W)
Volume serial of target data set for standard edit
recovery.
ZEDUSER Char 1 EDIT (R)
EDREC(W)
User data table extension for standard edit recovery.
Dialog variables
238  z/OS: z/OS ISPF Reference Summary

## Page 265

Table 19. Dialog function pool variables (continued)
Variable
Name Format Length Service (Access) Description
ZEIBSDN Char 54 EDIF (R)
EDIREC(W)
Backup data name for EDIF edit recovery.
ZEIROW Fixed 4 EDIF (R)
EDIREC(W)
Row number of entry in EDIF edit recovery table.
ZEITDSN Char 54 EDIF (R)
EDIREC(W)
Target data name for EDIF edit recovery.
ZEIUSER Char 1 EDIF (R)
EDIREC(W)
User data table extension variable for EDIF edit
recovery.
ZERRALRM Char 3 ALL(W) The value YES if an alarm was specified in the
message definition; otherwise, the value NO. Set
when ISPF services issue a return code of 8 or
greater.
ZERRHM Char 8 ALL(W) The name of a Help panel, if one was specified in the
message definition. Set when ISPF services issue a
return code of 8 or greater.
ZERRLM Char 512 ALL(W) Long-message text in which variables have been
resolved. Set when ISPF services issue a return code
of 8 or greater.
ZERRMSG Char 8 ALL(W) Message ID. Set when ISPF services issue a return
code of 8 or greater.
ZERRSM Char 24 ALL(W) Short-message text in which variables have been
resolved. Set when ISPF services issue a return code
of 8 or greater.
ZGEN Fixed 4 Any Edit macro The generation number for the PDSE member
generation being edited. This is the value at the time
that the edit session started.
ZGENH Fixed 4 Any Edit macro The highest generation number for the PDSE member
being edited. This value is only valid when a previous
generation of a member is being edited and it is the
value at the time that the edit session started.
ZGRPLVL Char 8 LMHIER (W) ISPF table variable that contains the level of this ISPF
library in the controlled hierarchy.
ZGRPNME Char 8 LMHIER (W) ISPF table variable that contains the ISPF library
group name.
ZHIAUTO Char 3 EDIT(R)
EDIF(R)
VIEW(R)
VIIF(R)
(SHARED) ON when AUTO language determination is
enabled, otherwise OFF.
Dialog variables
Chapter 7. Dialog variables  239

## Page 266

Table 19. Dialog function pool variables (continued)
Variable
Name Format Length Service (Access) Description
ZHILANG Char 8 EDIT(R)
EDIF(R)
VIEW(R)
VIIF(R)
(SHARED) Programming Language name.
ZHICOLOR Char 8 EDIT(R)
EDIF(R)
VIEW(R)
VIIF(R)
(SHARED) Coloring indicator as well as DO-IF
LOGIC enablement. OFF indicates HILITE is disabled
regardless of all other variable settings. Values are
ON, OFF, LOGIC, IFLOGIC, and DOLOGIC.
ZHIPAREN Char 3 EDIT(R)
EDIF(R)
VIEW(R)
VIIF(R)
(SHARED) ON when parenthesis matching is enabled,
otherwise OFF.
ZHIFIND Char 3 EDIT(R)
EDIF(R)
VIEW(R)
VIIF(R)
(SHARED) ON when Hilite FIND strings is enabled,
otherwise OFF.
ZHICURSR Char 3 EDIT(R)
EDIF(R)
VIEW(R)
VIIF(R)
(SHARED) ON when Hilite cursor phrase is enabled,
otherwise OFF.
ZLAC Char 2 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
Authorization code of the member.
ZLALIAS Char 8 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
Name of the real member of which this member is an
alias.
ZLAMODE Char 3 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
AMODE of the member.
ZLATTR Char 20 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
Load module attributes. See the z/OS ISPF Services
Guide.
Dialog variables
240  z/OS: z/OS ISPF Reference Summary

## Page 267

Table 19. Dialog function pool variables (continued)
Variable
Name Format Length Service (Access) Description
ZLCDATE Char 8 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Date on which the specified member was created. A
character string in the national format. For example,
yy/mm/dd or mm/dd/yy. If no value exists for this
variable, the PDF component will set the value to
blanks.
ZLC4DATE Char 10 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(W)
Date on which the specified member was created,
in 4-character year format. A character string in the
national format. For example, yyyy/mm/dd or mm/dd/
yyyy. If no value exists for this variable, the PDF
component will set the value to blanks.
ZLCNORC Fixed 4 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Current number of records in the specified member.
A number from 0 to 65 535. If no value exists for
this variable, the PDF component will set the value to
blanks.
ZLGENS Char 3 LMMDISP(R)
LMMFIND(R) 
LMMLIST(R)
ZLGENS=YES, if the member is contained in a PDSE
version 2 data set that is configured for member
generations. If ZLGENS=YES, ZLGMAX, ZLGNEW,
ZLGOLD, and ZLGSAV contain values.
ZLGMAX Char 10 LMMDISP(R)
LMMFIND(R) 
LMMLIST(R)
If ZLGENS=YES, contains the maximum number of
non-current generations that can be saved for the
member.
ZLGNEW Char 10 LMMDISP(R)
LMMFIND(R) 
LMMLIST(R)
If ZLGENS=YES, contains the absolute generation
number of the newest non-current generation that is
saved.
ZLGOLD Char 10 LMMDISP(R)
LMMFIND(R) 
LMMLIST(R)
If ZLGENS=YES, contains the absolute generation
number of the oldest non-current generation that is
saved.
ZLGSAV Char 10 LMMDISP(R)
LMMFIND(R) 
LMMLIST(R)
If ZLGENS=YES, contains the number of non-current
generations that are saved for the member.
ZLINORC Fixed 4 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Number of records in the specified member when it
was first created. A number from 0 to 65 535.
ZLLIB Fixed 4 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
Position of the specified member in the concatenated
data sets. A number from 1 to 4.
Dialog variables
Chapter 7. Dialog variables  241

## Page 268

Table 19. Dialog function pool variables (continued)
Variable
Name Format Length Service (Access) Description
ZLMDATE Char 8 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Date on which the specified member was last
modified. A character string in the national format.
(For example, yy/mm/dd or mm/dd/yy.) If no value
exists for this variable, the PDF component will set
the value to blanks.
ZLM4DATE Char 10 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(W)
Date on which the specified member was last
modified, in 4-character year format. A character
string in the national format. (For example,
yyyy/mm/dd or mm/dd/yyyy.) If no value exists for
this variable, the PDF component will set the value to
blanks.
ZLMEMBER Char 8 LMMDISP(W) Name of the current selected member.
ZLMNORC Fixed 4 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
The number of records that have been modified in the
specified member. A number from 0 to 65 535.
ZLMOD Fixed 4 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Modification level of the specified member. A number
from 0 to 99.
ZLMTIME Char 5 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Time when the specified member was last modified. A
character string in the form hh:mm.
ZLMSEC Char 2 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Seconds value of last modified time.
ZLSSI Char 8 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
SSI (System Status Index) of the load module.
ZLPDSUDA Char 62 LMMDISP(W) A character string containing the contents of the user
data area in the PDS directory entry of the specified
member if the member's statistics are not in PDF
format.
ZLRMODE Char 3 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
RMODE of the member.
ZLSIZE Char 8 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
Load module size (in Hex).
Dialog variables
242  z/OS: z/OS ISPF Reference Summary

## Page 269

Table 19. Dialog function pool variables (continued)
Variable
Name Format Length Service (Access) Description
ZLTTR Char 6 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
TTR of the member.
ZLUSER Char 7 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
User ID of user who last modified the specified
member; the user ID has a maximum length of 7
characters.
• For services that read from this variable, you must
use the ZLUSER8 variable if you want to specify an
8-character user ID.
• For services that write to this variable, when
the user ID is an 8-character value, this variable
contains the value '>7CHARS'; the 8-character user
ID can be obtained from the ZLUSER8 variable.
ZLUSER8 Char 8 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
User ID of the last user to change the given member.
• When 8-character user IDs are enabled on the
system, the user ID has a maximum length of 8
characters.
• When 8-character user IDs are not enabled on the
system, the user ID has a maximum length of 7
characters.
ZLVERS Fixed 4 LMMADD(R)
LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
LMMREP(R)
Version number of the specified member. A number
from 1 to 99. If no value exists for this variable, the
PDF component will set the value to blanks.
ZMEMCNT Char 8 LMMLIST(W) Number of members in the member list.
ZMLCOLS Char 80 LMMDISP(W) A character string that contains the member statistics
column headings that appear on the member list
panel display. This variable is only available for
member list panels.
ZMLCR Fixed 4 LMMDISP(W) The relative number in the member list of the member
that appears at the top of the member list display. Its
range is from 1-99 999. This variable is only available
for member list panels.
ZMLTR Fixed 4 LMMDISP(W) Number of members in the member list. Its range
is from 1-99 999. This variable is only available for
member list panels.
ZMSRTFLD Char 8 ALL(W) Contains the field name used to sort a member
list. Field name corresponds to the title line used
in member list panels, with the exceptions of the
'VV MM' field which is returned as VVMM, and the
attributes field which is returned as ATTRIBUT.
ZSCALIAS Char 1 LMINIT(W) Data set name is an alias ('Y' or 'N').
Dialog variables
Chapter 7. Dialog variables  243

## Page 270

Table 19. Dialog function pool variables (continued)
Variable
Name Format Length Service (Access) Description
ZSCLM Char 1 LMMDISP(W)
LMMFIND(W)
LMMLIST(W)
Last updater of member. 'Y' indicates SCLM was last
updater. 'N' indicates PDF.
ZSCMVOL Char 1 LMINIT(W) Data set name is multivolume ('Y' or 'N').
ZUSERMAC Char 8 EDIT(R) EDIF(R)
VIEW(R) VIIF(R)
Application-wide edit macro.
PDF non-modifiable variables
The following read-only variables are available to PDF component dialogs:
Table 20. Read-only variables available to PDF component dialogs
Variable
Name Format Length Service (Access) Description
ZCUNIT Char 8 none Unit name to be used for temporary allocations. This
variable comes from ISPF configuration table keyword
PDF _DEFAULT_UNIT.
ZCUSIZE Fixed 4 none Number of kilobytes available for use by the edit
UNDO command when running in SETUNDO STORAGE
mode. This variable comes from ISPF configuration
table Keyword UNDO_STORAGE_SIZE. See z/OS ISPF
Edit and Edit Macros for further information.
ZICFPRT Char 3 none ICF indicator. 'YES' - All foreground print requests will
be processed using ICF. 'NO' - ICF will not be used.
This variable comes from ISPF configuration table
keyword PRINT_USING_ICF.
ZPDFREL Char 8 none PDF version number in the form "PDF x.y ". The string
x.y identifies the version and release of z/OS:
• 7.5 means ISPF for z/OS Version 2 Release 5.0
• 7.4 means ISPF for z/OS Version 2 Release 4.0
• 7.3 means ISPF for z/OS Version 2 Release 3.0
• 7.2 means ISPF for z/OS Version 2 Release 2.0
ZSESS Char 8 none This variable contains either 'Y' or 'N' and
comes from the ISPF configuration table keyword
USE_SESSION_MANAGER. See the description of
the general system variable ZSM for additional
information.
1 Length limited only by ISPF restrictions on the length of table extension variables.
Dialog variables
244  z/OS: z/OS ISPF Reference Summary

## Page 271

Table 20. Read-only variables available to PDF component dialogs (continued)
Variable
Name Format Length Service (Access) Description
ZSWIND Char 4 none Sliding window value used by PDF for determining
the century of 2-character years. This variable
comes from ISPF configuration table keyword
YEAR_2000_SLIDING_RULE. Dates less than or equal
to this value are 20xx. Dates greater than this value
are 19xx.
Dialog variables
Chapter 7. Dialog variables  245

## Page 272

Dialog variables
246  z/OS: z/OS ISPF Reference Summary
