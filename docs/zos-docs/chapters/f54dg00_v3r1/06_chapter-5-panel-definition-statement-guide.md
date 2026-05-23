# Chapter 5. Panel definition statement guide

Source file: f54dg00_v3r1.md
Start page: 115
Page span: 115-160

## Page 115

Chapter 5. Panel definition statement guide
You can create ISPF panels in one of three ways:
1. Use the Dialog Tag Language (DTL) and ISPF DTL conversion utility only. With DTL, you create a source
file containing DTL tags that define what information you want for each panel. This source file is then
processed through the ISPF conversion utility to produce a preprocessed ISPF panel library member
ready for display.
2. Use DTL and panel definition statements. This option allows you to stop the conversion process at the
ISPF panel definition source level. You can then edit the resulting panel definition source file using any
of the panel definition statements available in this document.
3. Use panel definition statements only. Using panel definition statements, you define panels closely
resembling the finished panel. Each character position in the panel definition corresponds to the same
relative position on the display screen.
To create panels with DTL or to learn how to capture the panel definition source file, refer to the z/OS ISPF
Dialog Tag Language Guide and Reference.
This topic explains how to create panels using the panel definition statements. (This information applies
to the second and third options described above.) Both general overview information on panel definition
and specific information on each panel section is included. The topics are arranged as follows:
• An introduction to the panel definition sections
• General tips and guidelines for formatting panels
• Syntax rules and restrictions for panel definition
• A discussion of each panel section
• Using Z variables as field name placeholders
• Panel processing considerations
• Support for panel user exit routines
• Special requirements for defining menus, table display panels, and panels with dynamic or graphic
areas.
“Example of a CUA panel definition” on page 92 shows an example panel definition which uses CUA
panel-element attributes. See Figure 56 on page 174 for an example panel definition that does not use
CUA panel-element attributes.
Note:
1. You can use the ISPDPTRC command to trace both the execution of panel service calls (DISPLAY,
TBDISPL, and TBQUERY) and the processing that occurs within the Dialog Manager panel code. For
more information, refer to “Panel trace command (ISPDPTRC)” on page 317.
2. When not in TEST mode, the most recently accessed panel definitions are retained in virtual storage
for performance reasons. If you have modified a panel, use TEST mode to ensure that the updated
version of the panel is picked up by ISPF services. See “ISPF test and trace modes” on page 23 for
more information.
Introduction to panel definition sections
Each panel definition consists of a combination of the sections described in Table 8 on page
88. The sections )INEXIT to )PROC, if used, must be in the order listed in this table. The
sections )FIELD, )HELP, )LIST, and )PNTS, if used, can be in any order as long as they appear after the
sections )INEXIT to )PROC). )END must be the last section.
© Copyright IBM Corp. 1980, 2025 87

## Page 116

Table 8. Panel definition  sections
Section Required Description
)INEXIT No Panel input exit section. Identifies a program that is called by ISPF for
each source record read for the panel. The program is passed the panel
source record and can change the record, delete the record, or insert a
new record.
)CCSID No CCSID section. Specifies the Coded Character Set Identifier (CCSID)
used in the panel definition. If used, panel text characters are translated
to the terminal code page for display.
)PANEL No Panel section. Specifies a keylist to be used during the display of the
panel, and identifies where to find the keylist. Specifies that the panel is
to be displayed in CUA mode.
)ATTR No Attribute section. Defines the special characters in the body of the panel
definition that represent attribute (start of field) bytes. You can override
the default ISPF attribute characters.
)ABC No Action bar choice section. Defines a choice in the action bar, its
associated pull-down choices, and the actions to be taken for each pull-
down choice.
)ABCINIT Yes, if )ABC is
specified
Action bar choice initialization section. Specifies processing that is to
occur for an action bar choice before the panel is displayed.
)ABCPROC No Action bar choice processing section. Specifies processing that is to
occur for an action bar when the panel is submitted for processing.
)BODY Yes Body section. Defines the format of the panel as seen by the user and
defines the name of each variable field on the panel.
)MODEL Yes, for table
display
Model section. Defines the format of each row of scrollable data. This
section is required for table display panels. Only one )MODEL section is
allowed per panel.
)AREA No Scrollable area definition section. Defines a scrollable section of the
panel.
)INIT No Initialization section. Specifies the initial processing that is to occur
before the panel is displayed. This section is typically used to define how
variables are to be initialized.
)REINIT No Reinitialization section. Specifies processing that is to occur before a
panel is redisplayed.
)PROC No Processing section. Specifies processing that is to occur after the panel
has been displayed or redisplayed. This section is typically used to
define how variables are to be verified and translated.
)FIELD No Scrollable field section. Defines a field as scrollable, giving it the ability
to display and input a variable that is larger than the display area that
the dialog variable occupies.
)HELP No Field help section. Specifies the help panels to display when help is
requested for a field, list column, action bar choice, or pull-down choice
defined in the panel or reference phrase.
)LIST No List section. Specifies a list to build on the panel.
)PNTS No Point-and-shoot section. Contains an entry for each field on a panel that
has been designated as a point-and-shoot field.
88  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 117

Table 8. Panel definition  sections (continued)
Section Required Description
)END Yes End section. Specifies the end of the panel definition, and consists only
of the )END statement. ISPF ignores any data that appears on lines
following the )END statement.
Guidelines for formatting panels
Consider using the ISPF edit model facilities to help you create panel definitions.
When using Edit to create a panel definition, specify NUMBER OFF to prevent numbers from appearing in
the file. Numbers cause a panel syntax error when you attempt to process the panel definition.
ISPF panel definitions are stored in a panel library and are displayed by means of the SELECT, DISPLAY, or
TBDISPL service. Each panel definition is referred to by its name, which is the same as the member name
in the library.
You can create or change panel definitions by editing directly into the panel library. No compilation or
preprocessing step is required. Use the name of this panel library member as the panel-name parameter
when requesting dialog services, such as DISPLAY and SELECT.
As shown in Figure 34 on page 89, the first three displayable lines below the action bar, if present, in a
panel definition include:
• Panel ID and title area
• System-defined (default) areas for message display
• A command/option field
• A scroll field, if applicable.
You can override the location of the long message area and command field from the ISPF Settings panel.
Figure 34. Sample panel definition  format
Action Bar Line
The action bar line displays the action bar choice-description-text. You can define multiple action
bars for a panel. A separator line should follow the last action bar line. ISPF considers the panel line
following the last action bar choice as part of the action bar area. See “Defining the action bar choice
section” on page 133.
Title Line
The title line should contain a centered title indicating the function being performed or, where
appropriate, information critical to that function. Up to 17 characters at the start of this line can
be overlaid by the system commands SYSNAME, USERID, SCRNAME, or PANELID. Do not use the last
Chapter 5. Panel definition statement guide  89

## Page 118

26 characters of this line to display critical information if messages are to be shown in the default
short message area.
Short Messages
If short messages are used, they should provide a brief indication of either:
• Successful completion of a processing function
• Error conditions, accompanied by audible alarm.
Short messages temporarily overlay information currently displayed at the end of the first line, and
are removed from display on the next interaction. The original information is redisplayed when the
message is removed.
Use short messages consistently throughout the application, or not at all.
For table display, the short message area contains a top-row-displayed indicator, except when
overlaid by a function-requested message. Attribute bytes in the short message The TBDISPL service
automatically generates this indicator, and replaces data that was in the panel definition in that area.
Attribute bytes in the short message area can cause the top-row displayed indicator to be unreadable.
Command/Option Line
The command/option line generally contains the command field. This same field should be used for
option entry on menus. The command field, when the first input field on the panel or when identified
by using the keyword CMD on the header of the panel body section, can be named using any valid
variable name. However, the name ZCMD is generally used.
Cursor placement for viewing a panel differs, depending on the use of the name ZCMD or other
names. When you use ZCMD and cursor placement is not explicitly specified, ISPF skips over a blank
command field to place the cursor on a following input field. When you use a name other than ZCMD,
ISPF does not skip over a blank command field when placing the cursor during display.
Scroll Amount
For table display, Edit, and Browse panels, as well as panels with scrollable dynamic areas, the scroll
amount field should be on the right side of the command line. The scroll amount field must be the first
input field following the command field and must be exactly 4 characters in length. A scroll amount
field is not meaningful for other types of panels and should be omitted from them.
Long Messages
The long message line should generally be left blank, so that long messages do not overlay any
significant information. An exception to this rule might be made in the case of table display panels, to
allow as much scrollable data as possible to fit on the screen. An input field, such as the command
field, should not be located on the same line on which long messages are displayed. The display of
long messages will be superimposed on the input field, and results are unpredictable.
Requirements for specifying message and command line placement
The placement of the command line and long message field at the bottom of a logical screen is a
user-definable option. Placement is controlled by the system variable ZPLACE. You can select or deselect
Command line at bottom on the ISPF Settings panel, and the setting changes the value of ZPLACE.
ZPLACE can also be changed in a dialog.
The value of ZPLACE is stored in the application profile pool. To change the value, use the VPUT statement
in a panel definition, the VPUT service in a dialog function, or the ISPF Settings panel options. None of
these settings takes priority over the others. For example, an ISPF Settings panel selection can change
what a dialog set, and vice versa.
If the panel specifies ASIS on the )BODY statement for a panel, the command and message lines are not
repositioned, even if you specify placement at the BOTTOM. The command line moves only if all of these
are true:
• For primary windows:
1. If the WINDOW(w,d) keyword is specified on the header statement where w is less than the screen
width, then:
90  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 119

a. The keyword ASIS must not be specified on the )BODY header statement.
b. The first character of the command line must be an attribute character.
2. If the WINDOW(w,d) keyword is specified on the header statement where w is equal to the screen
width or the WINDOW keyword is not specified, then:
a. The keyword ASIS must not be specified on the )BODY header statement.
b. The first and last character of the command line must be an attribute character, and one of these
is true:
i) There is an attribute byte in the first column of the line following the command line.
ii) There is an attribute byte in the last column of the line preceding the command line.
3. For pop-up windows, the keyword ASIS is not specified on the )BODY header statement.
Command lines that move in panels designed for primary windows will continue to move if these panels
are displayed in pop-up windows. In addition, command lines in panels created using the DTL and
converted using the ISPF conversion utility will move in both primary and pop-up windows.
If requirement 2b1 is false, but 2b2 is true, ISPF changes the attribute byte in the last column of the line
preceding the command line to match the attribute byte in the last column of the command line. This
gives the same result as 2b1.
For the long message line to be moved, the panel must be designed so that the system default is used
to position the long message. That is, an alternate long message field cannot be specified by the panel
designer using the keyword 'LMSG' on the )BODY header statement.
The long message line is not moved unless the command line is moved, but the command line is moved
regardless of whether the long message field is moved.
Additional suggestions for designing panels
• Avoid cluttered panels. Split "busy" panels into two or more simpler panels that have less information
and are easier to read. Use scrollable areas where appropriate.
• Do not use the last available line in a panel body. For example, if the dialog can be used on 24-line
terminals, limit the body to 23 lines, or less. This is because in split-screen mode the maximum length
of a logical screen is one less than the length of the physical screen.
The PFSHOW|FKA command usually requires a minimum of two lines of a panel for displaying function
key status. Therefore, you should leave the bottom two panel lines blank.
• Place important input fields near the top of the panel and less important fields, especially optional input
fields, further down. In split-screen mode, the bottom of the panel might not be visible unless you
reposition the split line.
• Place important input fields near the top of a scrollable area to minimize the need for scrolling.
• Place the command line near the top of the panel. If the command line is near the bottom and you
enter split-screen mode, the command line cannot be visible on the screen. Therefore, if you do not
have function keys, you might not be able to continue processing the dialog. If, for a particular session,
you will not be entering the split-screen mode, you can use the option 0 (Settings) to specify that the
command line be placed at the bottom of the screen. However, if you want to place the command line at
the bottom, use the ZPLACE system variable.
• Where practical, align fields vertically on a panel, especially input fields. Group related input fields
under a common heading. Minimize the use of multiple input fields on the same line, so that the NEW
LINE key can be used to skip from one input field to the next.
• Use visual indicators for particular field types, such as arrows to indicate input fields, and colons to
indicate variable information that is protected. Examples:
FILE NAME ===> (arrow signals an input field)
EMPLOYEE SERIAL:  123456 (colon signals a protected field)
Chapter 5. Panel definition statement guide  91

## Page 120

To conform to the CUA guidelines, use leader dots and an ending colon for all protected fields, use
leader dots for all input fields, and use ===> for all command areas. For example:
EMPLOYEE NUMBER . :  015723
ADDRESS . . . . . .  6510 Main Street
CITY, STATE . . . .  Imperial, PA
Command  ===>
In any case, be consistent. Arrows, colons, and other visual signals are very confusing if used
inconsistently.
• Use highlighting sparingly. Too many intensified fields result in visual confusion. Do highlight the same
type of information on all panels.
• Use DTL to design CUA-based panels. The conversion process can be stopped at the ISPF panel
definition source level if you need to add additional processing.
Example of a CUA panel definition
This example shows many of the panel sections and panel-element attributes that are available to
support CUA panel definitions.
 )PANEL KEYLIST(ISPSAB,ISP)
 )ATTR  FORMAT(MIX)
  ! TYPE(AB)
  @ TYPE(ABSL)
  # TYPE(PT)
  $ TYPE(CH)
  < TYPE(FP)
  ¬ TYPE(NT)
  _ TYPE(NEF) PADC(_)
  ? TYPE(NEF) PADC(_) CAPS(ON)
  | TYPE(LEF) PADC(_)
  % TYPE(LI)
  ~ TYPE(LI) CAPS(ON)
 )ABC
   DESC('Options')
  PDC DESC('Create ')
  PDC DESC('Change ')
  PDC DESC('Delete ')
  PDC DESC('Browse ')
  PDC DESC('Exit Keylist Utility ')
 )ABCINIT
    .ZVARS=ZPDC
    &ZPDC=' '
    IF (&COPTIONS=CREATE)
       &ZPDC=1
    IF (&COPTIONS=CHANGE)
       &ZPDC=2
    IF (&COPTIONS=DELETE)
       &ZPDC=3
    IF (&COPTIONS=BROWSE)
       &ZPDC=4
    IF (&COPTIONS=EXIT)
       &ZPDC=5
 )ABCPROC
    VER (&ZPDC,LIST,1,2,3,4,5)
    IF (&ZPDC=1)
       &COPTIONS=CREATE
    IF (&ZPDC=2)
       &COPTIONS=CHANGE
    IF (&ZPDC=3)
       &COPTIONS=DELETE
    IF (&ZPDC=4)
       &COPTIONS=BROWSE
    IF (&ZPDC=5)
       &COPTIONS=EXIT
 )ABC
   DESC('Change Keylists')
  PDC DESC('Current panel keylist ')
  PDC DESC('Current dialog keylist ')
  PDC DESC('Specify keylist ')
 )ABCINIT
92  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 121

.ZVARS=ZPDC
    &ZPDC=' '
    IF (&CCHANGE=PANEL)
       &ZPDC=1
    IF (&CCHANGE=DIALOG)
       &ZPDC=2
    IF (&CCHANGE=ANY)
       &ZPDC=3
 )ABCPROC
    VER (&ZPDC,LIST,1,2,3)
    IF (&ZPDC=1)
       &CCHANGE=PANEL
    IF (&ZPDC=2)
       &CCHANGE=DIALOG
    IF (&ZPDC=3)
       &CCHANGE=ANY
 )BODY WINDOW(62,22) CMD(ZCMD)
 ^! Options! Change Keylists^
 @------------------------------------------------------------
 #                Keylist Utility for &kluappl
 ^Command ===>_Z
 ^
 <Enter keylist name?Z       ^<OR   ^
 ^
 ^Select one keylist name from the list below:                ^
 $Select  Keylist   T  -
)MODEL
 |Z^     ~Z       ^%Z^%Z
 )INIT
 .ZVARS = '( ZCMD KEYLISTN S KLUKLNFT SOURCET CURKEYL)'
 .HELP = ISP05800
 &ZCMD = ' '
 .ATTR(S)='JUST(LEFT) '
 .ATTR(KLUKLNFT)='JUST(LEFT) '
 .ATTR(SOURCET)='JUST(LEFT) '
 .ATTR(CURKEYL)='JUST(LEFT) '
 .CURSOR = 'KEYLISTN'
 )PROC
 VER (&KEYLISTN NAME)
 )HELP
 FIELD(ZABC01) PANEL(ISPKH2)
 FIELD(ZPDC0101) PANEL(ISPKH2A)
 FIELD(ZPDC0102) PANEL(ISPKH2B)
 FIELD(ZPDC0103) PANEL(ISPKH2C)
 FIELD(ZPDC0104) PANEL(ISPKH2D)
 FIELD(ZABC02) PANEL(ISPKH3)
 FIELD(ZPDC0201) PANEL(ISPKH3A)
 FIELD(ZPDC0202) PANEL(ISPKH3B)
 FIELD(ZPDC0203) PANEL(ISPKH3C)
 FIELD(KEYLISTN) PANEL(ISPKH1)
 )END                              ^
This panel definition will display the keylist utility panel, SAMPAN, shown in Figure 35 on page 94.
Chapter 5. Panel definition statement guide  93

## Page 122

Figure 35. Sample CUA panel (SAMPAN on ISPKLUP)
Factors that affect a panel's size
The total number of lines allowed in a panel definition depends on the storage size available. Panel
definitions can be 80-160 characters wide. However, the width cannot be greater than that of the physical
screen of the terminal used for the display. The WIDTH keyword in the panel definition determines the
width of a display. If you are defining a panel to be displayed in a pop-up window, use the WINDOW
keyword on the )BODY statement.
Two shared pool system variables, ZSCRMAXD and ZSCRMAXW, contain physical terminal screen depth
and width. These variables cannot be modified. When using terminals for which an alternate size is
available, these variables reflect the configuration that produces the largest screen buffer.
For example, in the case of a 3278-5 (or 3290 set up as a 3278-5), the available screen sizes are 24 x 80
and 27 x 132. Therefore, the values in ZSCRMAXD and ZSCRMAXW are 27 and 132, respectively. For the
3290, these variables contain the sizes of the hardware partition in which ISPF is operating.
Vertically scrollable panels
You can also define more information than can fit on the panel display by defining an AREA(SCRL)
attribute in the panel attribute section and by defining a panel )AREA section. You can scroll each area to
see and interact with the total content defined for the area. See “Defining the area section” on page 138
for further discussion of the )AREA section and scrollable panel areas.
Syntax rules and restrictions for panel definition
For panel definitions:
• All statements, variable names, keywords, and keyword values can be entered in either uppercase
or lowercase. ISPF translates variable names within the panel body or within panel statements to
uppercase before processing them. Values assigned to dialog variables in the panel body or in the
executable sections are stored as entered, in uppercase or lowercase. When symbolic substitution using
a double ampersand is attempted, the variable will not be updated because ISPF makes only one pass
when scanning for variable replacement.
94  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 123

• The command field cannot be longer than 255 characters. This is the first input field on the panel,
unless otherwise specified by using the CMD keyword on the )BODY statement. Fields other than the
command field can exceed 255 characters.
Fields are ended by the attribute character of a following field or by the end of the panel body. A panel
with a large number of variables can cause the literal table to exceed 64K bytes. ISPF issues a message
when this occurs. To proceed, the panel containing the variables must be divided into two or more
panels.
• All header statements, such as )ATTR and )BODY, must be coded starting in column 1. Statements
following the header need not begin in column 1.
• At least one attribute must be defined within the panel )BODY section. If the entire )BODY section is
defined as an AREA, (DYNAMIC, SCRL, ...), then that AREA variable must contain at least one attribute.
For example, if the panel )BODY is defined as a char AREA(DYNAMIC), there must be at least one
attribute variable defined within the Dynamic Area variable char.
• If a section is omitted, the corresponding header statement is also omitted. The )BODY header can be
omitted if all previous sections are omitted, and there is no need to override the default attribute bytes
by using a keyword on the )BODY statement.
• An )END statement is required as the last line of each panel definition. ISPF ignores any data that
appears on lines following the )END statement.
Using blanks and comments
These rules apply to the use of blanks and comment statements:
• In the attribute section, the attribute character and all keywords that follow must be separated by one
or more blanks. At least one keyword must follow the attribute character on the same line. Keywords
can be continued on succeeding lines.
• In the action bar choice, initialization, reinitialization, processing, and help sections, several statements
can occur on the same line, separated by one or more blanks. Statements cannot be split between
lines, except that listed items within parentheses and long strings within quotes can be continued on
succeeding lines (see “Formatting items in lists” on page 95).
• One or more blanks can occur on either side of operators such as an equal sign (=), a not-equal operator
(¬=), greater-than symbol (>), and not-greater-than operator (¬>). Embedded blanks cannot occur in
double-character operators such as the not-equal operator.
For example: ¬ = is invalid.
• One or more blanks can occur on either side of parentheses, except that a blank cannot follow the right
parenthesis that begins a header statement. These statements are all equivalent:
INTENS(LOW)
INTENS (LOW)
INTENS ( LOW )
One or more blanks must follow the closing parenthesis to separate it from the next statement or
keyword.
• Comments can be coded in the action bar choice, attribute, initialization, reinitialization, processing,
ccsid, panel, point-and-shoot, list, and help sections. Comments must be enclosed with the comment
delimiters, /* and */. The comment must be the last item on the line. Additional keywords or statements
that follow the comment on the same line are ignored. A comment cannot be continued on the next line.
For multi-line comments, the comment delimiters must be used on each line.
• Blank lines can occur anywhere within the action bar choice, attribute, initialization, reinitialization,
processing, and help sections.
Formatting items in lists
These rules apply to items in lists:
Chapter 5. Panel definition statement guide  95

## Page 124

• Listed items within parentheses can be separated by commas or one or more blanks. This rule also
applies to paired values within a TRANS statement. For example, these are equivalent:
TRANS (&XYZ 1,A 2,B 3,C MSG=xxxx)
TRANS (&XYZ 1 A 2 B 3 C MSG=xxxx)
TRANS (&XYZ, 1 , A , 2 , B , 3 , C , MSG=xxxx)
• Null items within a list are treated as blank items. For example, these are equivalent:
TRANS (&XXX N,‘ ’,  Y,YES,  *,‘ ’)
TRANS (&XXX N,,     Y,YES,  *,)
• Listed items within parentheses can be continued on one or more lines. For example:
TRANS (&CASE 1,‘THIS IS THE VALUE FOR CASE 1’
             2,‘THIS IS THE VALUE FOR CASE 2’)
 
Literal values within a list can be split between lines by coding a plus sign (+) as the last character on
each line that is to be continued. That is, the plus sign is used as a continuation character. For example:
TRANS (&CASE 1,‘ THIS IS THE VALUE   + 
      FOR CASE 1’  2,‘THIS IS THE   + 
      VALUE FOR CASE 2’)
 
Using variables and literal expressions in text fields and panel sections
These rules apply to literals and variables in text fields and panel sections:
• A literal is a character string not beginning with an ampersand or period. A literal value can be enclosed
in single quotes (‘’). It must be enclosed in single quotes if it begins with a single ampersand or a period,
or if it contains any of these special characters:
Blank < ( + | ) ; ¬ — , > : =
A literal can contain substitutable variables, consisting of a dialog variable name preceded by an
ampersand (&). The name and ampersand are replaced with the value of the variable, with trailing
blanks stripped, before the statement is processed. Trailing blanks are stripped from the variable before
the replacement is done. A double ampersand can be used to specify a literal character string starting
with, or containing, an ampersand.
In the DBCS environment, a mixed EBCDIC/DBCS literal can be specified as follows:
eeee[DBDBDBDB]eeeeee[DBDBDBDBDBDB]
In this example, e represents an EBCDIC character and DB represents a double-byte character. The
brackets [ and ] represent shift-out and shift-in characters, in which DBCS subfields must be enclosed.
These appear as blanks when displayed.
If a mixed literal contains two DBCS subfields, and
– The last character of the first subfield is a shift-in that terminates a DBCS subfield, and
– The first character of the second subfield is a shift-out that begins a DBCS subfield,
the shift-in and shift-out character pair is eliminated.
• In the panel )BODY or )AREA section, a variable can appear within a text field. In the action bar choice,
initialization, reinitialization, processing, and help sections, a variable can appear within a literal value.
The variable name and the preceding ampersand are replaced with the value of the corresponding
dialog variable. Trailing blanks are stripped from the variable before the replacement is done. For
example, if variable V has the value ABC then:
96  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 125

‘F &V G’  yields  ‘F ABC G’
‘F,&V,G’  yields  ‘F,ABC,G’
• A period (.) at the end of a variable name causes concatenation with the character string following the
variable. For example, if &V has the value ABC, then:
‘&V.LMN’  yields  ‘ABCLMN’
• A single ampersand followed by a blank or by a line-end is interpreted as a literal ampersand character,
not the beginning of a substitutable variable. An ampersand followed by a nonblank is interpreted as the
beginning of a substitutable variable.
• A double ampersand can be used to produce a character string starting with, or containing, an
ampersand. The double-character rule also applies to single quotes within literal values, if the literal is
enclosed within delimiting single quotes, and to a period if it immediately follows a variable name. That
is:
 &&  yields  &
 ‘’  yields  ' within delimiting single quotes
 ..  yields  . immediately following a variable name
Note: To add another layer of quotes, you must double the number of quotes used in the previous layer.
For example:
 ‘one o‘‘ne‘ yields one o‘ne
 ‘two t‘‘‘‘wo‘ yields two t‘‘wo
• When variable substitution occurs within a text field, left or right shifting extends to the end of the field,
defined by the occurrence of the next attribute byte. For left shifting, the right-most character in the
field is replicated (shifted in), provided it is a special (non-alphanumeric) character. For example:
%DATA SET NAME: &DSNAME ----------------------%
Assuming that the value of variable DSNAME is greater than 7 characters, the dashes are pushed to the
right, up to the next start of field (the next % in this example). If the value of DSNAME is fewer than 7
characters, additional dashes are pulled in from the right. Fields defined in a scrollable area end at the
end of the line where their definition starts. They will not wrap to the next line.
Validating DBCS strings
ISPF validates DBCS data as follows:
• All DBCS output values are checked to determine whether they contain valid 16-bit DBCS codes. If an
invalid code is found, it is replaced with the hexadecimal value 4195.
• The lengths of DBCS subfields in FORMAT(MIX) fields, and all FORMAT(DBCS) fields, are checked for an
even number of bytes. If an exception occurs, the data is displayed in EBCDIC format.
• Split-screen or a floating command line can result in a DBCS field or subfield being divided. If this
occurs in the middle of a DBCS character, the remainder of the byte is displayed as a blank and is
protected.
• If the division of a DBCS subfield results in no divided DBCS characters, but the shift-in character
is separated, the subfield is displayed as a DBCS field and is protected. However, if a divided DBCS
character results, the remainder of the byte is displayed as a blank and is protected, and the remainder
of the subfield is displayed as a DBCS field and is protected.
• If a DBCS field split results in the division of a DBCS character, the remainder of the byte is displayed as
a blank and is protected.
In all of the previous cases, no message is issued to the user.
Special requirements for defining certain panels
Special requirements exist for defining these types of panels:
Chapter 5. Panel definition statement guide  97

## Page 126

• Menus
• Help tutorials. See Chapter 7, “ISPF help and tutorial panels,” on page 257
• Table displays
• Panels containing dynamic areas
• Panels containing a graphic area.
Defining menus
A menu, also called a selection panel (Figure 36 on page 98), is a special type of panel.
Figure 36. Example of a menu (ISP@MSTR)
The sections that can be used in a menu definition are the same as those that can be used in other panel
definitions. However, a menu requires a processing section in addition to the body section. The processing
section must be in a special format.
Menu definitions are processed by the SELECT service. A menu must have an input field to allow users to
enter selection options. Generally, this is the command field, and is the first input field on the panel. This
field should be named ZCMD to be consistent with the field name used in this guide.
Besides ZCMD, a menu can have input fields to set up dialog variables needed by that application. Any
variables other than ZCMD and ZSEL (or OPT and SEL) that are set from a menu are automatically stored
in the shared variable pool.
Variables from the shared pool, including system variables, can also be displayed on a menu to provide
information to users.
The required processing section must provide for the variable ZCMD to be truncated at the first period and
then translated to a character string. The results must be stored in a variable named ZSEL.
The processing section of a menu is in this general format:
)PROC
  &ZSEL = TRANS( TRUNC(&ZCMD,‘.’)
           value, ‘string’
           value, ‘string’
98  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 127

⋮
           value, ‘string’
                  ‘ ’, ‘ ’
                    *, ‘?’      )
The maximum length for ZSEL is 80 characters. If ZSEL is assigned a string longer than 80 characters, the
string is truncated.
The ZCMD variable is truncated before translation to allow users to bypass one or more intermediate
menus. For example, 1.2 means primary option 1, suboption 2. This is generally called a nested option.
ZCMD is automatically stored, untranslated, as entered. When the SELECT service discovers that variable
ZCMD contains a period, it causes the next lower-level menu to be selected with an initial option of
everything following the first period. As long as the initial option is nonblank, the lower-level menu is
processed in the normal fashion but is not displayed to the user.
Each value is one of the options that can be entered on the menu. Each string contains selection
keywords indicating the action to occur. The selection keywords are:
'PANEL( pnl-name )
NEWAPPL
( appl-id )
PASSLIB
NEWPOOL
ADDPOP SUSPEND SCRNAME
'
'CMD( command )
NEWAPPL
( appl-id )
PASSLIB
NEWPOOL
SUSPEND NOCHECK LANG( APL
CREX
)
MODE( LINE
FSCR
) BARRIER NEST
SCRNAME
'
Chapter 5. Panel definition statement guide  99

## Page 128

'PGM( prog-name )
PARM( parameters )
NEWAPPL
( appl-id )
PASSLIB
NEWPOOL
SUSPEND
NOCHECK MODE( LINE
FSCR
) SCRNAME
'
EXIT
Except for EXIT, each string of keywords must be enclosed in single quotes because it contains
parentheses, and sometimes blanks.
These selection keywords are the same as those that can be specified for the SELECT service:
PANEL( panel-name )
CMD( command )
LANG( APL
CREX
) MODE( LINE
FSCR
)
BARRIER NEST
PGM( program-name )
MODE( LINE
FSCR
)
PARM( parameters )
NEWAPPL
( application-id )
PASSLIB
NEWPOOL
SUSPEND
SCRNAME(  screen_name )
The PANEL keyword, for example, is used to specify the name of a lower-level menu to be displayed.
The CMD and PGM keywords are used to invoke a dialog function coded in a command procedure or
programming language, respectively. NOCHECK, MODE, and EXIT are described in the following topics.
NOCHECK keyword
Normally, nested options are allowed only when each component of the option (up to, but not including
the last component) specifies a lower-level menu. For example, given these ZSEL keywords on a selection
panel definition:
&ZSEL = TRANS (TRUNC(&ZCMD,‘.’)
             1, ‘PANEL(DEF)’
               .
               .
100  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 129

8, ‘PGM(ABC)’
             9, ‘PGM(XYZ)’
A user can enter 1.x as a selection. This selection would be accepted by ISPF. However, if the developer
wants to allow a user to enter a nested option that selects a dialog function, in this case 8.x or 9.x, the
developer specifies the NOCHECK keyword as in this example:
&ZSEL = TRANS (TRUNC(&ZCMD,‘.’)
             1, ‘PANEL(DEF)’
               .
               .
             8, ‘PGM(ABC) NOCHECK’
             9, ‘PGM(XYZ) NOCHECK’
The NOCHECK keyword specifies that normal checking for validity is suspended. It is the responsibility
of the dialog function to interpret the meaning of the lower-level options. To allow this, the remaining
options, those to the right of the first period, are usually passed to the dialog function through any
appropriate variable that has been set equal to the .TRAIL panel control variable in the menu definition.
Example:
&ZSEL = TRANS (TRUNC (&ZCMD, ‘.’)
             1, ‘PANEL(DEF)’
             8, ‘PGM(ABC) NOCHECK’
             9, ‘PGM(XYZ) NOCHECK’
&NEXTOPT = .TRAIL
In this example, variable NEXTOPT contains the remainder of the TRUNC operation. If the user enters
8.5.2, program ABC is invoked and NEXTOPT is set to 5.2. If the user enters 9.7, program XYZ is
invoked and NEXTOPT is set to 7. Since variable NEXTOPT is unknown to the SELECT service, it is
automatically stored in the shared variable pool, where it can be accessed by the dialog function.
When the NOCHECK keyword is specified, a return code of 20 from the dialog function indicates that the
remaining options are invalid. If return code 20 is passed back from the function, ISPF displays an invalid
option.
MODE keyword
You can use the MODE keyword, with either the LINE or the FSCR option, on a SELECT service request to
control whether ISPF enters line mode or full-screen mode when a TSO command or dialog program is
invoked. This eliminates the need to control line mode by prefixing TSO commands with a percent sign.
EXIT keyword
The EXIT keyword, if used, applies only to a primary option menu. It terminates ISPF, using defaults for
list/log data set processing. EXIT need not be enclosed in single quotes.
Blank or invalid options (‘’ or *,‘?’)
If you use a blank ‘ ’ for the value (ZCMD variable is blank), use a blank as the string. This causes the
SELECT service to redisplay the menu. For primary option menus, the menu is redisplayed without a
message. For lower-level menus, an enter option message is displayed if the option field was left blank.
If you use an asterisk (*) for the value, indicating an invalid option was entered, use a question mark (?) as
the string. This causes the SELECT service to redisplay the menu with an invalid option message.
Defining primary option menus
A primary option menu is a selection panel that has special significance in terms of the way the RETURN
command operates, and in terms of the way a jump function, an option number preceded by an equal sign,
works. One type of primary option menu is the master application menu.
The first menu displayed when ISPF is invoked is usually treated as a primary option menu. For example,
if ISPF is invoked with:
Chapter 5. Panel definition statement guide  101

## Page 130

ISPSTART PANEL(XYZTOP)
panel XYZTOP is treated as a primary option menu.
To support an initial command stack being provided in an ISPF variable to a primary option menu
specified using the PANEL parameter, ISPF puts the variable name (or "ZSTART DEFAULT" when the
default cmd_stack_var_name value ZSTART is used) into the ZCMD variable. Then, the )PROC section of
the first primary option menu displayed is executed before the initial display of the panel. The primary
option menu in this scenario must not perform verification of the ZCMD variable in the )PROC section
unless the verification allows for the initial command stack variable name (for example, ZSTART) to be
stored in ZCMD. See “Syntax for issuing the ISPSTART command” on page 8 for more information on
initial command stack processing by a primary option menu.
Similarly, if ISPF is invoked with:
ISPSTART CMD(XYZ) or
ISPSTART PGM(XYZ)
and dialog XYZ subsequently issues:
SELECT PANEL(XYZTOP)
panel XYZTOP is treated as a primary option menu because it is the first invoked menu.
It is possible to write a dialog with no primary option menu by setting the variable ZPRIM to NO on the
first selection panel, panel XYZTOP in this example:
)INIT
  &ZPRIM = NO
 
In general, this approach is not recommended because the RETURN command then causes an immediate
exit from the dialog, which can be confusing to the user.
A dialog can have lower-level (nested) primary option menus. This technique is implemented by setting
variable ZPRIM to YES on a lower-level selection panel:
)INIT
  &ZPRIM = YES
 
Nested primary option menus should be used sparingly, since they can confuse the user. It is
recommended that there be only one primary option menu per major application.
Specifying the next menu to display
ISPF allows the display of menus that are arranged in a hierarchy. The path through the hierarchy is
automatically preserved as the user selects options from the various menus. As the user moves back up
through the hierarchy, the menus are redisplayed in reverse sequence from which they were selected.
While this is the standard mode of operation, it can be overridden. A developer can specify an alternative
mode of menu processing called explicit chain mode. In this mode, the parent menu is specified explicitly
in a system variable named ZPARENT. This variable can be set in a panel definition or in a dialog function:
• From a menu, ZPARENT specifies the name of the next menu to be displayed when the user enters the
END command. A menu that specifies itself as the parent is interpreted as a primary option menu. The
RETURN command stops at that menu.
• From a function, ZPARENT specifies the name of the next menu to be displayed when the function
completes execution. If a function is invoked from another function by the SELECT service, the lower-
level function can set ZPARENT. Upon completion of the lower-level function, the higher-level function
resumes execution. The setting of ZPARENT does not take effect until the higher-level function, the one
originally invoked from a menu, completes execution.
Note:
102  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 131

1. A value can be stored in ZPARENT in a function, or it can be stored in the )INIT, )REINIT, )PROC,
or )BODY section of a panel.
2. The value in ZPARENT takes effect only after display of a selection panel when the user enters the END
command.
3. When the ZPARENT variable is set from a dialog function, it must be explicitly copied to the shared
pool, using VPUT, to ensure that ISPF still has access to it after the function completes.
4. Once the ZPARENT variable is set:
• The hierarchy of menus traversed by the user is not preserved by ISPF.
• The NEWAPPL and NEWPOOL selection keywords are inoperable (ignored) while the dialog is in
explicit chain mode.
5. The ZPARENT variable is automatically reset to blank by ISPF each time it is used. If the dialog does
not continue to set ZPARENT, ISPF resumes normal mode. The hierarchy of menus, if any, up to the
point at which explicit chain mode was started is then restored.
6. Generally, a dialog should use either explicit chain mode or hierarchical chaining, the standard mode.
Chaining should not be mixed. If they are mixed, a function that sets ZPARENT should do so only after
completion of any hierarchical dialog that it invokes. For example, the setting of ZPARENT should be
the last thing the function does before returning control. Otherwise, results are unpredictable.
7. The ZPRIM variable is not applicable and is ignored when operating in explicit chain mode.
Example of a master application menu
A master application menu, named ISP@MSTR (See Figure 36 on page 98), is distributed with ISPF as
part of the panel library. This menu can be used, if desired, to allow selection of the various applications
available at an installation.
If used, the master menu should be the first menu displayed when the user logs on. It can be displayed
automatically by including this command in the user's TSO LOGON procedure:
ISPSTART
PANEL(ISP@MSTR)
When no keywords are specified on the ISPSTART command, PANEL (ISP@MSTR) is assumed.
The master application menu is generated from a DTL source file (Master application menu DTL source (1
of 4)). The menu selections are enabled for point-and-shoot selection.
The master application menu )INIT, )PROC, and )PNTS sections are included in Figure 37 on page 104 to
illustrate some of the special menu statement formats already discussed.
Chapter 5. Panel definition statement guide  103

## Page 132

)INIT
.ZVARS = '(ZCMD ZUSER ZTIME ZTERM ZKEYS ZSCREEN ZLANG ZAPPLID ZENVIR)'
.HELP = ISP00005
&ZPRIM   = YES             /* This is a primary option menu      */
IF (&ZLOGO = 'YES')        /*                              CT@MJC*/
  IF (&ZSPLIT = 'NO')      /* Not in split screen            @L5A*/
    IF (&ZCMD = &Z)        /* No command pending             @L5A*/
      IF (&ZLOGOPAN ¬= 'DONE') /* No logo displayed yet      @L5A*/
        .MSG = ISPLO999    /* Set logo information           @L5A*/
        .RESP = ENTER      /* Simulate enter                 @L5A*/
        &ZLOGOPAN = 'DONE' /*                                @L5A*/
        &ZCLEAN = 'NO'     /*                                @L5A*/
    IF (&ZCMD ¬= &Z)       /* Command pending                @L5A*/
      &ZLOGOPAN = 'DONE'   /*                                @L5A*/
    VPUT (ZLOGOPAN) SHARED /*                                @L5A*/
  IF (&ZSPLIT = 'YES')     /* In split screen                @V5A*/
    &ZLOGOPAN = 'DONE'
)PROC
/* This in a GML based panel generated by ISPDTLC.                    */
/*                                                                    */
/* Make changes by updating the GML source file                       */
/* and reconverting ISP@MSTR.                                         */
&ZCMDWRK = TRUNC(&ZCMD,'.')
&ZTRAIL=.TRAIL
&ZSEL = TRANS (TRUNC (&ZCMD,'.')
  1,'PANEL(ISP@PRIM) SCRNAME(PRIM)'
  X,EXIT
 ' ',' '
   *,'?')
)PNTS
FIELD(ZPS01001) VAR(ZCMD) VAL(1)
FIELD(ZPS01002) VAR(ZCMD) VAL(2)
FIELD(ZPS01003) VAR(ZCMD) VAL(3)
FIELD(ZPS01004) VAR(ZCMD) VAL(4)
FIELD(ZPS01005) VAR(ZCMD) VAL(5)
FIELD(ZPS01006) VAR(ZCMD) VAL(X)
FIELD(ZPS00001) VAR(ZCMD) VAL(END)
)END
/* 5655-042 (C) COPYRIGHT IBM CORP 1982, 2003 */
Figure 37. Master application menu definition 
Master application menu DTL source (1 of 4) shows the DTL source for panel ISP@MSTR. All of the
translatable text is defined with ENTITY tags and is placed at the beginning of the file. Special comments
bordered by a DTL comment line:
 <!-- ############################################ -->
 
identify the places where the source file can be modified and provide an explanation for including
additional options.
Master application menu DTL source (1 of 4)
<:-- ISP@MSTR selection menu -->
<:doctype dm system(
  <:ENTITY ispzmstr system -- common logic file imbed -->
<:-- Start of translatable panel text section                        -->
<:--   text delimited by " is to be translated                       -->
<:--   text should end with '">' as shown.                           -->
<:--     the '">' can be moved to the right for text expansion       -->
 <:-- panel title text follows - maximum length = 74 bytes           -->
  <:ENTITY panel_title
           "ISPF Master Application Menu">
 <:-- choice selection text entries follow                           -->
 <:-- choice text for this panel consists of 2 parts:                -->
 <:--   part 1 - point and shoot - primary description               -->
 <:--   part 2 - additional descriptive text                         -->
 <:--  if combined length of text for part 1 plus part 2 exceeds     -->
 <:--  54 bytes, the part 2 text will be folded into multiple lines  -->
 <:-- part 1 - point and shoot - primary description follows         -->
 <:-- pad short text with blanks, aligning the ending quote mark     -->
104  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 133

<:--     all text strings must be the same length, including blanks -->
 <:-- ############################################################## -->
 <:-- To add options 2, 3, 4, or 5 to this panel:                    -->
 <:--   - Replace the text below for "choice_n_pnts"                 -->
 <:--        (where "n" is the option number)                        -->
 <:--     with the point-and-shoot key identifying option text.      -->
 <:--                                                                -->
 <:-- To add new options to this panel:                              -->
 <:--   - repeat the text below for "choice_n_pnts"                  -->
 <:--        (where "n" is the option number)                        -->
 <:--     for the new option number and add it to the list           -->
 <:--     with the point-and-shoot key identifying option text.      -->
 <:--      for example:                                              -->
 <:--          <:ENTITY choice_6_pnts  "New option 6">               -->
 <:-- ############################################################## -->
   <:ENTITY choice_1_pnts  "Sample 1   ">
   <:ENTITY choice_2_pnts  ".          ">
   <:ENTITY choice_3_pnts  ".          ">
   <:ENTITY choice_4_pnts  ".          ">
   <:ENTITY choice_5_pnts  ".          ">
   <:ENTITY choice_X_pnts  "Exit       ">
 
Master application menu DTL source (2 of 4)
 <:-- part 2 - additional descriptive text                           -->
 <:-- ############################################################## -->
 <:-- To add options 2, 3, 4, or 5 to this panel:                    -->
 <:--   - Replace the text below for "choice_n_text"                 -->
 <:--        (where "n" is the option number)                        -->
 <:--     with the additional option description text.               -->
 <:--                                                                -->
 <:-- To add new options to this panel:                              -->
 <:--   - repeat the text below for "choice_n_text"                  -->
 <:--        (where "n" is the option number)                        -->
 <:--     for the new option number and add it to the list           -->
 <:--     with the additional option description text.               -->
 <:--      for example:                                              -->
 <:--          <:ENTITY choice_6_text "(Description for option 6)  ">-->
 <:-- ############################################################## -->
   <:ENTITY choice_1_text
       "Sample application 1        ">
   <:ENTITY choice_2_text
       "(Description for option 2)  ">
   <:ENTITY choice_3_text
       "(Description for option 3)  ">
   <:ENTITY choice_4_text
       "(Description for option 4)  ">
   <:ENTITY choice_5_text
       "(Description for option 5)  ">
   <:ENTITY choice_X_text
       "Terminate ISPF using list/log defaults">
 <:-- Status area labels          - maximum text length = 10 bytes   -->
  <:ENTITY status_userid  "Userid . :">
  <:ENTITY status_time    "Time . . :">
  <:ENTITY status_term    "Terminal :">
  <:ENTITY status_pfkeys  "Pf keys  :">
  <:ENTITY status_scrnum  "Screen . :">
  <:ENTITY status_lang    "Language :">
  <:ENTITY status_appl    "Appl ID  :">
  <:ENTITY status_rel     "Release  :">
 <:-- Generated panel comments    - maximum text length = 66 bytes   -->
  <:ENTITY panel_cmnt1
          "This in a GML based panel generated by ISPDTLC.">
  <:ENTITY panel_cmnt2
          "                                              ">
  <:ENTITY panel_cmnt3
          "Make changes by updating the GML source file  ">
  <:ENTITY panel_cmnt4
          "and reconverting ISP@MSTR.                    ">
 <:-- panel instruction text line - maximum text length = 78 bytes   -->
 <:-- panel instruction entities will be concatenated                -->
  <:ENTITY panel_instruct_1
    "Enter <ps var=zcmd value=END csrgrp=99>END</ps> ">
  <:ENTITY panel_instruct_2
    "command to terminate application">
Chapter 5. Panel definition statement guide  105

## Page 134

<:-- End of translatable panel text section                          -->
)>         <:-- DO NOT DELETE THIS LINE -->
Master application menu DTL source (3 of 4)
<varclass name=vcc type='char 80'>
<xlatl format=upper>
</xlatl>
<varclass name=vco type='char 7'>
<varlist>
  <vardcl name=zcmd varclass=vcc>
  <vardcl name=zuser varclass=vco>
  <vardcl name=ztime varclass=vco>
</varlist>
<copyr>5694-A01 (C) COPYRIGHT IBM CORP 1982, 2004
<panel name=isp@mstr help=isp00005 padc=user keylist=isrnsab applid=isr
       width=80 depth=24 menu prime window=no>&panel_title;
<cmdarea noinit>
<area depth=8 extend=force width=59 dir=horiz>
  <:-- selection options follow - left side of panel                -->
  <selfld type=menu selwidth=* trail=ztrail fchoice=1 entwidth=1
          tsize=12 selcheck=yes>
    <choice> <ps var=zcmd value=1 csrgrp=99>
        &choice_1_pnts;</ps>
        &choice_1_text;
      <action run=isp@prim type=panel scrname=prim>
 <:-- ############################################################## -->
 <:-- To add options 2, 3, 4, or 5 to this panel:                    -->
 <:--     add a <ACTION> tag provide the selection                   -->
 <:--     information for the generated ZSEL statement.              -->
 <:--                                                                -->
 <:--     <action run=newoptn2 type=panel scrname=opt2>              -->
 <:--       where:                                                   -->
 <:--             run=newoptn2   - provides the name of the panel,   -->
 <:--                              pgm, cmd                          -->
 <:--             type=panel     - provides the selection choice:    -->
 <:--                              panel, pgm, cmd                   -->
 <:--             scrname=opt2   - provides an optional screen name  -->
 <:-- ############################################################## -->
    <choice> <ps var=zcmd value=2 csrgrp=99>
        &choice_2_pnts;</ps>
        &choice_2_text;
    <choice> <ps var=zcmd value=3 csrgrp=99>
        &choice_3_pnts;</ps>
        &choice_3_text;
    <choice> <ps var=zcmd value=4 csrgrp=99>
        &choice_4_pnts;</ps>
        &choice_4_text;
    <choice> <ps var=zcmd value=5 csrgrp=99>
        &choice_5_pnts;</ps>
        &choice_5_text;
 
Master application menu DTL source (4 of 4)
 <:-- ############################################################## -->
 <:-- To add new options to this panel:                              -->
 <:--   - add a new <choice> tag to this list following the          -->
 <:--     pattern of the <choice> tags above.                        -->
 <:--     a new <ACTION> tag is required to provide the selection    -->
 <:--     information for the generated ZSEL statement.              -->
 <:--                                                                -->
 <:--     <choice> <ps var=zcmd value=6 csrgrp=99>                   -->
 <:--         &choice_6_pnts;</ps>                                   -->
 <:--         &choice_6_text;                                        -->
 <:--     <action run=newoptn6 type=panel scrname=opt6>              -->
 <:--       where:                                                   -->
 <:--             run=newoptn6   - provides the name of the panel,   -->
 <:--                              pgm, cmd                          -->
 <:--             type=panel     - provides the selection choice:    -->
 <:--                              panel, pgm, cmd                   -->
 <:--             scrname=opt6   - provides an optional screen name  -->
 <:-- ############################################################## -->
    <choice selchar=X> <ps var=zcmd value=X csrgrp=99>
106  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 135

&choice_X_pnts;</ps>
        &choice_X_text;
      <action run=exit     type=exit>
    <comment type=proc>&panel_cmnt1;
    <comment type=proc>&panel_cmnt2;
    <comment type=proc>&panel_cmnt3;
    <comment type=proc>&panel_cmnt4;
  </selfld>
</area>
  <:-- right side of option menu panel follows, status area  -->
<area dir=horiz>
  <region dir = vert>
    <divider>
    <dtacol pmtwidth=10 entwidth=8>
      <dtafld datavar=ZUSER usage=out>  &status_userid;
      <dtafld datavar=ZTIME usage=out>  &status_time;
      <dtafld datavar=ZTERM usage=out>  &status_term;
      <dtafld datavar=ZKEYS usage=out>  &status_pfkeys;
      <dtafld datavar=ZSCREEN usage=out>&status_scrnum;
      <dtafld datavar=ZLANG  usage=out> &status_lang;
      <dtafld datavar=ZAPPLID usage=out>&status_appl;
      <dtafld datavar=ZENVIR usage=out> &status_rel;
    </dtacol>
  </region
<:-- panel logic file imbed -->
  &ispzmstr;
</area>
<region>
  <info width=78>
    <lines>
&panel_instruct_1;&panel_instruct_2;
    </lines>
    <p>5694-A01 (C) COPYRIGHT IBM CORP 1982, 2003
  </info>
</region>
</panel>
To add a new application to the master menu, copy the ISP@MSTR DTL source file from the GML library to
a private data set. Locate the sections of code within the DTL comment lines:
 <!-- ############################################################## -->
and modify the DTL source code to:
1. Define the point-and-shoot option text
2. Define the option description text
3. Add an <ACTION> tag for each additional option.
See the z/OS ISPF Dialog Tag Language Guide and Reference for a description of Dialog Tag Language
syntax and information about compiling DTL panels.
Compile the modified DTL source file using the ISPDTLC command, and review the generated panel to
confirm that your changes have been processed.
Example of a primary option menu
ISPF primary option menu DTL source (part 1 of 4) shows a primary option menu panel DTL source
file definition. This is the sample primary option menu ISP@PRIM, distributed with ISPF. &ZPRIM=YES
specifies that this panel is a primary option menu.
The primary option menu )INIT, )PROC, and )PNTS sections are included in Figure 38 on page 108 to
illustrate some of the special menu statement formats already discussed.
The initialization section sets the control variable .HELP to the name of a tutorial page to be displayed if
a user enters the HELP command from this menu. It also initializes two system variables that specify the
tutorial table of contents and first index page.
The processing section specifies the action to be taken for each option entered by the user. If option 0 is
selected, program ISPISM is invoked. If option 1 is selected, panel ISPUCMA is displayed; and so on.
Chapter 5. Panel definition statement guide  107

## Page 136

For the tutorial, program ISPTUTOR is invoked and passed a parameter, ISP00000, which ISPTUTOR
interprets as the name of the first panel to be displayed. Panel ISP00000 is the first panel in the tutorial
for ISPF. Other applications should pass the name of the first tutorial page for that application.
)INIT
.ZVARS = '(ZCMD ZUSER ZTIME ZTERM ZKEYS ZSCREEN ZLANG ZAPPLID ZENVIR)'
.HELP = ISP00003
&ZPRIM = YES
&ZHTOP = ISP00003     /* Tutorial table of contents for this appl*/
&ZHINDEX = ISP91000   /* Tutorial index - 1st page for this appl */
VPUT (ZHTOP,ZHINDEX) PROFILE
)PROC
/* This in a GML based panel generated by ISPDTLC.                    */
/*                                                                    */
/* Make changes by updating the GML source file                       */
/* and reconverting ISP@PRIM.                                         */
&ZSEL = TRANS (TRUNC (&ZCMD,'.')
  0,'PGM(ISPISM) SCRNAME(SETTINGS)'
  1,'PANEL(ISPUCMA) SCRNAME(CMDS)'
  2,'PGM(ISPPREP) NEWAPPL SCRNAME(PREP)'
  3,'CMD(ISPDTLC) SCRNAME(DTLC)'
  7,'PGM(ISPYXDR) PARM(&ZTAPPLID) SCRNAME(DTEST) NOCHECK'
  T,'PGM(ISPTUTOR) PARM(ISP00000) SCRNAME(TUTOR)'
  X,EXIT
 ' ',' '
   *,'?' )
&ZTRAIL=TRAIL
)PNTS
FIELD(ZPS01001) VAR(ZCMD) VAL(0)
FIELD(ZPS01002) VAR(ZCMD) VAL(1)
FIELD(ZPS01003) VAR(ZCMD) VAL(2)
FIELD(ZPS01004) VAR(ZCMD) VAL(3)
FIELD(ZPS01005) VAR(ZCMD) VAL(4)
FIELD(ZPS01006) VAR(ZCMD) VAL(5)
FIELD(ZPS01007) VAR(ZCMD) VAL(7)
FIELD(ZPS01008) VAR(ZCMD) VAL(T)
FIELD(ZPS01009) VAR(ZCMD) VAL(X)
FIELD(ZPS00001) VAR(ZCMD) VAL(END)
)END
Figure 38. ISPF primary option menu definition 
ISPF primary option menu DTL source (part 1 of 4) shows the DTL source for panel ISP@PRIM. All of the
translatable text is defined with ENTITY tags and is placed at the beginning of the file. Special comments
bordered by a DTL comment line:
 <!-- ############################################################## -->
identify the places where the source file can be modified and provide an explanation for including
additional options.
ISPF primary option menu DTL source (part 1 of 4)
<!'-- ISP@PRIM selection menu -->
<!'doctype dm system(
  <!'ENTITY ispzprim system -- common logic file embed -->
<!'-- Start of translatable panel text section                       -->
<!'--  text delimited by " is to be translated                       -->
<!'--  text should end with '">' as shown.                           -->
<!'--    the '">' can be moved to the right for text expansion       -->
 <!'-- panel title text follows - maximum length = 74 bytes          -->
  <!'ENTITY panel_title
           "Sample Primary Option Menu">
 <!'-- choice selection text entries follow                          -->
 <!'-- choice text for this panel consists of 2 parts:               -->
 <!'--  part 1 - point and shoot - primary description               -->
 <!'--  part 2 - additional descriptive text                         -->
 <!'-- if combined length of text for part 1 plus part 2 exceeds     -->
 <!'-- 54 bytes, the part 2 text will be folded into multiple lines  -->
 <!'-- part 1 - point and shoot - primary description follows        -->
 <!'-- pad short text with blanks, aligning the ending quote mark    -->
108  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 137

<!'--    all text strings must be the same length, including blanks -->
 <!'-- ############################################################## -->
 <!'-- To add options 4, or 5 to this panel:                         -->
 <!'--  - Replace the text below for "choice_n_pnts"                 -->
 <!'--       (where "n" is the option number)                        -->
 <!'--    with the point-and-shoot key identifying option text.      -->
 <!'--                                                               -->
 <!'-- To add new options to this panel:                             -->
 <!'--  - repeat the text below for "choice_n_pnts"                  -->
 <!'--       (where "n" is the option number)                        -->
 <!'--    for the new option number and add it to the list           -->
 <!'--    with the point-and-shoot key identifying option text.      -->
 <!'--     for example:                                              -->
 <!'--         <!'ENTITY choice_8_pnts "New option 8">               -->
 <!'-- ############################################################## -->
   <!'ENTITY choice_0_pnts "Settings   ">
   <!'ENTITY choice_1_pnts "Commands   ">
   <!'ENTITY choice_2_pnts "ISPPREP    ">
   <!'ENTITY choice_3_pnts "ISPDTLC    ">
   <!'ENTITY choice_4_pnts ".          ">
   <!'ENTITY choice_5_pnts ".          ">
   <!'ENTITY choice_6_pnts ".          ">
   <!'ENTITY choice_7_pnts "Dialog Test">
   <!'ENTITY choice_T_pnts "Tutorial   ">
   <!'ENTITY choice_X_pnts "Exit       ">
 
ISPF primary option menu DTL source (part 2 of 4)
 <!'-- part 2 - additional descriptive text                          -->
 <!'-- ############################################################## -->
 <!'-- To add options 4, or 5 to this panel:                         -->
 <!'--  - Replace the text below for "choice_n_text"                 -->
 <!'--       (where "n" is the option number)                        -->
 <!'--    with the additional option description text.               -->
 <!'--                                                               -->
 <!'-- To add new options to this panel:                             -->
 <!'--  - repeat the text below for "choice_n_text"                  -->
 <!'--       (where "n" is the option number)                        -->
 <!'--    for the new option number and add it to the list           -->
 <!'--    with the additional option description text.               -->
 <!'--     for example:                                              -->
 <!'--         <!'ENTITY choice_8_text "(Description for option 8) ">-->
 <!'-- ############################################################## -->
   <!'ENTITY choice_0_text
       "Terminal and user parameters">
   <!'ENTITY choice_1_text
       "Create/change command table ">
   <!'ENTITY choice_2_text
       "Preprocessed panel utility  ">
   <!'ENTITY choice_3_text
       "ISPF DTL Conversion Utility ">
   <!'ENTITY choice_4_text
       "(Description for option 4)  ">
   <!'ENTITY choice_5_text
       "(Description for option 5)  ">
   <!'ENTITY choice_6_text
       "(Description for option 6)  ">
   <!'ENTITY choice_7_text
       "Perform dialog testing">
   <!'ENTITY choice_T_text
       "Display information about this application">
   <!'ENTITY choice_X_text
       "Terminate ISPF using list/log defaults">
 <!'-- Status area labels         - maximum text length = 10 bytes   -->
  <!'ENTITY status_userid "Userid . :">
  <!'ENTITY status_time   "Time . . :">
  <!'ENTITY status_term   "Terminal :">
  <!'ENTITY status_pfkeys "Pf keys  :">
  <!'ENTITY status_scrnum "Screen . :">
  <!'ENTITY status_lang   "Language :">
  <!'ENTITY status_appl   "Appl ID  :">
  <!'ENTITY status_rel    "Release  :">
 <!'-- Generated panel comments   - maximum text length = 66 bytes   -->
  <!'ENTITY panel_cmnt1
          "This in a GML based panel generated by ISPDTLC.">
  <!'ENTITY panel_cmnt2
          "                                              ">
  <!'ENTITY panel_cmnt3
Chapter 5. Panel definition statement guide  109

## Page 138

"Make changes by updating the GML source file  ">
  <!'ENTITY panel_cmnt4
          "and reconverting ISP@PRIM.                    ">
 <!'-- panel instruction text line - maximum text length = 78 bytes  -->
 <!'-- panel instruction entities will be concatenated               -->
  <!'ENTITY panel_instruct_1
    "Enter <ps var=zcmd value=END csrgrp=99>END</ps> ">
  <!'ENTITY panel_instruct_2
    "command to terminate application">
<!'-- End of translatable panel text section                         -->
)>         <!'-- DO NOT DELETE THIS LINE -->
ISPF primary option menu DTL source (part 3 of 4)
<varclass name=vcc type='char 80'>
<xlatl format=upper>
</xlatl>
<varclass name=vco type='char 7'>
<varlist>
  <vardcl name=zcmd varclass=vcc>
  <vardcl name=zuser varclass=vco>
  <vardcl name=ztime varclass=vco>
</varlist>
<copyr>5655-042 (C) COPYRIGHT IBM CORP 1982, 1996
<panel name=isp@prim help=isp00003 padc=user keylist=isrnsab applid=isr
       width=80 depth=24 menu prime window=no>&panel_title;
<cmdarea noinit>
<area depth=11 extend=force width=59 dir=horiz>
  <!'-- selection options follow - left side of panel               -->
  <selfld type=menu selwidth=* trail=ztrail fchoice=0 entwidth=1
          tsize=12>
    <choice> <ps var=zcmd value=0 csrgrp=99>
        &choice_0_pnts;</ps>
        &choice_0_text;
      <action run=ispism type=pgm scrname=settings>
    <choice> <ps var=zcmd value=1 csrgrp=99>
        &choice_1_pnts;</ps>
        &choice_1_text;
      <action run=ispucma type=panel scrname=cmds>
    <choice> <ps var=zcmd value=2 csrgrp=99>
        &choice_2_pnts;</ps>
        &choice_2_text;
      <action run=ispprep type=pgm newappl scrname=prep>
    <choice> <ps var=zcmd value=3 csrgrp=99>
        &choice_3_pnts;</ps>
        &choice_3_text;
      <action run=ispdtlc type=cmd scrname=dtlc>
 <!'-- ############################################################## -->
 <!'-- To add options 4, or 5 to this panel:                         -->
 <!'--    add a <ACTION> tag provide the selection                   -->
 <!'--    information for the generated ZSEL statement.              -->
 <!'--                                                               -->
 <!'--    <action run=newoptn4 type=panel scrname=opt4>              -->
 <!'--      where:run=                                               -->
 <!'--            run=newoptn4   - provides the name of the panel,   -->
 <!'--                             pgm, cmd                          -->
 <!'--            type=panel     - provides the selection choice:    -->
 <!'--                             panel, pgm, cmd                   -->
 <!'--            scrname=opt4   - provides an optional screen name  -->
 <!'-- ############################################################## -->
    <choice> <ps var=zcmd value=4 csrgrp=99>
        &choice_4_pnts;</ps>
        &choice_4_text;
    <choice> <ps var=zcmd value=5 csrgrp=99>
        &choice_5_pnts;</ps>
        &choice_5_text;
    <choice hide> <ps var=zcmd value=6 csrgrp=99>
        &choice_6_pnts;</ps>
        &choice_6_text;
    <choice> <ps var=zcmd value=7 csrgrp=99>
        &choice_7_pnts;</ps>
        &choice_7_text;
      <action run=ispyxdr type=pgm parm=&amp;ZTAPPLID nocheck scrname=dtest>
110  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 139

ISPF primary option menu DTL source (part 4 of 4)
 <!'-- ############################################################## -->
 <!'-- To add new options to this panel:                             -->
 <!'--  - add a new <choice> tag to this list following the          -->
 <!'--    pattern of the <choice> tags above.                        -->
 <!'--    a new <ACTION> tag is required to provide the selection    -->
 <!'--    information for the generated ZSEL statement.              -->
 <!'--                                                               -->
 <!'--    <choice> <ps var=zcmd value=8 csrgrp=99>                   -->
 <!'--        &choice_8_pnts;</ps>                                   -->
 <!'--        &choice_8_text;                                        -->
 <!'--    <action run=newoptn8 type=panel scrname=opt8>              -->
 <!'--      where:run=                                               -->
 <!'--            run=newoptn8   - provides the name of the panel,   -->
 <!'--                             pgm, cmd                          -->
 <!'--            type=panel     - provides the selection choice:    -->
 <!'--                             panel, pgm, cmd                   -->
 <!'--            scrname=opt8   - provides an optional screen name  -->
 <!'-- ############################################################## -->
    <choice selchar=T> <ps var=zcmd value=T csrgrp=99>
        &choice_T_pnts;</ps>
        &choice_T_text;
      <action run=isptutor type=pgm parm=ISP00000 scrname=tutor>
    <choice selchar=X> <ps var=zcmd value=X csrgrp=99>
        &choice_X_pnts;</ps>
        &choice_X_text;
      <action run=exit     type=exit>
    <comment type=proc>&panel_cmnt1;
    <comment type=proc>&panel_cmnt2;
    <comment type=proc>&panel_cmnt3;
    <comment type=proc>&panel_cmnt4;
  </selfld>
</area>
<!'-- right side of option menu panel follows, status area -->
<area dir=horiz>
  <region dir = vert>
    <divider>
    <dtacol pmtwidth=10 entwidth=8>
      <dtafld datavar=ZUSER usage=out>  &status_userid;
      <dtafld datavar=ZTIME usage=out>  &status_time;
      <dtafld datavar=ZTERM usage=out>  &status_term;
      <dtafld datavar=ZKEYS usage=out>  &status_pfkeys;
      <dtafld datavar=ZSCREEN usage=out>&status_scrnum;
      <dtafld datavar=ZLANG  usage=out> &status_lang;
      <dtafld datavar=ZAPPLID usage=out>&status_appl;
      <dtafld datavar=ZENVIR usage=out> &status_rel;
    </dtacol>
  </region
<!'-- panel logic file embed -->
  &ispzprim;
</area>
<region>
  <info width=78>
    <lines>
&panel_instruct_1;&panel_instruct_2;
    </lines>
<p>5655-042 (C) COPYRIGHT IBM CORP 1982, 1996
  </info>
</region>
</panel>
To add a new application to the primary option menu, copy the ISP@PRIM DTL source file from the GML
library to a private data set. Locate the sections of code within the DTL comment lines:
 <!-- ############################################################## -->
and modify the DTL source code to:
1. Define the point-and-shoot option text
2. Define the option description text
3. Add an <ACTION> tag for each additional option.
Chapter 5. Panel definition statement guide  111

## Page 140

See the z/OS ISPF Dialog Tag Language Guide and Reference for a description of Dialog Tag Language
syntax and information about compiling DTL panels.
Compile the modified DTL source file using the ISPDTLC command, and review the generated panel to
confirm that your changes have been processed.
The required input field, ZCMD, appears in the second line of the panel body. It is followed by a
description of the various options.
This menu also has eight variables within text fields at the upper-right corner of the screen. These
reference system variables from the shared variable pool that display user ID, time, terminal type, number
of function keys, screen number, language, application ID, and ISPF release number.
Defining table display panels
A table display panel is a special panel that is processed by the TBDISPL service. When it is displayed, it
has a fixed (nonscrollable) portion followed by a scrollable table portion. The fixed portion is defined by
the )BODY section in the panel definition. The scrollable portion is defined by the )MODEL section.
The fixed portion contains the command field and usually the scroll amount field. It can also include other
input fields as well as output fields, action bars, text, dynamic areas, scrollable areas, and a graphic area.
The scrollable portion is defined by up to eight model lines. These lines describe how each table row is
to be formatted within the scrollable data area. Attribute characters in the model lines indicate whether
each field is protected or user-modifiable.
If a single model line is specified in the panel definition, each row from the table corresponds to the
format of that line. This results in scrollable data that is in tabular format. For many applications, it may be
useful to define the left-most column in each line as an input field. The application user can enter a code
to be used by the dialog function to determine the particular processing for that row.
If multiple model lines are specified in the panel definition, each row from the table corresponds to
multiple lines on the screen. If desired, a separator line, consisting of blanks or dashes, for example, can
be specified as the first or last model line. This format may be useful for address lists or other repetitive
data in which each unit will not fit on a single line.
Each definition using the model lines on the display is known as a model set.
Table display vocabulary
This topic defines some terms related to table display. Figure 39 on page 113 illustrates those terms that
refer to parts of a TBDISPL display. The two main parts of a TBDISPL display are the fixed portion and
the scrollable portion. The fixed portion contains the command field and commonly a scroll amount field
and a top-row-displayed indicator. The scrollable portion contains the table information and usually, if the
screen is not filled, a bottom-of-data marker.
112  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 141

Command Field              Top-Row-Displayed Indicator
                 |                               |
                 |                               |
  +--------------V-------------------------------V------------+
  |  ------------------- Population Change ------ ROW 4 OF 10 | ----*  Scroll
  |  Command ==>                               Scroll ==> PAGE| <----- Amount
  |                                                           |     |  Field
  |  This table shows selected metropolitan areas which had a |     |
  |  large relative increase in population from 1970 to 1980. |     |  Fixed
  |                                                           |     |  Portion
  |  Metro area       State     Change                        |     |
  |                            (Percent)                      | ----*
  |  Fort Collins     co        +66.0                         | ----*
  |  West Palm Beach  fl        +64.3                         |     |  Scrollable
  |  Fort Lauderdale  fl        +63.6                         |     |  Portion
  |  Bryan            tx        +61.5                         |     |
  |  Reno             nv        +60.0                         |     |
  |  Provo            ut        +58.4                         |     |
  |  McAllen          tx        +56.1                         |     |  Bottom-
  |  ******************** BOTTOM OF DATA ******************** <-------- of-Data
  |                                                           | ----*  Marker
  +-----------------------------------------------------------+
Figure 39. Parts of a TBDISPL display
auto-selection
The process by which the row specified in the CSRROW parameter or .CSRROW control variable is
selected, even if the user did not explicitly select that row by modifying the corresponding model set
displayed on the screen.
Relevant concepts include: selected row, user-selection, CSRROW parameter, .CSRROW control
variable, AUTOSEL parameter, and .AUTOSEL control variable.
bottom-of-data marker
The low-intensity text that appears after the last displayed row in the last page of data in a TBDISPL
display. If there are no displayed rows, this marker will be the only information displayed in the
scrollable portion. The text BOTTOM OF DATA, with asterisks on each side, appears after the last row
on a table display. The dialog can define an alternate marker by assigning text to ZTDMARK.
ISPF uses the + default attribute character for the bottom-of-data marker. The default attribute
characters are %, +, and _. For a description of the default attribute characters see “Using default
attribute characters” on page 144. You can change the default attribute characters by using the
DEFAULT keyword on either the )ATTR or )BODY head statement. For example: DEFAULT(abc) where
a, b, and c are the 3 characters that take the place of %, +, and _, respectively. The default attribute
characters are position-sensitive. Thus, if you change the default character "b" in the second position
of the DEFAULT keyword parameter (ISPF's default character is +), it must maintain the characteristics
of TYPE(TEXT), INTENS(LOW), COLOR(BLUE) for the bottom-of-data marker to display correctly.
Relevant concepts include: system variable ZTDMARK.
command field
A required field in the fixed portion of a TBDISPL display where commands are entered. The command
field can be identified in the panel definition through use of the CMD parameter on the )BODY
statement. If the CMD parameter is not specified, the first input field is assumed to be the command
field.
Relevant concepts include: system commands, application commands, and function commands.
dynamic expansion
The process by which a table being displayed is expanded as needed if a user scrolls beyond the top
or bottom of data contained in the table at the time of the scroll request.
Relevant concepts include: scrolling and TBDISPL.
Chapter 5. Panel definition statement guide  113

## Page 142

fixed portion
The nonscrollable portion of a TBDISPL display. That is, the part of the display that is not affected by
the UP or DOWN commands. Note that both the fixed and scrollable portions are unaffected by the
LEFT and RIGHT commands. The fixed portion is defined by the )BODY section of the panel definition.
Relevant concepts include: scrollable portion, )BODY section.
model lines
The lines in the )MODEL section of a TBDISPL panel definition, which form a template, or model, for
the scrollable portion of a TBDISPL display.
Relevant concepts include: )MODEL section, model set, scrollable portion.
model set
The lines in the scrollable portion of a TBDISPL display that correspond to a particular table row.
Model sets are created by ISPF by replicating the model lines in the panel definition and then filling
in the fields with variable and table row information. Each model set on the display corresponds to a
table row. If there are n model lines, where n can be from 1 to 8, then each model set is made up of n
lines on the display.
Relevant concepts include: model lines, and scrollable portion.
pending END request
The situation that exists when a user has selected more than one row and has entered the END or
RETURN command. The dialog can choose to ignore the selected rows, or it can process the selected
rows in a TBDISPL series. In the latter case, each call of TBDISPL results in a return code of 8. When
all the selected rows have been processed, the dialog commonly honors the pending END request by
not invoking the TBDISPL service again.
Relevant concepts include: TBDISPL series, pending scroll request, and pending selected row.
pending scroll request
The situation that exists when a user has selected one or more rows, and has entered the UP or
DOWN command. After the dialog has processed all the selected rows, it can invoke TBDISPL without
the PANEL and MSG parameters to display the table and panel and have the pending scroll request
honored. A pending scroll request can also exist when a user has issued the UP or DOWN command
and the dialog is dynamically building the table. After adding the rows needed to satisfy the scroll
request, the dialog can invoke TBDISPL without the PANEL or MSG parameters and ISPF will honor
the pending scroll request.
Relevant concepts include: TBDISPL series, pending END request, pending selected row, and
controlling the top-row-displayed.
pending selected rows
Occurs when a user has selected more than one row in a single interaction. Upon return from the
TBDISPL display, the CRP is positioned to the first of the selected rows. The other rows, which remain
to be processed, are the pending selected rows.
Relevant concepts include: selected row, TBDISPL series, pending END request, pending scroll
request, system variable ZTDSELS.
scroll amount field
An optional field in the fixed portion of a TBDISPL display where scroll amounts, for example, PAGE,
HALF, or 10, are entered. If the input field immediately following the command field is exactly 4
characters long, it is assumed to be the scroll amount field.
Relevant concepts include: scrolling, and system variables ZSCROLLA, ZSCROLLN, and ZSCROLNL.
scrollable portion
The part of a TBDISPL display defined by the )MODEL section of the panel definition and made up of
model sets. It contains the ISPF table information. It is affected by the UP and DOWN commands.
Relevant concepts include: fixed portion, )MODEL section, model lines, and model sets.
114  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 143

select field
A field in the scrollable portion where line commands are entered. For example, a d entered into
the select field of a model set can indicate that the corresponding table row is to be deleted.
TBDISPL does not o fficial l y  identify any field as a select field. It is up to the dialog to determine
the characteristics or meaning of a select field.
Relevant concepts include: line commands, scrollable portion, model set, selected row, and user-
selection.
selected row
A row in an ISPF table that has been auto-selected or user-selected.
Relevant concepts include: auto-selection, user-selection, model set, pending selected row, system
variable ZTDSELS, POSITION parameter, and ROWID parameter.
TBDISPL series
A call of the TBDISPL service that results in a display where the user selects more than one row,
followed by calls of the TBDISPL service without the PANEL and MSG parameters to process the
pending selected rows.
Relevant concepts include: pending selected rows, pending END request, pending scroll request, and
system variable ZTDSELS.
top-row-displayed indicator
There are three possible texts for the top-row-displayed indicator:
• ROW x OF y
x is the current row pointer of the top row displayed. y is the total number of rows in the table.
• ROW x TO z OF y
x is the current row pointer of the top row displayed. z is the row pointer of the last visible table row.
z is calculated as the current row pointer of the top row displayed plus the number of lines displayed
minus one. y is the total number of rows in the table.
• ROW x FROM y
x is the row pointer of the table row that has met the criteria of the SCAN. y is the total number of
rows in the table.
The text used for the top-row-displayed indicator is determined by the CUA mode selected and by
whether ROWS is set to ALL or SCAN in the panel model section. Table 9 on page 115 is a summary
of the CUA mode and ROWS(ALL) or ROWS(SCAN) combinations and the resulting top-row-displayed
messages. CUA mode of YES is determined by the presence of a panel statement or by specifying CUA
MODE=YES on Option 0.
Table 9. Text for top-row-displayed indicator
CUA Mode ROWS Top-Row-Displayed Message Message ID
YES ALL ROW x TO z OF y ISPZZ102
YES SCAN ROW x FROM y ISPZZ103
NO ALL ROW x OF y ISPZZ100
NO SCAN ROW x OF y ISPZZ100
The message text appears right-justified on the top line of the display, or just below the action bar
separator line if an action bar is defined. Your dialog can define an alternate indicator if you assign
a message ID to ZTDMSG. TBDISPL invokes the GETMSG to get the short and long message text. If
a short message is found, it is used as the top-row-displayed indicator; if not, the long message text
is used. In either case, any variables in the messages are substituted with their current values. If
ZTDMSG does not exist, the long form of message ISPZZ100, ISPZZ102, or ISPZZ103 is used.
Chapter 5. Panel definition statement guide  115

## Page 144

If the model section for a table contains more than one line, it is possible that the entire model section
will not fit on the screen. In this case, the last rows of the table area are left blank. A partial model
section is not displayed. The only way to display a partial model section is if you request your function
keys to appear over your table display, or if you split your screen over your table display.
When you specify ROWS(SCAN) in your panel model section, ISPF finds only enough rows to fill the
display, thus providing a performance boost. Therefore, you cannot know the entire number of table
rows that meet your search criteria without scrolling through the complete table.
When a table is being built dynamically to satisfy scroll requests, you can make the top-row-displayed
indicator reflect the positioning in the logical table instead of the physical table. See the description of
ZTDLTOP and ZTDLROWS in z/OS ISPF Services Guide.
Relevant concepts include: system variables ZTDMSG, ZTDTOP, ZTDLTOP, ZTDROWS, and ZTDLROWS;
messages ISPZZ100, ISPZZ101, ISPZZ102, and ISPZZ103; and controlling the top-row-displayed.
user-selection
The process by which ISPF table rows are chosen or selected for processing by the user modifying the
corresponding model sets on the display. A user modifies  a model set by entering data into that model
set. Overtyping a model set with the same data does not cause the row to be selected.
Relevant concepts include: auto-selection, selected row, model set, and system variable ZTDSELS.
Requirements for attribute section
Attribute characters can be defined for use in the panel body and the model lines. In the )BODY
section, any attribute except EXTEND(ON) and SCROLL(ON) can be associated with any field or area.
In the )MODEL section, any attribute except those associated with dynamic and graphic areas can be used
with any field. That is, the attributes AREA, EXTEND, SCROLL, USERMOD, and DATAMOD are not allowed
in model lines.
Input and output fields default to CAPS(ON) and JUST(LEFT), in the )BODY section, but they default to
CAPS(OFF) and JUST(ASIS) in the )MODEL section.
An attribute section is required if the model line contains output fields. There is no default attribute
character for output fields.
Requirements for body section
The panel body section is required. It contains the nonscrollable data, which is the command field and,
commonly, the scroll amount field. The rules for their definition are:
Command field (required)
This field must not be longer than 255 characters.
The command field can have any desired name. The position of the command field can be specified
through use of the CMD parameter on the )BODY statement. If the CMD parameter is not specified, the
first input field is assumed to be the command field.
The command field is used, as on other types of panels, to enter ISPF commands and application-
defined commands, if any. Any commands entered in this field that are not recognized by ISPF are
automatically stored in the corresponding dialog variable. Upon return from TBDISPL, the dialog
function can interpret this field and take appropriate action. The ZCMD field is cleared each time
a TBDISPL request is received with the MSG or PANEL parameter. If the TBDISPL request contains
a table name and no other parameters, the ZCMD field contains what was entered on the previous
TBDISPL.
The ISPF commands are system commands, while the application-defined commands are application
commands. The commands processed by the dialog function are function commands.
Scroll amount field (optional)
If the input field immediately following the command field is exactly 4 characters long, it is assumed
to be the scroll amount field.
116  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 145

The field can have any desired name. Its initial value can be set in the )INIT section of the panel
definition to any valid scroll amount.
If no scroll amount field is specified, the system variable ZSCROLLD, which can be set by a dialog, is
used to determine the default scroll amount. If there is no scroll amount field and ZSCROLLD has not
been set, PAGE is assumed.
When a user enters a scroll request, variables ZSCROLLA, ZSCROLLN, and ZSCROLNL are set.
ZSCROLLA contains the value of the scroll amount field (MAX, CSR, for example). ZSCROLLN and
ZSCROLNL contain the number of lines or columns to scroll, computed from the value in the scroll
amount field or entered as a scroll number. For example, if a dialog is in split-screen mode and if 12
lines are currently visible and a user requests DOWN HALF, ZSCROLLN and ZSCROLNL each contain
a value of '6'. ZSCROLLN can support values up to '9999'. If a scroll number greater than '9999' is
specified ZSCROLLN is set to a value of '9999'. ZSCROLNL can support values up to '9999999'. The
system variable ZVERB contains the scroll direction, DOWN in this case. If ZSCROLLA has a value of
MAX, the values of ZSCROLLN and ZSCROLNL are not meaningful. 
These can appear in the )BODY section:
• Action bars
• Text
• Variables within text; for example, &XYZ
• Input fields
• Output fields
• Dynamic areas
• Scrollable areas
• Graphic areas.
Note:
1. Only one extendable area is allowed on a panel. This includes dynamic, scrollable, and graphic areas.
Requirements for model section
The panel body must be followed by a model section. This section begins with a )MODEL header
statement and is immediately followed by one or more model lines.
The )MODEL header statement must begin in column 1. These optional keywords can be specified on this
header:
• CLEAR(var-name,var-name …)
• ROWS(ALL|SCAN).
• SFIHDR
The CLEAR keyword identifies the dialog variable names within the model lines that are to be cleared to
blank before each row in the table is read. For example, you can use this to clear the values of extension
variables. Because extension variables might not exist in all the rows that are displayed, clearing them
ensures that previous values are not repeated in other lines to which they do not apply.
CLEAR is not processed when the EXIT panel statement is actioned. Use a GOTO to jump to a label before
the next panel section to bypass panel code and have CLEAR processing occur.
The ROWS keyword indicates whether all rows from the table are to be displayed, or whether the table
is to be scanned for certain rows to be displayed. The default is ROWS(ALL), which causes all rows to
be displayed. If ROWS(SCAN) is specified, the dialog must invoke the TBSARG service before invoking
TBDISPL. The search argument set up by the TBSARG service is used to scan the table. Only rows that
match the search argument are displayed.
Chapter 5. Panel definition statement guide  117

## Page 146

The SFIHDR keyword is used when a variable model line defines scrollable fields and scroll indicators
are required for the scrollable fields. SFIHDR indicates that the first variable model line defines scroll
indicator fields for scrollable fields that are defined on subsequent variable model lines. For an example
of using the SFIHDR keyword, see "Example—scroll indicator field in first variable model line" in z/OS ISPF
Services Guide.
One or more model lines must appear following the )MODEL header statement. A maximum of eight
model lines is allowed. Any attribute except those associated with dynamic, graphic, or scrollable areas
(AREA, EXTEND, SCROLL, USERMOD, and DATAMOD) can be used with any fields in the model lines. These
can appear in the )MODEL section:
• Text
• Variable model lines
• Input fields
• Output fields.
These cannot appear in the )MODEL section:
• Action bars
• Variables within text
• Dynamic areas
• Graphic areas
• Scrollable areas.
Typically, the first field within the model lines specifies the dialog variable into which a selection code,
entered by a user, will be stored. All remaining names correspond to columns in the table. However,
this arrangement is not required. Any name may or may not correspond to a column in the table, and a
selection code field need not be specified.
Text fields can be specified in the model line. A text attribute character can appear by itself to terminate
the preceding input or output field. Any characters that appear within a text field in the model line are
repeated in each line of the scrollable data. This includes the letter Z. It is not treated as a variable name
if it occurs in a text field.
Variable model lines can be specified in the panel definition. If a variable, a name preceded by an
ampersand, begins in column 1 of any model line, the value of that variable defines the model line.
These rules apply to variable model lines:
• The variable must be the only information on the model line. If any other data is present, an error
results.
• If the value of the variable is greater than the screen width, an error results.
• The variable can contain any character string that is a valid panel definition model line, except that the
variable cannot define a variable model line. A variable whose value is all blanks is acceptable.
• If the variable contains the character string OMIT starting in column 1, that variable model line will not
be used in the model definition.
• All model line variables must be initialized before the table display service is called with a nonblank
panel name. Changes to the variables that occur within the panel or the dialog function are not honored
until table display is called again with a nonblank panel name.
• If variable model lines are being used, the panel is retrieved from disk every time that table display is
called with a nonblank panel name and the value of the variable model line has changed.
• If the SFIHDR keyword is specified on the )MODEL header statement, the first variable model line is
assumed to define scroll indicator fields for scrollable fields that are defined on subsequent variable
model lines.
118  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 147

Requirements for initialization section
An initialization section, if present, is processed when the TBDISPL service is invoked with the panel
name specified.
If Z variables occur as name placeholders within the model lines or the fixed portion, an )INIT section is
needed. The real names of these fields are defined by assigning a name list, enclosed in parentheses if
more than one name is given, to the control variable, .ZVARS. For example:
)INIT
  .ZVARS  = '(NAME1,NAME2,NAME3)'
where NAME1, NAME2, and NAME3 are the actual variable names corresponding to the first, second, and
third Z variables in the body or model sections. For example, if one Z variable occurs as a placeholder
within the panel body and two Z variables occur as placeholders within the model lines, then NAME1
corresponds to the field in the body and NAME2 and NAME3 correspond to the two fields in the model
lines.
The )INIT section of a TBDISPL panel definition can contain any statement that is valid in an )INIT section
of a DISPLAY panel definition.
Requirements for reinitialization section
If a )REINIT section is included, it is executed when TBDISPL is reinvoked without a panel name or when
a redisplay occurs automatically because of the .MSG control variable being nonblank.
The )REINIT section of a TBDISPL panel definition can contain any statement that is valid in a )REINIT
section of a DISPLAY panel definition.
Any control variable except .ZVARS can be set within the )REINIT section. If table variables that are in the
model lines are referenced within the )REINIT section, then the values for the current row, as specified
by the CRP, are used. For example, if the .ATTR control variable is set for fields that are in the )MODEL
section, then only fields in the model set on the display that corresponds to the current selected row will
have their attributes changed.
Requirements for processing section
If a )PROC section is included, it is executed before control returns to the dialog function. It is not
executed while the user is scrolling.
The )PROC section of a TBDISPL panel definition can contain any statement that is valid in a )PROC
section of a DISPLAY panel definition.
Any control variable except .AUTOSEL and .ZVARS can be used in the )PROC section. If table variables
that are in the model lines are referenced within the )PROC section, then the values for the current row,
as specified by the CRP, are used. For example, if the .ATTR control variable is set for fields that are in
the )MODEL section, only fields in the model set on the display that corresponds to the current selected
row will have their attributes changed.
The )PROC section can check the value of ZTDSELS to determine if any rows were selected. This value and
its interpretation are:
0000
No selected rows
0001
One selected row (now the current row)
0002
Two selected rows, consisting of the current row and a pending selected row
0003
Three selected rows, consisting of the current row and two pending selected rows
...
And so forth.
Chapter 5. Panel definition statement guide  119

## Page 148

Using control variables
Two control variables, .AUTOSEL and .CSRROW, can be used in the executable—)INIT, )REINIT, and )PROC
—sections of a TBDISPL panel definition. They are ignored in a DISPLAY panel definition.
The .AUTOSEL and .CSRROW control variables can be used to control the selection (and preselection) of
a row in a table display. For more information about these variables, see “.AUTOSEL” on page 249 and
“.CSRROW” on page 250.
Processing panels by using the TBDISPL service
When a panel is displayed by the TBDISPL service, the model lines in the )MODEL section are duplicated
at the end of the logical screen. When the scrollable portion of the screen is being formatted, only full
units or duplications of these model lines are usually displayed. Two exceptions are:
• When the command line is repositioned to the bottom of the screen, the line above it, which can be a
model line, may be overlaid with a blank line and used as the long message line. This prevents table
display data from being overlaid with long message data.
• When the PFSHOW command is in effect, up to four additional lines can be overlaid.
Each input or output field that has a corresponding column in the table is initialized with data from
succeeding rows from the table. The first row displayed is the row pointed to by the CRP when TBDISPL
was issued.
Input or output fields in a model line that do not correspond to columns in the table are initialized, in all
rows, with the current contents of the corresponding dialog variables. If these fields are to be blank, the
corresponding variables must be set to blanks or null before each call of TBDISPL. The CLEAR keyword
can be used to specify that they are to be blanked.
A user can scroll the data up and down. Scroll commands, such as DOWN 5, apply to the number of table
entries to scroll up or down. For example, if three model lines are specified, DOWN 5 would scroll by 5
table entries, which corresponds to 15 lines on the display.
A user can enter information in any of the input fields within the fixed or scrollable portion of the panel.
Figure 40 on page 120 shows a sample panel definition for table display.
)ATTR
  @ TYPE(OUTPUT) INTENS(LOW)
)BODY
%----------------------------  EMPLOYEE LIST  ---------------------------------
%COMMAND INPUT ===>_ZCMD                                     %SCROLL ===>_AMT +
+ 
%EMPLOYEES IN DEPARTMENT@Z + 
+ 
+SELECT     ------ EMPLOYEE NAME -------          -- PHONE ---         EMPLOYEE
+ CODE      LAST         FIRST        MI          AREA NUMBER           SERIAL
)MODEL
  _Z+      @LNAME       @FNAME       @I          @PHA @PHNUM            @EMPSER
)INIT
  .ZVARS = '(DEPT SELECT)'
  &AMT  = PAGE
  .HELP = PERS123
)REINIT
  IF (.MSG = ' ')
     &SELECT = ' '
     REFRESH (SELECT)
)PROC
  IF (&ZTDSELS ¬= 0000)
     VER (&SELECT, LIST, A, D, U)
)END
Figure 40. Table display panel definition 
Assuming that the current contents of the table are as shown in Table 10 on page 121 and that dialog
variable DEPT contains '27', the resulting display is shown in Figure 41 on page 121.
120  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 149

Table 10. Table display data
EMPSER LNAME FNAME I PHA PHNUM
598304 Robert Richard P 301 555-1224
172397 Smith Susan A 301 555-8465
813058 Lowe Charles L 202 555-9557
395733 Adams John Q 202 555-1776
502774 Hsu Ann A 914 555-4156
 ----------------------------  EMPLOYEE LIST  --------------------  ROW 1 OF 5
 COMMAND INPUT ===> _                                         SCROLL ===> PAGE
 EMPLOYEES IN DEPARTMENT 27
 SELECT     ------ EMPLOYEE NAME -------       --- PHONE ---        EMPLOYEE
  CODE      LAST         FIRST        MI       AREA  NUMBER          SERIAL
            Robert       Richard      P        301   555-1224        598304
            Smith        Susan        A        301   555-8465        172397
            Lowe         Charles      L        202   555-9557        813058
            Adams        John         Q        202   555-1776        395733
            Hsu          Ann          A        914   555-4156        502774
 ******************************* BOTTOM OF DATA *******************************
 ⋮
Figure 41. Table as displayed
In this example, the select field (left-most column) does not correspond to a column in the table. It is
used to return a selection code, entered by the user and placed in a variable named SELECT. The other
variables in the model line correspond to variables in the table. The example also illustrates the use of
two Z variables as placeholders in the body of the panel and in the model line, the initialization of the
scroll amount field to PAGE, and the specification of a corresponding help panel.
The same table might be displayed by using several model lines with the panel definition shown in Figure
42 on page 121.
)ATTR
 @ TYPE(OUTPUT)  INTENS(LOW)
 # TYPE(INPUT)  PAD('_')
)BODY
%----------------------------  EMPLOYEE LIST  ---------------------------------
%COMMAND INPUT ===>_ZCMD                                     %SCROLL ===>_AMT +
+
%EMPLOYEES IN DEPARTMENT@Z +
+
+ENTER CHANGES ON THE LINES BELOW.
+
)MODEL
  #Z   +  SERIAL: @EMPSER +              LAST NAME:  @LNAME                 +
          PHONE:  @PHA@PHNUM   +         FIRST NAME: @FNAME                 +
                                         INITIAL:    @I                        +
          ---------------------------------------------------------------------
)INIT
  .ZVARS = '(DEPT SELECT)'
  &AMT = PAGE
  .HELP = PERS123
)END
Figure 42. Table display panel definition  with several model lines
The resulting display is shown in Figure 43 on page 122. An entry separator, consisting of a dashed line, is
also included as the last model line. In this example, the SELECT field has been increased to 4 characters,
with underscores used as pad characters.
Chapter 5. Panel definition statement guide  121

## Page 150

----------------------------  EMPLOYEE LIST  --------------------  ROW 1 OF 5
 COMMAND INPUT ===> _                                         SCROLL ===> PAGE
 EMPLOYEES IN DEPARTMENT 27
 ENTER CHANGES ON THE LINES BELOW.
   ___    SERIAL:  598304                LAST NAME:   Robert
          PHONE:   301 555-1224          FIRST NAME:  Richard
                                         INITIAL:     P
          ---------------------------------------------------------------------
   ___    SERIAL:  172397                LAST NAME:   Smith
          PHONE:   301 555-8465          FIRST NAME:  Susan
                                         INITIAL:     A
          ---------------------------------------------------------------------
   ___    SERIAL:  813058                LAST NAME:   Lowe
          PHONE:   202 555-9557          FIRST NAME:  Charles
                                         INITIAL:     L
          ---------------------------------------------------------------------
   ___    SERIAL:  395733                LAST NAME:   Adams
          PHONE:   202 555-1776          FIRST NAME:  John
                                         INITIAL:     Q
          ---------------------------------------------------------------------
   ___    SERIAL:  502774                LAST NAME:   Hsu
          PHONE:   914 555-4156          FIRST NAME:  Ann
                                         INITIAL:     J
          ---------------------------------------------------------------------
 ******************************* BOTTOM OF DATA *******************************
Figure 43. Table as displayed with several model lines
Formatting panels that contain dynamic areas
ISPF facilities permit the format and content of a display to be determined in the same dialog in which it is
displayed. This is called dynamic formatting. See “Specifying dynamic areas” on page 165 for information
about how to specify a dynamic area in the )ATTR section header.
Areas are reserved for this purpose in a panel definition and are called dynamic areas. A dynamic area can
encompass all or part of a panel display.
The format of a dynamic area is specified by a string of control and data characters, stored in a dialog
variable. This variable may have been produced either in the current dialog or, earlier, in another dialog
or program. The string usually contains a mixture of nondisplayable attribute characters and data to be
displayed. The name of the dialog variable is chosen by the panel designer. This name is placed in the
panel definition within the dynamic area.
A dialog uses the DISPLAY, TBDISPL, or SELECT service to display a panel containing a dynamic area.
After the display and after entry of any input by the user, data from within the dynamic area is stored in
the variable, associated with the area, and is available for processing by the dialog function.
When a panel is displayed, the number of lines in a dynamic area can be increased automatically to
accommodate the number of lines available on the terminal being used for the display.
Panel processing considerations
When you are defining a dynamic area and generating a dynamic character string that defines the format
of the data to be placed within that area on the panel, a number of rules apply:
• The area cannot be specified by using a Z-variable place-holder within the panel body.
• Within the dynamic area, all nonattribute characters are treated as data to be displayed. Unlike other
parts of the panel body, a variable name does not follow an attribute character.
• The dialog is responsible for ensuring data integrity, validity of attribute codes, and so on, for the
dynamic character string.
• If the dynamic area has a width that is less than the screen size, the panel designer must place the
appropriate attribute characters around this box so that the data within the area is not inadvertently
122  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 151

affected. For example, the panel designer can place fields with SKIP attributes following the right-most
boundaries so that the cursor is properly placed to the next or continued input field within the area.
• If the dialog must know the dimensions of the dynamic area before the data is formatted, this
information is available by invoking the PQUERY dialog service. All dialog services are described in
z/OS ISPF Services Guide.
• The scroll amount field is optional. On a panel with a scrollable area, if the input field following the
command field in the panel body is exactly 4 characters long, it is assumed to be the scroll amount
field. Otherwise, the system variable ZSCROLLD, which can be set by the dialog, is used to determine
the default scroll amount. If there is no scroll amount field and ZSCROLLD has not been set, PAGE is
assumed. ZSCROLLA contains the value of the scroll amount field, such as MAX or CSR. ZSCROLLN and
ZSCROLNL contain the scroll number computed from the value in the scroll amount field or entered
as a scroll number (number of lines or columns to scroll). For example, if a dialog is in split-screen
mode, 12 lines are currently visible, and a user requests DOWN HALF, ZSCROLLN and ZSCROLNL each
contain a value of '6'. ZSCROLLN can support values up to '9999'. If a scroll number greater than '9999'
is specified ZSCROLLN is set to a value of '9999'. ZSCROLNL can support values up to '9999999'. The
system variable ZVERB contains the scroll direction, DOWN in this case. If ZSCROLLA has a value of
MAX, the values of ZSCROLLN and ZSCROLNL are not meaningful.
• A nonblank input or output field preceding a dynamic area must be terminated by an attribute character.
• When variable substitution occurs within a text field in the panel body, the field must be terminated
by an attribute character, before a special character defining a dynamic area. See “Using variables and
literal expressions in text fields and panel sections” on page 96 for additional information about text
field variable substitution.
Although panel display processing cannot provide point-and-shoot support for dynamic areas, it does
provide the PAS(ON) keyword for TYPE(DATAOUT). The PAS(ON) keyword reflects the CUA point-and-
shoot color. It is up to application developers to provide the point-and-shoot function in programs they
develop.
Similarly, while the panel display service does not perform the scrolling for dynamic or graphic areas, it
does provide an interpretation of the user's scroll request.
The value for the SCROLL keyword cannot be specified as a dialog variable.
A panel cannot have more than one scrollable area or more than one extended area. The scrollable area
can be a panel with a scrollable area or a table display.
These rules are applied in Figure 44 on page 123.
)ATTR
 #   AREA(DYNAMIC)  SCROLL(ON)  EXTEND(ON)
)BODY
%-------------------- TITLE -----------------------
%COMMAND ===>_ZCMD                  +SCROLL ===>_AMT +
+
+  (Instructions for this panel ...)
+
#SAREA -------------------------------------------#
+
+  (More instructions for this panel ...)
+
)END
 
Figure 44. Panel definition  illustrating SCROLL and EXTEND
In this example, there are:
• 5 lines in the panel body before the extended area
• 3 more lines after the extended area.
This makes a total of 8 lines that are outside the dynamic area. Therefore, if the panel were displayed on
a 3278 Model 4, which has 43 lines, the depth or extent of the dynamic area would be 43 minus 8, or
Chapter 5. Panel definition statement guide  123

## Page 152

35 lines. In split-screen mode, the panel is still considered to have a 35-line scrollable area, even though
part of it is not visible.
In this example, the dynamically generated data string to be placed in the area is taken from the dialog
variable SAREA. If, for example, the dynamic area is 60 characters wide and 10 lines deep, the first 60
characters of the string are placed in the first line of the area, the next 60 characters are placed in the
second line of the area, and so on, until the last 60 characters are placed in the tenth line of the area.
Following a user interaction, the contents of the area are stored in the same variable.
The width of the dynamic area includes the special characters that designate the vertical sides. These
delimiter characters do not represent attribute characters.
A number of the capabilities described in the previous sections have implications for panel areas as well
as panel fields. These include:
• A REFRESH statement can be used to reset an area when reinitializing or redisplaying a panel. The
variable value is again read and placed in the area. Since the value also contains attribute information
that may have changed, the characteristics for each field are again analyzed.
• The cursor placement capability applies to dynamic areas. That is, .CURSOR can be assigned to a
dynamic area name and .CSRPOS can be assigned to a position within the dynamic area. The position
within an area applies within the rectangular bounds of that area.
• The .ATTRCHAR control variable can be used to override attribute characters that are used within
dynamic areas. In addition, .ATTRCHAR can be used to define a new attribute character that has not
been previously listed within the panel )ATTR section. Using .ATTRCHAR as a vehicle for defining new
attribute characters can be done only within the )INIT section and only for fields within dynamic areas
(TYPE(DATAIN) or TYPE(DATAOUT)).
• The PQUERY service can be invoked by the dialog function to determine the characteristics of the
dynamic area before the dialog function constructs the dynamic character string.
Character-level attribute support for dynamic areas
ISPF allows you to associate character-level attributes with individual characters within a dynamic area.
Each character in the dynamic area can be assigned characteristics of color and extended highlighting,
which override these attribute values identified in the field attribute. You can also specify that a graphic
escape (GE) order be used to display a graphic character from an alternate character set. See “Defining
the attribute section” on page 143 for more information.
These attributes are treated as character attributes only if they are used in the shadow variable for the
dynamic area; otherwise, they are treated as text. See “Specifying character attributes in a dynamic area”
on page 124 for more information on shadow variables.
Dialog variables can be substituted for the values of the COLOR, HILITE, and GE keywords in the same
way they are substituted for field attributes.
The .ATTRCHAR control variable may be used to override the COLOR, HILITE, and GE keywords for
character attributes in the same way it is used to override field attributes. The TYPE keyword cannot
be overridden from TYPE(CHAR) to any other type, nor can a different type value be overridden as
TYPE(CHAR). See “Relationship to Control variables .ATTR and .ATTRCHAR” on page 168.
See the z/OS ISPF Dialog Tag Language Guide and Reference for details on defining character attributes
within dynamic areas in panels created using DTL.
Specifying character attributes in a dynamic area
If a dynamic area is to contain character attributes, a shadow variable must be defined. The TYPE(CHAR)
attributes must be placed in this variable such that they map to the characters in the dynamic area
affected by the attribute. ISPF ignores any other characters or field attributes that are placed in this
shadow variable, but it is recommended that blanks be used as filler characters.
Note: If consecutive characters have the same character attributes (an entire word, for example), the
attribute character must be repeated in the shadow variable for EACH character affected. For panels to be
124  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 153

displayed on DBCS terminals, a TYPE(CHAR) attribute should only map to the first byte of a double-byte
character.
The shadow variable is associated with the dynamic area by placing the shadow variable name after the
dynamic area name in the panel definition. The two variable names must be separated by a comma only,
and the shadow variable name must be followed by a blank.
Note: The dynamic area and shadow variables cannot be Z variables in the panel source.
See the z/OS ISPF Dialog Tag Language Guide and Reference for details on specifying a shadow variable
using Dialog Tag Language.
Conflict resolution between attributes
If the terminal does not support the specified TYPE(CHAR) attribute of color or extended highlighting, this
attribute is ignored and defaults to the field attribute.
If the terminal does not support the graphic escape order, or if the character defined by TYPE(CHAR)
GE(ON) is not in the range '40'X through 'FE'X, ISPF does not place a GE order in the order stream before
this character and displays this character as a blank.
• The dialog can check the system variable ZGE to determine if the terminal supports the graphic escape
order. If it does not, the dialog can substitute different characters in the dynamic area.
Table 11. Characteristics of the ZGE system variable
Name Pool Type Len Description
ZGE shr non   3 Terminal support for graphic escape order:
• YES — graphic escape is supported
• NO — graphic escape is not supported
If a TYPE(CHAR) attribute is defined with other keywords such as INTENS, CAPS, JUST, or PAD in addition
to COLOR, HILITE, or GE, only the COLOR, HILITE, and GE keywords are recognized. If the GE keyword is
specified for any type other than TYPE(CHAR), TYPE(ABSL), TYPE(WASL), or TYPE(CH), it is ignored. If a
TYPE(CHAR) attribute is specified in the shadow variable that contains neither the COLOR nor the HILITE
keywords, the character defaults to the field attribute.
Any character attribute specified in the shadow variable that maps to the location of a field attribute
character in the dynamic area variable is ignored. (For instance, see Figure 45 on page 126. A $ in the first
character position of the variable SHADOW is ignored because the first character position in the variable
CATTAREA is a ¬ indicating a field attribute.)
On DBCS terminals, ISPF ignores any TYPE(CHAR) attribute that maps to a character that precedes the
first field attribute. Following the first field attribute, any TYPE(CHAR) attribute that maps to the second
byte of a double-byte character is ignored. In addition, the GE(ON) keyword specified for a TYPE(CHAR)
attribute that maps to a double-byte character is ignored.
A character attribute specifying the GE(ON) keyword can be defined within a TYPE(DATAIN) field.
However, any data typed into this character position might be returned to the dialog as an unpredictable
character.
Character attributes are associated with a character and not with the character's position in the buffer. If
a character is moved, for example, because of an insert or delete operation, the attribute moves with the
character.
The screen image recorded in the list data set as a result of the PRINT, PRINT-HI, PRINTL, or PRINTLHI
contains a blank character for all character attributes defined with the GE(ON) keyword.
Figure 45 on page 126 shows an example of the panel source for a panel with a dynamic area containing
character attributes.
Chapter 5. Panel definition statement guide  125

## Page 154

)ATTR
  * AREA(DYNAMIC)
  $ TYPE(CHAR) HILITE(REVERSE) COLOR(YELLOW)
  > TYPE(CHAR) COLOR(RED)
  # TYPE(CHAR) COLOR(BLUE) HILITE(USCORE)
  ^ TYPE(DATAOUT) INTENS(LOW) COLOR(WHITE)
 )BODY
  %-------------------CHARACTER ATTRIBUTE PANEL------------------------
  %COMMAND ===>_ZCMD
  +The following will contain character attributes:
  *CATTAREA,SHADOW   -----------------------------------------------*
  )END
Figure 45. Dynamic area with character attributes
The next example shows how the dynamic area and shadow variables are defined and initialized in a PL/I
program to display the panel shown.
  DECLARE CATTAREA CHAR(50) INIT    /* Dynamic Area Variable */
    ('^These words contain character attributes: Fox Cat');
  DECLARE SHADOW CHAR(50) INIT  /* Shadow of Dynamic Area Variable */
    ('                                           $## >  ');
In the panel displayed from the examples shown, the F in the word Fox is yellow and displayed in reverse
video, the ox in the word Fox is blue and underscored, the C in the word Cat is red with no highlighting,
and the at in the word Cat as well as the rest of the sentence, defaults to the field attribute and is
displayed low intensity and white with no highlight.
Formatting panels that contain a graphic area
ISPF panel definition syntax allows specification of a graphic area within a panel. An ISPF display can
contain a picture or graph generated through use of the Graphical Data Display Manager (GDDM) licensed
program. A graphic area defined within a panel definition provides part of the interface between ISPF
and GDDM. A graphic area can contain either a picture, constructed by use of GDDM services or a
graph, constructed by use of the GDDM Presentation Graphics Feature (PGF). Graphic areas can contain
alphanumeric fields within them, represented in the usual panel field syntax. These fields can partially
overlap the graphic area.
Formatting of a graphic area display is controlled by GDDM.
When specifying a graphic area display, the dialog developer issues a request for the ISPF GRINIT service
specifying the name of the panel definition in which the graphic area is defined. This request establishes
the interface to GDDM. Next, calls to GDDM that request GDDM services specify the picture to appear in
that graphic area. Then the ISPF DISPLAY service is used to display the panel.
The dialog must provide an 8-byte area, called an application anchor block (AAB), which is on a full-word
boundary, to the GRINIT call. This AAB identifies the ISPF/GDDM instance and must be used in all GDDM
calls made by the dialog. Within the ISPF/GDDM instance, the dialog cannot perform any of these GDDM
calls:
ASREAD  FSSHOR  ISFLD   MSPCRT  MSQMOD  PTNSEL  WSCRT
FSSHOW  ISQFLD  MSPQRY  MSQPOS  PTSCRT  WSDEL   WSIO
FSENAB  FSTERM  ISXCTL  MSPUT   MSREAD  PTSDEL  WSMOD
FSEXIT  GSREAD  MSCPOS  MSQADS  PTNCRT  PTSSEL  WSSEL
FSINIT  ISCTL   MSDFLD  MSQGRP  PTNDEL  PTSSPP  WSSWP
FSRNIT  ISESCA  MSGET   MSQMAP  PTNMOD  SPINIT
ISPF GDDM services do not run in the background, and thus, cannot be requested in a batch environment.
See “Defining the attribute section” on page 143 for information using the AREA keyword in the )ATTR
section to define a graphic area.
126  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 155

Graphics panel processing considerations
ISPF automatically switches into graphics interface mode when the GRINIT service is requested. This
mode continues for the life of the ISPF session. GDDM is called to perform all full-screen displays from
this point on, or until a request for the dialog service GRTERM is issued. These notes apply to graphics
interface mode:
Stacked TSO commands
The field mark key is not available to enter commands at one time.
5550 terminals
GDDM graphics are supported through the Japanese 3270PC/G Version 3 emulator program. The
ISPF-GDDM interface allows DBCS and mixed-character fields in the panel body, outside the graphics
area, to be displayed through GDDM. Full color and highlighting are supported through use of the
Japanese 3270PC/G Version 3 and 3270PC Version 5 emulator programs.
3290 terminals
The vertical split function is disabled. Panels are displayed with a larger-size character set. The
partition jump key is not functional.
Alternate screen widths
You cannot use GDDM with terminal devices whose primary width is different from their alternate
width. For example, 3278 model 5.
Autoskip facility
When entering data in a field, GDDM automatically moves the cursor to the next input field when the
preceding field is full.
First field attribute
GDDM requires that the first field on a panel begin with an attribute character. Therefore, the ISPF/
GDDM interface copies the attribute character for the last field on a panel to the first panel position.
This can result in the first byte of the panel data being overlaid.
Data transfer
The entire screen buffer is sent to the terminal even if no fields have been modified.
NUMERIC (ON)
The numeric lock feature is not active when using GDDM.
Graphic output
GDDM calls issued from an application are used to define graphic primitives for the next full-screen
output and are unknown to ISPF. Any full-screen output, following the ISPF full-screen output
containing the graphic area, can cause the loss of the graphic primitives on the ISPF panel. Hence, the
application can be required to reissue the GDDM calls.
Pop-up windows
Pop-up windows cannot be displayed over graphic areas nor can graphic areas be displayed over
pop-up windows.
Using DBCS-related variables in panels
These rules apply to substituting DBCS-related variables in panel text fields.
• If the variable contains MIX format data, each DBCS subfield must be enclosed with shift-out and
shift-in characters.
Example:
eeee[DBDBDBDBDB]eee[DBDBDB]
ee... represents a field of EBCDIC characters; DBDB... represents a field of DBCS characters; [ and ]
represent shift-out and shift-in characters.
• If the variable contains DBCS format data only, the variable must be preceded by the ZE system
variable, without an intervening blank.
Example:
Chapter 5. Panel definition statement guide  127

## Page 156

...text...&ZE&DBCSVAR..text...
• If the variable contains EBCDIC format data, and it is to be converted to the corresponding DBCS
format data before substitution, the variable must be preceded by the ZC system variable, without an
intervening blank.
Example:
...text...&ZC&DBCSVAR..text...
The ZC and ZE system variables can be used only for the two purposes described. When variable
substitution causes a subfield length of zero, the adjacent shift-out and shift-in characters are removed.
Using preprocessed panels
You can store preprocessed panel definitions to reduce transition time. These preprocessed panel
definitions are in an encoded format, and cannot be edited directly.
Preprocessed panel data sets must be defined to ISPF as you would define other data sets. This can
be either by normal allocation before invoking ISPF, or dynamically during an ISPF session by using the
LIBDEF service. ISPF provides a dialog, ISPPREP, for creating preprocessed panels. This dialog can be run
either in batch mode or interactively.
You invoke the ISPPREP dialog by:
• Issuing the ISPPREP command from the command line
• Selecting it from the Compilers pull-down on the ISR@PRIM panel.
• Specifying ISPPREP with the PGM keyword on the SELECT service request
To run ISPPREP by using the SELECT service, issue ISPPREP with no parameters. For example, entering
ISPEXEC SELECT PGM(ISPPREP) displays this selection panel:
                       Preprocessed Panel Utility
 Specify input and output data set names below:
 Panel input data set:
   Data set name  . .
   Member . . . . . .                   (* for all members)
   Volume serial  . .                   (If not cataloged )
 Panel output data set:
   Data set name  . .
   Member . . . . . .                   (blank or member name)
   Volume serial  . .                   (If not cataloged    )
 Enter "/" to select option
    Replace like-named members
 /  Save statistics for members
 Command ===>                                                         
  F1=Help      F2=Split     F3=Exit      F9=Swap     F10=Actions  F12=Cancel
Figure 46. Panel for specifying preprocessed panel data sets (ISPPREPA)
Entering ISPPREP from a command line or invoking ISPPREP from the Functions choice on the action bar
of the ISPF Primary Option Menu also causes this selection panel to be displayed.
To run ISPPREP in batch mode, include the PARM keyword and the panel-input and panel-output
identifiers on the SELECT service request. For example:
ISPEXEC SELECT PGM(ISPPREP) PARM(INPAN(‘ISPFPROJ.GRE.PANELS(PANA)’),
OUTPAN(‘ISPFPROJ.PXY.PANELS(PANB)’) EXEC)
128  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 157

requests the SELECT service to convert member PANA in ISPFPROJ.GRE.PANELS to the internal format
and to write it to member PANB in ISPFPROJ.PXY.PANELS.
Note: The previous example must be run from a REXX or CLIST command procedure.
You can control whether existing members in the output data set having the same identification as that
specified will be replaced. In batch mode, use the NOREPL|REPLACE parameter with the PARM keyword
for specifying whether members are to be replaced. In interactive mode, use the line provided on the
panel shown in Figure 46 on page 128 for specifying whether members are to be replaced.
ISPPREP converts panel input data set members to an internal format and writes them to the specified
output panel data set members. A given panel file can contain a mixture of preprocessed panels and
regular panel definitions.
ISPPREP does not destroy the source panels from which it creates preprocessed panels. However, you
should save those panels in case they must be updated in the future. When the preprocessed panels are
ready for use, you can use them to replace the corresponding source files for the ISPPLIB defaults.
ISPPREP provides an option to generate statistics for preprocessed panels. ISPF provides the version
(always 1), modification counter, creation date last-modified date, current number of lines, initial number
of lines, number of modified lines (always 0), and user ID for the message or panel. These statistics are
visible on memberlist displays such as ISPF BROWSE and EDIT. The statistics are placed in the ISPF
directory.
Restrictions for using ISPPREP
When using ISPPREP, you should note that certain restrictions apply to those panel definitions that can be
converted to their internal format. These restrictions apply only when creating preprocessed panels and
are based on the fact that preprocessed panels cannot have dynamically defined width, depth, or source
records.
These restrictions apply to panel definitions to be converted:
1. The use of a dialog variable with the WIDTH keyword on the )BODY header statement of a panel
definition is not allowed.
2. The specification of EXTEND(ON) for the attribute character of a dynamic, graphic, or scrollable area is
not allowed.
3. The use of a dialog variable to define a model line in a table display panel definition is not allowed.
4. The specification of an INEXT section is not allowed.
For DBCS panels, the correct character set must be loaded before invoking ISPPREP. Select the 3278KN
character set in your ISPF settings before converting panels to be displayed on a 5550 3270 Kanji
Emulation terminal.
Preprocessed panel objects should not be copied from a fixed to a variable record format data set. Blank
data could be lost. This can cause the product to abend or can create a display error when the copied
panel object is used by display processing. Use ISPPREP to transfer preprocessed panel objects to a
variable record format data set or when the receiving data set logical record length or logical record
format is not the same as the source data set.
ISPPREP output data sets must conform to the same LRECL limits as ISPPLIB.
Using ISPPREP with the SELECT service
You can use the PGM keyword of the SELECT service to invoke ISPPREP. The syntax for invoking ISPPREP
is as follows:
ISPEXEC SELECT PGM(ISPPREP)
PARM(INPAN(  PDSin), OUTPAN(  PDSout) PARM options )
Chapter 5. Panel definition statement guide  129

## Page 158

PARM options
,INVOL(  volser# ) ,OUTVOL(  volser# ) , NOREPL
REPLACE
, STATS
NOSTATS
,EXEC
The PARM keyword on the SELECT indicates that ISPPREP is to be run in batch mode. The absence of
the PARM keyword indicates that ISPPREP is to be run as an interactive dialog and that PDSin, the panel
input library, and PDSout, the panel output library, are to be specified on a data-entry panel. Both the
ISPPREP command and option 2 on the ISP@PRIM primary option panel select ISPPREP in interactive
mode.
The panel input and panel output library identifiers, whether specified on the SELECT statement when in
batch mode or on the data entry panel when in interactive mode, follow the same guidelines.
PDSin (panel input library)
The name of the library of panel definitions to be converted to their internal format. PDSin must be in
the form:
(‘ partitioned data set name
( member)
‘)
The member name can be specified either by indicating the specific name or by coding an asterisk.
Coding an asterisk for the member name indicates that all members in the specified data set are to be
converted to preprocessed panels. This allows conversion of all panel definitions within a data set in
one call of ISPPREP.
You cannot specify the same name for the input partitioned data set and for the output data set,
even if you specify REPLACE unless the data sets exist on different volumes and you specify the
appropriate volume serial numbers by using the INVOL and/or OUTVOL parameters.
When running in batch mode, you are not required to enter a member name. The absence of the
member name is equivalent to coding an asterisk for the member name. In interactive mode, failure
to explicitly state a member name or an asterisk causes the data-entry panel to be redisplayed with a
message prompting the user for the member name.
PDSout (panel output library)
The name of the library to which the preprocessed panels will be written.
The form of PDSout is the same as that of PDSin. You can specify a blank or name for the member
name. A blank indicates that the member name specified for PDSin is to be used as the member name
for PDSout.
Coding an asterisk for a member name in PDSout is invalid.
INVOL (input PDS volume serial number)
Specifies the serial number of the volume on which PDSin is stored. If this parameter is omitted, the
system catalog is searched.
It must be used when the data set exists but is not cataloged. INVOL is optionally specified in batch
mode as well as in interactive mode. In batch mode the keyword (INVOL) is specified along with the
volume serial number as part of the SELECT statement.
OUTVOL (output PDS volume serial number)
Specifies the serial number of the volume on which PDSout resides. If this parameter is omitted, the
system catalog is searched.
130  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 159

It must be used when the data set exists but is not cataloged. OUTVOL is optionally specified in batch
mode as well as in interactive mode. In batch mode the keyword (OUTVOL) is specified along with the
volume serial number as part of the SELECT statement.
NOREPL, REPLACE
A keyword that specifies whether existing partitioned data set members are to be replaced in PDSout.
The default is NOREPL in batch mode. In interactive mode, an option must be specified.
STATS, NOSTATS
User controls whether member statistics are to be saved in the ISPF directory. The default option is
STATS.
EXEC
Specifies that ISPPREP is being executed from a CLIST or REXX command procedure. The EXEC
parameter causes the return code to be set to 24 if a space-related abend occurs on the output file.
Any panel specified in the panel input library that is already a preprocessed panel is copied directly to the
panel output library (contingent on the NOREPL|REPLACE specification).
ISPPREP should be invoked with the NEWAPPL keyword specified on the SELECT statement. (This is
necessary because ISPPREP issues LIBDEF service calls.) If NEWAPPL is not specified, any LIBDEF issued
before the execution of ISPPREP can no longer be in effect.
Examples of using ISPPREP
• Convert PDS member PANA, in ISPFPROJ.GRE.PANELS, and write the preprocessed panel to member
PANB, in ISPFPROJ.PXY.PANELS, if it does not already exist. Both PDSs are cataloged.
SELECT PGM(ISPPREP) PARM(INPAN(‘ISPFPROJ.GRE.PANELS(PANA)’),
                         OUTPAN(‘ISPFPROJ.PXY.PANELS(PANB)’),
                         NOREPL) NEWAPPL
• Convert PDS member PANA, in ISPFPROJ.GRE.PANELS, and unconditionally write the preprocessed
panel to member PANB, in ISPFPROJ.PXY.PANELS. Both PDSs are cataloged.
SELECT PGM(ISPPREP) PARM(INPAN(‘ISPFPROJ.GRE.PANELS(PANA)’),
                         OUTPAN(‘ISPFPROJ.PXY.PANELS(PANB)’),
                         REPLACE) NEWAPPL
• Convert the entire PDS ISPFPROJ.GRE.PANELS, which contains three members (PANA, PANB, and
PANC), and unconditionally write the preprocessed panels to PDS ISPFPROJ.PXY.PANELS, which
contains three members also (PANA, PANB, and PANC). Both PDSs are cataloged.
SELECT PGM(ISPPREP) PARM(INPAN(‘ISPFPROJ.GRE.PANELS(*)’),
                         OUTPAN(‘ISPFPROJ.PXY.PANELS( )’),
                         REPLACE) NEWAPPL
• Convert the entire PDS ISPFPROJ.GRE.PANELS, which contains four members (PAN1, PAN2, PAN3, and
PAN4) and is cataloged. If the members do not already exist, write the preprocessed panels to PDS
ISPFPROJ.PXY.PANELS, which is not cataloged
SELECT PGM(ISPPREP) PARM(INPAN(‘ISPFPROJ.GRE.PANELS(*)’),
                         OUTPAN(‘ISPFPROJ.PXY.PANELS( )’),
                         OUTVOL(TSOPK7),NOREPL) NEWAPPL
• Convert the entire PDS ISPFPROJ.GRE.PANELS and unconditionally write the preprocessed panels to
PDS ISPFPROJ.PXY.PANELS. Both PDSs are not cataloged.
SELECT PGM(ISPPREP) PARM(INPAN(‘ISPFPROJ.GRE.PANELS(*)’),
                         INVOL(TSOPK7),
                         OUTPAN(‘ISPFPROJ.PXY.PANELS( )’),
                         OUTVOL(TSOPK7),REPLACE) NEWAPPL
Chapter 5. Panel definition statement guide  131

## Page 160

Handling error conditions and return codes
There are two general classes of error conditions involved with ISPPREP: those associated with the dialog
itself, and those associated with the conversion of individual panel definitions.
The dialog error conditions encountered cause immediate termination of ISPPREP conversion processing.
If you are operating in interactive mode and recovery is possible, the data-entry panel is redisplayed with
an appropriate message. Otherwise, ISPPREP will terminate. Dialog errors include conditions such as:
invalid input or output PDS names; a reference to a nonexistent PDS; or a reference to an uncataloged
PDS without providing the correct volume serial number.
Panel conversion error conditions apply only to the current panel being converted. They are usually due to
an error in the panel definition. If such an error is encountered, processing of the current panel definition
halts, and processing of the next panel definition (if it exists) begins. A panel conversion error associated
with one panel definition does not affect the conversion of subsequent panel definitions.
ISPPREP logs error and informational messages in ISPLOG. Any error conditions encountered cause an
appropriate message and return code to be written to the log. This is also true for any conditions that
warrant an informational message.
When ISPPREP is run in the foreground, the program uses the ISPF CONTROL ERRORS CANCEL service to
cause a terminating dialog box to be displayed when a return code of 12 or greater is encountered.
If ISPPREP is run in the background (batch TSO), then CONTROL ERRORS CANCEL is not set and
ISPPREP passes the return code back to the calling program. If ISPPREP has issued a message, variables
ZERRMSG, ZERRSM, and ZERRLM are written to the shared pool and the message is written to the log.
These return codes are possible from ISPPREP:
0
Normal completion.
4
Panel definition cannot be processed (see restrictions); NOREPL is specified and the panel (member)
already exists in the output library.
8
Panel definition contains syntax errors; panel already in use (enqueue failed) or panel (member) not
found.
12
Invalid syntax or keyword in parameter string; data set is not found.
16
Data set allocation or open failure.
20
Severe error.
24
A space-related abend occurred while ISPPREP was being executed from a CLIST or REXX command
procedure with the EXEC parameter specified.
Since ISPPREP can convert a number of panel definitions to their internal format in one call, a number
of conditions may arise that generate a return code other than ‘0’. ISPPREP returns the highest return
code generated. However, if invoked in interactive mode, ISPPREP will return ‘0’ unless an unrecoverable
dialog error is encountered, in which case the code returned is ‘20’. Refer to the log for a more
comprehensive look at ISPPREP's results.
132  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
