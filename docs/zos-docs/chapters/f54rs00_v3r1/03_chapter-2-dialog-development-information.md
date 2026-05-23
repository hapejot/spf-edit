# Chapter 2. Dialog development information

Source file: f54rs00_v3r1.md
Start page: 55
Page span: 55-70

## Page 55

Chapter 2. Dialog development information
This topic contains information relevant to dialog developers.
Invoking the ISPF DTL conversion utility
ISPDTLC      (for interactive interface)
OR
ISPDTLC ?    (for help information)
OR
(command syntax)
© Copyright IBM Corp. 1989, 2024 29

## Page 56

ISPDTLC source-filespec (
REPLACE
NOREPLACE
SCREEN
DISK
NODBCS
DBCS
NOKANA
KANA
KEYLAPPL= xxxx
NOPANEL
PANEL
NOMSGSUPP
MSGSUPP
NOCUASUPP
CUASUPP
PREP
NOPREP
CUAATTR
NOCUAATTR
NOLSTVIEW
LSTVIEW
STATS
NOSTATS
NOSCRIPT
SCRIPT
NOLISTING
LISTING
NOFORMAT
FORMAT
NOMSGEXPAND
MSGEXPAND
LOGREPL
NOLOGREPL
LISTREPL
NOLISTREPL
ACTBAR
NOACTBAR
GUI
NOGUI
VERSION
NOVERSION
NOMERGESAREA
MERGESAREA
NODISPLAY
DISPLAY
NODISPLAYW
DISPLAYW
DSNCHK
NODSNCHK
GRAPHIC
NOGRAPHIC
ZVARS
NOZVARS
NODBALIGN
DBALIGN
NOPLEB
PLEB
NOMCOMMENT
MCOMMENT
NOV3PADC
V3PADC
NOGENACC
GENACC
NOZISPFRC
ZISPFRC PROFILE= data-set-name
PROFDDN= ddname |*
MAXFILES=
25
nnn
national-language
Panel definition sections
All parameters on header statements are optional. When preparing a panel header statement, use only
one line.
Panel definition statements and functions
30  z/OS: z/OS ISPF Reference Summary

## Page 57

Coded Character Set Identifier Section
)CCSID NUMBER(  ccsid-number )
Panel Section
)FIELD FIELD( field-name )
LEN( value
field-name
)
IND( value
field-name
) LIND( value
field-name
)
RIND( value
field-name
) SIND( value
field-name
)
LCOL( field-name ) RCOL( field-name ) SCALE(  field-name )
SCROLL( value
field-name
NOLR
)
Attribute Section
)ATTR
DEFAULT ( def1def2def3 ) FORMAT ( EBCDIC
DBCS
MIX
)
OUTLINE (
NONE
L
R
O
U
BOX
)
Panel definition statements and functions
Chapter 2. Dialog development information  31

## Page 58

Action Bar Choice Section
)ABC DESC( choice-description-text )
MNEM( number )
PDC
DESC( choice-description-text ) UNAVAIL(  variable )
MNEM( number )
ACC( key1
+key2 +key3
)
PDSEP(
OFF
ON
)
ACTION RUN( command-name )
PARM( command-parms )
Action Bar Choice Initialization Section
)ABCINIT
Note: Only valid when the Action Bar Choice section is specified.
Action Bar Choice Processing Section
)ABCPROC
Note: Only valid when the Action Bar Choice section is specified.
Body Section
)BODY
CMD( field-name ) SMSG( field-name ) LMSG( field-name )
ASIS WINDOW(  width, depth)
OUTLINE(
L R O U
NONE
BOX )
DEFAULT(  def1def2def3 ) KANA WIDTH(  width)
EXPAND( xy) FORMAT( EBCDIC
DBCS
MIX
)
Panel definition statements and functions
32  z/OS: z/OS ISPF Reference Summary

## Page 59

Note: All keywords must be specified on the same panel line.
Model Section
)MODEL
CLEAR(
,
var-name )
ROWS(
ALL
SCAN )
SFIHDR
Area Section
)AREA name
DEPTH( depth)
Initialization Section
)INIT
Reinitialization Section
)REINIT
Processing Section
)PROC
Field Section
)FIELD FIELD( field-name )
IND( field-name , value) RIND( field-name , value)
LCOL( field-name ) SCALE(  field-name )
LEN( value
field-name
) LIND( field-name , value)
SIND( field-name , value) RCOL( field-name )
SCROLL( value
field-name
)
Panel definition statements and functions
Chapter 2. Dialog development information  33

## Page 60

Help Section
)HELP FIELD( field-name ) PANEL( help-panel-name )
MSG( msg_name )
PASSTHRU
List Section
)LIST list-name VAL( value) CHOICE(  value)
)PNTS FIELD( field-name
ZPS xxyyy
) VAR( variable ) VALUE(  value)
DEPTH( depth) IMAGE( image-name ) IMAGEP(  image-name )
TEXT(' text') PLACE( a,b,l,r )
)END
Panel statements and built-in functions
Attribute section
attrchar
AREA(DYNAMIC)
EXTEND(
OFF
ON SCROLL(
OFF
ON
USERMOD(  usermod-code ) DATAMOD(  datamod-code )
AREA(GRAPHIC)
EXTEND(
OFF
ON )
AREA(SCRL)
EXTEND(
OFF
ON )
ATTN(
OFF
ON )
CAPS( ON
OFF
IN
OUT
)
Panel definition statements and functions
34  z/OS: z/OS ISPF Reference Summary

## Page 61

CKBOX(
OFF
ON )
COLOR(  value) CSRGRP(  x)
COMBO(
OFF
ON
name
)
CUADYN(  value)
DDLIST(
OFF
ON
name
)
DEPTH( d)
FORMAT( EBCDIC
DBCS
MIX
) HILITE( value)
GE(
OFF
ON ) INTENS(
HIGH
LOW
NON
)
JUST( LEFT
RIGHT
ASIS
)
LISTBOX(
OFF
ON
name
)
NOJUMP(
OFF
ON ) NUMERIC(
OFF
ON )
OUTLINE
(NONE)
(BOX)
(
L R O U
)
Panel definition statements and functions
Chapter 2. Dialog development information  35

## Page 62

PAD( char
NULLS
USER
) PADC( char
NULLS
USER
)
PAS(
OFF
ON ) RADIO(
OFF
ON )
REP( char)
SKIP(
OFF
ON )
TYPE( value)
UNAVAIL(
OFF
ON )
WIDTH(  w)
Note: Common User Access (CUA) attribute TYPE values listed below are identified in the section that
follows.
Panel definition statements and functions
36  z/OS: z/OS ISPF Reference Summary

## Page 63

TYPE( AB
ABSL
CEF
CH
CHAR
CT
DATAIN
DATAOUT
DT
EE
ET
FP
LEF
LI
LID
NEF
NT
PIN
PS
PT
RP
SAC
SI
SUC
VOI
WASL
WT
)
TYPE( GRPBOX
SC
)
CUA attribute TYPE values
TYPE Value
Description
AB
Action Bar Unselected Choices
ABSL
Action Bar Separator Line
CEF
Choice Entry Field
CH
Column Heading
CHAR
Character attributes in a dynamic area
Panel definition statements and functions
Chapter 2. Dialog development information  37

## Page 64

CT
Caution Text
DATAIN
Input (unprotected) field in a dynamic area
DATAOUT
Output (protected) field in a dynamic area
DT
Descriptive Text
EE
Error Emphasis
ET
Emphasized Text
FP
Field Prompt
GRPBOX
Group Box
INPUT
Input (unprotected) field
LEF
List Entry Field
LI
List Items
LID
List Item Description
NEF
Normal Entry Field
NT
Normal Text
OUTPUT
Output (protected) field
PIN
Panel Instruction
PS
Point-and-Shoot
PT
Panel Title
RP
Reference Phrase
SAC
Select Available Choices
SC
Selected choice
SI
Scroll Information
SUC
Select Unavailable Choices
TEXT
Text (protected) field
VOI
Variable Output Information
Panel definition statements and functions
38  z/OS: z/OS ISPF Reference Summary

## Page 65

WASL
Work Area Separator Line
WT
Warning Text
Initialization, Reinitialization, and Processing sections
variable =
value
LVLINE( areaname )
PFK( value)
TRANS ( variable
,
value
MSG= message-id
)
TRUNC ( variable , value)
ADDSOSI(  variable )
DELSOSI(  variable )
ONEBYTE(  variable )
TWOBYTE(  variable )
GOTO label
IF ( variable operator
,
value ) ELSE EXIT
PANEXIT ((
,
value ),
PGM , exit-add
, exit-data ,MSG= msgid
LOAD , exit-mod
, exit-data ,MSG= msgid
REXX , rexx-name
, exit-data ,MSG= msgid ,TSOENV
)
REFRESH(
,
field )
Panel definition statements and functions
Chapter 2. Dialog development information  39

## Page 66

*REXX
(
*,
,
value
,( member) ,TSOENV
)
TOG ( mode, fld , &variable
, value1, value2
)
VEDIT ( variable
,MSG= value
)
VGET name-list
ASIS
SHARED
PROFILE
SYMDEF
SYMNAMES(  symname-list )
VPUT name-list
ASIS
SHARED
PROFILE
VER ( variable
,NONBLANK
, keyword
,
value
,MSG= message-id
VSYM name-list
VER keywords
ALPHA      ALPHAB     BIT        DBCS       DSNAME     DSNAMEF
DSNAMEFM   DSNAMEPQ   DSNAMEQ    EBCDIC     ENUM       FILEID
HEX        IDATE      INCLUDE    ITIME      JDATE      JSTD
LEN        LIST       LISTV      LISTVX     LISTX      MIX
NAME       NAMEF      NUM        PICT       PICTCN     RANGE
STDDATE    STDTIME    VSYM
Panel control variables
.
ALARM=NO)
ALARM=YES)
ALARM= blank )
ALARM= null)
ALARM= variable )
Panel definition statements and functions
40  z/OS: z/OS ISPF Reference Summary

## Page 67

.ATTR( field ) = '
,
 keyword ( value) '
.ATTRCHAR(  char) = '
,
 keyword ( value) '
.
AUTOSEL=YES)
AUTOSEL=NO)
.CSRPOS =cursor-position
.CSRROW =table-row-number
.CURSOR =field-name
.HELP =panel-name
.MSG =message-id
.NRET = ON
OFF
DSN
LIB
.PFKEY =1
Notes:
1 Contains function key pressed by user (PF01,PF02, ...,PF24).
.RESP =
ENTER
END
.TRAIL =1
Notes:
1 Contains remainder from TRUNC operation.
.ZVARS ='( name-list )'
Panel definition statements and functions
Chapter 2. Dialog development information  41

## Page 68

Message definitions
msgid
' short message ' .HELP= panel-name
*
NOKANA
KANA
.WINDOW= RESP
NORESP
LRESP
LNORESP
.TYPE= NOTIFY
WARNING
ACTION
CRITICAL
' long message '
+ ' long message '
+
' long message '
+
' long message '
Skeleton control statements
)BLANK
number
)CM comment
)DEFAULT abcdefg
)DO
do-expression
WHILE while-expression UNTIL until-expression
FOREVER
count
)ITERATE )LEAVE
DOT
)ENDDO
)DOT table-name
SCAN
( name-cond-pairs )
)ENDDOT
)IF relational-expression THEN
control-statement
)ELSE
control-statement
)NOP
Message definitions
42  z/OS: z/OS ISPF Reference Summary

## Page 69

)IM skel-name
NT OPT EXT
NOEXT
)REXX
 variable
REXX=
%
 rexxname
,(TSOENV)
)ENDREXX
)SEL relational-expression )ENDSEL
)SET variable = expression
)SETF variable = expression
)TB
 value 1
Notes:
1 Maximum of 16 values.
)TB
 value 1
A
Notes:
1 Maximum of 16 values.
)TBA
 value 1
Notes:
1 Maximum of 16 values.
Skeleton control statements
Chapter 2. Dialog development information  43

## Page 70

Skeleton control statements
44  z/OS: z/OS ISPF Reference Summary
