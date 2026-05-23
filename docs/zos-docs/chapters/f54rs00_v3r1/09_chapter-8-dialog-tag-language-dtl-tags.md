# Chapter 8. Dialog Tag Language (DTL) tags

Source file: f54rs00_v3r1.md
Start page: 273
Page span: 273-302

## Page 273

Chapter 8. Dialog Tag Language (DTL) tags
The following table is an alphabetic summary of the supported Dialog Tag Language (DTL) tags for z/OS
3.1 ISPF. The table shows the tag, tells whether an end tag is required (Yes) or optional (No), and lists the
tag's attributes (if any) and the tag content (if any) in italics. The table also lists which tags you can nest
within the tag, as well as which tags you can code the tag within.
Table 21. Tag summary
Tag End tag Attributes Nested tags Used within
AB Yes MNEMGEN=YES | NO
ABSEPSTR=ab-separator-string
ABSEPCHAR=ab-separator-character
ABC PANEL
ABC No HELP=NO | YES | help-panel-name |
 *help-message-id | %varname | *%varname
PDCVAR=pdc-variable-name
choice-description-text
COMMENT
M
PDC
PDSEP
SOURCE
AB
ACTION No RUN=internal-command-name | %varname
 PARM=parameters | %varname
 APPLCMD=NO | YES
 TYPE=CMD | PGM | PANEL | EXIT
  NEWAPPL | NEWAPPL=application-id
  NEWWINDOW
  PASSLIB
  NEWPOOL
  SUSPEND
  SCRNAME=screen-name
  NOCHECK
  ADDPOP
  OPT=option | %varname
  MODE=LINE | FSCR
  LANG=APL | CREX
  BARRIER
  NEST
  SETVAR=variable-name
  VALUE=1 | string | %varname
  TOGVAR=variable-name
  VALUE1=0 | string | %varname
  VALUE2=1 | string | %varname
 
CHOICE
PDC
Summary of DTL tags
© Copyright IBM Corp. 1989, 2024 247

## Page 274

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
AREA Yes MARGINW=1 | n
MARGIND=0
INDENT=n
DEPTH=n | *
 EXTEND=OFF | ON | FORCE
 DIV=NONE | BLANK | SOLID | DASH | TEXT
    DIVWIDTH=MAX | MIN
    FORMAT=START | CENTER | END
    TEXT=divider-text
WIDTH=n
DIR=VERT | HORIZ
COMMENT
DA
DIVIDER
DTACOL
DTAFLD
GA
GENERATE
GRPHDR
INFO
LSTFLD
PNLINST
REGION
SELFLD
SOURCE
HELP
PANEL
ASSIGNI No VALUE=test-value
RESULT=assigned-value
  ASSIGNL
ASSIGNL Yes DESTVAR=destination-variable-name ASSIGNI DTAFLD
ATTENTION Yes text
DL
FIG
HP
LINES
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
XMP
LI
LP
P
Summary of DTL tags
248  z/OS: z/OS ISPF Reference Summary

## Page 275

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
ATTR No ATTRCHAR=code
TYPE=DATAIN | DATAOUT | CHAR
INTENS=HIGH | LOW | NON | %varname
CAPS=OFF | ON | IN | OUT | %varname
JUST=ASIS | LEFT | RIGHT | %varname
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
SKIP=OFF | ON | %varname
GE=OFF | ON | %varname
COLOR=WHITE | RED | BLUE | GREEN |
        PINK | YELLOW | TURQ | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
NUMERIC=OFF | ON | %varname
FORMAT=EBCDIC | DBCS | MIX | %varname
OUTLINE=NONE | L | R | O | U | BOX | %varname
PAS=OFF | ON | %varname
CKBOX=OFF | ON | %varname
CUADYN=CEF | EE | LEF | NEF | VOI | LID
  | LI | CH | CT | DT | ET | FP | NT | PIN
  | PT | SAC | SI | SUC | WASL | WT  | %varname
CSRGRP=NO | YES | n
ATTN=OFF | ON | %varname
  DA
BOTINST No COMPACT
instruction-text
HP
PS
RP
PANEL
CAUTION Yes text
DL
FIG
HP
LINES
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
XMP
LI
LP
P
CHDIV No TYPE=NONE | SOLID | DASH | TEXT
GUTTER=1 | n
FORMAT=START | CENTER | END
divider-text
HP
SELFLD
CHOICE
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  249

## Page 276

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
CHECKI No TYPE=
  RANGE
       PARM1=low-bound | %varname
       PARM2=high-bound | %varname
    ALPHA
    CHARS
         PARM1=EQ
         PARM2=character-set
    VALUES
         PARM1=EQ
         PARM2=value-list
    VALUESX
         PARM1=NE
         PARM2=value-list
    BIT
    NAME
    NAMEF
    PICT
         PARM1=EQ
         PARM2=pictstring
    PICTCN
         PARM1=mask-character
         PARM2=field-mask
         PARM3=string
    NUM
    DBCS
    LISTV
         PARM1=EQ
         PARM2=%varlist
    LISTVX
         PARM1=NE
         PARM2=%varlist
    ALPHAB
    LEN
         PARM1=operator | %varname
         PARM2=length | %varname
    EBCDIC
    ENUM
    DSNAME
    DSNAMEF
    DSNAMEFM
    DSNAMEPQ
    DSNAMEQ
    MIX
    HEX
    FILEID
    INCLUDE
         PARM1=IMBLK
         PARM2=ALPHA | ALPHAB | NUM
         PARM3=ALPHA | ALPHAB | NUM
 
  CHECKL
Summary of DTL tags
250  z/OS: z/OS ISPF Reference Summary

## Page 277

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
CHECKI No TYPE=
  IDATE
  STDDATE
  JDATE
  JSTD
  ITIME
  STDTIME
  IPADDR4
CHECKL
CHECKL Yes MSG=message-identifier CHECKI VARCLASS
CHOFLD No DATAVAR=field-data
VARCLASS=variable-class-name
HELP=NO | YES | help-panel-name |
    *help-message-id | %varname | *%varname
USAGE=BOTH | IN | OUT
REQUIRED=NO | YES
 MSG=message-identifier
AUTOTAB=NO | YES
ENTWIDTH=n
FLDSPACE=n
ALIGN=START | CENTER | END
DISPLAY=YES | NO
NOENDATTR
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                  %varname
PSVAR=point-and-shoot-variable | %varname
PSVAL=point-and-shoot-value | %varname
PAS=%varname
EXPAND
ATTRCHANGE=NO | YES | NEW
INIT=initial-value
IMAPNAME=image-name | %varname
 IMAPNAMEP=image-namep | %varname
 PLACE=ABOVE | BELOW | LEFT | RIGHT
               | %varname
ATTRCHAR=code
CAPS=OFF | ON
choice-description-text
ACTION
COMMENT
HP
PS
RP
SOURCE
CHOICE
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  251

## Page 278

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
CHOICE No NAME=choice-name
HELP=NO | YES | help-panel-name |
     *help-message-id | %varname | *%varname
CHECKVAR=variable-name
  MATCH=1 | string
  NOMATCH=0 | string
AUTOTAB=YES | NO
SELCHAR='char(s),n'
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                  %varname
HIDE
HIDEX
UNAVAIL=variable-name
  UNAVAILMAT=1 | string
TRUNC=n
AUTOSEL=YES | NO
choice-description-text
ACTION
CHOFLD
COMMENT
HP
PS
RP
SOURCE
SELFLD
CMD No NAME=internal-command-name
ALTDESCR=command-description
external-command-name
CMDACT
T
CMDTBL
CMDACT No ACTION=
  'SELECT=select-parameters'
  'ALIAS=internal-command-name  parameters'
  PASSTHRU
  SETVERB
  BACKWARD
  CANCEL
  EXIT
  EXHELP
  FKA
  FORWARD
  HELP
  PANELID
  RETRIEVE
  %varname
  application-command
    ASIS
  CMD
Summary of DTL tags
252  z/OS: z/OS ISPF Reference Summary

## Page 279

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
CMDAREA No HELP=NO | YES | help-panel-name |
     *help-message-id | %varname | *%varname
PMTLOC=BEFORE
NOINIT
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                 %varname
NAME=cmdarea-variable-name
ENTWIDTH=n
PMTTEXT=YES | NO
CMDLOC=DEFAULT | ASIS
CMDLEN=DEFAULT | MAX
AUTOTAB=NO | YES
SCROLLVAR=scroll-variable
SCRVHELP=NO | YES | scroll-help-panel-name
   |*scroll-help-message-id | %varname
   | *%varname
SCROLLTAB=NO | YES
SCRCAPS=OFF | ON
PSBUTTON=cmd-pb-text
  PSVAR=point-and-shoot-variable | %varname
  PSVAL=point-and-shoot-value | %varname
  IMAPNAME=image-name | %varname
   IMAPNAMEP=image-namep | %varname
   PLACE=ABOVE | BELOW | LEFT
               | RIGHT | %varname
CAPS=OFF | ON
NOJUMP=OFF | ON
VARDCL=YES | NO
command-prompt-text
HP PANEL
CMDTBL Yes APPLID=application-identifier
SORT=NO | YES
CMD  
COMMENT No TYPE=END | CCSID | PANEL | ATTR | ABCINIT |
     ABCPROC | INIT | REINIT | PROC | HELP |
     PNTS | LIST
comment-text
  ABC
AREA
CHOICE
DA
DTACOL
DTAFLD
HELP
LSTCOL
LSTFLD
LSTGRP
MSGMBR
PANEL
PDC
REGION
SELFLD
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  253

## Page 280

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
COMPOPT No REPLACE | NOREPLACE
SCREEN | DISK
NODBCS | DBCS
NOKANA | KANA
KEYLAPPL=xxxx
NOPANEL | PANEL
NOMSGSUPP | MSGSUPP
NOCUASUPP | CUASUPP
PREP | NOPREP
CUAATTR | NOCUAATTR
NOLSTVIEW | LSTVIEW
STATS | NOSTATS
NOSCRIPT | SCRIPT
NOLISTING | LISTING
NOFORMAT |  FORMAT
NOMSGEXPAND | MSGEXPAND
LOGREPL | NOLOGREPL
LISTREPL | NOLISTREPL
ACTBAR | NOACTBAR
GUI | NOGUI
VERSION | NOVERSION
NOMERGESAREA | MERGESAREA
NODISPLAY | DISPLAY
NODISPLAYW | DISPLAYW
DSNCHK | NODSNCHK
GRAPHIC | NOGRAPHIC
ZVARS | NOZVARS
NODBALIGN | DBALIGN
NOMCOMMENT | MCOMMENT
NOVPADC | PADC
ADD
RESET
national-language
None
COPYR No copyright-text
Summary of DTL tags
254  z/OS: z/OS ISPF Reference Summary

## Page 281

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
DA Yes NAME=varname
EXTEND=OFF | ON | FORCE
LVLINE=variable-name
SCROLL=OFF | ON | CMDLINE
USERMOD=usermod-code | %varname
DATAMOD=datamod-code | %varname
DEPTH=n | *
WIDTH=n
SHADOW=shadow-name
DIV=NONE | BLANK | SOLID | DASH | TEXT
FORMAT=START | CENTER | END
TEXT=divider-text
SCROLLVAR=scroll-variable
SCRVHELP=NO | YES | scroll-help-panel-name
  |*scroll-help-message-id | %varname | *%varname
SCROLLTAB=NO | YES
SCRCAPS=OFF | ON
INITATTR=NT | CT | ET | WT | WASL
HELP=NO | YES | help-panel-name |
     *help-message-id | %varname | *%varname
 
ATTR
COMMENT
SOURCE
AREA
PANEL
REGION
DD No definition - descrip tion 
DL
FIG
HP
LINES
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
XMP
DL
DDHD No definition - descrip tion -header HP
PS
RP
DL
DIVIDER No TYPE=NONE | SOLID | DASH | TEXT
GAP=YES | NO
GUTTER=1 | n
NOENDATTR
FORMAT=START | CENTER | END
divider-text
HP
AREA
DTACOL
PANEL
REGION
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  255

## Page 282

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
DL Yes TSIZE=10 | 'S1, S2,... Sn'
BREAK=NONE | FIT | ALL
COMPACT
NOSKIP
INDENT=n
FORMAT=START | CENTER | END
DIVEND=NO | YES
SPLIT=NO | YES
DD
DDHD
DLDIV
DT
DTHD
DTDIV
DTHDIV
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
NT
PD
WARNING
XMP
DLDIV No TYPE=NONE | SOLID | DASH | TEXT
GAP=YES | NO
GUTTER=1 | n
FORMAT=START | CENTER | END
divider-text
HP DL
DT No FORMAT=START | CENTER | END
NOSKIP
SPLIT=NO | YES
definition - t erm 
DTSEG
HP
PS
RP
DL
DTACOL Yes PMTWIDTH=n | * | **
ENTWIDTH=n
DESWIDTH=n | *
SELWIDTH=n | *
FLDSPACE=n
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                  %varname
PMTFMT=CUA | ISPF | NONE | END
AUTOTAB=NO | YES
ATTRCHANGE=NO | YES | NEW
PMTLOC=BEFORE | ABOVE
DBALIGN=YES | NO | PROMPT | FIELD | FORCE
VARCLASS=variable-class-name
REQUIRED=NO | YES
CAPS=OFF | ON
VARDCL=YES | NO
COMMENT
DIVIDER
DTAFLD
GRPHDR
SELFLD
SOURCE
AREA
PANEL
REGION
Summary of DTL tags
256  z/OS: z/OS ISPF Reference Summary

## Page 283

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
DTAFLD No NAME=field-name
DATAVAR=field-data
VARCLASS=variable-class-name
HELP=NO | YES | help-panel-name |
     *help-message-id | %varname | *%varname
USAGE=BOTH | IN | OUT
REQUIRED=NO | YES
  MSG=message-identifier
AUTOTAB=NO | YES
ENTWIDTH=n
PMTWIDTH=n | * | **
DESWIDTH=n | *
FLDSPACE=n
ALIGN=START | CENTER | END
PMTLOC=BEFORE | ABOVE
DISPLAY=YES | NO
NOENDATTR
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                   %varname
PMTFMT=CUA | ISPF | NONE | END
PSVAR=point-and-shoot-variable | %varname
PSVAL=point-and-shoot-value | %varname
PAS=%varname
CSRGRP=NO | YES | n
EXPAND
FLDWIDTH=n
ATTRCHANGE=NO | YES | NEW
INIT=initial-value
DEPTH=n | %varname
IMAPNAME=image-name | %varname
  IMAPNAMEP=image-namep | %varname
  PLACE=ABOVE | BELOW | LEFT |
        RIGHT | %varname
DBALIGN=YES | NO | PROMPT | FIELD | FORCE
PMTSKIP=NO | YES
DESSKIP=NO | YES
FLDTYPE=CUA | ISPF
COLOR=WHITE | RED | BLUE | GREEN |
    PINK | YELLOW | TURQ | %varname
INTENS=HIGH | LOW | NON | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
ATTRCHAR=code
CAPS=OFF | ON
NOJUMP=OFF | ON
AUTOTYPE=PROJECT | GROUP1 | GROUP2 |
             GROUP3 | GROUP4 | TYPE |
             MEMBER | DSN
AUTOVOL=volser-name
AUTODMEM=YES | NO
VARDCL=YES | NO
prompt-text
ASSIGNL
COMMENT
DTAFLDD
HP
PS
RP
SOURCE
SCRFLD
AREA
DTACOL
PANEL
REGION
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  257

## Page 284

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
DTAFLDD No description HP
PS
RP
DTAFLD
DTDIV No DL
DTHD No definition - t erm -header HP
PS
RP
DL
DTHDIV No DL
DTSEG No DT
FIG Yes FRAME=RULE | NONE
WIDTH=PAGE | COL
NOSKIP
fig ur e - c ont ent 
DL
FIGCAP
HP
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
XMP
ATTENTION
CAUTION
DD
INFO
LI
LP
NT
PD
WARNING
FIGCAP No fig ur e - c ap tion - t e xt HP
PS
RP
FIG
GA No NAME=graphic-area-name
EXTEND=OFF | ON | FORCE
DEPTH=n | *
WIDTH=n
DIV=NONE | BLANK | SOLID | DASH | TEXT
  FORMAT=START | CENTER | END
  TEXT=divider-text
LVLINE=variable-name
  AREA
PANEL
REGION
GENERATE Yes SUBSTITUTE=NO | YES ATTR
COMMENT
SOURCE
AREA
HELP
PANEL
REGION
Summary of DTL tags
258  z/OS: z/OS ISPF Reference Summary

## Page 285

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
GRPHDR No FORMAT=START | CENTER | END | NONE
WIDTH=n
FMTWIDTH=n
INDENT=n
HEADLINE=NO | YES
DIV=NONE | BLANK | SOLID | DASH
DIVLOC=AFTER | BEFORE | BOTH
COMPACT
STRIP
group-heading-text
HP
PS
RP
AREA
DTACOL
PANEL
REGION
HELP Yes NAME=help-panel-name
HELP=hhelp-panel-name | %varname
HELPDEF=helpdef-id
WIDTH=50 | n | FIT
DEPTH=10 | n | FIT
CCSID=n
TUTOR
KEYLIST=key-list-name
  KEYLTYPE=PRIVATE | SHARED
  APPLID=application-id
EXPAND=xy
WINTITLE=window-title
APPTITLE=application-title
MERGESAREA=NO | YES
MSGLINE=YES | NO
IMAPNAME=image-name | %varname
  IMAPROW=n | %varname
  IMAPCOL=n | %varname
ZUP=zup-id
ZCONT=zcont-id
help-panel-title
AREA
COMMENT
DIVIDER
GENERATE
HP
INFO
REGION
SOURCE
TEXTLINE
 
HELPDEF No ID=helpdef-id
HELP=hhelp-panel-name | %varname
WIDTH=n | FIT
DEPTH=n | FIT
CCSID=n
KEYLIST=key-list-name
  KEYLTYPE=PRIVATE | SHARED
  APPLID=application-id
EXPAND=xy
WINTITLE=window-title
APPTITLE=application-title
MERGESAREA=NO | YES
IMAPNAME=image-name | %varname
IMAPROW=n | %varname
IMAPCOL=n | %varname
H1 No COMPACT
heading-text
  INFO
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  259

## Page 286

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
H2/H3/H4 No COMPACT
heading-text
HP
PS
RP
INFO
HP Yes TYPE=ET | CH | CT | FP  | LEF | LI
  | NT | PT | SAC | TEXT | WASL | WT
COLOR=WHITE | RED | BLUE | GREEN |
      PINK | YELLOW | TURQ | %varname
INTENS=HIGH | LOW | NON | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
INTENSE=varname
phrase-to-be-highlighted
  ATTENTION
BOTINST
CAUTION
CHDIV
CHOICE
CMDAREA
DD
DDHD
DIVIDER
DT
DTAFLD
DTAFLDD
DTHD
FIG
FIGCAP
GRPHDR
H2
H3
H4
HELP
LI
LINES
LP
LSTCOL
LSTGRP
NOTE
NT
P
PANEL
PD
PNLINST
PT
SELFLD
TOPINST
WARNING
XMP
Summary of DTL tags
260  z/OS: z/OS ISPF Reference Summary

## Page 287

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
INFO Yes WIDTH=format-width | *
INDENT=n DIVIDER
DL
FIG
Hn
LINES
NOTE
NOTEL
NT
OL
P
PARML
SL
SOURCE
UL
XMP
AREA
HELP
PANEL
REGION
KEYI No KEY=virtual-key
CMD=internal-command-name
CASE=UPPER | MIXED
FKA=NO | YES | LONG | SHORT
PARM=parm-string
FKA-text
  KEYL
KEYL Yes NAME=key-list-name
HELP=help-panel-name
ACTION=UPDATE | DELETE
APPLID=application-id
KEYI  
LI No SPACE=NO | YES
NOSKIP
item-text
ATTENTION
CAUTION
DL
FIG
HP
LINES
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
WARNING
XMP
NOTEL
OL
SL
UL
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  261

## Page 288

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
LINES Yes NOSKIP
text DL
HP
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
XMP
ATTENTION
CAUTION
DD
INFO
LI
LP
NT
PD
WARNING
LIT Yes literal-display-value   XLATI
LP No NOSKIP
implied-paragraph ATTENTION
CAUTION
DL
FIG
HP
LINES
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
WARNING
XMP
NOTEL
OL
SL
UL
Summary of DTL tags
262  z/OS: z/OS ISPF Reference Summary

## Page 289

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
LSTCOL No DATAVAR=column-data
VARCLASS=variable-class-name
HELP=NO | YES | help-panel-name |
    * help-message-id | %varname | *%varname
USAGE=BOTH | IN | OUT
REQUIRED=NO | YES
  MSG=message-id
COLWIDTH=data-width
ALIGN=START | CENTER | END
AUTOTAB=NO | YES
LINE=n
CLEAR
POSITION=n
FORMAT=START | CENTER | END
TEXT=descriptive-text
TEXTLOC=BEFORE | AFTER
TEXTFMT=START | CENTER | END
TEXTLEN=n
TEXTSKIP=NO | YES
NOENDATTR
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                  %varname
PAS=OFF | ON | %varname
CSRGRP=NO | YES | n
ATTRCHANGE=NO | YES | NEW
COLSPACE=n
COLTYPE=CUA | ISPF | EE | VOI | LID
COLOR=WHITE | RED | BLUE | GREEN |
       PINK | YELLOW | TURQ | %varname
INTENS=HIGH | LOW | NON | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
CAPS=OFF | ON
DISPLAY=YES | NO
VARDCL=YES | NO
column-heading
COMMENT
HP
PS
RP
SOURCE
SCRFLD
LSTFLD
LSTGRP
LSTFLD Yes RULES=NONE | HORIZ | VERT | BOTH
ROWS=NOSCAN | SCAN | %varname
DIV=NONE | BLANK | SOLID | DASH | char
SCROLLVAR=scroll-variable
SCRVHELP=NO | YES | scroll-help-panel-name
  |*scroll-help-message-id | %varname | *%varname
SCROLLTAB=NO | YES
SCRCAPS=OFF | ON
ATTRCHANGE=NO | YES | NEW
VARDCL=YES | NO
COMMENTL
STCOL
LSTGRP
LSTVAR
SOURCE
AREA
PANEL
REGION
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  263

## Page 290

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
LSTGRP Yes HEADLINE=NO | YES | DASH
ALIGN=CENTER | START | END
column-group-heading
COMMENT
HP
LSTCOL
LSTGRP
LSTVAR
PS
RP
SOURCE
LSTFLD
LSTGRP
LSTVAR No DATAVAR=variable-model-name
LINE=n
column-heading
COMMENT
HP
PS
RP
SOURCE
LSTFLD
LSTGRP
M No mnemonic-character   ABC
PDC
MSG No SUFFIX=message-suffix-number
HELP=help-panel-name | %varname | *
MSGTYPE=INFO | WARNING | ACTION
        | CRITICAL | %varname
LOCATION=AREA | MODAL | MODAL(L) |
  MODELESS | MODELESS (L) | %varname
DISP=KANA | NOKANA
ALARM=NO | YES | %varname
ABBREV=NONE | KEYWORD | VALUE | BOTH
FORMAT=FLOW | ASIS
SMSG=short-message-text
message-text
VARSUB MSGMBR
MSGMBR Yes NAME=message-member-name
CCSID=n
WIDTH=76 | 68
COMMENTM
SG
 
NOTE No NOSKIP
INDENT=n
TYPE=ET | CH | CT | FP  | LEF | LI
  | NT | PT | SAC | TEXT | WASL | WT
COLOR=WHITE | RED | BLUE | GREEN |
      PINK | YELLOW | TURQ | %varname
INTENS=HIGH | LOW | NON | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
TEXT=alternate-note-heading
note-text
HP
PS
RP
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
PD
WARNING
XMP
Summary of DTL tags
264  z/OS: z/OS ISPF Reference Summary

## Page 291

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
NOTEL Yes COMPACT
NOSKIP
SPACE=NO | YES
INDENT=n
TYPE=ET | CH | CT | FP  | LEF | LI
  | NT | PT | SAC | TEXT | WASL | WT
COLOR=WHITE | RED | BLUE | GREEN |
      PINK | YELLOW | TURQ | %varname
INTENS=HIGH | LOW | NON | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
TEXT=alternate-note-heading
 
LI
LP
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
PD
WARNING
XMP
NT Yes NOSKIP
INDENT=n
TYPE=ET | CH | CT | FP  | LEF | LI
  | NT | PT | SAC | TEXT | WASL | WT
COLOR=WHITE | RED | BLUE | GREEN |
      PINK | YELLOW | TURQ | %varname
INTENS=HIGH | LOW | NON | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
TEXT=alternate-note-heading
note-text
DL
FIG
HP
LINES
OL
P
PARML
PS
RP
SL
UL
XMP
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
PD
WARNING
XMP
OL Yes COMPACT
NOSKIP
SPACE=NO | YES
INDENT=n
TEXT=OL-heading-text
LI
LP
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
NT
PD
WARNING
XMP
P No COMPACT
INTENSE=varname
INDENT=n
OFFSET=n
SPACE=NO | YES
paragraph-text
ATTENTION
CAUTION
HP
PS
RP
WARNING
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
NT
PD
WARNING
XMP
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  265

## Page 292

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
PANDEF No ID=pandef-id
HELP=help-panel-name | %varname
DEPTH=n | FIT
WIDTH=n | FIT | %varname
KEYLIST=key-list-name
  KEYLTYPE=PRIVATE | SHARED
  APPLID=application-id
CCSID=n
WINDOW=YES | NO
WINTITLE=window-title
APPTITLE=application-title
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                 %varname
EXPAND=xy
MERGESAREA=NO | YES
ENTKEYTEXT=enter-key-text
IMAPNAME=image-name | %varname
IMAPROW=n | %varname
IMAPCOL=n | %varname
TMARGIN=n
BMARGIN=n
   
Summary of DTL tags
266  z/OS: z/OS ISPF Reference Summary

## Page 293

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
PANEL Yes NAME=panel-name
HELP=help-panel-name | %varname
PANDEF=pandef-id
DEPTH=22 | n | FIT
WIDTH=76 | n | FIT | %varname
KEYLIST=key-list-name
  KEYLTYPE=PRIVATE | SHARED
  APPLID=application-id
CURSOR=cursor-field
  CSRINDEX=index-value
  CSRPOS=position-value
CCSID=n
MENU
PRIME
TUTOR
WINDOW=YES | NO
WINTITLE=window-title
APPTITLE=application-title
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                 %varname
EXPAND=xy
MSGLINE=YES | NO
TITLINE=YES | NO
CMDLINE=YES | NO
ATTRUSE=NO | YES | ALL
ENDATTR=DEFAULT | TEXT
TYPE=BOTH | GUI | NOGUI
SMSG=short-msg-fieldname
LMSG=long-msg-fieldname
ASIS
ACTBAR
MERGESAREA=NO | YES
PANELSTMT=YES | NO
ENTKEYTEXT=enter-key-text
IMAPNAME=image-name | %varname
  IMAPROW=n | %varname
  IMAPCOL=n | %varname
TMARGIN=n
BMARGIN=n
ERRORCHECK=NO | YES
ZUP=zup-id
ZCONT=zcont-id
AUTONRET=NO | YES
AUTOTCMD=NO | YES | PROC
panel-title-text
AB
AREA
BOTINST
CMDAREA
COMMENT
DA
DIVIDER
DTACOL
DTAFLD
GA
GENERATE
GRPHDR
HP
INFO
LSTFLD
PNLINST
REGION
SELFLD
SOURCE
TEXTLINE
TOPINST
 
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  267

## Page 294

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
PARML Yes TSIZE=10 | 'S1 S2... Sn'
BREAK=ALL | FIT | NONE
COMPACT
SKIP
INDENT=n
FORMAT=START | CENTER | END
DIVEND=NO | YES
SPLIT=NO | YES
PLDIV
PT
PTDIV
PD
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
NT
PD
WARNING
XMP
PD No parameter-description
DL
FIG
HP
LINES
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
XMP
PARML
PDC No HELP=NO | YES | help-panel-name |
     *help-message-id | %varname | *%varname
UNAVAIL=unavail-variable-name
CHECKVAR=check-variable-name
  MATCH=1 | match-string
ACC1=key1
ACC2=key2
ACC3=key3
pull-down-description-text
ACTION
COMMENT
M
SOURCE
ABC
PDSEP No PDC
PLDIV No TYPE=NONE | SOLID | DASH | TEXT
GAP=YES | NO
GUTTER=1 | n
FORMAT=START | CENTER | END
divider-text
HP PARML
PNLINST No COMPACT
instruction-text
HP
PS
RP
AREA
REGION
PANEL
Summary of DTL tags
268  z/OS: z/OS ISPF Reference Summary

## Page 295

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
PS Yes VAR=point-and-shoot-variable-name | %varname
VALUE=point-and-shoot-value | %varname | *
CSRGRP=NO | YES | n
DEPTH=n | %varname
IMAPNAME=image-name | %varname
  IMAPNAMEP=image-namep | %varname
  PLACE=ABOVE | BELOW | LEFT |
        RIGHT | %varname
point-and-shoot-text
ATTENTION
BOTINST
CAUTION
CHOFLD
CHOICE
DD
DDHD
DT
DTAFLD
DTAFLDD
DTHD
FIG
FIGCAP
GRPHDR
H2
H3
H4
LI
LINES
LP
LSTCOL
LSTGRP
NOTE
NT
P
PD
PNLINST
PT
SELFLD
TOPINST
WARNING
XMP
PT No FORMAT=START | CENTER | END
NOSKIP
SPLIT=NO | YES
parameter-term
HP
PS
PTSEG
RP
PARML
PTDIV No PARML
PTSEG No PT
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  269

## Page 296

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
REGION Yes DIR=VERT | HORIZ
INDENT=n
WIDTH=n | *
DEPTH=n | *
 EXTEND=OFF | ON | FORCE
ALIGN=YES | NO
GRPBOX=NO | YES
  GRPWIDTH=n
  GRPBXVAR=variable-name
  GRPBXMAT=1 | string
LOCATION=DEFAULT | TITLE
group-box-title
COMMENT
DA
DIVIDER
DTACOL
DTAFLD
GA
GENERATE
GRPHDR
INFO
LSTFLD
PNLINST
REGION
SELFLD
SOURCE
AREA
HELP
PANEL
REGION
RP Yes HELP= help-panel-name | help-message-id
  | %varname | *%varname
reference-phrase
  ATTENTION
BOTINST
CAUTION
CHOFLD
CHOICE
DD
DDHD
DT
DTAFLD
DTAFLDD
DTHD
FIG
FIGCAP
GRPHDR
H2
H3
H4
LI
LINES
LP
LSTCOL
LSTGRP
NOTE
NT
P
PD
PNLINST
PT
SELFLD
TOPINST
WARNING
XMP
Summary of DTL tags
270  z/OS: z/OS ISPF Reference Summary

## Page 297

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
SCRFLD Yes DISPLEN= n | %varname
INDVAR=ind-var
INDVAL='ind-chars'
LINDVAR=lind-var
LINDVAL='lind-char'
RINDVAR=rind-var
RINDVAL='rind-char'
SINDVAR=sind-var
SINDVAL='sind-chars'
LCOLIND=lcol-var
LCOLDISP= NO | YES
RCOLIND=rcol-var
RCOLDISP= NO | YES
SCALE=scale-var
SCROLL= ON | OFF | %varname
FLDSPOS= BELOW | ABOVE
COMMENT
SOURCE
DTAFLD
LSTCOL
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  271

## Page 298

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
SELFLD Yes NAME=field-name
HELP=NO | YES | help-panel-name |
     *help-message-id | %varname | *%varname
TYPE=SINGLE | MULTI | MENU | MODEL |
    TUTOR
PMTLOC=ABOVE | BEFORE
PMTWIDTH=n | * | **
SELWIDTH=n | *
ENTWIDTH=2 | n | 'e1 e2...en'
REQUIRED=NO | YES
 MSG=message-identifier
FCHOICE=1 | 0
AUTOTAB=YES | NO
DEPTH=n | *
 EXTEND=OFF | ON | FORCE
TRAIL='trail-var-1 trail-var-2 ... trail-var-n'
CHOICECOLS=1 | n
CHOICEDEPTH=n | *
CWIDTHS='w1 w2...wn'
PAD=NULLS | USER | char | %varname
PADC=NULLS | USER | char | %varname
OUTLINE=NONE | L | R | O | U | BOX |
                 %varname
SELMSG=selfld-msg-identifier
SELMSGU=selfld-msg-unavailable
INIT=YES | NO | init-value
VERIFY=YES | NO
REFRESH=YES | NO
SELFMT=START | END
CHKBOX=YES | NO
ZGUI=YES | NO
CSRGRP=NO | YES | n
TSIZE='s1 s2...sn'
LISTTYPE=RADIO | LISTBOX | DDLIST | COMBO
 LISTREF=list-name
 LISTDEPTH=n
DBALIGN=YES | NO | FIELD | FORCE
NOSEL=no-selection-value
SELDEFAULT=x
PMTSKIP=NO | YES
FLDTYPE=CUA | ISPF
COLOR=WHITE | RED | BLUE | GREEN |
      PINK | YELLOW | TURQ | %varname
INTENS=HIGH | LOW | NON | %varname
HILITE=USCORE | BLINK | REVERSE | %varname
SELCHECK=NO |YES
VARDCL=YES | NO
field -pr omp t - t e xt 
CHDIV
CHOICE
COMMENT
HP
PS
RP
SOURCE
AREA
DTACOL
PANEL
REGION
Summary of DTL tags
272  z/OS: z/OS ISPF Reference Summary

## Page 299

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
SL Yes COMPACT
NOSKIP
SPACE=NO | YES
INDENT=n
TEXT='SL-heading-text'
LI
LP
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
NT
PD
WARNING
XMP
SOURCE Yes TYPE=PROC | REINIT | INIT | ABCINIT |
            ABCPROC
text
  ABC
AREA
CHOICE
DA
DTACOL
DTAFLD
HELP
LSTCOL
LSTFLD
LSTGRP
PANEL
PDC
REGION
SELFLD
T No     CMD
TEXTLINE Yes   DTAFLD
TEXTSEG
HELP
PANEL
TEXTSEG No EXPAND=AFTER | BEFORE | BOTH
WIDTH=n
text      
HP TEXTLINE
TOPINST No COMPACT
instruction-text
HP
PS
RP
PANEL
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  273

## Page 300

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
UL Yes COMPACT
NOSKIP
SPACE=NO | YES
INDENT=n
TEXT=UL-heading-text
LI
LP
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
NT
PD
WARNING
XMP
VARCLASS No NAME=variable-class-name
TYPE='CHAR maximum length'
   'DBCS maximum length'
   'MIXED maximum length'
   'ANY maximum length'
   'EBCDIC maximum length'
   '%varname maximum length'
   ITIME
   STDTIME
   IDATE
   STDDATE
   JDATE
   JSTD
   'VMASK maximum-length'
   'NUMERIC total-digits 0 | fractional-digits'
MSG=message-identifier
CHECKL
XLATL
 
VARDCL No NAME=name
VARCLASS=variable-class-name
  VARLIST
VARLIST Yes   VARDCL  
VARSUB No VAR=variable-name   MSG
WARNING Yes text
DL
FIG
HP
LINES
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
XMP
LI
LP
P
Summary of DTL tags
274  z/OS: z/OS ISPF Reference Summary

## Page 301

Table 21. Tag summary (continued)
Tag End tag Attributes Nested tags Used within
XLATI No VALUE=internal-value
displayed-value
LIT XLATL
XLATL Yes FORMAT=NONE | UPPER
 TRUNC=n | char
MSG=message-identifier
 
XLATI VARCLASS
XMP Yes NOSKIP
text DL
HP
NOTE
NOTEL
NT
OL
P
PARML
PS
RP
SL
UL
ATTENTION
CAUTION
DD
FIG
INFO
LI
LINES
LP
NT
PD
WARNING
Summary of DTL tags
Chapter 8. Dialog Tag Language (DTL) tags  275

## Page 302

Summary of DTL tags
276  z/OS: z/OS ISPF Reference Summary
