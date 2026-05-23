# Chapter 12. Tag reference

Source file: f54dt00_v3r1.md
Start page: 211
Page span: 211-494

## Page 211

Chapter 12. Tag reference
This chapter contains an alphabetical reference of the Dialog Tag Language (DTL) tags.
Each reference listing contains:
• A diagram of the valid syntax for the tag
• A list describing the tag attributes
• A description of the tag
• Conditions of usage
• A table of the tags that can be nested within the tag
• An example of how the tag is used within DTL source markup.
Rules for variable names
Variable names supplied as attribute values on DTL tags must have these characteristics:
• 1-8 characters in length
• The first character must be A-Z, a-z, @, #, or $.
• Remaining characters, if any, can be A-Z, a-z, @, #, $, or 0-9.
Lowercase characters are translated to their uppercase equivalents
Names composed of valid characters that are longer than 8 bytes are truncated to 8 bytes. Names that
are not valid are set to blank.
Rules for “%variable” names
When a "%varname" notation is found as an attribute value, the "%varname" entry must have these
characteristics:
• 2-9 characters in length
• The first character is a “%”.
• The second character must be A-Z, a-z, @, #, or $.
• Remaining characters, if any, can be A-Z, a-z, @, #, $, or 0-9.
Lowercase characters are translated to their uppercase equivalents
The first position of a valid name is replaced by an “&”.
Names composed of valid characters that are longer than 9 bytes are truncated to 9 bytes. Names that
are not valid are set to blank.
It is the responsibility of the application to provide a valid value in the variable before the panel is
displayed.
AB (Action Bar)
The AB tag defines an action bar on an application panel.
Rules for variable names
© Copyright IBM Corp. 1989, 2024 179

## Page 212

Syntax
<AB
MNEMGEN=
YES
NO
ABSEPSTR=ab-separator-string
ABSEPCHAR=ab-separator-character
> </AB>
Parameters
MNEMGEN=YES | NO
Note: When the conversion utility is operating in DBCS mode, the default value for MNEMGEN is NO.
This attribute controls the automatic generation of mnemonic characters for the entire action bar.
When MNEMGEN=NO, mnemonic characters are determined only by the use of the M tag within action
bar choice description text. See “Mnemonic choice selection” on page 36 and “M (Mnemonic)” on
page 351 for additional information.
When MNEMGEN=YES, the NOGUI invocation option is ignored and mnemonics are generated
automatically.
ABSEPSTR=ab-separator-string
This attribute provides a string of data to be overlaid at the right end of the action bar separator line.
ABSEPCHAR=ab-separator-character
This attribute provides a replacement character for the action bar separator line. When the GRAPHIC
invocation option has been specified, the action bar separator defaults to a solid line for host display.
You can use the ABSEPCHAR attribute to provide a different character such as a dash.
Comments
The AB tag defines an action bar on an application panel. The action bar appears on the panel above the
panel title line. The action bar provides a way for users to view all actions that apply to the panel it is
coded within.
The conversion utility inserts a line between the action bar and the panel title line. The GRAPHIC
invocation option creates a solid line. NOGRAPHIC creates a dashed line. If required by the length or
number of action bar choices, the conversion utility formats multiple lines for the action bar.
ABC tags, which you code within an AB definition, define application panel choices for the action bar. PDC
tags, which you code within ABC tag definitions, define the action bar pull-down choices.
To define an action bar and its associated pull-downs, you code the AB tag (and other tags that define the
action bar choices and pull-downs) within a PANEL definition.
Restrictions
• The AB tag requires an end tag.
• You must code the AB tag within a PANEL definition. Each application panel can include only one action
bar. See “PANEL (Panel)” on page 376 for a complete description of this tag.
• You must code at least one ABC tag within an action bar definition.
• To conform to CUA rules, you must include a help action bar choice.
AB
180  z/OS: z/OS ISPF DTL Guide

## Page 213

Processing
Table 3. The tag you can code within an AB definition 
Tag Reference Usage Required
ABC “ABC (Action Bar Choice)” on page 182 Multiple Yes
Examples
Here is markup that contains the action bar markup for the application panel illustrated in Figure 86 on
page 182.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampbody system>)>
&sampvar1;
<PANEL NAME=ab KEYLIST=keylxmp>Library Card Registration
<AB>
<ABC>File
  <PDC>Add Entry
    <ACTION RUN=add>
  <PDC>Delete Entry
    <ACTION RUN=delete>
  <PDC>Update Entry
    <ACTION RUN=update>
  <PDC>Exit
    <ACTION RUN=exit>
<ABC>Search
  <PDC CHECKVAR=whchsrch MATCH=1>Search on name
    <ACTION SETVAR=whchsrch VALUE=1>
    <ACTION RUN=search>
  <PDC CHECKVAR=whchsrch MATCH=2>Search on card number
    <ACTION SETVAR=whchsrch VALUE=2>
    <ACTION RUN=search>
<ABC>Help
  <PDC>Extended Help...
    <ACTION RUN=exhelp>
  <PDC>Keys Help...
    <ACTION RUN=keyshelp>
</AB>
&sampbody;
</PANEL>
AB
Chapter 12. Tag reference  181

## Page 214

File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number if applicable.
 Then select an action bar choice.
 Date . . . :
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 86. Action bar
ABC (Action Bar Choice)
The ABC tag defines a choice in an action bar and serves as a base for associated pull-down choice tags.
Syntax
<ABC
HELP=
NO
YES
help-panel-name
*help-message-id
%varname
*%varname
PDCVAR=pdc-variable-name
>
choice-description-text
</ABC>
Parameters
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help on the action bar choice.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on an action bar choice and no help is defined, the extended help panel is
displayed. If an extended help panel is not defined for the panel, the application or ISPF tutorial is
invoked.
ABC
182  z/OS: z/OS ISPF DTL Guide

## Page 215

The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information on creating help panels. For information about
creating messages, see “MSG (Message)” on page 352.
PDCVAR=pdc-variable-name
This attribute provides the name of a variable to contain the value of the pull-down choice. When a
variable name is provided, it replaces the default ZPDC variable name. The pdc-variable-name is not
initialized to blank.
The pdc-variable-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
choice-description-text
This is the text that appears in the action bar. The text is limited to 64 bytes.
If the choice-description-text exceeds the panel width, the conversion utility issues a warning
message and truncates the text. If the choice-description-text for multiple ABC tags exceeds the panel
width, the conversion utility formats a multiple-line action bar.
Comments
The ABC tag defines a choice in an action bar and serves as a base for associated pull-down choice tags.
The pull-down choices appear in a pull-down when the action bar choice is selected.
If the text of an action bar choice contains multiple words, multiple blanks between the words are not
compressed.
Restrictions
• You must code the ABC tag within an AB definition. See “AB (Action Bar)” on page 179 for a complete
description of this tag.
• You must code at least one PDC tag within each ABC definition. See “PDC (Pull-Down Choice)” on page
390 for a complete description of this tag.
• The maximum number of action bar choices that is generated is 40.
Processing
Table 4. Tags you can code within an ABC definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
M “M (Mnemonic)” on page 351 Single No
PDC “PDC (Pull-Down Choice)” on page 390 Multiple Yes
PDSEP “PDSEP (Pull-Down Separator)” on page 394 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is markup that shows the use of the PDCVAR attribute to specify an application variable for the first
action bar choice. It produces the action bar on the application panel shown in Figure 87 on page 184.
ABC
Chapter 12. Tag reference  183

## Page 216

<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampbody system)>
&sampvar1;
<PANEL NAME=abc1 KEYLIST=keylxmp>Library Card Registration
<AB>
<ABC PDCVAR=foptns>File
  <PDC>Add Entry
    <ACTION RUN=add>
  <PDC>Delete Entry
    <ACTION RUN=delete>
  <PDC>Update Entry
    <ACTION RUN=update>
  <PDC>Exit
    <ACTION RUN=exit>
<ABC>Search
  <PDC CHECKVAR=whchsrch MATCH=1>Search on name
    <ACTION SETVAR=whchsrch VALUE=1>
    <ACTION RUN=search>
  <PDC CHECKVAR=whchsrch MATCH=2>Search on card number
    <ACTION SETVAR=whchsrch VALUE=2>
    <ACTION RUN=search>
<ABC>Help
  <PDC>Extended Help...
    <ACTION RUN=exhelp>
  <PDC>Keys Help...
    <ACTION RUN=keyshelp>
</AB>
&sampbody;
</PANEL>
   File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number if applicable.
 Then select an action bar choice.
 Date . . . :
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 87. Action bar choices
ACTION (Action)
The ACTION tag defines the action that occurs when a pull-down choice or a selection field choice is
selected.
ACTION
184  z/OS: z/OS ISPF DTL Guide

## Page 217

Syntax
<ACTION Keyword 1
Keyword 2
Keyword 3
>
</ACTION>
Keyword 1
RUN= internal-command-name
%varname
PARM= parameters
%varname
APPLCMD=
NO
YES
ISPF options
Keyword 2
SETVAR=variable-name
VALUE=
1
string
%varname
Keyword 3
TOGVAR=variable-name
VALUE1=
0
string
%varname
VALUE2=
1
string
%varname
ISPF options
TYPE=
CMD
PGM
PANEL
EXIT
NEWAPPL
NEWAPPL=application-id NEWWINDOW
PASSLIB NEWPOOL SUSPEND
SCRNAME=screen-name NOCHECK ADDPOP
OPT= option
%varname
MODE= LINE
FSCR
LANG= APL
CREX
BARRIER NEST
ACTION
Chapter 12. Tag reference  185

## Page 218

Parameters
RUN=internal-command-name | %varname
When the ACTION tag is associated with a PDC tag, this attribute specifies the internal name of a
command to be executed. The command is found in the application or system command table unless
APPLCMD=YES is specified. The search for the command follows the normal command processing
rules. For information on defining commands, see “CMD (Command Definition)” on page 233.
The RUN action is an ending action. Thus, if multiple ACTION tags are coded for a given pull-down,
those following a RUN action are ignored.
When the ACTION tag is associated with a CHOICE tag (under a SELFLD tag that specifies
TYPE=MENU or TYPE=MODEL), the TYPE attribute and related RUN attribute values are:
TYPE
RUN attribute value
CMD
Command name
PGM
Program name
PANEL
Panel name
When the ACTION tag is associated with a CHOICE tag under a SELFLD tag that specifies
TYPE=TUTOR, the TYPE attribute is forced to PANEL. The RUN attribute must provide a panel name.
None of the other ISPF selection menu attributes are valid for tutorial panels.
If TYPE=CMD is specified and the internal-command-name should start with a %, you must code an
additional % before the internal-command-name to distinguish it from a variable name. (For example,
to specify the internal-command-name “%abc”, code “%%abc”. If TYPE=EXIT is specified, the RUN
attribute is required for conversion utility processing, but is not used in the generated panel.
Note: This attribute is not supported if the ACTION tag is associated with a CHOICE tag under a
SELFLD tag that specifies TYPE=SINGLE or TYPE=MULTI.
PARM=parameters | %varname
These are the command parameters. These parameters are passed to command processing
with the command specified on the RUN attribute. Command processing handles the specified
parameters the same way parameters entered in the command area are handled. You can specify
the name of a dialog variable (using % notation) whose value at run time is passed as the
parameter data. When the ACTION tag is associated with a PDC tag, the conversion utility limits
the length of the command parameters to 72 single-byte characters.
When a ACTION tag is used to build a menu selection choice for TYPE=CMD or TYPE=PGM, and
the NEWWINDOW attribute has been specified, the conversion utility limits the length of the
command parameters to 249 single-byte characters; otherwise, the parameter is added to the
selection as coded.
APPLCMD=NO | YES
This attribute specifies whether the command provided by the RUN attribute is to be passed
directly to the application, bypassing the command table search. When APPLCMD=YES, the length
of the command name is limited to 7 bytes to allow the passthru character ">" to be prefixed to
the command name.
This attribute is valid only on an ACTION tag that is associated with a PDC tag.
Here is a list of attributes that are valid only when generating an ISPF selection menu or edit
model selection menu. (When the SELFLD tag specifies TYPE=TUTOR, the TYPE attribute is forced
to "PANEL" and none of the other ISPF selection menu attributes are valid.)
ACTION
186  z/OS: z/OS ISPF DTL Guide

## Page 219

TYPE=CMD | PGM | PANEL | EXIT
This attribute specifies the type of selection to be generated for the selection menu. The attributes
NEWAPPL, NEWWINDOW, PASSLIB, NEWPOOL, SUSPEND, SCRNAME, NOCHECK, ADDPOP, OPT,
MODE, LANG, BARRIER, and NEST are not valid when TYPE=EXIT is specified.
NEWAPPL=application-id
The NEWAPPL keyword may be specified with or without an application identifier. This attribute
specifies that the NEWAPPL keyword (and the application identifier, if present) are added to the
selection menu choice.
NEWWINDOW
This attribute specifies that the selection menu choice is created specifying the ISPSTRT
programming interface. The NEWWINDOW attribute is valid only when TYPE=PANEL, TYPE=PGM,
or TYPE=CMD.
PASSLIB
This attribute specifies that the PASSLIB keyword is added to the selection menu choice.
NEWPOOL
This attribute specifies that the NEWPOOL keyword is added to the selection menu choice.
SUSPEND
This attribute specifies that the SUSPEND keyword is added to the selection menu choice.
SCRNAME=screen-name
This attribute specifies that the SCRNAME keyword is added to the selection menu choice. ISPF
reserved values for screen-name are LIST, NEXT, PREV, ON, and OFF.
NOCHECK
This attribute specifies that the NOCHECK keyword is added to the selection menu choice. The
NOCHECK attribute is valid only when TYPE=CMD or TYPE=PGM.
ADDPOP
This attribute specifies that the ADDPOP keyword is added to the selection menu choice. The
ADDPOP attribute is valid only when TYPE=PANEL.
OPT=option | %varname
This attribute specifies that the OPT keyword is added to the selection menu choice to specify an
initial option for the panel. The OPT attribute is valid only when TYPE=PANEL.
MODE=LINE | FSCR
This attribute specifies that the MODE keyword is added to the selection menu choice. The MODE
attribute is valid only when TYPE=CMD or TYPE=PGM.
LANG=APL | CREX
This attribute specifies that the LANG keyword is added to the selection menu choice. The LANG
attribute is valid only when TYPE=CMD. LANG(CREX) is optional if the compiled REXX has been
link-edited to include any of the stubs EAGSTCE, EAGSTCPP, or EAGSTMP.
BARRIER
This attribute specifies that the BARRIER keyword is added to the selection menu choice. The
BARRIER attribute is valid only when TYPE=CMD.
NEST
This attribute specifies that the NEST keyword is added to the selection menu choice. The NEST
attribute is valid only when TYPE=CMD.
SETVAR=variable-name
This attribute sets a value into a dialog variable. The SETVAR attribute names the variable to set. The
variable-name must be coded without the leading % sign.
VALUE=1 | string | %varname
This is the value to set into the variable named on the SETVAR attribute. If you code the SETVAR
attribute but omit the VALUE attribute, ISPF assigns the variable a value of 1. You can specify the
name of a variable (using % notation) whose value at run time sets the value of the variable.
When defining the ACTION tag for selection fields, be aware that the variable name defined in the
SELFLD tag for single-choice selection fields or in the CHOICE tag for multiple-choice selection fields
ACTION
Chapter 12. Tag reference  187

## Page 220

contains the value entered by the user when the selection is made. In addition, if the CHECKVAR
attribute is specified in the CHOICE tag, the value of the MATCH attribute associated with the choice
is set into the variable named by the CHECKVAR attribute. Therefore, it is not necessary to use the
ACTION tag SETVAR attribute for the application to know which selection field choice or choices were
made by the user.
TOGVAR=variable-name
This attribute allows you to alternate the value of a single variable between two values. The TOGVAR
attribute names the variable to set. The variable-name must be coded without the leading % sign.
The function of the TOGVAR action can be depicted as follows:
           if (TOGVAR-variable-name = VALUE1-string)
             TOGVAR-variable-name = VALUE2-string
           else
             TOGVAR-variable-name = VALUE1-string
VALUE1=0 | string | %varname
This is the value to set into the variable named on the TOGVAR attribute if it is not currently equal
to this value. If you code the TOGVAR attribute, but omit the VALUE1 attribute, the variable is
assigned a value of 0. You can specify the name of a variable (using % notation) whose value at
run time sets the value of the variable.
VALUE2=1 | string | %varname
This is the value to set into the variable named on the TOGVAR attribute if it is currently equal
to the value specified with the VALUE1 attribute. If you code the TOGVAR attribute, but omit the
VALUE2 attribute, the variable is assigned a value of 1. You can specify the name of a variable
(using % notation) whose value at run time sets the value of the variable.
Comments
The ACTION tag defines the action that occurs when a pull-down choice or a selection field choice is
selected. Code the ACTION tag within the PDC or CHOICE definition it is associated with. You can specify
multiple ACTION tags for a given choice. The conversion utility builds the logic to carry out the actions in
the order in which you code the ACTION tags.
When defining action bar pull-downs, you should code the SETVAR attribute in the ACTION tags
associated with each PDC tag if the application needs to know which pull-down choice the user selected.
Unlike selection fields, there is no variable name associated with a pull-down definition and the PDC
CHECKVAR variable is not set to indicate the user's choice. Therefore, dialogs must refer to the SETVAR
variable-name to determine the pull-down choice the user has selected.
The TYPE, NEWAPPL, NEWWINDOW, PASSLIB, NEWPOOL, SUSPEND, SCRNAME, NOCHECK, ADDPOP,
OPT, MODE, LANG, BARRIER, and NEST attributes are used by the conversion utility to build an ISPF
selection menu. They are valid only when they appear on an ACTION tag associated with a CHOICE tag
which is nested within a SELFLD tag that specifies TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR (when
the SELFLD tag specifies TYPE=TUTOR, the only valid selection menu attribute is TYPE=PANEL). They
are not processed in other situations. See the z/OS ISPF Dialog Developer's Guide and Reference for a
description of the function of these keywords in ISPF option menus.
Restrictions
• You must code the ACTION tag within the PDC or CHOICE definition it is associated with. See “PDC
(Pull-Down Choice)” on page 390 and “CHOICE (Selection Choice)” on page 226 for descriptions of
these tags.
• You must code one (and only one) of these attributes on each ACTION tag: RUN, SETVAR, or TOGVAR.
• You can code the RUN attribute when:
– The ACTION tag is associated with a PDC tag.
ACTION
188  z/OS: z/OS ISPF DTL Guide

## Page 221

– The ACTION tag is associated with a CHOICE tag under a SELFLD tag that specifies TYPE=MENU,
TYPE=MODEL, or TYPE=TUTOR.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
None.
Examples
Here is markup where each of the PDC tags have associated ACTION tags that specify the command
that is executed when the pull-down choice is selected. Many of the PDC tags have additional ACTION
tags associated with them that specify the SETVAR attribute to let the application know which pull-down
choice was selected.
The use of ACTION tags associated with CHOICE tags is illustrated in the example for “PS (Point-and-
Shoot)” on page 398.
<!DOCTYPE DM SYSTEM>
<PANEL NAME=action1>Library Card Listing
<AB>
<ABC>File
    <PDC>Add Entry
        <ACTION SETVAR=fchoice VALUE=add>
        <ACTION RUN=add>
    <PDC>Delete Entry
        <ACTION SETVAR=fchoice VALUE=delete>
        <ACTION RUN=delete>
    <PDC>Update Entry
        <ACTION SETVAR=fchoice VALUE=update>
        <ACTION RUN=update>
    <PDC>Exit
        <ACTION RUN=exit>
<ABC>Sort sequence
    <PDC CHECKVAR=whchsort MATCH=1>Sort on name
        <ACTION SETVAR=whchsort VALUE=1>
        <ACTION RUN=sort>
    <PDC CHECKVAR=whchsort MATCH=2>Sort on card number
        <ACTION SETVAR=whchsort VALUE=2>
        <ACTION RUN=sort>
<ABC>Help
    <PDC>Extended Help...
        <ACTION RUN=exhelp>
    <PDC>Keys Help...
        <ACTION RUN=keyshelp>
</AB>
<TOPINST>Choose the size of the list needed.
<TOPINST>Then select action bar choice "Sort sequence" to
indicate the desired sort sequence.
<AREA>
  <SELFLD NAME=aa PMTWIDTH=30 PMTLOC=before SELWIDTH=38>Choose
  one of the following
    <CHOICE>New this month
    <CHOICE>New this year
    <CHOICE>All (this will take time to process)
  </SELFLD>
</AREA>
<CMDAREA>Enter a command
</PANEL>
AREA (Area)
The AREA tag defines portions of a panel body, one or more of which can be scrollable.
AREA
Chapter 12. Tag reference  189

## Page 222

Syntax
<AREA
MARGINW=
1
n MARGIND=
0
INDENT=n DEPTH= n
*
DEPTH options WIDTH=n
DIR=
VERT
HORIZ
> </AREA>
DEPTH options
EXTEND=
OFF
ON
FORCE
DIV=
NONE
BLANK
SOLID
DASH
TEXT
DIV options
DIV options
DIVWIDTH=
MAX
MIN
FORMAT= START
CENTER
END
TEXT=divider-text
Parameters
MARGINW=1 | n
This attribute defines a margin along the left and right sides of the panel area. This attribute allows
you to specify the width of the margin in characters. The minimum value you can specify is 1 and the
maximum value is 32. If you do not specify a value, the margin is set to 1.
The MARGINW cannot be larger than one half the panel width minus 2. Specification of the MARGINW
should always allow enough room in the panel body section of the ISPF panel being generated to
contain all non-wrapped data without truncation. Specification of one half the panel width minus 2
results in no panel area in which panel body text can be written.
MARGIND=0
This attribute defines a margin along the top and bottom of the panel area.
The conversion utility only supports a margin depth of zero in an effort to use all of the available space
on the panel body. Any definition of margin depth that is not equal to zero is changed to zero.
INDENT=n
This attribute defines the number of columns to indent the current AREA from the current MARGINW
value.
DEPTH=n | *
This attribute defines the minimum size of a scrollable panel area. If DEPTH is not specified for HELP
panels, the conversion utility generates multiple HELP panels for compatibility with previous releases.
When EXTEND=OFF, the minimum DEPTH is 2 lines. When EXTEND=ON, the minimum DEPTH is
AREA
190  z/OS: z/OS ISPF DTL Guide

## Page 223

1 line. When DEPTH=*, the conversion utility reserves the remaining available panel depth for the
scrollable area.
EXTEND=OFF | ON | FORCE
This attribute defines the runtime display size for the scrollable area. If EXTEND=ON is specified,
the panel definition is expanded from the minimum DEPTH to the size of the logical screen. Only
one EXTEND=ON attribute value is allowed on a panel. The first tag (AREA, DA, GA, REGION,
SELFLD) with EXTEND=ON is accepted; the EXTEND attribute on any subsequent AREA tag is
ignored.
If you intend to display the panels in a pop-up window, it is recommended that you code
EXTEND=OFF.
If the EXTEND attribute is specified without a DEPTH attribute, a warning message is issued and
the EXTEND attribute is ignored.
If EXTEND=FORCE is specified within a horizontal area, the EXTEND(ON) keyword is added to
the scrollable area attribute statement in the )ATTR panel section. The conversion utility issues
a message to advise of a potential error if other panel fields are formatted on or after the last
defined line of the scrollable area.
DIV=NONE | BLANK | SOLID | DASH | TEXT
This attribute specifies the type of divider line to be placed before and after the scrollable area.
If this attribute is not specified, or has the value NONE, no divider line is generated. The value
BLANK produces a blank line. You must specify SOLID, DASH, or TEXT to produce a visible divider
line. When the GRAPHIC invocation option is specified, SOLID produces a solid line for host
display and DASH produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH
produce a dashed line. A visible divider formats with a non-displayable attribute byte on each end
of the line.
If the DIV attribute is found without the DEPTH attribute, a warning message is issued and the DIV
attribute is ignored.
DIVWIDTH=MAX | MIN
This attribute specifies the width of the divider line. If DIVWIDTH=MAX, the divider line
extends across the entire width of the panel defined by the AREA tag. If DIVWIDTH=MIN, the
divider line is formatted to allow for the MARGINW and INDENT attribute values.
FORMAT=START | CENTER | END
This attribute specifies the position of the divider-text within the divider line. You must specify
both the FORMAT attribute and the TEXT attribute to create a divider line containing text.
TEXT=divider-text
This attribute specifies the text to be placed on the divider line. You must specify both the
FORMAT attribute and the TEXT attribute to create a divider line containing text.
WIDTH=n
This attribute defines the width of a panel area. If WIDTH is not specified the area formats to the
remaining available panel width.
DIR=VERT | HORIZ
This attribute allows areas to be placed side by side on the panel. You use the WIDTH attribute in
combination with the DIR attribute to tell the conversion utility to position areas horizontally. When
the current horizontal AREA right boundary reaches or exceeds the right panel boundary, the next
AREA is formatted below the current AREA(s), at the left edge of the panel.
Comments
The AREA tag defines portions of a panel body. The conversion utility uses the DEPTH attribute value to
reserve lines in the formatted panel body for a scrollable area. The DEPTH value cannot be more than the
number of panel body lines still available for formatting when the AREA tag is processed.
AREA
Chapter 12. Tag reference  191

## Page 224

The maximum DEPTH that you can specify is 2 lines less than the DEPTH value specified on the HELP or
PANEL tag.
Note:
1. If you specify the CMDAREA tag within your DTL source file, it must appear before the AREA tag when
DEPTH=* is specified. The AREA tag DEPTH may have to be adjusted to allow for additional lines which
result from tags present within the panel definition following the end AREA tag.
2. For HELP panels, the presence of additional tags following a scrollable area causes the conversion
utility to reserve 4 lines at the bottom of the screen to provide for the function key area.
The first line of the scrollable area is always reserved for the scrolling indicator line, which is provided by
ISPF at run time. If all of the scrollable portion of the panel is displayed within the available DEPTH, the
scroll indicator line is blank; otherwise, the value "More:   +", "More:   - +", or "More:  -" appears. On
application panels, this portion includes the interactive fields and text of the panel. On help panels, this
portion is the area of the panel that contains help text.
The DIR attribute is used to place entire areas side by side on the panel. Formatting within the AREA tag is
always in a vertical direction. Panel areas are formatted horizontally when multiple AREA tags (specifying
DIR=HORIZ) are placed sequentially in the DTL source file. Any other tag which occurs between an end
AREA tag and a start AREA tag causes the overall panel formatting direction to be set to vertical.
Restrictions
• The AREA tag requires an end tag.
• You must code AREA tags within a HELP or PANEL definition. See “HELP (Help Panel)” on page 303 and
“PANEL (Panel)” on page 376 for descriptions of these tags.
• Only one area can be defined with EXTEND=ON. This includes other AREA tags as well as any dynamic
area defined by the DA tag, graphic area defined by the GA tag, scrollable section lists defined by the
SELFLD tag, or scrollable regions defined by the REGION tag.
Processing
Application panel
Table 5. Tags you can code within an AREA definition  on an application panel
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
DA “DA (Dynamic Area)” on page 250 Multiple No
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
DTACOL “DTACOL (Data Column)” on page 269 Multiple No
DTAFLD “DTAFLD (Data Field)” on page 275 Multiple No
GA * “GA (Graphic Area)” on page 295 Single No
GENERATE “GENERATE (Generate)” on page 298 Multiple No
GRPHDR “GRPHDR (Group Header)” on page 300 Multiple No
INFO “INFO (Information Region)” on page 317 Multiple No
LSTFLD * “LSTFLD (List Field)” on page 341 Single No
PNLINST “PNLINST (Panel Instruction)” on page 396 Multiple No
REGION “REGION (Region)” on page 405 Multiple No
SELFLD “SELFLD (Selection Field)” on page 421 Multiple No
AREA
192  z/OS: z/OS ISPF DTL Guide

## Page 225

Table 5. Tags you can code within an AREA definition  on an application panel (continued)
Tag Reference Usage Required
SOURCE “SOURCE (Source)” on page 435 Multiple No
Note: Tags marked with * are not valid within an ISPF selection menu panel.
Help panel
Table 6. Tags you can code within an AREA definition  on a help panel
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
GENERATE “GENERATE (Generate)” on page 298 Multiple No
INFO “INFO (Information Region)” on page 317 Multiple No
REGION “REGION (Region)” on page 405 Multiple No
Examples
Here is an example application panel that contains four data fields and two selection fields coded within
the AREA definition. The top instructions and command area are coded outside of the AREA definition. In
addition, the panels illustrate a scrollable panel. Figure 88 on page 194, Figure 89 on page 194 and Figure
90 on page 195, show the formatted results.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar2 system>
  <!entity sampabc system>)>
&sampvar2;
<PANEL NAME=area1 KEYLIST=keylxmp>File-A-Case
<AB>
&sampabc;
</AB>
<TOPINST COMPACT>
         Type in client's name and case number (if applicable).
<TOPINST>Then select an action bar choice.
<AREA>
  <DTAFLD DATAVAR=caseno PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25>Case no
    <DTAFLDD>(A 7-digit number)
  <DTAFLD DATAVAR=name PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25>Name
    <DTAFLDD>(Last, First, M.I.)
  <DTAFLD DATAVAR=address PMTWIDTH=12 ENTWIDTH=25>Address
  <DIVIDER>
  <SELFLD NAME=casesel PMTWIDTH=30 PMTLOC=before SELWIDTH=38>Choose
  one of the following
    <CHOICE CHECKVAR=case MATCH=civ>Civil
    <CHOICE CHECKVAR=case MATCH=estate>Real estate
    <CHOICE CHECKVAR=case MATCH=environ>Environmental
  </SELFLD>
</AREA>
<AREA DEPTH=6>
  <SELFLD TYPE=multi PMTWIDTH=35 SELWIDTH=50>Check type of offense committed
    <CHOICE NAME=patin HELP=patin CHECKVAR=val>Patent infringement
    <CHOICE NAME=defa HELP=defame CHECKVAR=def>Defamation
    <CHOICE NAME=cont HELP=cont CHECKVAR=qua>Breach of valid contract
    <CHOICE NAME=priv HELP=priv CHECKVAR=pri>Invasion of privacy
    <CHOICE NAME=incr HELP=incr CHECKVAR=icr>Interference with
            contractual relations
    <CHOICE NAME=disp HELP=disp CHECKVAR=dis>Improper disposal of
            medical by-products
    <CHOICE NAME=fraud HELP=fraud CHECKVAR=fra>Fraud
  </SELFLD>
</AREA>
AREA
Chapter 12. Tag reference  193

## Page 226

<CMDAREA>Enter a command
</PANEL>
   File  Search  Help
 -------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
 Case no  . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following    __  1.  Civil
                                    2.  Real estate
                                    3.  Environmental
                                                            More:     +
 Check type of offense committed
 _  Patent infringement
 _  Defamation
 _  Breach of valid contract
 Enter a command ===> ____________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 88. Application panel area
After scrolling, the panel looks like this:
   File  Search  Help
 -------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
 Case no  . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following    __  1.  Civil
                                    2.  Real estate
                                    3.  Environmental
                                                            More:   - +
 _  Breach of valid contract
 _  Invasion of privacy
 _  Interference with contractual relations
 _  Improper disposal of medical by-products
 Enter a command ===> ____________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 89. Application panel area
After scrolling, the last choice in the list is visible.
AREA
194  z/OS: z/OS ISPF DTL Guide

## Page 227

File  Search  Help
 -------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
 Case no  . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following    __  1.  Civil
                                    2.  Real estate
                                    3.  Environmental
                                                            More:   -
 _  Invasion of privacy
 _  Interference with contractual relations
 _  Improper disposal of medical by-products
 _  Fraud
 Enter a command ===> ____________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 90. Application panel area
An example of horizontal AREA formatting is shown in “Multiple AREA tags” on page 42.
ASSIGNI (Assignment List Item)
The ASSIGNI tag defines an assignment in an assignment list.
Syntax
<ASSIGNI
VALUE=test-value RESULT=assigned-value
>
</ASSIGNI>
Parameters
VALUE=test-value
This attribute specifies the value to be matched when performing the assignment.
The value of the data field variable is compared to the value of each VALUE attribute in succession
until a match is found. The test-value must be the same case as the value to be matched. You can
specify XLATL FORMAT=UPPER within the variable class associated with the data field to convert user
input to uppercase before the assignment test is processed.
When ISPF finds a match, it assigns the value in the RESULT attribute to the variable named on the
ASSIGNL tag. If ISPF does not find a match, no assignment is made.
If you omit this attribute, any value satisfies the test and ISPF assigns assigned-value to the dialog
variable.
If a test-value appears more than once in the list, the first occurrence is used.
RESULT=assigned-value
This attribute specifies the resulting value of the assignment if a match occurs on the test-value
specified by VALUE.
ASSIGNI
Chapter 12. Tag reference  195

## Page 228

Assigned-value specifies the character string value for the conversion utility to assign to the variable
named on the ASSIGNL tag. If you omit this attribute, the test-value is assigned to the variable.
Comments
The ASSIGNI tag defines an assignment in an assignment list. Each ASSIGNI tag provides information
necessary to assign a value (the RESULT attribute) to a variable (specified with the ASSIGNL tag) based on
the test-value (the VALUE attribute) of the variable named on the DTAFLD tag. As many ASSIGNI tags as
are necessary (up to a limit of 126) can be included within the assignment list. The ISPF TRANS() function
is used for ASSIGNI processing.
If both the VALUE and RESULT attributes are omitted, the DESTVAR attribute of the ASSIGNL tag is
assigned the value of the data field's data variable (DATAVAR).
Restrictions
• You must code an ASSIGNI tag within an ASSIGNL definition. See “ASSIGNL (Assignment List)” on page
196 for a complete description of this tag.
Processing
None.
Examples
Here is source file markup that contains an application panel containing a data field. Within the data field
is an assignment list that sets the dialog variable rmtype to 1 when "SINGLE" is entered in the data field,
or to 2 when "DOUBLE" is entered in the data field. The associated variable declarations and variable
classes are also shown.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=roomvar TYPE='char 6'>
  <XLATL FORMAT=upper>
  </XLATL>
<VARCLASS NAME=rmtypvar TYPE='char 1'>
<VARLIST>
  <VARDCL NAME=room   VARCLASS=roomvar>
  <VARDCL NAME=rmtype VARCLASS=rmtypvar>
</VARLIST>
<PANEL NAME=assigni DEPTH=12 WIDTH=50>Hotel Register
<AREA>
  <DTAFLD DATAVAR=room ENTWIDTH=6 DESWIDTH=20 PMTWIDTH=15>Room type
    <ASSIGNL DESTVAR=rmtype>
    <ASSIGNI VALUE=SINGLE RESULT=1>
    <ASSIGNI VALUE=DOUBLE RESULT=2>
    </ASSIGNL>
    <DTAFLDD>(Single or Double)
</AREA>
<BOTINST>Press Enter to continue.
</PANEL>
ASSIGNL (Assignment List)
The ASSIGNL tag defines an assignment list.
ASSIGNL
196  z/OS: z/OS ISPF DTL Guide

## Page 229

Syntax
<ASSIGNL DESTVAR=destination-variable-name > </ASSIGNL>
Parameters
DESTVAR=destination-variable-name
DESTVAR specifies the variable that receives the assignment value. You can code multiple assignment
lists if you need to assign values to additional variables.
Note: If the destination-variable-name is a variable name used for another field on the panel, the
value of the other field is overlaid by the assignment value. The destination-variable-name should only
be used for an output field variable.
Comments
The ASSIGNL tag defines an assignment list. ASSIGNI tags, which define the elements of the assignment
list, are coded within the ASSIGNL tag.
Assignment lists are optional and provide a means of assigning a value to one variable based on the
content of another. ISPF compares the value of the variable specified with the DATAVAR attribute of the
DTAFLD tag against the values in the ASSIGNI tags.
Processing of assignment lists occurs at panel end (after translates and checks).
Restrictions
• The ASSIGNL tag requires an end tag.
• You must code an ASSIGNL tag within the DTAFLD definition it is associated with. See “DTAFLD (Data
Field)” on page 275 for a complete description of this tag.
Processing
Table 7. Tags you can code within an ASSIGNL definition 
Tag Reference Usage Required
ASSIGNI “ASSIGNI (Assignment List Item)” on page 195 Multiple No
Examples
In this example markup, the assignment list assigns a value to the variable decimal dependent on the
value the user enters in the data field variable hexvar. The associated variable declarations and variable
classes are also shown.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=varcls1 TYPE='char 1'>
<VARCLASS NAME=varcls2 TYPE='char 2'>
<VARLIST>
  <VARDCL NAME=hexvar  VARCLASS=varcls1>
  <VARDCL NAME=decimal VARCLASS=varcls2>
</VARLIST>
<PANEL NAME=assignl>Hex to Decimal
<TOPINST>Enter a hexadecimal digit.
<AREA>
<DTAFLD DATAVAR=hexvar PMTWIDTH=23 ENTWIDTH=1>Hexadecimal Value
  <ASSIGNL DESTVAR=decimal>
ASSIGNL
Chapter 12. Tag reference  197

## Page 230

<ASSIGNI VALUE=0>
    <ASSIGNI VALUE=1>
    <ASSIGNI VALUE=2>
    <ASSIGNI VALUE=3>
    <ASSIGNI VALUE=4>
    <ASSIGNI VALUE=5>
    <ASSIGNI VALUE=6>
    <ASSIGNI VALUE=7>
    <ASSIGNI VALUE=8>
    <ASSIGNI VALUE=9>
    <ASSIGNI VALUE=a RESULT=10>
    <ASSIGNI VALUE=b RESULT=11>
    <ASSIGNI VALUE=c RESULT=12>
    <ASSIGNI VALUE=d RESULT=13>
    <ASSIGNI VALUE=e RESULT=14>
    <ASSIGNI VALUE=f RESULT=15>
    <ASSIGNI RESULT="??">
  </ASSIGNL>
<DTAFLD DATAVAR=decimal USAGE=out PMTWIDTH=23 ENTWIDTH=2>Decimal Value
</AREA>
</PANEL>
ATTENTION (Attention)
The ATTENTION tag defines text that alerts the user to a risk of possible error conditions in the system.
Syntax
<ATTENTION>
text
</ATTENTION>
Parameters
text
This is the text of the attention message.
Comments
The ATTENTION tag defines text that alerts the user to a risk of possible error conditions in the system.
The ATTENTION tag is one of the tags that alert the user of a possible risk; CAUTION and WARNING are
the others.
Code an attention statement before the text to which it pertains so that the user can read about the
possible risks before reading the text.
When an attention statement is displayed, the string "Attention:" (or its translated equivalent) appears on
the screen before the text of the statement.
You can code additional paragraphs of text by coding the P (paragraph) tag within an ATTENTION
definition. You can also code other tags allowed in an information area within an ATTENTION definition.
Restrictions
• The ATTENTION tag requires an end tag.
• You must code the ATTENTION tag within an INFO definition. See “INFO (Information Region)” on page
317 for a complete description of this tag.
• The ATTENTION tag must be immediately preceded by a P, LI, or LP tag. If the ATTENTION tag is coded
on the same line as one of these tags, there can be no intervening blanks. See “P (Paragraph)” on page
370, “LI (List Item)” on page 325, and “LP (List Part)” on page 330 for descriptions of these tags.
ATTENTION
198  z/OS: z/OS ISPF DTL Guide

## Page 231

• You cannot nest ATTENTION, WARNING or CAUTION tags within each other.
Processing
Table 8. Tags you can code within an ATTENTION definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains a warning statement. The warning statement starts at the left
margin because it is embedded in the LP tag.
<!DOCTYPE DM SYSTEM>
<HELP NAME=attentn DEPTH=20>Help For Changing a File
<AREA>
<INFO>
  <OL>
    <LI>Type over the existing data
    in the entry fields with the new data.
      <LP><ATTENTION>Performing the next step will save
      all changes and delete the existing data.
      <P>To quit this function without
      deleting the existing data, press F12.
      </ATTENTION>
    <LI>Press Enter to save the
    updated data.
  </OL>
</INFO>
</AREA>
</HELP>
ATTENTION
Chapter 12. Tag reference  199

## Page 232

Help For Changing a File
 1.  Type over the existing data in the entry
     fields with the new data.
 Attention: Performing the next step will save all
 changes and delete the existing data.
 To quit this function without deleting the
 existing data, press F12.
 2.  Press Enter to save the updated data.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 91. Attention statement
ATTR (Attribute)
The ATTR tag defines a panel attribute used within a dynamic area or a preformatted displayable
panel section. See the z/OS ISPF Dialog Developer's Guide and Reference for a complete discussion of
panel )ATTR section keywords.
ATTR
200  z/OS: z/OS ISPF DTL Guide

## Page 233

Syntax
<ATTRATTRCHAR=codeTYPE= DATAIN
DATAOUT
CHAR
INTENS=
HIGH
LOW
NON
%varname
CAPS= OFF
ON
IN
OUT
%varname
JUST= ASIS
LEFT
RIGHT
%varname
PAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
SKIP=
OFF
ON
%varname
GE=
OFF
ON
%varname
COLOR= WHITE
RED
BLUE
GREEN
PINK
YELLOW
TURQ
%varname
HILITE= USCORE
BLINK
REVERSE
%varname
NUMERIC=
OFF
ON
%varname
FORMAT= EBCDIC
DBCS
MIX
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
PAS=
OFF
ON
%varname
CKBOX=
OFF
ON
%varname
CUADYN= CEF
EE
LEF
NEF
VOI
LID
LI
CH
CT
DT
ET
FP
NT
PIN
PT
SAC
SI
SUC
WASL
WT
%varname
CSRGRP=
NO
YES
n
ATTN=
OFF
ON
%varname
>
</ATTR>
ATTR
Chapter 12. Tag reference  201

## Page 234

Parameters
ATTRCHAR=code
This attribute can be a single character or a two-position entry of valid hex digits. If you enter an
incorrect value, a warning message is issued and the value is set to null. Hex entries are converted to
character. Hex values ‘00’-‘2F’ are reserved for use by the conversion utility.
TYPE=DATAIN | DATAOUT | CHAR
This attribute specifies the characteristic of the field within the dynamic area. Use DATAIN and
DATAOUT attribute values for specifying unprotected or protected fields, respectively, within the
dynamic area. The CHAR attribute value defines a character attribute that is recognized only when
found within a shadow variable.
When the ATTR tag is coded within the GENERATE tag, TYPE can also be specified as any CUA
attribute type, or as %varname.
INTENS=HIGH | LOW | NON | %varname
This attribute defines the intensity of a field. You can define this attribute as a variable name preceded
by a “%”.
CAPS=OFF | ON | IN | OUT | %varname
This attribute specifies the uppercase or lowercase attribute of a field. You can define this attribute as
a variable name preceded by a “%”.
JUST=ASIS | LEFT | RIGHT | %varname
This attribute specifies how the contents of the field are to be justified when displayed. You can define
this attribute as a variable name preceded by a “%”.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PADC=NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
SKIP=OFF | ON | %varname
This attribute specifies the autoskip attribute of the field. You can define this attribute as a variable
name preceded by a “%”.
GE=OFF | ON | %varname
This attribute specifies whether ISPF places a graphic escape order before the character defined as a
character level attribute by TYPE=CHAR. You can define this attribute as a variable name preceded by
a “%”.
COLOR=WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute specifies the color of the field. You can define this attribute as a variable name preceded
by a “%”.
HILITE=USCORE | BLINK | REVERSE | %varname
This attribute specifies the extended highlighting attribute of the field. You can define this attribute as
a variable name preceded by a “%”.
NUMERIC=OFF | ON | %varname
This attribute specifies whether Numeric Lock is to be activated for data typed in the field. You can
define this attribute as a variable name preceded by a “%”.
FORMAT=EBCDIC | DBCS | MIX | %varname
This attribute specifies the character format for the field. You can define this attribute as a variable
name preceded by a “%”.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
PAS=OFF | ON | %varname
This attribute controls the availability of the point-and-shoot function for dynamic areas. You can
define this attribute as a variable name preceded by a “%”.
ATTR
202  z/OS: z/OS ISPF DTL Guide

## Page 235

CKBOX=OFF | ON | %varname
This attribute controls the generation of check boxes for dynamic areas when the panel is displayed
by a client that is using the JSON API. You can define this attribute as a variable name preceded by
“%”.
CUADYN=CEF | EE | LEF | NEF | VOI | LID | LI | CH | CT | DT | ET | FP | NT | PIN | PT | SAC | SI | SUC |
WASL | WT | %varname
This attribute specifies a standard CUA attribute for the DATAIN and DATAOUT panel attribute
definitions.
Values CEF, EE, LEF, and NEF are valid when TYPE=DATAIN. The remaining values are valid when
TYPE=DATAOUT. You can define this attribute as a variable name preceded by a “%”.
The conversion utility issues a warning message if the CUADYN attribute is specified and the
invocation option is NOCUAATTR.
Note: If you specify other attribute before the CUADYN attribute, the CUADYN attribute can override
previously specified attributes. For example:
SKIP=ON CUADYN=FP
In the above example, CUADYN changes the SKIP attribute to OFF.
CSRGRP=NO | YES | n
The CSRGRP attribute is valid only when TYPE=DATAOUT. When CSRGRP=YES, the conversion utility
generates a cursor group number to be used for this DATAOUT attribute. When CSRGRP=n, the
number provided is used for this attribute. The PAS attribute must be specified as ON or %varname.
ATTN=NO | YES | %varname
This attribute specifies the attention-select attribute of the field. You can define this attribute as a
variable name preceded by a "%".
Comments
The ATTR tag is used to create an entry in the )ATTR panel section for fields to be displayed within a
dynamic area, or preformatted panel section.
Restrictions
• You must code an ATTR tag within a Dynamic Area or GENERATE tag definition. See “DA (Dynamic
Area)” on page 250 and “GENERATE (Generate)” on page 298 for a complete description of these tags.
• If ATTRCHAR is not specified, an error is logged and the output panel is not saved.
• If ATTRCHAR is a duplicate of a previously specified attribute, or conflicts with an attribute reserved by
the conversion utility, an error is logged and the output panel is not saved.
• If TYPE is not specified, an error is logged and the output panel is not saved. If TYPE is specified, but the
value is invalid, the value is set to DATAIN.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
None.
Examples
ATTR
Chapter 12. Tag reference  203

## Page 236

<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=attr KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST> Type in patron's name and card number (if applicable)
<AREA>
  <DTACOL PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25 SELWIDTH=25>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8>Date
    <DTAFLD DATAVAR=cardno ENTWIDTH=7>Card No.
      <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name>Name
      <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address>Address
  </DTACOL>
  <DIVIDER>
  <DA NAME=darea DIV=solid DEPTH=6 SHADOW=shadwvar>
    <ATTR ATTRCHAR=#  TYPE=datain    PADC='_'  COLOR=BLUE>
    <ATTR ATTRCHAR=|  TYPE=dataout   COLOR=green>
    <ATTR ATTRCHAR=$  TYPE=char      COLOR=red>
  </DA>
</AREA>
<CMDAREA>Enter a command
</PANEL>
BOTINST (Bottom Instruction)
The BOTINST tag defines bottom instructions for an application panel.
Syntax
<BOTINST
COMPACT
>
instruction-text </BOTINST>
Parameters
COMPACT
This attribute causes the bottom instruction to format without a blank line before the text.
instruction-text
This is the text of the bottom instruction. The instruction-text must fit in the remaining panel depth.
Comments
The BOTINST tag defines bottom instructions for an application panel. The instruction-text formats as a
paragraph based on the width of the application panel. You can code multiple paragraphs of instruction
text by using a new bottom instruction tag for each new paragraph.
If the COMPACT attribute is not specified, the conversion utility inserts a blank line before the bottom
instruction text.
Restrictions
• You must code the BOTINST within a PANEL definition. See “PANEL (Panel)” on page 376 for a complete
description of this tag.
• You cannot code a BOTINST tag within an AREA definition. If you define an area for the panel, code the
BOTINST tag after the AREA end tag.
BOTINST
204  z/OS: z/OS ISPF DTL Guide

## Page 237

Processing
Table 9. Tags you can code within a BOTINST definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
This application panel markup contains two bottom instructions. Figure 92 on page 205 shows the
formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=choiccls TYPE='char 2'>
<VARLIST>
  <VARDCL NAME=choices VARCLASS=choiccls>
</VARLIST>
<PANEL NAME=botinst1 WIDTH=35 DEPTH=22>Four Choices
<AREA>
  <SELFLD NAME=choices PMTWIDTH=20 SELWIDTH=33>Choose one:
    <CHOICE>Raindrops on roses
    <CHOICE>Whiskers on kittens
    <CHOICE>Bright copper kettles
    <CHOICE>Warm woolen mittens
  </SELFLD>
</AREA>
<BOTINST>Press Enter to continue.
<BOTINST>Press F12 to cancel.
</PANEL>
           Four Choices
 Choose one:
 __  1.  Raindrops on roses
     2.  Whiskers on kittens
     3.  Bright copper kettles
     4.  Warm woolen mittens
 Press Enter to continue.
 Press F12 to cancel.
  F1=Help    F3=Exit   F12=Cancel
Figure 92. Bottom instructions
CAUTION (Caution)
The CAUTION tag defines a statement that alerts the user of a possible risk.
CAUTION
Chapter 12. Tag reference  205

## Page 238

Syntax
<CAUTION>
text
</CAUTION>
Parameters
text
This is the text of the caution statement.
Comments
The CAUTION tag defines a statement that alerts the user of a possible risk. Use the CAUTION tag to alert
the user to a condition that might have serious consequences, such as the deletion of a file.
The CAUTION tag is one of the tags that alert the user to a possible risk; the others are ATTENTION and
WARNING.
Code a caution statement before the text to which it pertains so that the user can read about the
possible risks before reading the text. When a caution statement is displayed, the string "CAUTION:" or its
translated equivalent appears on the screen and the caution text displays on the line after.
You can code additional paragraphs of caution text by coding the P (paragraph) tag within a CAUTION
definition. You can also code other tags allowed in an information area within a CAUTION definition.
CAUTION text is formatted with an attribute byte that causes it to be emphasized.
Restrictions
• The CAUTION tag requires an end tag.
• A CAUTION tag must be immediately preceded by an LI, LP, or P tag. See “LI (List Item)” on page 325,
“LP (List Part)” on page 330, and “P (Paragraph)” on page 370 for descriptions of these tags.
• You must code the CAUTION tag only within an INFO definition. See “INFO (Information Region)” on
page 317 for a complete description of this tag.
• You cannot nest ATTENTION, CAUTION, or WARNING tags within each other.
Processing
Table 10. Tags you can code within a CAUTION definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
CAUTION
206  z/OS: z/OS ISPF DTL Guide

## Page 239

Table 10. Tags you can code within a CAUTION definition  (continued)
Tag Reference Usage Required
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains a caution statement. Figure 93 on page 207 shows the formatted
result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=caution DEPTH=20>Help for DELETE Command
<AREA>
  <INFO>
    <P>The DELETE command erases the specified file from storage.
    <P><CAUTION>
    Issuing the DELETE command permanently
    removes the file from storage.
    There is no possibility of recovery.
    </CAUTION>
    <P>You can exit from the DELETE operation
    by pressing F12.
  </INFO>
</AREA>
</HELP>
             Help for DELETE Command
 The DELETE command erases the specified file
 from storage.
 CAUTION:
 Issuing the DELETE command permanently removes
 the file from storage. There is no possibility
 of recovery.
 You can exit from the DELETE operation by
 pressing F12.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 93. Caution statement
CHDIV (Choice Divider)
The CHDIV tag creates a blank or visible divider between CHOICE tags.
CHDIV
Chapter 12. Tag reference  207

## Page 240

Syntax
<CHDIV
TYPE=
NONE
SOLID
DASH
TEXT
GUTTER=
1
n
FORMAT=
START
CENTER
END
>
divider-text </CHDIV>
Parameters
TYPE=NONE | SOLID | DASH | TEXT
This attribute specifies the type of divider line. The line width is one character.
The default value is NONE, which produces a blank line. You must specify SOLID, DASH, or TEXT to
produce a visible divider line. When the GRAPHIC invocation option is specified, SOLID produces a
solid line for host display and DASH produces a dashed line. When NOGRAPHIC is specified, both
SOLID and DASH produce a dashed line.
GUTTER=1 | n
This attribute specifies the total width of the divider. If the GUTTER value is an even number, the
conversion utility increases the number by 1 so that the divider is centered within the defined width.
The minimum and default GUTTER value is 1.
FORMAT=START | CENTER | END
This attribute specifies the position of the divider text within the width of the divider line. The divider
width is the same as the CHOICE tag text formatting width.
divider-text
This is the text of the choice divider.
Comments
The CHDIV tag creates a blank or solid divider between CHOICE tags.
Restrictions
• You must code the CHDIV tag within an SELFLD definition. See “SELFLD (Selection Field)” on page 421
for a description of this tag.
Processing
Table 11. Tags you can code within a CHDIV definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
Examples
CHDIV
208  z/OS: z/OS ISPF DTL Guide

## Page 241

Here is an example that shows the creation of an ISPF selection menu. The CHDIV tag is included to
separate the Exit option from the other selection choices. The FCHOICE attribute specifies that the first
selection number is 0. The choice selection for Exit is specified on the CHOICE tag. The ACTION tag for
the Exit choice selection specifies both the RUN and TYPE attributes because RUN is required on the
ACTION tag and TYPE is necessary to specify the ISPF selection for the generated ZSEL panel statement.
<!doctype dm system ()>
<!--  Sample selection menu -->
<varclass name=vc1 type='char 80'>
  <xlatl format=upper>
  </xlatl>
<varlist>
  <vardcl name=zcmd varclass=vc1>
</varlist>
<panel name=chdiv1 menu keylist=keylxmp>
       Sample Selection Panel with CHDIV tag
  <topinst>This is a selection panel.
  <selfld type=menu   pmtloc=before fchoice=0 trail=nextsel
          selwidth=40 pmtwidth=10>Select an option
    <choice checkvar=xtest1 match=a>
            Selection #0 (Command Selch0)
      <action run=Selch0>
    <choice checkvar=xtest1 match=b>
            Selection #1 (Command Selch1)
      <action run=Selch1 parm='1 2 3 4'
       passlib newpool suspend>
    <choice checkvar=xtest1 match=c>
            Selection #2 (Command Selch2)
      <action run=Selch2 parm=1234>
    <choice checkvar=xtest1 match=d>
            Selection #3 (Command Selch3)
      <action run=Selch3 parm=abcd>
    <choice checkvar=xtest1 match=e>
            Selection #4 (Command Selch4)
      <action run=Selch4 parm='a b c d'>
    <chdiv>
    <choice selchar=X>
            Exit
      <action run=exit type=exit>
  </selfld>
  <cmdarea>
</panel>
Figure 94 on page 209 shows the formatted result.
                   Sample Selection Panel with CHDIV tag
 Option ===> _____________________________________________________________
 
 This is a selection panel.
 
 Select an
 option . . 0  Selection #0 (Command Selch0)
            1  Selection #1 (Command Selch1)
            2  Selection #2 (Command Selch2)
            3  Selection #3 (Command Selch3)
            4  Selection #4 (Command Selch4)
 
            X  Exit
 
 
 
 
 
 
 Option ===> _____________________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 94. Choice divider
CHDIV
Chapter 12. Tag reference  209

## Page 242

CHECKI (Validity Check Item)
The CHECKI tag defines a test of an input value.
Syntax
<CHECKI TYPE= First set of keywords
First set of keywords
CHECKI
210  z/OS: z/OS ISPF DTL Guide

## Page 243

RANGE PARM1= low-bound
%varname
PARM2= high-bound
%varname
ALPHA
CHARS PARM1=EQ PARM2=character-set
VALUES PARM1=EQ PARM2=value-list
VALUESX PARM1=NE PARM2=value-list
BIT
NAME
NAMEF
PICT PARM1=EQ PARM2=pictstring
PICTCN PARM1=mask-character PARM2=field-mask PARM3=string
NUM
DBCS
LISTV PARM1=EQ PARM2=%varlist
LISTVX PARM1=NE PARM2=%varlist
ALPHAB
LEN PARM1= operator
%varname
PARM2= length
%varname
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
PARM2= ALPHA
ALPHAB
NUM
PARM3= ALPHA
ALPHAB
NUM
IDATE
STDDATE
JDATE
JSTD
ITIME
STDTIME
IPADDR4
>
</CHECKI>
CHECKI
Chapter 12. Tag reference  211

## Page 244

Parameters
TYPE=
This attribute specifies the type of check to be performed. The valid types are:
RANGE
This allows you to check for an integer value within a range. The specified range includes the end
points. The range delimiters can include 16 digits. The range delimiters can also contain a sign (-
or +). If no sign is coded, the value is assumed to be positive.
Important: In ISPF, the VER(variable RANGE,lower,upper) statement limits the length of the
specified variable to 16 digits. If you specify the CHECKI TYPE=RANGE on a variable that is longer
than 16 positions, the variable may not be correctly validated. For example, assume an application
developer defines a data field with a length of 20 and defines this validity check for the field:
<CHECKI TYPE=RANGE PARM1=1 PARM2=9999999999999999>
If the number 12345678901234567890 were entered into the field, only the first 16 digits would
be verified and the number would be within the defined range, even though the entire number
entered is outside of the defined range.
PARM1=low-bound | %varname
This attribute supplies the low value, if any or the name of a variable that contains the
low value. If you do not supply a value, the default is "-" followed by sixteen 9s (that is,
-9999...99). Negative values must be coded with the minus sign on the left.
ISPF restrictions on the VER(variable RANGE,lower,upper) apply. The lower value specified in
PARM1 can be positive or negative. The length of the lower limit is limited to 16 digits, in
addition to the plus or minus sign if used.
PARM2=high-bound | %varname
This attribute supplies the high value, if any or the name of a variable that contains the high
value. If you do not supply a value, the default is sixteen 9s (that is, 9999...99). Negative
values must be coded with the minus sign on the left.
ISPF restrictions on the VER(variable RANGE,lower,upper) apply. The upper value specified in
PARM2 can be positive or negative. The length of the upper limit is limited to 16 digits, in
addition to the plus or minus sign if used.
ALPHA
This limits the character set to A-Z, a-z, and #, $, @. The conversion utility builds the VER(variable
ALPHA) statement.
CHARS
Specifies the CHARS check of characters allowed within an input string.
The conversion utility uses the TYPE=CHARS check to support ISPF VER(variable BIT),
VER(variable HEX) and VER(variable NUM). BIT, HEX, and NUM are the only types of support
provided by ISPF for the TYPE=CHARS check.
PARM1=EQ
This attribute contains EQ to specify that PARM2 contains a value that must be matched.
If PARM1 contains any other value, the check is ignored and the conversion utility issues a
warning message.
PARM2=character-set
This attribute specifies a set of characters to be matched.
• If TYPE=CHARS, PARM1=‘EQ’, and PARM2=‘01’, the conversion utility generates
VER(variable BIT).
• If TYPE=CHARS, PARM1=‘EQ’, and PARM2=‘0123456789ABCDEFabcdef’, the conversion
utility generates VER(variable HEX).
• If TYPE=CHARS, PARM1=‘EQ’, and PARM2=‘0123456789’, the conversion utility generates
VER(variable NUM).
CHECKI
212  z/OS: z/OS ISPF DTL Guide

## Page 245

Note: If one of these options is used, the PARM2 value must be exactly as specified. If PARM2
contains any other value, the check is ignored and the conversion utility issues a warning
message.
VALUES
Specifies that the value entered must be the same as one of the values specified in PARM2.
The VER LIST statement built by the conversion utility is case-sensitive to the values entered in
PARM2 (no folding of PARM2 to uppercase). This means that ISPF looks for an exact match in the
verification process. You can specify XLATL FORMAT=UPPER within the variable class definition
before the CHECKL tag to convert user input to uppercase before the VALUES check is processed.
PARM1=EQ
This attribute contains EQ to specify that PARM2 contains values that must be matched. If
PARM1 contains any other value, the check is ignored and the conversion utility issues a
warning message.
PARM2=value-list
This attribute specifies the list of values. If the list contains more than one value, it must be
enclosed in quotes. If a value in the list contains blanks, then it must be enclosed in single
quotes and the entire list enclosed in double quotes. Each value in the list must be separated
by blanks or enclosed quotes. For example:
dog
‘dog cat bird lion’
"parsley onion ‘black pepper’ garlic"
The maximum number of values allowed is 100.
Note: You should surround the entire value for PARM2 with double quotes and then surround
any value in the list that contains blanks with single quotes. Double quotes surrounding a list
are supported by the conversion utility.
The conversion utility generates VER(variable LIST,value-list) from PARM2.
VALUESX
Specifies that the value entered cannot be any of the values specified in PARM2. This is the
opposite of VALUES.
The VER LISTX statement built by the conversion utility is case-sensitive to the values entered in
PARM2 (no folding of PARM2 to uppercase). This means that ISPF looks for an exact match in the
verification process. You can specify XLATL FORMAT=UPPER within the variable class definition
before the CHECKL tag to convert user input to uppercase before the VALUES check is processed.
PARM1=NE
this attribute contains ne to specify that parm2 contains values that cannot be entered. If
parm1 contains any other value, the check is ignored and the conversion utility issues a
warning message.
PARM2=VALUE-LIST
This attribute specifies the list of values. If the list contains more than one value, it must be
enclosed in quotes. If a value in the list contains blanks, then it must be enclosed in single
quotes and the entire list enclosed in double quotes. Each value in the list must be separated
by blanks or enclosed quotes. For example:
dog
‘dog cat bird lion’
"parsley onion ‘black pepper’ garlic"
The maximum number of values allowed is 100.
Note: You should surround the entire value for PARM2 with double quotes and then surround
any value in the list that contains blanks with single quotes. Double quotes surrounding a list
are supported by the conversion utility.
The conversion utility generates VER(variable LISTX,value-list) from PARM2.
CHECKI
Chapter 12. Tag reference  213

## Page 246

BIT
Specifies that the variable must be all 0’s and 1’s. The conversion utility builds the VER(variable
BIT) statement.
NAME
Specifies that the variable must contain a valid name, following the rules of member names. The
conversion utility builds the VER(variable NAME) statement.
NAMEF
Specifies that the variable must contain a valid name filter. The conversion utility builds the
VER(variable NAMEF) statement.
PICT
Specifies that the variable must contain characters that match the corresponding type of
character in pictstring.
PARM1=EQ
This attribute contains EQ to specify that PARM2 contains values that must be matched. If
PARM1 contains any other value, the check is ignored and the conversion utility issues a
warning message.
PARM2=pictstring
This ‘pictstring’ parameter must be the actual value to be used in the generated VER
statement. ISPF does not support a variable name for this value.
If PARM2 contains invalid characters as defined by ISPF, the check is ignored and the
conversion utility issues a warning message.
The conversion utility builds the VER(variable PICT,pictstring) statement.
PICTCN
Specifies that the variable must contain specific constants along with other standard ISPF picture
verification characters.
PARM1=mask-character
The mask-character is any special character that represents itself. It cannot be one of the ISPF
picture string characters (C,A,N,X,9,c,a,n,x)
PARM2=field-mask
The field-mask provides the required characters for the field. All other field positions must be
represented by the mask-character. For example, if the field is to contain a string VnnRnnMnn
(for Version, Release, and Modification) and the mask-character is an asterisk (*), the field
mask is V**R**M**.
PARM3=string
The string must be the same length as the field-mask. It contains all of the required characters
in the same positions as the field-mask. The positions defined with the mask-character in
the field-mask contain one of the standard ISPF picture characters (C,A,N,X,9,c,a,n,x). To
complete the example provided for PARM2, the string is VnnRnnMnn. The resulting verification
statement is:
VER(variable,PICTCN,*,V**R**M**,VnnRnnMnn)
The variable is verified for V in position 1, R in position 4, M in position 7, and numeric
characters in positions 2,3,5,6,8, and 9.
The conversion utility builds the VER(variable,PICTCN,mask-character,field-mask,string)
statement.
NUM
Specifies that the variable must contain all numeric characters (0-9). The conversion utility builds
the VER(variable NUM) statement.
DBCS
Specifies that the variable must contain all valid DBCS characters. The conversion utility builds the
VER(variable DBCS) statement.
CHECKI
214  z/OS: z/OS ISPF DTL Guide

## Page 247

LISTV
Specifies that the variable must be one of the values provided by varlist.
PARM1=EQ
This attribute contains EQ to specify that PARM2 contains values that must be matched. If
PARM1 contains any other value, the check is ignored and the conversion utility issues a
warning message.
PARM2=%varlist
This attribute must be a variable name entered with "%" as the first character. The variable
contents are provided by the application and must be a list of valid values.
The conversion utility builds the VER(variable LISTV,&varlist) statement.
LISTVX
Specifies that the variable cannot be any of the values provided by varlist. This is the opposite of
LISTV.
PARM1=NE
This attribute contains NE to specify that PARM2 contains values that cannot be entered.
If PARM1 contains any other value, the check is ignored and the conversion utility issues a
warning message.
PARM2=%VARLIST
This attribute must be a variable name entered with "%" as the first character. The variable
contents are provided by the application and must be a valid list of excluded values.
The conversion utility builds the VER(variable LISTVX,&varlist) statement.
ALPHAB
Specifies that the variable must be all alphabetic characters (A-Z or a-z). The conversion utility
builds the VER(variable ALPHAB) statement.
LEN
Specifies that the variable must satisfy the condition expressed by the relational operator and the
expected length.
PARM1=operator | %varname
This attribute can be a relational operator (EQ, LT, GT, LE, GE, NE, NG, or NL) or a variable name
that contains a relational operator. If a variable name is entered, it must be preceded by a “%”.
PARM2=length | %varname
The parameter must be either a number or a variable name. If a number is entered, it must be
in the range of 1-99999. If a variable name is entered, it must be preceded by a "%".
The conversion utility builds the VER(variable operator,length) statement.
EBCDIC
Specifies that the variable must contain all valid EBCDIC characters. The conversion utility builds
the VER(variable EBDIC) statement.
ENUM
Specifies that the variable can contain extended numeric notation. The conversion utility builds
the VER(variable ENUM) statement.
DSNAME
Specifies that the variable must contain a valid TSO data set name. The conversion utility builds
the VER(variable DSNAME) statement.
DSNAMEF
Specifies that the variable must contain a valid TSO data set name filter. The optional member
name cannot be specified as a member pattern. A missing close parentheses and ending
quotation mark are automatically added by ISPF. The conversion utility builds the VER(variable
DSNAMEF) statement.
DSNAMEFM
Specifies that the variable must contain a valid data set name. The optional member name
can be specified as a member pattern. A missing close parentheses and ending quotation mark
CHECKI
Chapter 12. Tag reference  215

## Page 248

are automatically added by ISPF. The conversion utility builds the VER(variable DSNAMEFM)
statement.
DSNAMEPQ
Specifies that the variable must contain a valid TSO data set name. A missing close parentheses
and ending quotation mark are automatically added by ISPF. The conversion utility builds the
VER(variable DSNAMEPQ) statement.
DSNAMEQ
Specifies that the variable must contain a valid TSO data set name. A missing ending quotation
mark is automatically added by ISPF. The conversion utility builds the VER(variable DSNAMEQ)
statement.
MIX
Specifies that the variable must contain all valid DBCS and EBCDIC characters. The conversion
utility builds the VER(variable MIX) statement.
HEX
Specifies that the variable must contain all hexadecimal characters (0-9, a-f or A-F). The
conversion utility builds the VER(variable HEX) statement.
FILEID
Specifies that the variable must contain a valid file ID in CMS syntax. The conversion utility builds
the VER(variable FILEID) statement.
See the z/OS ISPF Dialog Developer's Guide and Reference for additional information concerning
panel variable validation.
INCLUDE
Specifies that the variable must contain valid characters from at least one of the ISPF-defined VER
groups ALPHA, ALPHAB or NUM.
PARM1=IMBLK
This attribute contains IMBLK to specify that the IMBLK keyword be added to the generated
VER statement. If PARM1 contains any other value, it is reset to the value IMBLK.
PARM2=ALPHA | ALPHAB | NUM
This attribute must contain either the value ALPHA, ALPHAB, or NUM. If PARM2 is not
specified or contains any other value, the INCLUDE check is ignored and the conversion utility
issues a warning message.
PARM3=ALPHA | ALPHAB | NUM
This attribute must contain either the value ALPHA, ALPHAB, or NUM. The value specified for
PARM3 should be different than the value specified for PARM2. If the values for PARM2 and
PARM3 are the same, the PARM3 value is ignored and the conversion utility issues a warning
message.
The conversion utility builds the VER(variable INCLUDE [,IMBLK], parm2 [,parm3]) statement.
IDATE
Specifies that the variable must contain a valid 8 character international date. The conversion
utility builds the VER(variable,IDATE) statement.
STDDATE
Specifies that the variable must contain a valid 10 character standard date. The conversion utility
builds the VER(variable,STDDATE) statement.
JDATE
Specifies that the variable must contain a valid 6 character Julian date. The conversion utility
builds the VER(variable,JDATE) statement.
JSTD
Specifies that the variable must contain a valid 8 character standard Julian date. The conversion
utility builds the VER(variable,JSTD) statement.
CHECKI
216  z/OS: z/OS ISPF DTL Guide

## Page 249

ITIME
Specifies that the variable must contain a valid 5 character international time. The conversion
utility builds the VER(variable,ITIME) statement.
STDTIME
Specifies that the variable must contain a valid 8 character standard time. The conversion utility
builds the VER(variable,STDTIME) statement.
IPADDR4
Specifies that the variable must contain a valid 15-position IP address. The conversion utility
builds the VER(variable,IPADDR4) statement.
Comments
The CHECKI tag defines a test of an input value. Validity checking occurs only on input.
Restrictions
• You must code the CHECKI tag within a CHECKL definition. The conversion utility supports only one
CHECKI within each CHECKL definition. If multiple CHECKI definitions are coded within a single CHECKL
definition, the additional CHECKI tags are ignored and are not syntax checked. See “CHECKL (Validity
Check List)” on page 218 for a complete description of this tag.
• The conversion utility builds a VER statement in the ISPF )PROC section of the panel definition for a
CHECKI tag.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
None.
Examples
In this example, variables associated with the variable class aa must have a value that contains only
alphabetic characters. Values associated with the variable class bb can only be SINGLE or DOUBLE.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=aa TYPE=‘char 18’>
  <CHECKL MSG=msgf881>
    <CHECKI TYPE=ALPHA>
  </CHECKL>
<VARCLASS NAME=bb TYPE=‘char 6’>
  <XLATL FORMAT=upper>
  </XLATL>
  <CHECKL MSG=msgf883>
    <CHECKI TYPE=VALUES PARM1=EQ PARM2="SINGLE DOUBLE">
  </CHECKL>
<VARLIST>
  <VARDCL NAME=checka VARCLASS=aa>
  <VARDCL NAME=checkb VARCLASS=bb>
</VARLIST>
<PANEL NAME=checki>CHECKI audits
  <DTAFLD DATAVAR=checka ENTWIDTH=18 PMTWIDTH=20>Enter Last Name
  <DTAFLD DATAVAR=checkb ENTWIDTH=6 PMTWIDTH=20>Enter Room Type
  <CMDAREA>
</PANEL>
CHECKI
Chapter 12. Tag reference  217

## Page 250

CHECKL (Validity Check List)
The CHECKL tag defines a validity check for input variables.
Syntax
<CHECKL
MSG=message-identifier
> </CHECKL>
Parameters
MSG=message-identifier
This attribute identifies the message to be issued if the value fails the embedded test. The conversion
utility adds this mes sage -identifier  to the VER statement generated by the enclosed CHECKI tag. If
you do not specify your own message, ISPF issues a message specified on the enclosing VARCLASS
tag or the default message associated with the type of VER statement generated. See “MSG
(Message)” on page 352 for information about creating messages.
Comments
The CHECKL tag defines a validity check for input variables. The CHECKI tag coded within the check list
performs the validation test.
Field validity checking follows standard ISPF conventions based on the verification statements generated.
For details, see “CHECKI (Validity Check Item)” on page 210.
Restrictions
• The CHECKL tag requires an end tag.
• You must code the CHECKL tag within a VARCLASS definition. See “VARCLASS (Variable Class)” on page
445 for a complete description of this tag.
• The CHECKL tag must be coded after all XLATL tags in the same variable class.
Processing
Table 12. Tags you can code within a CHECKL definition 
Tag Reference Usage Required
CHECKI “CHECKI (Validity Check Item)” on page 210 Single No
Examples
Here is source file markup that contains two variable classes that each contain a validity check list. The
second variable class also contains a translate list that translates variable values to uppercase. Notice
that the translate list is coded in the variable class before the validity check list.
CHECKL
218  z/OS: z/OS ISPF DTL Guide

## Page 251

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=aa TYPE='char 18'>
  <CHECKL MSG=msgf881>
    <CHECKI TYPE=ALPHA>
  </CHECKL>
<VARCLASS NAME=bb TYPE='char 6'>
  <XLATL FORMAT=upper>
  </XLATL>
  <CHECKL MSG=msgf883>
    <CHECKI TYPE=VALUES PARM1=EQ PARM2="SINGLE DOUBLE">
  </CHECKL>
<VARLIST>
  <VARDCL NAME=checka VARCLASS=aa>
  <VARDCL NAME=checkb VARCLASS=bb>
</VARLIST>
<PANEL NAME=checkl>CHECKL audits
  <DTAFLD DATAVAR=checka ENTWIDTH=18 PMTWIDTH=20>Enter Last Name
  <DTAFLD DATAVAR=checkb ENTWIDTH=6 PMTWIDTH=20>Enter Room Type
  <CMDAREA>
</PANEL>
CHOFLD (Choice Data Field)
The CHOFLD tag defines an input field, an output field, or an input/output field within the text of a CHOICE
tag.
CHOFLD
Chapter 12. Tag reference  219

## Page 252

Syntax
<CHOFLD DATAVAR=field-data
VARCLASS=variable-class-name
HELP=
NO
YES
help-panel-name
*help-message-id
%varname
*%varname
USAGE=
BOTH
IN
OUT
REQUIRED=
NO
YES
YES MSG=message-identifier
AUTOTAB=
NO
YES
ENTWIDTH=n FLDSPACE=n
ALIGN=
START
CENTER
END
DISPLAY=
YES
NO
NOENDATTR PAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
PSVAR= point-and-shoot-variable
%varname
PSVAL= point-and-shoot-value
%varname
PAS=%varname
EXPAND
ATTRCHANGE=
NO
YES
NEW
INIT=initial-value
IMAP options ATTRCHAR=code
CAPS=
OFF
ON
>
choice-description-text </CHOFLD>
IMAP options
CHOFLD
220  z/OS: z/OS ISPF DTL Guide

## Page 253

IMAPNAME= image-name
%varname IMAPNAMEP= image-namep
%varname
PLACE=
ABOVE
BELOW
LEFT
RIGHT
%varname
Parameters
DATAVAR=field-data
This attribute specifies the variable name for the data in the field. The value coded must be a
variable-name without the leading % notation.
VARCLASS=variable-class-name
This attribute specifies the name of the variable class, defined using a VARCLASS tag, that overrides
the default variable class referred to by the VARDCL that declared the data variable for this field.
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for this choice data field.
This is field-level help.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help for the choice data field and no help is defined, the extended help panel is
displayed. If an extended help panel is not defined for the panel, the application or ISPF tutorial is
invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
USAGE=BOTH | IN | OUT
This attribute indicates whether the field is for input only, output only, or both.
REQUIRED=NO | YES
This attribute indicates if the field requires input. This attribute is valid only when USAGE=IN or BOTH.
If REQUIRED=YES is coded, a VER(variable,NONBLANK) statement is built by the conversion utility
and placed in the )PROC section of the ISPF panel generated.
MSG=message-identifier
This attribute specifies the message that is displayed when the user does not complete a required
entry (defined with the REQUIRED attribute). If you do not specify a mes sage -identifier , ISPF
displays a default message.
If you specify the MSG attribute and REQUIRED=YES, a VER(variable,NONBLANK,MSG=message-
identifier) statement is built by the conversion utility and placed in the )PROC section of the ISPF
panel generated. If you specify the MSG attribute and REQUIRED=NO (the default), the conversion
utility issues a warning message.
See “MSG (Message)” on page 352 for information about creating messages.
CHOFLD
Chapter 12. Tag reference  221

## Page 254

Note: You can specify messages pertaining to other validations using XLATL and CHECKL tags
within a VARCLASS definition. See the descriptions of these tags for additional information.
AUTOTAB=NO | YES
When AUTOTAB=YES, the cursor moves to the next field capable of input when the user enters the
last character in this field. If no other field capable of user input exists on the panel, the cursor returns
to the beginning of this field.
AUTOTAB=YES is valid only when the value for USAGE is either BOTH or IN. If specified, this attribute
overrides the AUTOTAB value of the DTACOL tag.
ENTWIDTH=n
This attribute specifies the number of bytes used for the choice data field. The minimum width is 1
and the maximum is the remaining available panel width, less the required amount of space for field
attributes. If ENTWIDTH is not provided on either the CHOFLD tag or the enclosing DTACOL tag, the
conversion utility uses the width determined by the TYPE value of the associated VARCLASS.
If specified, this attribute overrides the ENTWIDTH value of the DTACOL tag.
FLDSPACE=n
This attribute specifies the number of bytes reserved for the choice data field. The minimum width is
2 and the maximum is the remaining available panel (or region) width. The FLDSPACE value should
include the actual entry width plus the number of entry field attributes. If the value specified by
ENTWIDTH is less than the specified FLDSPACE value, the entry field is padded with blanks to the
FLDSPACE value. This creates blank space between the entry field and following choice-description-
text and allows you to align description text from successive CHOFLD definitions.
If specified, this attribute overrides the FLDSPACE value of the DTACOL tag.
ALIGN=START | CENTER | END
This attribute specifies the alignment of data within the display field after all translations have been
performed. Use this attribute to align the data with the start, the end, or the center of the display field.
This is accomplished in the conversion utility by using an attribute character for the field that specifies
JUST(LEFT) if ALIGN=START or JUST(RIGHT) if ALIGN=END. ALIGN=CENTER uses an attribute
character for the field that specifies JUST(ASIS).
Alignment occurs if the transformed value of the dialog variable is shorter than the display width of
the field. When ALIGN=END, no underscore is padding performed. Instead, blanks are used.
DISPLAY=YES | NO
This attribute specifies whether data displays on the screen as the user types it in. If you specify
NO, the data is not displayed. This attribute is useful when creating fields for passwords or other
information which you do not want to appear on the screen.
NOENDATTR
This attribute, which is valid only when WINDOW=NO is specified on the PANEL tag or DIR=HORIZ is
specified on the REGION tag, specifies that no ending attribute is placed after the choice data field.
NOENDATTR is ignored for the last field on each panel line unless WINDOW=NO has been specified on
the PANEL tag. NOENDATTR is valid only when the CHOFLD tag is followed by a CHOFLD, CHOICE, or
CHDIV tag.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
If specified, this attribute overrides the PAD value of the DTACOL tag.
PADC=NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
If specified, this attribute overrides the PADC value of the DTACOL tag.
CHOFLD
222  z/OS: z/OS ISPF DTL Guide

## Page 255

OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
If specified, this attribute overrides the OUTLINE value of the DTACOL tag.
PSVAR=point-and-shoot-variable | %varname
This attribute provides the name of a variable that is to be set when a CHOFLD is clicked on for
point-and-shoot selection. You can define this attribute as a variable name preceded by a “%”.
The point-and-shoot-variable must follow the standard naming convention described in “Rules for
variable names” on page 179.
PSVAL=point-and-shoot-value | %varname
This attribute provides the value to be placed in the field specified by the PSVAR attribute. You
can define this attribute as a variable name preceded by a “%”. To specify a blank value, the "' '"
(quotation mark, apostrophe, blank, apostrophe, quotation mark) coding notation should be used.
PAS=%varname
This attribute can be used to provide a variable name to specify ON or OFF for point-and-shoot.
When PSVAR and PSVAL have been specified without the PAS attribute, the point-and-shoot field is
automatically enabled.
EXPAND
The EXPAND attribute, used in combination with EXPAND=xy on the PANEL definition, causes the
expand characters to be added to the CHOFLD definition, effectively allowing ENTWIDTH to expand.
The ENTWIDTH value must be specified as 4 or greater for the EXPAND function to operate.
Note: If the PANEL tag attribute EXPAND is defined with no value specified, the CHOFLD tag EXPAND
attribute is not used.
ATTRCHANGE=NO | YES | NEW
When ATTRCHANGE=YES or ATTRCHANGE=NEW, the conversion utility formats an additional
entry in the panel )ATTR section (that can apply to multiple data fields) instead of creating a
unique .ATTR(field-name) entry in the )INIT section for this field. With this option, multiple CHOFLD
tags with the same characteristics require fewer panel logic statements. ATTRCHANGE=NEW creates
a new entry. ATTRCHANGE=YES uses an existing entry, if possible.
INIT=initial-value
When the INIT attribute is specified, the conversion utility adds a statement to the panel )INIT section
to initialize the field to the initial-value.
IMAPNAME= image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAMEP=image-namep | %varname
The attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
PLACE=ABOVE | BELOW | LEFT | RIGHT
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
ATTRCHAR=code
This attribute can be a single character or a two-position entry of valid hex digits. If you enter an
incorrect value, a warning message is issued and the value is set to null. Hex entries are converted to
character. Hex values '00'-'2F' are reserved for use by the conversion utility.
CAPS=OFF | ON
When CAPS=ON, the data in the field is displayed in uppercase characters.
choice-description-text
This is the text for the choice data field. The choice-description-text appears following the field.
CHOFLD
Chapter 12. Tag reference  223

## Page 256

Comments
The CHOFLD tag is similar to the DTAFLD tag. When the enclosing SELFLD tag is defined within a DTACOL
tag, the CHOFLD tag uses default values defined by the DTACOL tag in the same way as the DTAFLD tag.
The CHOFLD tag defines an input field, an output field, or an input/output field within CHOICE tag
description text on an application panel.
The formatted width of the field is 2 positions more than the ENTWIDTH value to provide for an attribute
byte both before and after the field.
When the maximum number of requested attributes for a panel is exceeded, the conversion utility issues
error message ISPC804E. The number of requested attributes includes attribute override entries. These
are .ATTR entries that are added by the conversion utility for attributes that are specified on CHOFLD,
DTACOL, DTAFLD, LSTCOL, and LSTFLD tags. If the same set of attributes is specified on multiple tags,
duplicate .ATTR entries are added by default. Adding the parameter ATTRCHANGE=YES to the tags
causes the compiler to instead add a single entry in the panel )ATTR section for each unique set of
attributes specified. The entry for a set of attributes is then shared by all tags that specify that set of
attributes.
Restrictions
• You must code the CHOFLD tag within a CHOICE tag definition. The CHOFLD tag can be placed
anywhere within the choice-description-text. See “CHOICE (Selection Choice)” on page 226 for a
description of this tag.
• The variable name specified in the DATAVAR attribute should have an associated VARDCL definition. See
“VARDCL (Variable Declaration)” on page 449 for a complete description of this tag.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
%varname entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
Table 13. Tags you can code within a CHOFLD definition 
Tag Reference Usage Required
ACTION “ACTION (Action)” on page 184 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is source file markup that contains an application panel that is similar to the example for the CHOICE
tag. In this example, the first selection field is modified to illustrate the CHOFLD tag. The first choice
includes a panel input/output field named cardtype which must be completed when the new choice is
selected.
Notice that the reference CHOICE source file has been modified to:
CHOFLD
224  z/OS: z/OS ISPF DTL Guide

## Page 257

• Add a VARCLASS for the cardtype field before the include file which has both VARCLASS and VARDCL
tags.
• Add a VARDCL for the cardtype field after the include file which has both VARCLASS and VARDCL tags.
• Add the CHOFLD tag to define the choice data field.
• Add a DTACOL tag definition to allow for a SOURCE tag that provides an audit of cardtype only when new
is selected.
Figure 95 on page 226 shows the formatted result.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>)>
<VARCLASS NAME=char9cls TYPE='char 9'>
  <XLATL FORMAT=upper>
  </XLATL>
&sampvar1;
<VARLIST>
  <VARDCL NAME=cardtype VARCLASS=char9cls>
</VARLIST
<PANEL NAME=chofld KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST>Type in patron's name and card number (if applicable).
<TOPINST>Then select an action bar choice.
<AREA>
  <DTAFLD DATAVAR=curdate PMTWIDTH=12 ENTWIDTH=8 USAGE=out>Date
  <DTAFLD DATAVAR=cardno PMTWIDTH=12 ENTWIDTH=7 DESWIDTH=25>Card No
    <DTAFLDD>(A 7-digit number)
  <DTAFLD DATAVAR=name PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25>Name
    <DTAFLDD>(Last, First, M.I.)
  <DTAFLD DATAVAR=address PMTWIDTH=12 ENTWIDTH=25>Address
  <DIVIDER>
  <REGION DIR=horiz>
  <SELFLD NAME=cardsel PMTWIDTH=30 SELWIDTH=38>Choose
  one of the following
    <CHOICE CHECKVAR=card MATCH=new>
            New    Type:
    <CHOFLD datavar=cardtype autotab=yes entwidth=9>
          (Permanent or Temporary)
<CHOICE CHECKVAR=card MATCH=renew>Renewal
<CHOICE CHECKVAR=card MATCH=replace>Replacement
  </SELFLD>
 <DTACOL>
    <SOURCE>
IF (&CARDSEL = 1)
  VER(&CARDTYPE,NB,LIST,TEMPORARY,PERMANENT)
    </SOURCE>
  </DTACOL>
  <SELFLD TYPE=multi PMTWIDTH=30 SELWIDTH=25>Check valid branches
   <CHOICE NAME=north HELP=nthhlp CHECKVAR=nth>North Branch
   <CHOICE NAME=south HELP=sthhlp CHECKVAR=sth>South Branch
   <CHOICE NAME=east HELP=esthlp CHECKVAR=est>East Branch
    <CHOICE NAME=west HELP=wsthlp CHECKVAR=wst>West Branch
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>Enter a command
</PANEL>
CHOFLD
Chapter 12. Tag reference  225

## Page 258

File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number (if applicable).
 Then select an action bar choice.
 Date . . . :
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New      Type: Z              _  North Branch
         (Permanent or Temporary)      _  South Branch
     2.  Renewal                       _  East Branch
     3.  Replacement                   _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 95. Choice data fields 
CHOICE (Selection Choice)
The CHOICE tag defines information about a choice in a selection field.
CHOICE
226  z/OS: z/OS ISPF DTL Guide

## Page 259

Syntax
<CHOICE
NAME=choice-name
HELP=
NO
YES
help-panel-name
*help-message-id
%varname
*%varname
CHECKVAR=variable-name
MATCH=
1
string NOMATCH=
0
string
AUTOTAB=
YES
NO
SELCHAR='char(s),n'
PAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
HIDE HIDEX
UNAVAIL=variable-name
UNAVAILMAT=
1
string
TRUNC=n
AUTOSEL=
YES
NO
> choice-description-text
</CHOICE>
Parameters
NAME=choice-name
Specifies the name of the choice. The choice-name must follow the standard naming convention
described in “Rules for variable names” on page 179.
CHOICE
Chapter 12. Tag reference  227

## Page 260

Note: This attribute is required for choices defined for a multiple-choice selection field because the
choice-name is used as the input field for multiple choice selections.
For multiple-choice selection fields, the choice-name can also be used to position the cursor on the
choice or to position a pop-up.
Note: This attribute is not supported by the conversion utility for single-choice selection fields. In this
case, the NAME value of the SELFLD tag is used as the field name.
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests for a multiple-choice selection
field. This is field-level help.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a choice and no help is defined, the extended help panel is displayed. If
an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MULTI.
CHECKVAR=variable-name
This attribute defines a variable whose value indicates whether the choice is preselected when the
selection field is displayed. If the value of the variable is equivalent to the string you specify with the
MATCH attribute, the item is marked as selected when the panel displays.
The preselection indicator depends on the value of the TYPE attribute from the SELFLD tag.
Table 14. Host Display indicators for particular TYPEs
TYPE LISTTYPE Host Display Indicator
MULTI n/a slash
SINGLE (not used)
RADIO
LISTBOX
DDLIS
COMBO
Choice number
Choice number
Choice number
Choice number
Choice number
MENU n/a Choice number
MODEL n/a Choice number
TUTOR n/a Choice number
When the SELFLD tag has been specified with TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR, the
CHOICE number (or SELCHAR value) is placed in the command line.
The variable-name is updated to the value specified by the MATCH attribute when the user selects the
choice being defined. For multiple-choice selection fields (SELFLD TYPE=MULTI), if you do not select
a choice, or you deselect a choice, the associated variable-name is set to the value of the NOMATCH
attribute or to 0 if the NOMATCH attribute is not specified.
Use a different variable for variable-name than what has been specified for choice-name.
CHOICE
228  z/OS: z/OS ISPF DTL Guide

## Page 261

Do not use the same variable for the variable-name as you use for the variable-name specified for the
SETVAR or TOGVAR attributes of the ACTION tag.
For single-choice selection fields (SELFLD TYPE=SINGLE), ISPF selection menus (SELFLD
TYPE=MENU), edit model selection menus (SELFLD TYPE=MODEL), or tutorial selection menus
(SELFLD TYPE=TUTOR), the variable-name should be the same for all of the choices. For multiple-
choice selection fields (SELFLD TYPE=MULTI), the variable-name should be different for each choice.
The CHECKVAR attribute value must be specified without a leading % sign. The variable-name must
follow the standard naming convention described in “Rules for variable names” on page 179.
MATCH=1 | string
Defines the value for the check variable that causes the item to be preselected. The string can be
any character string. MATCH=1 is the default.
NOMATCH=0 | string
Defines the value for setting the check variable when the item is not selected. NOMATCH=0 is the
default.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MULTI.
AUTOTAB=YES | NO
When AUTOTAB=YES, the cursor moves to the next field capable of input when the user enters the
last character in this field. If no other field capable of user input exists on the panel, the cursor
remains on this field.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MULTI.
SELCHAR='char(s),n'
This attribute specifies an alphanumeric character(s) to be used as the selection menu, edit model
selection menu, or tutorial selection menu choice in place of the normal numeric value automatically
supplied by the conversion utility. The number of characters accepted is controlled by the ENTWIDTH
attribute of the SELFLD tag. The char(s) value is used as coded, that is, it is not uppercase.
When the HIDE attribute is also specified, the number of characters to be used for the hidden choice
selection may be specified as part of the SELCHAR attribute. If specified, the n value overrides the
number of characters normally obtained from the ENTWIDTH attribute of the SELFLD tag. The n value
can be a numeric value from 1 to the number of bytes provided as the char(s) value, or you can specify
an “*” to tell the conversion utility to use all of the char(s) provided for the choice selection. The n
value is ignored when the HIDE attribute is not specified.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MENU,
TYPE=MODEL, or TYPE=TUTOR.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MULTI.
PADC=NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MULTI.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MULTI.
HIDE
This attribute causes a choice entry for a single-choice, menu-choice, model-choice, or tutor-choice
selection to be removed from the selection list display.
CHOICE
Chapter 12. Tag reference  229

## Page 262

This allows the creation of a numbered selection list when the choice numbers are not continuous by
adding a ‘dummy’ CHOICE tag at the appropriate place in the DTL source. The number assigned to the
hidden CHOICE does not appear in the selection list. Normal )INIT and )PROC section entries are not
affected.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=SINGLE,
TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR.
HIDEX
This attribute causes a choice entry for a model-choice selection to be removed both from the
selection list display and from the selection processing.
This attribute is used in combination with the TRUNC attribute and the SELCHAR attribute to supply an
alternate CHOICE tag definition with an alternate hidden model selection keyword.
For example, if an edit model panel has a selectable description of "VER", but you also want to allow
the full word "VERIFY" to select the same model, two CHOICE tags are required. The first one defines
the choice with the text "VER". The alternate CHOICE uses the same SELCHAR information, adds the
attribute HIDEX and TRUNC=3, and specifies the tag text as "VERIFY". The conversion utility uses the
first definition to build the panel text and the selection processing statement and uses the alternate
CHOICE to accept the entry "VERIFY" by truncating it to "VER".
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MODEL.
UNAVAIL=variable-name
This attribute defines a variable whose value indicates whether the choice is available when the
selection field is displayed. If the value of the variable is equivalent to the string you specify with the
UNAVAILMAT attribute (or to the default value "1"), the item is displayed as an unavailable choice.
UNAVAILMAT=1 | string
Defines the value for the UNAVAIL variable that causes the choice to be unavailable. The string can
be any character string. UNAVAILMAT=1 is the default.
TRUNC=n
This attribute is used for model-choice selection to specify the minimum number of characters
required to identify the model choice. If the TRUNC attribute is not specified, the entire model choice
name must be used to identify the model selection.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MODEL.
AUTOSEL=YES | NO
This attribute is used for tutor-choice selection to control the automatic selection of this choice by
tutorial processing. When AUTOSEL=NO, the choice is not automatically selected.
choice-description-text
The text of the selection choice.
Comments
The CHOICE tag defines a choice within a selection field. The behavior and appearance of the choice
depends on whether it is coded within a single-choice, multiple-choice, or menu-choice selection field.
For menu-choice selection fields, the text is preceded by a number (not followed by a period), the input
field is the command line, and the choice selection is displayed with the CUA type Normal Text (NT).
For a single-choice selection list:
• When the LISTTYPE attribute of the SELFLD tag is not specified, the text is preceded by a number
(followed by a period), the conversion utility provides an input field before the first choice for entry
of the number of the selected choice, and the choice selection is displayed with the CUA type Select
Available Choices (SAC).
• When LISTTYPE=RADIO is specified on the SELFLD tag, the choice selection can be displayed as a radio
button by a client that is using the JSON API.
CHOICE
230  z/OS: z/OS ISPF DTL Guide

## Page 263

The field name for single-choice selection fields is the value specified for the NAME attribute of the
SELFLD tag. The default field name for an ISPF selection menu choice is the field name used to identify
the command line, normally ZCMD.
The text of each choice in a multiple-choice selection field is preceded by an input field. The field name
for multiple-choice selection fields is the value specified for the NAME attribute of the CHOICE tag.
You can define an action for each choice using the SETVAR or TOGVAR attribute in an ACTION tag
associated with the choice. Typically, an application knows what choice was selected by the application
user by the value in the selection field name. The CHOICE field name for a multi-choice selection is set
to a "/" when control is returned to the application. The SELFLD field name contains the number of the
choice for single choice selection when control is returned to the application. The command line variable
name contains the number of a menu selection choice when control is returned to the application.
Alternatively, the application can use the value of the check variable or use SETVAR or TOGVAR to set
another named variable.
Restrictions
• You must code the CHOICE tag within a SELFLD definition. See “SELFLD (Selection Field)” on page 421
for a complete description of this tag.
• If coded within a multiple-choice selection field (SELFLD TYPE=MULTI), the choice-name can have an
associated VARDCL definition.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
• If the choice-description-text contains HP (Emphasized Text) or RP (Reference Phrase) tags, the
UNAVAIL attribute is ignored.
Processing
Table 15. The tags you can code within a CHOICE definition 
Tag Reference Usage Required
ACTION “ACTION (Action)” on page 184 Multiple No
CHOFLD “CHOFLD (Choice Data Field)” on page 219 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is application panel markup that contains two selection fields. The first is a single-choice selection
field that can be preselected depending on the value assigned to the variable card. When card is equal to
new, renew, or replace, the selection field's input data field is assigned a value of 1, 2, or 3, respectively;
otherwise, it is not preselected and the input data field remains blank.
The second selection field is a multiple-choice selection field. This field can be preselected by assigning
values to the variables nth, sth, est and wst. If the given variable equals 1, the corresponding selection
CHOICE
Chapter 12. Tag reference  231

## Page 264

field is marked with a /. More than one of the choices may be selected. Any nonblank character in the
choice entry-field selects that choice. Preselected choices can be deselected by typing a blank character
over the field.
Figure 96 on page 232 shows the formatted result.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=choice1 KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST>Type in patron's name and card number (if applicable).
<TOPINST>Then select an action bar choice.
<AREA>
  <DTAFLD DATAVAR=curdate PMTWIDTH=12 ENTWIDTH=8 USAGE=out>Date
  <DTAFLD DATAVAR=cardno PMTWIDTH=12 ENTWIDTH=7 DESWIDTH=25>Card No
    <DTAFLDD>(A 7-digit number)
  <DTAFLD DATAVAR=name PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25>Name
    <DTAFLDD>(Last, First, M.I.)
  <DTAFLD DATAVAR=address PMTWIDTH=12 ENTWIDTH=25>Address
  <DIVIDER>
  <REGION DIR=horiz>
  <SELFLD NAME=cardsel PMTWIDTH=30 SELWIDTH=38>Choose
  one of the following
    <CHOICE CHECKVAR=card MATCH=new>New
    <CHOICE CHECKVAR=card MATCH=renew>Renewal
    <CHOICE CHECKVAR=card MATCH=replace>Replacement
  </SELFLD>
  <SELFLD TYPE=multi PMTWIDTH=30 SELWIDTH=25>Check valid branches
    <CHOICE NAME=north HELP=nthhlp CHECKVAR=nth>North Branch
    <CHOICE NAME=south HELP=sthhlp CHECKVAR=sth>South Branch
    <CHOICE NAME=east HELP=esthlp CHECKVAR=est>East Branch
    <CHOICE NAME=west HELP=wsthlp CHECKVAR=wst>West Branch
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>Enter a command
</PANEL>
   File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number (if applicable).
 Then select an action bar choice.
 Date . . . :
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 96. Selection field  choices
CHOICE
232  z/OS: z/OS ISPF DTL Guide

## Page 265

CMD (Command Definition)
The CMD tag defines a command within an application command table.
Syntax
<CMD NAME=internal-command-name
ALTDESCR=command-description
>
external-command-name </CMD>
Parameters
NAME=internal-command-name
This attribute specifies an internal name for the command. The internal-command-name must have
these characteristics:
• 2-8 single-byte characters in length
• The first (or only) character must be A-Z, a-z, @, #, or $.
• Remaining characters, if any, can be A-Z, a-z, @, #, $, —, or 0-9.
Lowercase characters are translated to their uppercase equivalents.
The internal-command-name is used in two ways:
• As the command table search criteria when:
– A key defined in the current key list is pressed
– A pull-down choice with an associated RUN action is selected
– A command is entered in the command area of a panel.
• As the value passed to dialogs when the command action is PASSTHRU or SETVERB. See “CMDACT
(Command Action)” on page 234 for more information about the PASSTHRU and SETVERB
command actions.
ALTDESCR=command-description
This attribute provides a description of the command. It is placed in the ISPF variable ZCTDESC. The
command-description text length is limited to 80 bytes.
external-command-name
Specifies the external name for this command.
Note: The external-command-name must be equal to the internal-command-name. You must use the
external-command-name to support the ability provided by ISPF for truncated command entry and
the T (truncation) tag. For more information, see “T (Truncation)” on page 437.
Comments
The CMD tag defines a command within an application command table. The defined command can
be issued by an application user by entering the internal-command-name in the panel command area,
or pressing a function key, or selecting a pull-down choice that references the command's internal-
command-name. See “KEYI (Key Item)” on page 319 and “ACTION (Action)” on page 184 for additional
information.
The action to be taken when a command is issued is defined with the CMDACT tag. See “CMDACT
(Command Action)” on page 234 for information about defining command actions.
CMD
Chapter 12. Tag reference  233

## Page 266

Restrictions
• The CMD tag must be coded within a CMDTBL definition. See “CMDTBL (Command Table)” on page 243
for a complete description of this tag.
Processing
Table 16. The tags you can code within a CMD definition 
Tag Reference Usage Required
CMDACT “CMDACT (Command Action)” on page 234 Single Yes
T “T (Truncation)” on page 437 Single No
Examples
Here is source file markup that contains a command table that defines the commands UPDATE, ADD,
DELETE, and SEARCH. The DELETE and UPDATE commands have defined truncations.
<!DOCTYPE DM SYSTEM>
<CMDTBL APPLID=conv>
  <CMD NAME=update>Upd<T>ate
    <CMDACT ACTION='alias add'>
  <CMD NAME=add>Add
    <CMDACT ACTION=setverb>
  <CMD NAME=delete>Del<T>ete
    <CMDACT ACTION=passthru>
  <CMD NAME=search>Search
    <CMDACT ACTION=passthru>
</CMDTBL>
This table shows the resultant ISPF application command table.
Table 17. ISPF application command table
ZCTVERB ZCTTRUNC ZCTACT
UPDATE 3 ALIAS ADD
ADD 0 SETVERB
DELETE 3 PASSTHRU
SEARCH 0 PASSTHRU
CMDACT (Command Action)
The CMDACT tag defines the action that occurs when the associated command is issued.
CMDACT
234  z/OS: z/OS ISPF DTL Guide

## Page 267

Syntax
<CMDACT
MIXC
ACTION=
'SELECT select-parameters'
'ALIAS internal-command-name
parameters
'
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
'%varname'
application-command
ASIS
>
</CMDACT>
Parameters
MIXC
Specifies that the following ACTION attribute is not to be converted to uppercase.
ACTION=
This attribute indicates the action that should be performed when the associated command is issued.
The ACTION attribute value is limited to 240 characters. The value must be one of these:
SELECT select-parameters
Causes the ISPF SELECT service to be issued.
ALIAS internal-command-name
Provides an alternate way to express a command. For example, you can assign QUIT as an alias for
the command EXIT.
The ALIAS internal-command-name has a maximum length of 8 characters.
In the command table, an alias must precede the command for which it is an alias.
You can create a chain of command aliases in a command table, as long as the result is a valid
executable action. The last command and parameter values that ISPF encounters in the alias
chain are the ones executed. The command and the parameter values do not necessarily come
from the same command definition entry. For example:
Command Name
Command Action
EASYKEY
ALIAS CMD PARM1 PARM2
CMDACT
Chapter 12. Tag reference  235

## Page 268

CMD
ALIAS CMD1 PARM3
CMD1
ALIAS CMD2
In this example, if the EASYKEY command is issued, the command that would ultimately be
executed would be CMD2 PARM3.
parameters
If any ALIAS parameters are specified, they take precedence over any parameters included
with the command when issued from a command line or the ACTION tag RUN attribute when a
pull-down choice is selected.
If the ALIAS internal-command-name does not include parameters, ISPF accepts parameters
from the command line or ACTION tag.
PASSTHRU
The PASSTHRU action causes the command and any parameters to be passed to the dialog
program in the ZCMD dialog variable.
SETVERB
This is an alternate way to pass a command to the dialog. The SETVERB action causes the
internal-command-name to be passed to the dialog in the ZVERB dialog variable. Any command
parameters are passed in the ZCMD dialog variable.
BACKWARD
Specifies the ISPF system command BACKWARD as the command action.
CANCEL
Specifies the ISPF system command CANCEL as the command action.
EXIT
Specifies the ISPF system command EXIT as the command action.
EXHELP
Specifies the ISPF system command EXHELP as the command action.
FKA
Specifies the ISPF system command FKA as the command action.
FORWARD
Specifies the ISPF system command FORWARD as the command action.
HELP
Specifies the ISPF system command HELP as the command action.
PANELID
Specifies the ISPF system command PANELID as the command action.
RETRIEVE
Specifies the ISPF system command RETRIEVE as the command action.
%varname
You can specify a command action dynamically at run time by specifying the name of a variable
(using % notation) for the ACTION attribute. If you specify a variable name, ISPF retrieves
the action value when the command is issued. The variable value must be one of the actions
previously listed.
The "%varname" entry must follow the naming conventions described in “Rules for “%variable”
names” on page 179.
application-command
Specifies an application-unique command as the command action. The command action is
created as an ALIAS unless the ASIS keyword is specified.
ASIS
Specifies that the application-unique command is to be created without the ALIAS
designation.
CMDACT
236  z/OS: z/OS ISPF DTL Guide

## Page 269

Comments
The CMDACT tag defines the action that occurs when the associated command is issued.
Restrictions
• The CMDACT tag must be coded within the CMD definition it is associated with. See “CMD (Command
Definition)” on page 233 for a complete description of this tag.
• You must specify the ACTION attribute on the CMDACT tag.
Processing
None.
Examples
Here is source file markup contains a command table that defines the commands UPDATE, ADD, DELETE
and SEARCH. The ADD command sets the ZVERB variable equal to add. The DELETE command sets the
ZCMD variable to delete. The UPDATE command is an alias for ADD.
<!DOCTYPE DM SYSTEM>
<CMDTBL APPLID=conv>
  <CMD NAME=update>Upd<T>ate
    <CMDACT ACTION='alias add'>
  <CMD NAME=add>Add
    <CMDACT ACTION=setverb>
  <CMD NAME=delete>Del<T>ete
    <CMDACT ACTION=passthru>
  <CMD NAME=search>Search
    <CMDACT ACTION=passthru>
</CMDTBL>
This table shows the resultant ISPF application command table.
Table 18. ISPF application command table
ZCTVERB ZCTTRUNC ZCTACT
UPDATE 3 ALIAS ADD
ADD 0 SETVERB
DELETE 3 PASSTHRU
SEARCH 0 PASSTHRU
CMDAREA (Command Area)
The CMDAREA tag defines a command entry area on an application panel.
CMDAREA
Chapter 12. Tag reference  237

## Page 270

Syntax
<CMDAREA
HELP=
NO
YES
help-panel-name
*help-message-id
%varname
*%varname
PMTLOC=
BEFORE NOINIT
PAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
NAME=cmdarea-variable-name
ENTWIDTH=n
PMTTEXT=
YES
NO
CMDLOC=
DEFAULT
ASIS CMDLEN=
DEFAULT
MAX
AUTOTAB=
NO
YES
SCROLLVAR=scroll-variable
SCRVHELP=
NO
YES
scroll-help-panel-name
*scroll-help-message-id
%varname
*%varname
SCROLLTAB=
NO
YES SCRCAPS=
OFF
ON
PSBUTTON=cmd-pb-text-| PB group |
CAPS=
OFF
ON
NOJUMP=
OFF
ON VARDCL=
YES
NO
>
command-prompt-text </CMDAREA>
PB Group
CMDAREA
238  z/OS: z/OS ISPF DTL Guide

## Page 271

PSVAR= point-and-shoot-variable
%varname
PSVAL= point-and-shoot-value
%varname
| IMAP Options |
IMAP Options
IMAPNAME= image-name
%varname IMAPNAMEP= image-namep
%varname
PLACE=
ABOVE
BELOW
LEFT
RIGHT
%varname
Parameters
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for the command area.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a choice and no help is defined, the extended help panel is displayed. If
an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
PMTLOC=BEFORE
This attribute defines the location of the prompt text. The text defined by command-prompt-text
appears on the same line as the command area entry field.
NOINIT
This attribute controls the initial display of the command line. When this attribute is specified, the
ZCMD field is not initialized to blanks before the panel is displayed.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PADC= NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
CMDAREA
Chapter 12. Tag reference  239

## Page 272

NAME=cmdarea-variable-name
This attribute specifies a command area name to replace the default name ZCMD.
The cmdarea-variable-name must follow the standard naming convention described in “Rules for
variable names” on page 179.
ENTWIDTH=n
This attribute is used to specify the length of the command field. It is used in combination with
WINDOW=NO on the PANEL tag to create a command line which is longer than a single panel line.
PMTTEXT=YES | NO
This attribute is used to control the formatting of the command-prompt-text. When PMTTEXT=NO, the
command-prompt-text is not used, leaving only the “===>” indicator for the command field.
CMDLOC=DEFAULT | ASIS
This attribute is used to control the placement of the command line in the generated panel. When
CMDLOC=DEFAULT (or when CMDLOC is not specified) the command area is placed at line 2 in the
panel, and the display position is controlled by the option specified on the Settings panel. When
CMDLOC=ASIS is specified, the command area is placed in the generated panel in the same relative
position as the CMDAREA tag is found in the DTL source, and the Settings option is ignored when the
panel is displayed.
CMDLEN=DEFAULT | MAX
This attribute is used to control the length of the command line in the generated panel. When
CMDLEN=DEFAULT (or when CMDLEN is not specified) the command line length is taken from the
specified (or defaulted) WIDTH attribute of the PANEL tag. When CMDLEN=MAX is specified, the
command line length is taken from the record length of the output panel file.
This attribute is valid only when WINDOW=NO is specified on the PANEL tag.
AUTOTAB=NO | YES
When AUTOTAB=YES, the cursor moves to the next input field when you enter the last character in the
command field. If there is no other input field on the panel, the cursor returns to the beginning of the
command line.
SCROLLVAR=scroll-variable
This attribute specifies the name of a variable that the application uses to obtain scrolling information.
The scroll-variable must follow the standard naming convention described in “Rules for variable
names” on page 179.
If the attribute is specified, the conversion utility creates a scroll entry on the command line, providing
that the resulting command area allows at least 8 bytes for a command entry.
SCRVHELP=NO | YES | scroll-help-panel-name | *scroll-help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for the field specified with
the SCROLLVAR attribute.
When SCRVHELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a choice and no help is defined, the extended help panel is displayed. If
an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The scroll-help-panel-name must follow the standard naming convention described in “Rules for
variable names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
SCROLLTAB=NO | YES
When SCROLLTAB=YES, the cursor moves to the next input field when you enter the last character in
the scroll amount field. If there is no other input field on the panel, the cursor returns to the beginning
of the command line.
CMDAREA
240  z/OS: z/OS ISPF DTL Guide

## Page 273

SCRCAPS=OFF | ON
When SCRCAPS=ON, the data in the scroll field is displayed in uppercase characters.
PSBUTTON=cmd-pb-text
This attribute requires that the PSVAR and PSVAL attributes also be specified.
This attribute specifies that a command push button is to be placed at the end of the command line,
provided that the resulting command area allows at least 8 bytes for a command entry. The push
button text area is created as a point-and-shoot field.
PSVAR=point-and-shoot-variable | %varname
This attribute provides the name of a variable that is to be set when the cmd-pb-text is clicked
on for point-and-shoot selection. You can define this attribute as a variable name preceded by a
percent (%) sign.
The point-and-shoot-variable must follow the standard naming convention described in “Rules for
variable names” on page 179.
PSVAL=point-and-shoot-value | %varname
This attribute provides the value to be placed in the field specified by the PSVAR attribute. You can
define this attribute as a variable name preceded by a percent (%) sign. To specify a blank value,
use the coding notation “' '” (quotation mark, apostrophe, blank space, apostrophe, quotation
mark).
IMAPNAME=image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
IMAPNAMEP=image-namep | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
PLACE=ABOVE | BELOW | LEFT | RIGHT | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
CAPS=OFF | ON
When CAPS=ON, the data in the field is displayed in uppercase characters.
NOJUMP=OFF | ON
When NOJUMP=ON, the JUMP function is disabled for the field.
VARDCL=YES | NO
When VARDCL=NO the cmdarea-variable-name is not checked to the declared variable information
provided with the VARCLASS and VARDCL tags.
command-prompt-text
The command-prompt-text specifies the prompt text for the command entry area. The maximum
prompt text (not including the command area prefix ===>) is 59 bytes for a standard 76 byte-width
panel. The conversion utility reserves 8 bytes for a minimum command entry field and 3 additional
bytes are required for panel attributes. One blank is placed between the command-prompt-text
and the command area prefix. One blank is placed between the end of the command line and the
right panel boundary (unless the WINDOW=NO attribute has been specified) to prevent the cursor
from skipping into the right panel window border. These formatting considerations mean that the
maximum length of the command-prompt-text for a panel 76 bytes in width is 59. If the length
of the command-prompt-text exceeds the available space, a message is issued and the command-
prompt-text is truncated. If your panel requires that the Scroll field be added to the Command line,
or the SCROLLVAR attribute is specified in the CMDAREA definition, the command-prompt-text must
be further reduced to allow for the Scroll field. If your panel specifies the PSBUTTON attribute, the
command-prompt-text must be further reduced to allow for the Command push button.
If you do not provide command-prompt-text, the word "Command" (or its translated equivalent) is
the default, unless you are creating an ISPF selection panel, in which case the word "Option" (or
its translated equivalent) is the default. The Common User Access command area prefix (===>) is
always added automatically in front of the entry field.
CMDAREA
Chapter 12. Tag reference  241

## Page 274

Comments
The CMDAREA tag defines a command entry area on an application panel. The command entry area
extends to the right side of the panel, unless limited by the ENTWIDTH attribute or the presence of a
Scroll field. Application users use the command entry area to enter commands.
Note: If you specify the CMDAREA tag within your DTL source file:
• It must appear before the AREA, DA, GA, REGION, or SELFLD tag when DEPTH=* is specified.
• It must appear before the SELFLD tag when TYPE=MENU and CHECKVAR or UNAVAIL attributes are
specified on nested CHOICE tags.
Restrictions
• You must code the CMDAREA tag within a PANEL definition. You can code only one command area
definition for each panel. See “PANEL (Panel)” on page 376 for a complete description of this tag.
• The data entered on the command line is processed “as is”. To translate the data to uppercase, you
must either provide a VARDCL definition for the field ZCMD with a reference to a VARCLASS containing
an XLATL tag which specifies FORMAT=UPPER, or specify CAPS=ON.
• You cannot code the CMDAREA tag within an AREA definition. The Command area is generated at the
top of the panel source to allow for floating of the command line. See the z/OS ISPF Dialog Developer's
Guide and Reference for more information.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
Table 19. The tags you can code within a CMDAREA definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
Examples
Here is application panel markup that contains a command area. The command-prompt-text "Use this
area to enter a command" is specified in the markup to override the default text “Command”. Figure 97 on
page 243 shows the formatted result.
CMDAREA
242  z/OS: z/OS ISPF DTL Guide

## Page 275

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=choiccls TYPE='char 2'
<VARCLASS NAME=vccmd TYPE='char 62'>
  <XLATL FORMAT=upper>
  </XLATL>
</VARCLASS>
<VARLIST>
  <VARDCL NAME=sample VARCLASS=choiccls>
  <VARDCL NAME=zcmd  VARCLASS=vccmd>
</VARLIST>
<PANEL NAME=cmdarea1>Choose a Virtue
  <TOPINST>Select a choice.
  <AREA>
    <SELFLD NAME=sample PMTWIDTH=10 SELWIDTH=20>Virtues:
      <CHOICE>Faith
      <CHOICE>Hope
      <CHOICE>Charity
    </SELFLD>
  </AREA>
  <BOTINST>Now press Enter.
 <CMDAREA>Use this area to enter a command
</PANEL>
                            Choose a Virtue
 Select a choice.
 Virtues:
 __  1.  Faith
     2.  Hope
     3.  Charity
 Now press Enter.
 Use this area to enter a command ===> ____________________________________
  F1=Help    F3=Exit   F12=Cancel
Figure 97. Command area
CMDTBL (Command Table)
The CMDTBL tag provides support to define the ISPF application command table.
Syntax
<CMDTBL APPLID=application-identifier
SORT=
NO
YES
> </CMDTBL>
CMDTBL
Chapter 12. Tag reference  243

## Page 276

Parameters
APPLID=application-identifier
This attribute specifies an application identifier. This identifier is used as a prefix to the string "CMDS"
to form the name of the command table. The applic ation -identifier  must have these characteristics:
• 1-4 characters in length
• The first (or only) character must be A-Z or a-z @, #, or $.
• Remaining characters, if any, must be A-Z, a-z, @, #, $, or 0-9.
Lowercase characters are translated to their uppercase equivalents.
The name of the command table is member name xxxxCMDS, where xxxx represents the applic ation - 
identifier .
Command tables are updated using ISPF table services. Input is obtained from the ISPTLIB DDname
allocation and output is written to the ISPTABL DDname allocation. See the description of how to
allocate libraries before starting ISPF in the z/OS ISPF User's Guide Vol I for more information about
the use of ISPTLIB and ISPTABL.
SORT=NO | YES
When SORT=YES is specified, the command table is sorted in command-name sequence. Any
commands defined as an ALIAS to other commands are placed in the command table first, in
command-name sequence. The regular commands follow the ALIAS entries in command-name
sequence.
If SORT=NO or the SORT attribute is not specified, commands are placed in the command table in the
sequence the CMD tags are encountered in the DTL source file.
Comments
The command table tag provides support to define the ISPF application command table. ACTION tags and
definitions of key lists reference the command definitions within an application command table.
Note: To access commands through the use of the key list function keys, specify the KEYLAPPL ID
invocation parameter for the conversion utility with the same APPLID value used for the CMDTBL tag.
Note: You can use the TSO ISPCMDTB command to convert existing command tables to DTL. To
use ISPCMDTB, ensure that the command table is in your table concatenation (ISPCMDTB), type TSO
ISPCMDTB applid (where applid is the application id of the command table). This places you in an edit
session containing the DTL version of the command table. Use the editor CREATE or REPLACE commands
to save the table to your DTL source data set.
Restrictions
• The CMDTBL tag requires an end tag.
• You cannot code the CMDTBL tag within any other tag definition.
• You can code only one command table for any application.
Processing
Table 20. The tags you can code within a CMDTBL definition 
Tag Reference Usage Required
CMD “CMD (Command Definition)” on page 233 Multiple Yes
CMDTBL
244  z/OS: z/OS ISPF DTL Guide

## Page 277

Examples
This source file markup contains a command table that defines the commands UPDATE, ADD, DELETE and
SEARCH.
<!DOCTYPE DM SYSTEM>
<CMDTBL APPLID=conv>
  <CMD NAME=update>Upd<T>ate
    <CMDACT ACTION='alias add'>
  <CMD NAME=add>Add
    <CMDACT ACTION=setverb>
  <CMD NAME=delete>Del<T>ete
    <CMDACT ACTION=passthru>
  <CMD NAME=search>Search
    <CMDACT ACTION=passthru>
</CMDTBL>
This table shows the resultant ISPF application command table.
Table 21. ISPF Application Command Table
ZCTVERB ZCTTRUNC ZCTACT
UPDATE 3 ALIAS ADD
ADD 0 SETVERB
DELETE 3 PASSTHRU
SEARCH 0 PASSTHRU
COMMENT (Comment)
The COMMENT tag adds comment text to the generated panel or message member.
Syntax
<COMMENT
TYPE=
END
CCSID
PANEL
ATTR
ABCINIT
ABCPROC
INIT
REINIT
PROC
HELP
PNTS
LIST
>
comment-text
</COMMENT>
COMMENT
Chapter 12. Tag reference  245

## Page 278

Parameters
TYPE=END | CCSID | PANEL | ATTR | ABCINIT | ABCPROC | INIT | REINIT | PROC | HELP | PNTS |
LIST
This attribute specifies the section of the panel that is to contain the comment text. The default is
END. TYPE=END is assumed if the COMMENT tag is used within the MSGMBR tag.
COMMENT tags that specify the TYPE as ABCINIT or ABCPROC must follow an ABC or PDC tag.
When a COMMENT tag is coded within a HELP panel, the TYPE value is limited to CCSID, PANEL, ATTR,
INIT, PROC, or END.
comment-text
The comment-text is flowed to a width of 66 bytes. The conversion utility adds "/* " before and " */"
after the resulting text.
When no comment-text is present, a blank comment line is added to the specified (or defaulted) panel
section.
Comments
The COMMENT tag adds comments to the generated ISPF format panel. If the PREP conversion option
has been specified, the comments are not part of the final panel because they are not processed by the
ISPPREP utility.
Lines of text from a COMMENT tag are added to the specified panel section when encountered in the DTL
source file.
Note: If the panel section specified is not generated by other conversion processing, comments are
formatted in this way:
TYPE
Position of comments
CCSID
Following the )PANEL statement.
LIST
Before the )END statement.
Comments added to the )END panel section are placed following any entries from the COPYR tag and
comments containing the ISPDTLC version number and panel creation date. Lines placed in the )END
section of a HELP panel are added to each continuation HELP panel.
Restrictions
• You must code the COMMENT tag within an ABC, AREA, CHOICE, DA, DTACOL DTAFLD, HELP, LSTCOL,
LSTFLD, LSTGRP, PANEL, PDC, REGION or SELFLD tag definition.
Processing
None.
Examples
Here is source file markup that contains a comment of several lines that are placed after the )END panel
statement. Figure 98 on page 247 shows portion of the ISPF format panel containing the formatted result.
<!doctype dm system>
<!-- COMMENT tag example - PANEL tag -->
<!--   )END section - after CMDAREA tag  -->
<varclass name=vc1 type='char 10'>
<varclass name=vc2 type='char 6'>
COMMENT
246  z/OS: z/OS ISPF DTL Guide

## Page 279

<varlist>
<vardcl name=lst1 varclass=vc1>
<vardcl name=lst2 varclass=vc2>
</varlist>
<panel name=comment1 depth=19 width=50>
This is panel Comment1
<LSTFLD >
<LSTGRP headline=yes>
<LSTCOL colwidth=10 datavar=lst1 usage=in varclass=vc1 line=1
        required=yes autotab=yes align=end help=h1 msg=abcd101>COL1
<LSTCOL colwidth=6  datavar=lst2 usage=in varclass=vc2 line=2
        required=yes autotab=yes align=end help=h1 msg=abcd101>COL2
</LSTGRP>
</LSTFLD>
<cmdarea>
<comment type=end>
  comment line 1
    comment line 2
      comment line 3
  comment line 4
    comment line 5
      comment line 6
  comment line 7
    comment line 8
      comment line 9
</panel>
⋮
)END
/* comment line 1 comment line 2 comment line 3 comment line 4        */
/* comment line 5 comment line 6 comment line 7 comment line 8        */
/* comment line 9                                                     */
Figure 98. Comment text added to a panel
COMPOPT (Compiler Options)
The COMPOPT tag sets compiler options for the current source file.
COMPOPT
Chapter 12. Tag reference  247

## Page 280

Syntax
<COMPOPT
REPLACE
NOREPLACE
SCREEN
DISK
NODBCS
DBCS NOKANA
KANA
KEYLAPPL=xxxx
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
NOMCOMMENT
MCOMMENT
NOV3PADC
V3PADC
ADD RESET national-language
>
</COMPOPT>
With the exception of ADD and RESET, all of the option keywords used for the COMPOPT tag are the same
as those used for “Conversion utility syntax” on page 156. See that chapter for information about these
keywords.
The COMPOPT tag keyword RESET restores the conversion utility options to their original invocation
settings.
Comments
The COMPOPT tag can be placed within the Doctype definition to encompass the entire DTL source file
or it can be placed before the first PANEL, HELP, MSGMBR, KEYL, or CMDTBL tag that requires a compiler
option change.
Unless the ADD option is specified when the COMPOPT tag is processed, all conversion utility options
except PANEL, DISK, SCREEN, DISPLAY, DISPLAYW, DBCS, and KANA are first reset to the defined default
values. The options specified on the COMPOPT tag are then applied.
COMPOPT
248  z/OS: z/OS ISPF DTL Guide

## Page 281

When the ADD option is included, the original options remain in effect and the options from the COMPOPT
tag are added to the current list. ADD overrides any existing option.
The options set by this tag remain in effect for the current source file until another COMPOPT tag is
processed. If you are converting a list of members, either from member list selections or from a DTLLST
list of members, the conversion utility options are reset to their original invocation settings when the
current source file is completed.
The PROFILE and PROFDDN options defined as part of the conversion utility invocation syntax are not
supported by the COMPOPT tag.
Restrictions
None.
Processing
None.
Examples
This source file markup contains a compiler options line that specifies the compiler options to be used
converting this source file:
<!doctype dm system>
<varclass name=vc1 type='char 10'>
<varclass name=vc2 type='char 6'>
<varlist>
<vardcl name=lst1 varclass=vc1>
<vardcl name=lst2 varclass=vc2>
</varlist>
<compopt noprep noreplace>
<panel name=compopt depth=19 width=50>
This is panel Compopt
<LSTFLD >
  <LSTGRP headline=yes>
    <LSTCOL colwidth=10 datavar=lst1 usage=in varclass=vc1 line=1
            required=yes autotab=yes align=end help=h1 msg=abcd101>COL1
    <LSTCOL colwidth=6  datavar=lst2 usage=in varclass=vc2 line=2
            required=yes autotab=yes align=end help=h1 msg=abcd101>COL2
  </LSTGRP>
</LSTFLD>
<cmdarea>
 </panel>
COPYR (Copyright)
The COPYR tag adds copyright text to the generated panel or message member.
Syntax
<COPYR>
copyright-text </COPYR>
Parameters
copyright-text
The copyright-text is limited to 66 bytes. It is automatically formatted as a panel comment with a "/* "
in front and a " */" following the supplied text.
COPYR
Chapter 12. Tag reference  249

## Page 282

Comments
The COPYR tag adds copyright information to the panel.
The COPYR tag must be placed before the first PANEL, HELP, or MSGMBR definition within the DTL source
file that is to contain the copyright information.
You can use multiple COPYR tags. Each tag creates one comment line, which is placed after the )END
panel statement, or the last message in the message member, in the order found in the DTL source.
The copyright-text is added to each subsequent panel or message member generated from the same DTL
source file member. If the PREP conversion option has been specified, the copyright is not part of the final
panel because comments are not processed by the ISPPREP utility.
Restrictions
None.
Processing
None.
Examples
Here is source file markup that contains two copyright lines that are placed after the )END panel
statement. Figure 99 on page 250 shows a portion of the ISPF format panel containing the formatted
result.
<!doctype dm system>
<!-- COPYR tag example - PANEL tag -->
<varclass name=vc1 type='char 10'>
<varclass name=vc2 type='char 6'>
<varlist>
<vardcl name=lst1 varclass=vc1>
<vardcl name=lst2 varclass=vc2>
</varlist>
<copyr>Copyright statement 1
<copyr>Copyright statement 2
<panel name=copyrt1 depth=19 width=50>
This is panel Copyrt1
<LSTFLD >
<LSTGRP headline=yes>
<LSTCOL colwidth=10 datavar=lst1 usage=in varclass=vc1 line=1
        required=yes autotab=yes align=end help=h1 msg=abcd101>COL1
<LSTCOL colwidth=6  datavar=lst2 usage=in varclass=vc2 line=2
        required=yes autotab=yes align=end help=h1 msg=abcd101>COL2
</LSTGRP>
</LSTFLD>
<cmdarea>
</panel>
⋮
)END
/* Copyright statement 1 */
/* Copyright statement 2 */
Figure 99. Copyright statement added to a panel
DA (Dynamic Area)
The DA tag defines a dynamic area in the panel )BODY section.
DA
250  z/OS: z/OS ISPF DTL Guide

## Page 283

Syntax
<DA NAME=varname
EXTEND=
OFF
ON
FORCE
LVLINE=variable-name
SCROLL=
OFF
ON
CMDLINE
USERMOD= usermod-code
%varname
DATAMOD= datamod-code
%varname
DEPTH= n
*
WIDTH=n SHADOW=shadow-name
DIV=
NONE
BLANK
SOLID
DASH
TEXT
DIV options
SCROLLVAR=scroll-variable
SCRVHELP=
NO
YES
scroll-help-panel-name
*scroll-help-message-id
%varname
*%varname
SCROLLTAB=
NO
YES SCRCAPS=
OFF
ON
INITATTR=
NT
CT
ET
WT
WASL
HELP=
NO
YES
help-panel-name
*help-message-id
%varname
*%varname
>
</DA>
DIV options
DA
Chapter 12. Tag reference  251

## Page 284

FORMAT= START
CENTER
END
TEXT=divider-text
Parameters
NAME=varname
This attribute defines the name of a dynamic area. This name is the dialog variable specified by
the application that contains the data for the dynamic area. The varname must follow the standard
naming convention described in “Rules for variable names” on page 179.
EXTEND=OFF | ON | FORCE
This attribute defines the runtime display size of the dynamic area. If EXTEND=ON is specified, the
dynamic area definition is expanded to the size of the logical screen. If you intend to display the
panels in a pop-up window, use EXTEND=OFF (which is the default).
If EXTEND=FORCE is specified within a horizontal area or region, the EXTEND(ON) keyword is added
to the dynamic area attribute statement in the )ATTR panel section. The conversion utility issues a
message to advise of a potential display error if other panel fields are formatted on or after the last
defined line of the dynamic area.
LVLINE=variable-name
This attribute allows you to specify the name of a variable that contains the result of the ISPF function
LVLINE. The variable-name must follow the standard naming convention described in “Rules for
variable names” on page 179.
SCROLL=OFF | ON | CMDLINE
If you specify SCROLL=ON or SCROLL=CMDLINE, ISPDTLC adds the scroll amount field provided by
the SCROLLVAR attribute to the command line.
If you specify SCROLL=ON, ISPDTLC also automatically enables scrolling commands by adding
SCROLL(ON) to the dynamic area attribute definition.
Note: When SCROLL(ON) is not part of the dynamic area attribute definition, data in the scroll amount
field is available to the application exactly as entered.
The first dynamic area on a panel that specifies SCROLL=ON or SCROLL=CMDLINE (with a valid
SCROLLVAR attribute) controls the creation of the scroll amount field. The specification of the SCROLL
attribute on subsequent DA tags is ignored.
USERMOD=usermod-code | %varname
This attribute specifies a single-character or a 2-position hexadecimal value to be substituted for
attribute characters in a dynamic area variable following user interaction. You can define this attribute
as a variable name preceded by a “%”.
DATAMOD=datamod-code | %varname
This attribute specifies a single-character or a 2-position hexadecimal value to be substituted for
attribute characters in a dynamic area following user interaction. You can define this attribute as a
variable name preceded by a “%”.
DEPTH=n | *
This attribute specifies the number of lines reserved for the dynamic area definition.
If the DA tag is to be formatted in the panel )BODY section, that is, the tag is not within a scrollable
area:
• The maximum DEPTH value is the DEPTH value specified on the PANEL tag, reduced by the number
of divider lines (if the DIV attribute is specified) and any other lines previously used by text or
interactive fields.
• If the DEPTH value is specified as an asterisk (*), the conversion utility reserves the remaining
available panel depth for the dynamic area.
DA
252  z/OS: z/OS ISPF DTL Guide

## Page 285

If the DA tag is defined within a scrollable area (see “AREA (Area)” on page 189), * cannot be specified
as the depth value. The maximum DEPTH value is limited by the ISPF runtime environment.
WIDTH=n
This attribute specifies the number of columns reserved in the panel )BODY section for the dynamic
area definition. If the dynamic area width is less than the PANEL width, the conversion utility adds
an attribute byte immediately following the right dynamic area boundary. The minimum width for a
dynamic area is the length of varname plus two (2) positions. The maximum value is the remaining
panel width.
SHADOW=shadow-name
This attribute provides a name for a shadow variable name which is used to define character
level attributes within the dynamic area string. The shadow-name must follow the standard naming
convention described in “Rules for variable names” on page 179.
DIV=NONE | BLANK | SOLID | DASH | TEXT
This attribute specifies the type of divider line to be placed before and after the dynamic area. If
this attribute is not specified or has the value NONE, no divider line is generated. The value BLANK
produces a blank line. You must specify SOLID, DASH, or TEXT to produce a visible divider line. When
the GRAPHIC invocation option is specified, SOLID produces a solid line for host display and DASH
produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH produce a dashed line.
A visible divider line formats with a non-displayable attribute byte on each end of the line.
FORMAT=START | CENTER | END
This attribute specifies the position of the divider-text within the divider line. You must specify
both the FORMAT attribute and the TEXT attribute to create a divider line containing text.
TEXT=divider-text
This attribute specifies the text to be placed on the divider line. You must specify both the
FORMAT attribute and the TEXT attribute to create a divider line containing text.
SCROLLVAR=scroll-variable
This attribute specifies the name of a variable that the application uses to obtain scrolling information.
The scroll-variable must follow the standard naming convention described in “Rules for variable
names” on page 179.
SCRVHELP=NO | YES | scroll-help-panel-name | *scroll-help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for the field specified with
the SCROLLVAR attribute.
When SCRVHELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a choice and no help is defined, the extended help panel is displayed. If
an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The scroll-help-panel-name must follow the standard naming convention described in “Rules for
variable names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
SCROLLTAB=NO | YES
If you specify SCROLLTAB=YES, the cursor moves to the next input field when the user enters the last
character in the scroll amount field. If there is no other input field on the panel, the cursor moves to
the beginning of the command line.
SCRCAPS=OFF | ON
When SCRCAPS=ON, the data in the scroll field is displayed in uppercase characters.
DA
Chapter 12. Tag reference  253

## Page 286

INITATTR=NT | CT | ET | WT | WASL
This attribute specifies the last attribute found before the start of the dynamic area. This allows the
developer control of the initial color for the area. The conversion utility replaces the last attribute
found before the dynamic area with the attribute specified.
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies whether help is available for the dynamic area.
When HELP=YES, requesting help when the cursor is within the dynamic area causes control to return
to the application. It is the application's responsibility to process the help request. You can specify
either a help panel or a message identifier. If a message identifier is used, it must be prefixed with an
asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help in a dynamic area and no help is defined, the extended help panel is
displayed. If an extended help panel is not defined for the panel, the application or ISPF tutorial is
invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
Comments
The DA tag defines a dynamic area in the panel )BODY or )AREA sections.
If you specify the CMDAREA tag within your DTL source file, it must appear before the DA tag when
DEPTH=* is specified. The DA tag DEPTH may have to be adjusted to allow for additional lines which
result from tags present within the panel definition following the end DA tag.
See the z/OS ISPF Dialog Developer's Guide and Reference for a discussion of dynamic areas.
Restrictions
• You must code the DA tag within a PANEL, AREA, or REGION tag. If found anywhere else, an error is
logged and the output panel is not saved.
• If NAME is not valid or not specified, an error is logged and the output panel is not saved.
• You can use the EXTEND=ON attribute only once within a panel, and EXTEND=ON cannot be specified
on a DA tag coded within a scrollable area. If EXTEND is already active, either from a DA tag, or from an
AREA, GA, SELFLD or REGION tag, a warning message is logged and the EXTEND attribute is ignored.
• You can use the SCROLLVAR attribute only once within a panel.
• If you specify the SCROLLVAR attribute, you must also specify the attribute SCROLL=ON or
SCROLL=CMDLINE.
• The resulting scroll entry on the command line must leave at least eight positions for the command
entry field.
• If you specify the SCRVHELP attribute, you must also specify the SCROLLVAR attribute.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
DA
254  z/OS: z/OS ISPF DTL Guide

## Page 287

Processing
Table 22. The tags you can code within a DA definition 
Tag Reference Usage Required
ATTR “ATTR (Attribute)” on page 200 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=da KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST> Type in patron's name and card number (if applicable)
<AREA>
    <DTACOL PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25 SELWIDTH=25>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8>Date
    <DTAFLD DATAVAR=cardno ENTWIDTH=7>Card No.
      <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name>Name
      <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address>Address
   </DTACOL>
  <DIVIDER>
  <DA NAME=darea DIV=solid DEPTH=6 SHADOW=shadwvar>
    <ATTR ATTRCHAR=#  TYPE=datain    PADC='_'  COLOR=BLUE>
    <ATTR ATTRCHAR=|  TYPE=dataout   COLOR=green>
    <ATTR ATTRCHAR=$  TYPE=char      COLOR=red>
  </DA>
</AREA>
<CMDAREA>Enter a command
</PANEL>
DD (Definition Description)
The DD tag defines the description of a term in a definition list.
Syntax
<DD>
definition-description </DD>
Parameters
definition-description
This is the text for the description of a definition list term.
Comments
The DD tag defines the description of a term in a definition list.
DD
Chapter 12. Tag reference  255

## Page 288

Restrictions
• You must code the DD tag within a DL definition. See “DL (Definition List)” on page 261 for a complete
description of this tag.
• Each DD tag must follow an associated DT tag within the definition list. You can code only one DD tag for
each DT tag.
Processing
Table 23. The tags you can code within a DD definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains a definition list with three definition descriptions. Figure 100 on
page 257 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=dd DEPTH=22 WIDTH=64>Help for Markup
<AREA>
<INFO>
  <P>Here are some definitions:
  <DL TSIZE=2 BREAK=all>
    <DT>markup
    <DD>Text that is added to document data in order to
    convey information about it.
    There are three types of markup the DTL uses:  tags, references,
    and markup declarations.
    <DT>markup declaration
    <DD>Markup that controls how other markup of a document
    is to be interpreted, for example document type and entity declarations.
    <DT>markup language
    <DD>A set of characters, conventions, and rules to control
    the interpretation of document data.
    The Dialog Tag Language is a markup language.
DD
256  z/OS: z/OS ISPF DTL Guide

## Page 289

</DL>
</INFO>
</AREA>
</HELP>
                        Help for Markup
 Here are some definitions:
 markup
   Text that is added to document data in order to convey
   information about it. There are three types of markup the
   DTL uses:  tags, references, and markup declarations.
 markup declaration
   Markup that controls how other markup of a document is to be
   interpreted, for example document type and entity
   declarations.
 markup language
   A set of characters, conventions, and rules to control the
   interpretation of document data. The Dialog Tag Language is
   a markup language.
  F1=Help         F3=Exit         F5=Exhelp       F6=Keyshelp
  F7=PrvTopic     F8=NxtTopic    F10=PrvPage     F11=NxtPage
 F12=Cancel
Figure 100. Definition  descriptions
DDHD (Definition Description Header)
The DDHD tag defines the heading for the description column of a definition list.
Syntax
<DDHD>
definition-description-header </DDHD>
Parameters
definition-description-header
This is the text of the definition description header.
Comments
The DDHD tag defines the heading for the description column of a definition list. You can code multiple
DDHD tags within a definition list.
The conversion utility inserts a blank line between the header and the list items unless the COMPACT
attribute is specified on the DL tag.
Restrictions
• You must code the DDHD tag within a DL definition. See “DL (Definition List)” on page 261 for a
complete description of this tag.
• Each DDHD tag must be paired with and follow a DTHD tag. See “DTHD (Definition Term Header)” on
page 287 for a complete description of this tag.
DDHD
Chapter 12. Tag reference  257

## Page 290

Processing
Table 24. The tags you can code within a DDHD definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is help panel markup that contains a definition description header with the text “Meaning”. Figure
101 on page 258 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=ddhd DEPTH=18>Prefix Help
<AREA>
<INFO>
  <P>The following list defines each of the valid prefixes.
  <DL TSIZE=12>
    <DTHD>Prefix
    <DDHD>Meaning
    <DT>AU
    <DD>Automotive
    <DT>HB
    <DD>Health and beauty
    <DT>LG
    <DD>Lawn and garden
    <DT>SG
    <DD>Sporting goods
  </DL>
</INFO>
</AREA>
</HELP>
                   Prefix Help
 The following list defines each of the valid
 prefixes.
 Prefix      Meaning
 AU          Automotive
 HB          Health and beauty
 LG          Lawn and garden
 SG          Sporting goods
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 101. Definition  description header
DIVIDER (Area Divider)
The DIVIDER tag creates a blank or visible divider within the text portion of an application panel.
DIVIDER
258  z/OS: z/OS ISPF DTL Guide

## Page 291

Syntax
<DIVIDER
TYPE=
NONE
SOLID
DASH
TEXT
GAP=
YES
NO
GUTTER=
1
n
NOENDATTR
FORMAT= START
CENTER
END
>
divider-text </DIVIDER>
Parameters
TYPE=NONE | SOLID | DASH | TEXT
This attribute specifies the type of divider line. The line width is one character.
The default value is NONE, which produces a blank line. You must specify SOLID, DASH, or TEXT to
produce a visible divider line. When the GRAPHIC invocation option is specified, SOLID produces a
solid line for host display and DASH produces a dashed line. When NOGRAPHIC is specified, both
SOLID and DASH produce a dashed line.
GAP=YES | NO
When GAP=NO, the divider line completely crosses from one side of the text area to the other. When
GAP=YES, a 1-character gap remains at each end of the divider line. However, GAP=YES is ignored
and set to NO for dividers coded within horizontal regions.
GUTTER=1 | n
This attribute specifies the total width of the divider. If the GUTTER value is an even number, the
conversion utility increases the number by 1 so that the divider is centered within the defined width.
The minimum GUTTER value is 1. If GUTTER=1 on a DIVIDER within a horizontal region, then the
TYPE value must be NONE.
The default GUTTER value for a DIVIDER within a vertical region is 1. The default GUTTER value
for dividers within horizontal regions is 3 to allow for an attribute byte on each side of the divider
character.
NOENDATTR
This attribute is valid only when the DIVIDER tag is coded within a horizontal region. It specifies that
no ending attribute character is placed after the divider character.
Note: The minimum divider space that can be specified for a horizontal region is 1.
When the GUTTER value is 1, the divider character is set to blank.
When the GUTTER value is 2, a solid divider may be specified. The divider character is placed in the
second position of the 2-character GUTTER space.
FORMAT=START | CENTER | END
This attribute specifies the position of the divider text within the width of the divider line.
divider-text
This is the text of the area divider line.
DIVIDER
Chapter 12. Tag reference  259

## Page 292

Comments
The DIVIDER tag creates a blank or solid divider within the text portion of an application panel. A
horizontally formatted visible divider is created when you specify the TYPE attribute value as SOLID or
DASH. When the GRAPHIC invocation option is specified, SOLID produces a solid line for host display and
DASH produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH produce a dashed
line. A vertically formatted SOLID or DASH divider is the "|" character which is obtained from the ISPF
literals table. The direction of the divider is determined by the tag definition it is coded within. Here are
the details for formatting for dividers:
• Dividers coded within an AREA, HELP, or PANEL tag definition format horizontally.
• Dividers coded within a vertical region format horizontally.
• Dividers coded within a horizontal region format vertically.
The divider line can be formatted with descriptive text. When this feature is used, the FORMAT attribute
must be specified. If FORMAT is not specified, the tag text is ignored. You control the text padding
with the TYPE attribute. If TYPE=TEXT, the divider-text is padded with blanks. When TYPE=SOLID or
TYPE=DASH, the divider-text is padded with the specified character.
Restrictions
• You must code the DIVIDER tag within an AREA, DTACOL, HELP, PANEL, or REGION definition. See
“AREA (Area)” on page 189, “DTACOL (Data Column)” on page 269, “HELP (Help Panel)” on page 303,
“PANEL (Panel)” on page 376, and “REGION (Region)” on page 405 for descriptions of these tags.
Processing
Table 25. The tags you can code within a DIVIDER definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
Examples
Here is application panel markup that contains four DIVIDER definitions. The first divider is blank. The
second divider is solid with a gutter size of 2 and a GAP=NO value. The third and fourth dividers are solid.
Figure 102 on page 261 shows the formatted result.
DIVIDER
260  z/OS: z/OS ISPF DTL Guide

## Page 293

<!DOCTYPE DM SYSTEM(
  <!entity sampvar3 system>)>
&sampvar3;
<PANEL NAME=divider DEPTH=22 WIDTH=70>Print a Document
<AREA>
  <DTACOL PMTWIDTH=20 ENTWIDTH=8 SELWIDTH=40 DESWIDTH=35>
    <DTAFLD DATAVAR=file>File name
      <DTAFLDD>Name of the document to be printed
    <DIVIDER TYPE=none>
    <SELFLD NAME=type PMTLOC=before>Type style for printing
      <CHOICE>Prestige Elite (12 pitch)
      <CHOICE>Courier (10 pitch)
      <CHOICE>Essay Standard (proportional)
      <CHOICE>Essay Bold (proportional)
    </SELFLD>
  </DTACOL>
  <DIVIDER TYPE=solid GUTTER=2 GAP=no>
  <DTACOL PMTWIDTH=20 ENTWIDTH=2 DESWIDTH=35>
    <DTAFLD DATAVAR=marg>Left margin
      <DTAFLDD>Number of spaces in the left margin
    <DIVIDER TYPE=solid>
    <DTAFLD DATAVAR=copy>Copies
      <DTAFLDD>Number of copies
    <DIVIDER TYPE=solid>
    <DTAFLD DATAVAR=duplx ENTWIDTH=1>Duplex
      <DTAFLDD>1 = Yes (Print both sides of paper)
      <DTAFLDD>2 = No (Print one side only)
  </DTACOL>
</AREA>
</PANEL>
                           Print a Document
 File name  . . . . . ________  Name of the document to be printed
 Type style for
 printing . . . . . . __  1.  Prestige Elite (12 pitch)
                          2.  Courier (10 pitch)
                          3.  Essay Standard (proportional)
                          4.  Essay Bold (proportional)
 --------------------------------------------------------------------
 Left margin  . . . . __  Number of spaces in the left margin
  ------------------------------------------------------------------
 Copies . . . . . . . __  Number of copies
  ------------------------------------------------------------------
 Duplex . . . . . . . _  1 = Yes (Print both sides of paper)
                         2 = No (Print one side only)
  F1=Help    F3=Exit   F12=Cancel
Figure 102. Area dividers
DL (Definition List)
The DL tag defines a list of terms and their corresponding definitions within an information region.
DL
Chapter 12. Tag reference  261

## Page 294

Syntax
<DL
TSIZE=
10
's1 s2... sn' BREAK=
NONE
FIT
ALL
COMPACT NOSKIP INDENT= n
FORMAT=
START
CENTER
END
DIVEND=
NO
YES
SPLIT=
NO
YES
> </DL>
Parameters
TSIZE=10 | 's1 s2... sn'
This attribute specifies the space to be allocated for the definition term. The default value is 10
characters. The minimum TSIZE value is 0 and the maximum is 40. When multiple TSIZE values are
specified, a DT tag must be coded for each value. The sizes are applied to the DT tags in the order the
tags are encountered in the DTL source file.
BREAK=NONE | FIT | ALL
This attribute controls the formatting of the definition terms and descriptions. If BREAK=NONE, the
term is on the same line as the description, spilling into the description area if the length exceeds
TSIZE. If BREAK=FIT, the description is on the line below the term if the term exceeds the TSIZE
value. If BREAK=ALL, every definition is on the line below the term.
COMPACT
This attribute causes the list to format without a blank line between the items in the list. If you code
DDHD and DTHD tags in a compact definition list, the list formats without a blank line between the
headers and list items.
NOSKIP
This attribute causes the list to format without creating a blank line before the first line of the list.
INDENT=n
This attribute specifies that the definition list is to be indented from the current left margin.
FORMAT=START | CENTER | END
This attribute specifies the placement of the DT tag text within the space specified by TSIZE. The DL
tag FORMAT setting applies to all of the DT tags within the definition list.
DIVEND=NO | YES
This attribute specifies whether a divider character is formatted following the DDHD and DD tag text.
When DIVEND=YES the formatting width of the DDHD and DD text is reduced to allow space for the
divider character.
SPLIT=NO | YES
This attribute controls the format of the last DT tag in a multiple DT tag group. It is used only
when BREAK=ALL or when BREAK=FIT and the DT tag text length exceeds the TSIZE value. When
SPLIT=YES, the text following the last DT tag in the DT group (typically one or two dashes) is placed in
front of the first line of the formatted DD tag text. The DL tag SPLIT setting applies to all of the DT tag
groups within the definition list.
DL
262  z/OS: z/OS ISPF DTL Guide

## Page 295

Comments
The DL tag defines a list of terms and their corresponding definitions within an information region. You use
the DT and DD tags to identify the terms that you are defining and their descriptions, respectively. You use
the DTHD and the DDHD tags to define headings for the term and description columns in definition lists.
The conversion utility inserts a blank line before the definition list unless NOSKIP is specified.
If you do not specify a TSIZE value, the space allocated for the term size is 10 characters. If any term
is longer than 10 characters and BREAK=NONE (the default) is specified, the term extends into the
description line. If the term is still too long to fit, it wraps to the next line.
The definition description is an implied paragraph, and can contain any text items. For example, you
can insert additional paragraphs in a definition description by using the paragraph (P) tag following the
description paragraph. Other tags that you want to nest within the definition list (such as OL, SL, or UL)
must follow the DD tag within the list.
Restrictions
• The DL tag requires an end tag.
• You must code the DL tag within an INFO definition. See “INFO (Information Region)” on page 317 for a
complete description of this tag.
• If you code DDHD and DTHD tags within the definition list, they must precede the first DT tag.
Processing
Table 26. The tags you can code within a DL definition 
Tag Reference Usage Required
DD “DD (Definition Description)” on page 255 Multiple No
DDHD “DDHD (Definition Description Header)” on page 257 Multiple No
DLDIV “DLDIV (Definition List Divider)” on page 265 Multiple No
DT “DT (Definition Term)” on page 267 Multiple No
DTDIV “DTDIV (Definition Term Divider)” on page 286 Multiple No
DTHD “DTHD (Definition Term Header)” on page 287 Multiple No
DTHDIV “DTHDIV (Definition Term Header Divider)” on page
288
Multiple No
Examples
Here is help panel markup that contains a definition list that uses the default BREAK value of NONE,
which formats the definition descriptions on the same line as the associated terms. Definition term and
description headers are also included. Figure 103 on page 264 shows the formatted result of the markup.
Figure 104 on page 264 shows how the same definition list would format with a BREAK value of FIT.
Figure 105 on page 265 shows how the same definition list would format with a BREAK value of ALL.
DL
Chapter 12. Tag reference  263

## Page 296

<!DOCTYPE DM SYSTEM>
<HELP NAME=dl DEPTH=22 WIDTH=60>Employee Code Help
  <AREA>
  <INFO>
    <P>The following list defines the valid employee codes.
    <DL TSIZE=11>
      <DTHD>Code
      <DDHD>Meaning
      <DT>Full-time
      <DD>Indicates that the employee works a
      regular schedule of 40 hours or more weekly.
      <DT>Part-time
      <DD>Indicates that the employee works a regular
      schedule of 20 to 40 hours weekly.
      <DT>Supplemental
      <DD>Indicates that the employee works less than
      20 hours weekly.
      No regular schedule is in place.
    </DL>
  </INFO>
  </AREA>
</HELP>
                     Employee Code Help
 The following list defines the valid employee codes.
 Code       Meaning
 Full-time  Indicates that the employee works a regular
            schedule of 40 hours or more weekly.
 Part-time  Indicates that the employee works a regular
            schedule of 20 to 40 hours weekly.
 Supplemental Indicates that the employee works less than
            20 hours weekly. No regular schedule is in
            place.
  F1=Help        F3=Exit        F5=Exhelp      F6=Keyshelp
  F7=PrvTopic    F8=NxtTopic   F10=PrvPage    F11=NxtPage
 F12=Cancel
Figure 103. Definition  List (BREAK=NONE)
                     Employee Code Help
 The following list defines the valid employee codes.
 Code       Meaning
 Full-time  Indicates that the employee works a regular
            schedule of 40 hours or more weekly.
 Part-time  Indicates that the employee works a regular
            schedule of 20 to 40 hours weekly.
 Supplemental
            Indicates that the employee works less than 20
            hours weekly. No regular schedule is in place.
  F1=Help        F3=Exit        F5=Exhelp      F6=Keyshelp
  F7=PrvTopic    F8=NxtTopic   F10=PrvPage    F11=NxtPage
 F12=Cancel
Figure 104. Definition  List (BREAK=FIT)
DL
264  z/OS: z/OS ISPF DTL Guide

## Page 297

Employee Code Help
 The following list defines the valid employee codes.
 Code       Meaning
 Full-time
            Indicates that the employee works a regular
            schedule of 40 hours or more weekly.
 Part-time
            Indicates that the employee works a regular
            schedule of 20 to 40 hours weekly.
 Supplemental
            Indicates that the employee works less than 20
            hours weekly. No regular schedule is in place.
  F1=Help        F3=Exit        F5=Exhelp      F6=Keyshelp
  F7=PrvTopic    F8=NxtTopic   F10=PrvPage    F11=NxtPage
 F12=Cancel
Figure 105. Definition  List (BREAK=ALL)
DLDIV (Definition List Divider)
The DLDIV tag creates a blank or visible divider within the text portion of an application panel.
Syntax
<DLDIV
TYPE=
NONE
SOLID
DASH
TEXT
GAP=
YES
NO
GUTTER=
1
n
FORMAT= START
CENTER
END
>
divider-text </DLDIV>
Parameters
TYPE=NONE | SOLID | DASH | TEXT
This attribute specifies the type of divider line. The line width is one character.
The default value is NONE, which produces a blank line. You must specify SOLID, DASH, or TEXT to
produce a visible divider line. When the GRAPHIC invocation option is specified, SOLID produces a
solid line for host display and DASH produces a dashed line. When NOGRAPHIC is specified, both
SOLID and DASH produce a dashed line.
GAP=YES | NO
When GAP=NO, the divider line completely crosses from one side of the text area to the other. When
GAP=YES, a 1-character gap remains at each end of the divider line.
DLDIV
Chapter 12. Tag reference  265

## Page 298

GUTTER=1 | n
This attribute specifies the total width of the divider. If the GUTTER value is an even number, the
conversion utility increases the number by 1 so that the divider is centered within the defined width.
The minimum GUTTER value, and the default, is 1.
FORMAT=START | CENTER | END
This attribute specifies the position of the divider text within the width of the divider line.
divider-text
This is the text of the area divider line.
Comments
The DLDIV tag creates a blank or solid divider within the text portion of an application panel. A
horizontally formatted visible divider is created when you specify the TYPE attribute value as SOLID or
DASH. When the GRAPHIC invocation option is specified, SOLID produces a solid line for host display and
DASH produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH produce a dashed
line.
The divider line can be formatted with descriptive text. When this feature is used, the FORMAT attribute
must be specified. If FORMAT is not specified, the tag text is ignored. You control the text padding
with the TYPE attribute. If TYPE=TEXT, the divider-text is padded with blanks. When TYPE=SOLID or
TYPE=DASH, the divider-text is padded with the specified character.
Restrictions
• You must code the DLDIV tag within a DL tag.
Processing
Table 27. The tags you can code within a DLDIV definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
Examples
Here is an example that shows the use of the DLDIV tag in combination with the multiple DT tag function
and the DIVEND attribute of the DL tag. Figure 106 on page 267 shows the formatted result.
DLDIV
266  z/OS: z/OS ISPF DTL Guide

## Page 299

<!DOCTYPE DM SYSTEM>
<HELP NAME=dldiv DEPTH=22 WIDTH=60>Employee Code Help
  <AREA>
  <INFO>
    <P>The following list defines the valid employee codes.
    <DL TSIZE=14 BREAK=none>
      <DLDIV TYPE=SOLID>
      <DTHD>Code
      <DDHD>Meaning
      <DLDIV TYPE=SOLID>
      <DT NOSKIP>Full-time
      <DD>Indicates that the employee works a
      regular schedule of 40 hours or more weekly.
      <DT>Part-time
      <DD>Indicates that the employee works a regular
      schedule of 20 to 40 hours weekly.
      <DT>Supplemental
      <DD>Indicates that the employee works less than
      20 hours weekly.
      No regular schedule is in place.
    </DL>
  </INFO>
  </AREA>
</HELP>
                    Employee Code Help
 The following list defines the valid employee codes.
 ----------------------------------------------------------
 Code          Meaning
 ----------------------------------------------------------
 Full-time     Indicates that the employee works a regular
               schedule of 40 hours or more weekly.
 Part-time     Indicates that the employee works a regular
               schedule of 20 to 40 hours weekly.
 Supplemental  Indicates that the employee works less than
               20 hours weekly. No regular schedule is in
               place.
Figure 106. Definition  list dividers
DT (Definition Term)
The DT tag defines a term in a definition list.
Syntax
<DT>
FORMAT= START
CENTER
END
NOSKIP
SPLIT=
NO
YES
>
definition-term </DT>
DT
Chapter 12. Tag reference  267

## Page 300

Parameters
FORMAT=START | CENTER | END
This attribute specifies the placement of the DT tag text within the space specified by TSIZE. The DT
tag FORMAT setting overrides the FORMAT specified in the enclosing DL tag.
NOSKIP
This attribute causes the definition term to be formatted without a blank line before the term. It is
used to control the formatting of the definition term when COMPACT has not been specified on the
enclosing DL tag. When the DL tag TSIZE attribute specifies that multiple DT tags are to be formatted
for each DD tag, NOSKIP should be coded on the first DT tag. It is ignored for the second and
subsequent DT tags.
SPLIT
This attribute controls the format of the last DT tag in a multiple DT tag group. It is used only
when BREAK=ALL or when BREAK=FIT and the DT tag text length exceeds the TSIZE value. When
SPLIT=YES, the text following the last DT tag in the DT group (typically one or two dashes) is placed
in front of the first line of the formatted DD tag text. The DT tag SPLIT setting overrides the SPLIT
specified in the enclosing DL tag.
definition-term
This is the text of the definition term.
Comments
The DT tag defines a term in a definition list.
Restrictions
• You must code the DT tag within a DL definition. See “DL (Definition List)” on page 261 for a complete
description of this tag.
• Each DT tag must be paired with and precede a DD tag.
Processing
Table 28. The tags you can code within a DT definition 
Tag Reference Usage Required
DTSEG “DTSEG (Definition Term Segment)” on page 290 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is help panel markup that contains a definition list with three definition terms. Each definition term is
paired with an associated definition description. Figure 107 on page 269 shows the formatted result.
DT
268  z/OS: z/OS ISPF DTL Guide

## Page 301

<!DOCTYPE DM SYSTEM>
<HELP NAME=dt DEPTH=22 WIDTH=64>Help for Markup
<AREA>
<INFO>
  <P>Here are some definitions:
  <DL TSIZE=2 BREAK=all>
    <DT>markup
    <DD>Text that is added to document data in order to
    convey information about it.
    There are three types of markup the DTL uses:  tags, references,
    and markup declarations.
    <DT>markup declaration
    <DD>Markup that controls how other markup of a document
    is to be interpreted, for example document type and entity declarations.
    <DT>markup language
    <DD>A set of characters, conventions, and rules to control
    the interpretation of document data.
    The Dialog Tag Language is a markup language.
  </DL>
</INFO>
</AREA>
</HELP>
                        Help for Markup
 Here are some definitions:
 markup
   Text that is added to document data in order to convey
   information about it.  There are three types of markup the
   DTL uses: tags, references, and markup declarations.
 markup declaration
   Markup that controls how other markup of a document is to be
   interpreted, for example document type and entity
   declarations.
 markup language
   A set of characters, conventions, and rules to control the
   interpretation of document data. The Dialog Tag Language is
   a markup language.
  F1=Help        F3=Exit        F5=Exhelp      F6=Keyshelp
  F7=PrvTopic    F8=NxtTopic   F10=PrvPage    F11=NxtPage
 F12=Cancel
Figure 107. Definition  Terms
DTACOL (Data Column)
The DTACOL tag defines default values for data fields (DTAFLD) and selection fields (SELFLD) that are
coded within a DTACOL definition.
DTACOL
Chapter 12. Tag reference  269

## Page 302

Syntax
<DTACOL
PMTWIDTH= n
*
**
ENTWIDTH=n
DESWIDTH= n
*
SELWIDTH= n
*
FLDSPACE=n PAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
PMTFMT=
CUA
ISPF
NONE
END
AUTOTAB=
NO
YES
ATTRCHANGE=
NO
YES
NEW
PMTLOC=
BEFORE
ABOVE
DBALIGN=
YES
NO
PROMPT
FIELD
FORCE
VARCLASS=variable-class-name
REQUIRED=
NO
YES CAPS=
OFF
ON
VARDCL=
YES
NO
> </DTACOL>
DTACOL
270  z/OS: z/OS ISPF DTL Guide

## Page 303

Parameters
PMTWIDTH=n | * | **
This attribute specifies the number of bytes reserved for prompts for data fields and selection fields
coded within the data column. The minimum width is 0 and the maximum is the remaining available
panel width. When you specify PMTWIDTH=*, the conversion utility uses the length of the prompt
text as the prompt width. When you specify PMTWIDTH=**, the conversion utility uses the maximum
available space as the prompt width. If PMTFMT=CUA is specified (or defaulted) and the prompt text
has fewer characters than the field allows, leader dots fill the remaining spaces. For output-only data
fields, a colon is also added as the last character in the prompt width space. If any prompt contains
more characters than the width you specify, the prompt is word-wrapped to fit on multiple lines.
Note: Any field within the data column defining a prompt width overrides the DTACOL PMTWIDTH
value.
ENTWIDTH=n
This attribute specifies the number of bytes reserved for data fields coded within the data column.
The minimum width is 1 and the maximum is the remaining available panel (or region) width.
Note: Any data field within the data column defining an entry width overrides the DTACOL ENTWIDTH
value.
DESWIDTH=n | *
This attribute specifies the number of bytes reserved for the description text of the enclosed DTAFLDD
tags. The minimum width is 0. When you specify DESWIDTH=*, the conversion utility uses the length
of the description text as the description width. If the text is longer than the width you specify, the text
is word-wrapped to fit on multiple lines.
Note: Any data field within the data column defining a description width overrides the DTACOL
DESWIDTH value.
SELWIDTH=n | *
This attribute specifies the number of bytes reserved for choices in selection fields coded within the
data column. The minimum width value is 1 and the maximum is the remaining available panel width.
If the width required by the choice-text and its entry field exceeds the specified SELWIDTH value, the
text is wrapped to multiple conversion utility will use the remaining available panel (or region) width.
Note: Any selection field within the data column defining a selection width overrides the DTACOL
SELWIDTH value.
FLDSPACE=n
This attribute specifies the number of bytes reserved for the data field. The minimum width is 2 and
the maximum is the remaining available panel (or region) width. The FLDSPACE value should include
the actual entry width plus the number of entry field attributes. If the value specified by ENTWIDTH
(plus attributes) is less than the specified FLDSPACE value, the entry field is padded with blanks to the
FLDSPACE value. This creates blank space between the entry field and description text provided by
the DTAFLDD tag and allows you to align description text from successive DTAFLD definitions.
Note: Any data field within the data column defining field space overrides the DTACOL FLDSPACE
value.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
Note: Any data field within the data column defining PAD overrides the DTACOL PAD value.
PADC=NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
Note: Any data field within the data column defining PADC overrides the DTACOL PADC value.
DTACOL
Chapter 12. Tag reference  271

## Page 304

OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
Note: Any data field within the data column defining OUTLINE overrides the DTACOL OUTLINE value.
PMTFMT=CUA | ISPF | NONE | END
This attribute controls the generation of prompt leader characters. The default is to create CUA leader
dots. When ISPF is specified, and at least 4 bytes of prompt text space remain following the prompt
text, the “===>” character string is placed in the rightmost 4 positions of the prompt text space. When
NONE is specified, no leader characters are added to the prompt text. When END is specified, the
prompt text is right justified within the prompt text space.
Note: Any data field within the data column defining PMTFMT overrides the DTACOL PMTFMT value.
AUTOTAB=NO | YES
When AUTOTAB=YES, the cursor moves to the next field capable of input when the user enters the
last character in this field. If no other field capable of user input exists on the panel, the cursor returns
to the beginning of this field.
Note: Any data field within the data column defining AUTOTAB overrides the DTACOL AUTOTAB value.
ATTRCHANGE=NO | YES | NEW
When ATTRCHANGE=YES or ATTRCHANGE=NEW, the conversion utility formats an additional entry
in the panel )ATTR section (that can apply to multiple data fields) instead of creating a unique
".ATTR(field-name)" entry in the )INIT section for each field. With this option, multiple DTAFLD tags
with the same characteristics require fewer panel logic statements. ATTRCHANGE=NEW creates a
new entry. ATTRCHANGE=YES uses an existing attribute, if possible.
Note: Any data field within the data column defining ATTRCHANGE overrides the DTACOL
ATTRCHANGE value.
PMTLOC=BEFORE | ABOVE
This attribute defines the prompt location for the enclosed DTAFLD and SELFLD tags.
Note: Any data field or selection field within the data column defining PMTLOC overrides the DTACOL
PMTLOC value.
DBALIGN=YES | NO | PROMPT | FIELD | FORCE
This attribute defines the DBALIGN value for the enclosed DTAFLD tags.
Note: Any data field within the data column defining DBALIGN overrides the DTACOL DBALIGN value.
VARCLASS=variable-class-name
This attribute defines the name of the variable class for enclosed CHOFLD and DTAFLD tags.
Note: Any data field within the data column defining VARCLASS overrides the DTACOL VARCLASS
value.
REQUIRED=NO | YES
This attribute defines whether the fields for enclosed CHOFLD and DTAFLD tags require input.
Note: Any data field within the data column defining REQUIRED overrides the DTACOL REQUIRED
value.
CAPS=OFF | ON
This attribute defines whether the fields for enclosed CHOFLD and DTAFLD tags are displayed in
uppercase characters.
Note: Any data field within the data column defining CAPS overrides the DTACOL CAPS value.
VARDCL=YES | NO
When VARDCL=NO the data field name is not checked to the declared variable information provided
with the VARCLASS and VARDCL tags for enclosed CHOFLD and DTAFLD tags.
Note: Any data field within the data column defining VARDCL overrides the DTACOL VARDCL value.
DTACOL
272  z/OS: z/OS ISPF DTL Guide

## Page 305

Comments
The DTACOL tag defines default attribute values for data fields (DTAFLD), choice data fields (CHOFLD),
and selection fields (SELFLD) that are coded within a DTACOL definition. This allows you to define
common values for fields coded within the data column within a single tag definition.
The xxxWIDTH attributes are convenient for aligning fields on an application panel. Fields are laid out
within the data column along boundaries established by the values specified on the DTACOL tag. This
example shows those boundaries:
Figure 108. PMTWIDTH, ENTWIDTH, FLDSPACE, and DESWIDTH attributes
The prompt width (PMTWIDTH) is valid for data fields and selection fields coded within the data column
description. The entry width (ENTWIDTH), field space (FLDSPACE), and description width (DESWIDTH) are
only used by enclosed DTAFLD tags. The selection width (SELWIDTH) is used only by enclosed SELFLD
tags. All of the previous cases stated are true only when the enclosed DTAFLD or SELFLD tags do not
specify values that override the DTACOL values.
Note: The SELFLD tag does not use the ENTWIDTH, DESWIDTH, FLDSPACE, PAD, PADC, OUTLINE,
AUTOTAB, ATTRCHANGE, DBALIGN, VARCLASS, REQUIRED, or CAPS attributes of the DTACOL tag.
If the combined PMTWIDTH, ENTWIDTH, and DESWIDTH values exceed the remaining available panel (or
region) width, the conversion utility issues a warning message and attempts to fit the data in the available
width by wrapping the text.
For data fields, first priority is given to the entry field. Second and third priorities are given to the prompt
and description fields, respectively. These fields use the available width remaining after the width of the
entry field is determined.
Note: Word wrapping can result in word truncation if insufficient width is available for the text.
When the maximum number of requested attributes for a panel is exceeded, the conversion utility issues
error message ISPC804E. The number of requested attributes includes attribute override entries. These
are .ATTR entries that are added by the conversion utility for attributes that are specified on CHOFLD,
DTACOL, DTAFLD, LSTCOL, and LSTFLD tags. If the same set of attributes is specified on multiple tags,
duplicate .ATTR entries are added by default. Adding the parameter ATTRCHANGE=YES to the tags
causes the compiler to instead add a single entry in the panel )ATTR section for each unique set of
attributes specified. The entry for a set of attributes is then shared by all tags that specify that set of
attributes.
Restrictions
• The DTACOL tag requires an end tag.
• You must code the DTACOL tag within an AREA, PANEL, or REGION definition. You can code a DTACOL
definition anywhere within these tags, but the start and end tags must enclose any DTAFLD or SELFLD
tags to which it applies. See “AREA (Area)” on page 189, “PANEL (Panel)” on page 376, and “REGION
(Region)” on page 405 for descriptions of these tags.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
DTACOL
Chapter 12. Tag reference  273

## Page 306

Processing
Table 29. The tags you can code within a DTACOL definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
DTAFLD “DTAFLD (Data Field)” on page 275 Multiple No
GRPHDR “GRPHDR (Group Header)” on page 300 Multiple No
SELFLD “SELFLD (Selection Field)” on page 421 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is application panel markup that contains a data column that provides default width values for the
enclosed data fields and data field descriptions. The ENTWIDTH value specified on the first and second
data fields override the ENTWIDTH value specified on the DTACOL tag. Figure 109 on page 275 shows the
formatted result.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=dtacol2 KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST> Type in patron's name and card number (if applicable)
<TOPINST> Then select an action bar choice.
<AREA>
  <DTACOL PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25 SELWIDTH=25
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8>Date
    <DTAFLD DATAVAR=cardno ENTWIDTH=7>Card No.
      <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name>Name
      <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address>Address
  </DTACOL>
  <DIVIDER>
  <REGION DIR=horiz>
  <SELFLD NAME=cardsel PMTWIDTH=30 SELWIDTH=38>Choose
  one of the following
    <CHOICE CHECKVAR=card MATCH=new>New
    <CHOICE CHECKVAR=card MATCH=renew>Renewal
    <CHOICE CHECKVAR=card MATCH=replace>Replacement
  </SELFLD>
  <SELFLD TYPE=multi PMTWIDTH=30 SELWIDTH=25>Check valid branches
    <CHOICE NAME=north HELP=nthhlp CHECKVAR=nth>North Branch
    <CHOICE NAME=south HELP=sthhlp CHECKVAR=sth>South Branch
    <CHOICE NAME=east HELP=esthlp CHECKVAR=est>East Branch
    <CHOICE NAME=west HELP=wsthlp CHECKVAR=wst>West Branch
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>Enter a command
</PANEL>
DTACOL
274  z/OS: z/OS ISPF DTL Guide

## Page 307

File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number if applicable.
 Then select an action bar choice.
 Date . . . : 08/29/90
 Card No. . . _______                    (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 109. Data column
DTAFLD (Data Field)
The DTAFLD tag defines an input field, an output field, or an input/output field on an application panel.
DTAFLD
Chapter 12. Tag reference  275

## Page 308

Syntax
<DTAFLDNAME=field-nameDATAVAR=field-data
VARCLASS=variable-class-nameHELP= NOYEShelp-panel-name
*help-message-id
%varname*%varname
USAGE=BOTHINOUT
REQUIRED= NOYESYESMSG=message-identifier
AUTOTAB=NOYES ENTWIDTH=n
PMTWIDTH=n***
DESWIDTH=n*
FLDSPACE=nALIGN=STARTCENTEREND
PMTLOC=BEFOREABOVE DISPLAY=YESNO
NOENDATTRPAD= NULLSUSERchar%varname
PADC=NULLSUSERchar%varname
OUTLINE=NONELROUBOX%varname
PMTFMT=CUAISPFNONEEND
PSVAR=point-and-shoot-variable
%varname
PSVAL=point-and-shoot-value
%varname
PAS=%varname
CSRGRP=NOYESn
EXPANDFLDWIDTH=n
ATTRCHANGE=NOYESNEW
INIT=initial-value
DEPTH=n%varname
IMAPNAME=image-name
%varname
IMAP group
DBALIGN=YESNOPROMPTFIELDFORCE
PMTSKIP=NOYES
DESSKIP=NOYES FLDTYPE=CUAISPF
COLOR=WHITEREDBLUEGREENPINKYELLOWTURQ%varname
INTENS=HIGHLOWNON%varname
HILITE=USCOREBLINKREVERSE%varname
ATTRCHAR=code
CAPS=OFFON NOJUMP=OFFON
AUTOTYPE=PROJECTGROUP1GROUP2GROUP3GROUP4TYPEMEMBERDSN
AUTOVOL=volser-nameAUTODMEM=YESNO VARDCL=YESNO
> prompt-text</DTAFLD>
IMAP group
DTAFLD
276  z/OS: z/OS ISPF DTL Guide

## Page 309

IMAPNAMEP= image-namep
%varname PLACE=
ABOVE
BELOW
LEFT
RIGHT
%varname
Parameters
NAME=field-name
This attribute specifies the name of the field. The field -name  must follow the standard naming
convention described in “Rules for variable names” on page 179.
The field -name  can be used by:
• The PANEL tag to position the cursor
• The ISPF DISPLAY or TBDISPL services to position the cursor
• The ISPF ADDPOP service to position a pop-up.
DATAVAR=field-data
This attribute specifies the variable name for the data in the field. The value coded must be a
variable-name without the leading % notation. The conversion utility considers NAME and DATAVAR to
be synonymous. However, the value you assign DATAVAR has precedence. For example, if you specify
different values for the DATAVAR and NAME attributes, the conversion utility uses the DATAVAR value
as the name of the field on the panel.
Compatibility considerations
DATAVAR is a required attribute for the DTAFLD tag. For compatibility between releases, you can
code either the NAME or the DATAVAR attributes, or both.
VARCLASS=variable-class-name
This attribute specifies the name of the variable class, defined using a VARCLASS tag, that overrides
the default variable class referred to by the VARDCL that declared the data variable for this field.
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for this data field. This is
field-level help.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help for the data field and no help is defined, the extended help panel is
displayed. If an extended help panel is not defined for the panel, the application or ISPF tutorial is
invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
USAGE=BOTH | IN | OUT
This attribute indicates whether the field is for input only, output only, or both.
For USAGE=OUT, the conversion utility inserts a colon as the last character of the data field prompt to
indicate to the user that it is an output-only field.
DTAFLD
Chapter 12. Tag reference  277

## Page 310

REQUIRED=NO | YES
This attribute indicates if the field requires input. This attribute is valid only when USAGE=IN or BOTH.
If REQUIRED=YES is coded, a VER(variable,NONBLANK) statement is built by the conversion utility
and placed in the )PROC section of the ISPF panel generated.
MSG=message-identifier
This attribute specifies the message that is displayed when the user does not complete a required
entry (defined with the REQUIRED attribute). If you do not specify a mes sage -identifier , ISPF
displays a default message.
If you specify the MSG attribute and REQUIRED=YES, a VER(variable,NONBLANK,MSG=message-
identifier) statement is built by the conversion utility and placed in the )PROC section of the ISPF
panel generated. If you specify the MSG attribute and REQUIRED=NO (the default), the conversion
utility issues a warning message.
See “MSG (Message)” on page 352 for information about creating messages.
Note: You can specify messages pertaining to other validations using XLATL and CHECKL tags
within a VARCLASS definition. See the descriptions of these tags for additional information.
AUTOTAB=NO | YES
When AUTOTAB=YES, the cursor moves to the next field capable of input when the user enters the
last character in this field. If no other field capable of user input exists on the panel, the cursor returns
to the beginning of this field.
AUTOTAB=YES is valid only when the value for USAGE is either BOTH or IN. If specified, this attribute
overrides the AUTOTAB value of the DTACOL tag.
ENTWIDTH=n
This attribute specifies the number of bytes used for the data field. The minimum width is 1 and the
maximum is the remaining available panel width less the required amount of space for field attributes.
If ENTWIDTH is not provided on either the DTAFLD tag or the enclosing DTACOL tag, the conversion
utility uses the width determined by the TYPE value of the associated VARCLASS.
If specified, this attribute overrides the ENTWIDTH value of the DTACOL tag.
PMTWIDTH=n | * | **
This attribute specifies the number of bytes used for the data field prompt-text. The minimum width
is 0 and the maximum is the remaining available panel (or region) width less the required amount of
space for field attributes. When you specify PMTWIDTH=*, the conversion utility uses the length of
the prompt text as the prompt width. When you specify PMTWIDTH=**, the conversion utility uses
the maximum available space as the prompt width. If PMTFMT=CUA is specified (or defaulted) and
the prompt-text has fewer characters than the field allows, leader dots fill the remaining spaces. If
any prompt contains more characters than the width you specify, the prompt is word-wrapped to fit
on multiple lines. If PMTWIDTH is not specified and prompt-text is present, the PMTWIDTH value
defaults to the length of the prompt-text.
If specified, this attribute overrides the PMTWIDTH value of the DTACOL tag.
DESWIDTH=n | *
This attribute specifies the number of bytes used for the description text of enclosed DTAFLDD tags.
The minimum width is 0. When you specify DESWIDTH=*, the conversion utility uses the length of the
description text as the description width. If the text is longer than the width you specify, the text is
word-wrapped to fit on multiple lines.
If specified, this attribute overrides the DESWIDTH value of the DTACOL tag.
FLDSPACE=n
This attribute specifies the number of bytes reserved for the data fields coded within the data column.
The minimum width is 2 and the maximum is the remaining available panel (or region) width. The
FLDSPACE value should include the actual entry width plus the number of entry field attributes. If the
value specified by ENTWIDTH is less than the specified FLDSPACE value, the entry field is padded
with blanks to the FLDSPACE value. This creates blank space between the entry field and description
DTAFLD
278  z/OS: z/OS ISPF DTL Guide

## Page 311

text provided by the DTAFLDD tag and allows you to align description text from successive DTAFLD
definitions.
If specified, this attribute overrides the FLDSPACE value of the DTACOL tag.
ALIGN=START | CENTER | END
This attribute specifies the alignment of data within the display field after all translations have been
performed. Use this attribute to align the data with the start, the end, or the center of the display field.
This is accomplished in the conversion utility by using an attribute character for the field that specifies
JUST(LEFT) if ALIGN=START or JUST(RIGHT) if ALIGN=END. ALIGN=CENTER uses an attribute
character for the field that specifies JUST(ASIS).
Alignment occurs if the transformed value of the dialog variable is shorter than the display width of
the field. When ALIGN=END, no underscore is padding performed. Instead, blanks are used.
PMTLOC=BEFORE | ABOVE
This attribute specifies whether the prompt-text of the data field appears above or in front of the data
field.
DISPLAY=YES | NO
This attribute specifies whether data displays on the screen as the user types it in. If you specify
NO, the data is not displayed. This attribute is useful when creating fields for passwords or other
information which you do not want to appear on the screen.
NOENDATTR
This attribute, which is valid only when WINDOW=NO is specified on the PANEL tag or DIR=HORIZ
is specified on the REGION tag, specifies that no ending attribute is placed after the data field.
NOENDATTR is ignored for the last field on each panel line unless WINDOW=NO has been specified
on the PANEL tag. NOENDATTR is valid only when the DTAFLD tag is followed by a DTAFLD, DTAFLDD,
DIVIDER, or SELFLD tag.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
If specified, this attribute overrides the PAD value of the DTACOL tag.
PADC= NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
If specified, this attribute overrides the PADC value of the DTACOL tag.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
If specified, this attribute overrides the OUTLINE value of the DTACOL tag.
PMTFMT=CUA | ISPF | NONE | END
This attribute controls the generation of prompt leader characters. The default is to create CUA leader
dots. When ISPF is specified, and at least 4 bytes of prompt text space remain following the prompt
text, the “===>” character string is placed in the rightmost 4 positions of the prompt text space. When
NONE is specified, no leader characters are added to the prompt text. When END is specified, the
prompt text is right justified within the prompt text space.
If specified, this attribute overrides the PMTFMT value of the DTACOL tag.
PSVAR=point-and-shoot-variable | %varname
This attribute provides the name of a variable that is to be set when a DTAFLD is clicked on for
point-and-shoot selection. You can define this attribute as a variable name preceded by a “%”.
The point-and-shoot-variable must follow the standard naming convention described in “Rules for
variable names” on page 179.
DTAFLD
Chapter 12. Tag reference  279

## Page 312

PSVAL=point-and-shoot-value | %varname
This attribute provides the value to be placed in the field specified by the PSVAR attribute. You can
define this attribute as a variable name preceded by a “%”. To specify a blank value, the "' '"
(quotation mark, apostrophe, blank, apostrophe, quotation mark) coding notation should be used.
PAS=%varname
This attribute can be used to provide a variable name to specify ON or OFF for point-and-shoot.
When PSVAR and PSVAL have been specified without the PAS attribute, the point-and-shoot field is
automatically enabled.
CSRGRP=NO | YES | N
When CSRGRP=YES, the conversion utility generates a cursor group number to be used for this data
field. When CSRGRP=n, the number provided is used for this field. The PAS attribute must be specified
as %varname.
The CSRGRP attribute is accepted for all data fields. It is used at run time for output fields only.
EXPAND
The EXPAND attribute, used in combination with EXPAND=xy on the PANEL definition, causes the
expand characters to be added to the DTAFLD definition, effectively allowing ENTWIDTH to expand.
The ENTWIDTH value must be specified as 4 or greater for the EXPAND function to operate.
Note: If the PANEL tag attribute EXPAND is defined with no value specified, the DTAFLD tag EXPAND
attribute is not used.
FLDWIDTH=n
The FLDWIDTH attribute, used in combination with WINDOW=NO on the PANEL definition, provides
the width of a DTAFLD which spans multiple lines.
FLDWIDTH cannot be used within any horizontal region.
ATTRCHANGE=NO | YES | NEW
When ATTRCHANGE=YES or ATTRCHANGE=NEW, the conversion utility formats an additional entry
in the panel )ATTR section (that can apply to multiple data fields) instead of creating a unique
".ATTR(field-name)" entry in the )INIT section for this field. With this option, multiple DTAFLD tags
with the same characteristics require fewer panel logic statements. ATTRCHANGE=NEW creates a
new entry. ATTRCHANGE=YES uses an existing entry, if possible.
INIT=initial-value
When the INIT attribute is specified, the conversion utility adds a statement to the panel )INIT section
to initialize the field to the initial-value.
DEPTH=n | %varname
This attribute defines the depth reserved for the field. The minimum value is 1 and the maximum
value is the remaining panel depth. This attribute is accepted in order to support existing DTL source
files that use it. However, although the space is reserved, point-and-shoot does not function in the
additional reserved space..
IMAPNAME=image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAMEP=image-namep | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
PLACE=ABOVE | BELOW | LEFT | RIGHT
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
DBALIGN=YES | NO | PROMPT | FIELD | FORCE
This attribute defines the DBALIGN value. DBALIGN is used only for DBCS language conversions when
PMTLOC=ABOVE and the DBALIGN invocation option is specified.
When DBALIGN=PROMPT, the start position of the prompt-text is shifted 1 position to the right.
When DBALIGN=FIELD, the start position of the data field is shifted 1 position to the right.
DTAFLD
280  z/OS: z/OS ISPF DTL Guide

## Page 313

When DBALIGN=YES, and the prompt-text starts with a DBCS character, the data field is shifted. If
DBALIGN=YES and the prompt text starts with an SBCS character or the prompt text is not provided,
no shifting is done.
When DBALIGN=FORCE, both the prompt-text and the data field are shifted. DBALIGN=YES and
DBALIGN=FORCE are useful to align a DTAFLD with another DTAFLD or SELFLD tag.
When DBALIGN=NO, no alignment adjustment is made.
PMTSKIP=NO | YES
This attribute is used for horizontal formatting of input fields. When PMTSKIP=YES, and the previous
DTAFLD definition includes the NOENDATTR attribute, the cursor moves past the prompt text to the
input field when the user enters the last character in the previous field. If there is no other input field
on the panel, the cursor returns to the first input field on the panel.
DESSKIP=NO | YES
This attribute is used for horizontal formatting of input fields. When DESSKIP=YES, and the current
DTAFLD definition includes the NOENDATTR attribute, the cursor skips over the description text
provided by the DTAFLDD tag to the next input field when the user enters the last character in the
current field. If there is no other input field on the panel, the cursor returns to the first input field on
the panel.
FLDTYPE=CUA | ISPF
This attribute defines the attribute type to be applied to the field. TYPE=CUA, the default, causes
the field to display using the standard CUA attribute. When FLDTYPE=ISPF, a non-CUA attribute entry
is generated for the )ATTR section, and you can specify the color, intensity, and highlighting of the
attribute. See the COLOR, INTENS, and HILITE attributes that follow for more information. These
attributes are not valid when FLDTYPE=CUA.
Note: IF DISPLAY=NO is specified, an .ATTR(...) is created to override this field.
COLOR=WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute specifies the color of the field. You can define this attribute as a variable name preceded
by a percent (%) sign.
INTENS=HIGH | LOW | NON | %varname
This attribute defines the intensity of the field. You can define this attribute as a variable name
preceded by a percent (%) sign.
HILITE=USCORE | BLINK | REVERSE | %varname
This attribute specifies the extended highlighting attribute of the field. You can define this attribute as
a variable name preceded by a percent (%) sign.
ATTRCHAR=code
This attribute can be a single character or a two-position entry of valid hex digits. If you enter an
incorrect value, a warning message is issued and the value is set to null. Hex entries are converted to
character. Hex values ‘00’-‘2F’ are reserved for use by the conversion utility.
CAPS=OFF | ON
When CAPS=ON, the data in the field is displayed in uppercase characters.
NOJUMP=OFF | ON
When NOJUMP=ON, the JUMP function is disabled for the field.
AUTOTYPE=PROJECT | GROUP1 | GROUP2 | GROUP3 | GROUP4 | TYPE | MEMBER | DSN
This attribute specifies that ISPF panel logic be added to support the AUTOTYPE function.
AUTOTYPE=DSN is specified for data set name fields.
The other attribute values are used for ISPF- format project, group, type, and member name fields.
Multiple data fields can be specified with AUTOTYPE=DSN. Only one field can be specified with each
of the other listed attribute values.
AUTOVOL = volser name
This attribute specifies an associated panel field for volume name when AUTOTYPE=DSN.
DTAFLD
Chapter 12. Tag reference  281

## Page 314

AUTODMEM = YES | NO
This attribute specifies whether a member name is part of the data set name when AUTOTYPE=DSN.
VARDCL = YES | NO
When VARDCL=NO the field name is not checked to the declared variable information provided with
the VARCLASS and VARDCL tags.
prompt-text
This is the prompt text for the data field. The prompt-text appears in front of or above the field,
depending on the setting of the PMTLOC attribute. If you do not specify prompt text, no text appears
for the field.
If the prompt-text exceeds the width defined for a prompt, it is word-wrapped to multiple lines.
Comments
The DTAFLD tag defines an input field, an output field, or an input/output field on an application panel.
The formatted width of the field is 2 positions more than the ENTWIDTH value to provide for an attribute
byte both before and after the field.
If PMTLOC=ABOVE, an attribute is placed both before and after the prompt text reserved space. If
PMTLOC=BEFORE (or PMTLOC is not specified), and the DTAFLD is being formatted in a horizontal region,
then an additional byte is used for the field prompt attribute when the field prompt is not at the left edge
of the panel.
The DTAFLDD tag can be used to provide the description text for the data field.
When the maximum number of requested attributes for a panel is exceeded, the conversion utility issues
error message ISPC804E. The number of requested attributes includes attribute override entries. These
are .ATTR entries that are added by the conversion utility for attributes that are specified on CHOFLD,
DTACOL, DTAFLD, LSTCOL, and LSTFLD tags. If the same set of attributes is specified on multiple tags,
duplicate .ATTR entries are added by default. Adding the parameter ATTRCHANGE=YES to the tags
causes the compiler to instead add a single entry in the panel )ATTR section for each unique set of
attributes specified. The entry for a set of attributes is then shared by all tags that specify that set of
attributes.
Restrictions
• You must code the DTAFLD tag within an AREA, DTACOL, PANEL, or REGION definition. See “AREA
(Area)” on page 189, “DTACOL (Data Column)” on page 269, “PANEL (Panel)” on page 376, and
“REGION (Region)” on page 405 for descriptions of these tags.
• The variable name specified in the DATAVAR attribute should have an associated VARDCL definition. See
“VARDCL (Variable Declaration)” on page 449 for a complete description of this tag.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
Table 30. The tags you can code within a DTAFLD definition 
Tag Reference Usage Required
ASIGNL “ASSIGNL (Assignment List)” on page 196 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
DTAFLDD “DTAFLDD (Data Field Description)” on page 284 Multiple No
DTAFLD
282  z/OS: z/OS ISPF DTL Guide

## Page 315

Table 30. The tags you can code within a DTAFLD definition  (continued)
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SCRFLD “SCRFLD (Scrollable Field)” on page 413 Single No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is source file markup that contains an application panel with three data fields and the variable
declarations and classes associated with the data fields. The Date field is an output-only field that
displays the current date. The Name and Password fields are input/output fields. The Password field is
defined as a required field, and specifies DISPLAY=NO, so the user input for this field is not displayed. A
data column specifying a default prompt width for the data fields is also defined. Figure 110 on page 284
shows the formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=date TYPE='char 8'>
<VARCLASS NAME=name TYPE='char 25'>
<VARCLASS NAME=password TYPE='char 8'>
<VARLIST>
  <VARDCL NAME=curdate VARCLASS=date>
  <VARDCL NAME=namevar VARCLASS=name>
  <VARDCL NAME=passvar VARCLASS=password>
</VARLIST>
<PANEL NAME=dtafld1 HELP=loghelp>System Logon
<TOPINST>Complete the following fields, then press Enter.
<AREA>
  <DTACOL PMTWIDTH=12>
    <DIVIDER>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8 FLDSPACE=27>Date
  <DTAFLDD>(Current Date)
    <DIVIDER>
    <DTAFLD DATAVAR=namevar ENTWIDTH=25 DESWIDTH=25>Name
      <DTAFLDD>(Last, First)
    <DIVIDER>
    <DTAFLD DATAVAR=passvar REQUIRED=yes ENTWIDTH=8 DISPLAY=no>Password
  </DTACOL>
</AREA>
</PANEL>
DTAFLD
Chapter 12. Tag reference  283

## Page 316

System Logon
 Complete the following fields, then press Enter.
 Date . . . : 08/29/90                   (Current Date)
 Name . . . . _________________________  (Last, First)
 Password . .
  F1=Help    F3=Exit   F12=Cancel
Figure 110. Data fields 
DTAFLDD (Data Field Description)
The DTAFLDD tag defines descriptive text associated with a data field.
Syntax
<DTAFLDD>
description </DTAFLDD>
Parameters
description
This is the descriptive text associated with the data field.
Comments
The DTAFLDD tag defines descriptive text associated with a data field. For example, it could explain what
the application user can type into the field.
The text appears in the area defined by the DESWIDTH attribute of the DTAFLD or DTACOL tag.
You can specify more than one DTAFLDD tag for a given field. Each data field description starts a new line.
Restrictions
• You must code the DTAFLDD tag within the DTAFLD definition it is associated with. See “DTAFLD (Data
Field)” on page 275 for a complete description of this tag.
DTAFLDD
284  z/OS: z/OS ISPF DTL Guide

## Page 317

Processing
Table 31. The tags you can code within a DTAFLDD definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is application panel markup that contains two data fields that each have associated data field
descriptions. Figure 111 on page 285 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=filecls TYPE='char 8'>
<VARCLASS NAME=copycls TYPE='char 2'>
<VARLIST>
  <VARDCL NAME=file    VARCLASS=filecls>
  <VARDCL NAME=copynum VARCLASS=copycls>
</VARLIST>
<PANEL NAME=dtafldd>Print a File
<TOPINST>Type in the name of the file you want to print
and the number of copies, then press Enter.
<AREA>
  <DTAFLD DATAVAR=file PMTWIDTH=12 ENTWIDTH=8 DESWIDTH=30>Filename
    <DTAFLDD>(Maximum of 8 characters)
  <DTAFLD DATAVAR=copynum PMTWIDTH=12 ENTWIDTH=2 DESWIDTH=8>Copies
    <DTAFLDD>(1 - 99)
</AREA>
<CMDAREA>Enter a command
</PANEL>
                                Print a File
 Type in the name of the file you want to print and the number of copies,
 then press Enter.
 Filename . . ________  (Maximum of 8 characters)
 Copies . . . __  (1 - 99)
 Enter a command ===> ______________________________________________________
  F1=Help    F3=Exit   F12=Cancel
Figure 111. Data field  descriptions
DTAFLDD
Chapter 12. Tag reference  285

## Page 318

DTDIV (Definition Term Divider)
The DTDIV tag defines a visible vertical divider (|) between multiple DT tags.
Syntax
<DTDIV>
</DTDIV>
Comments
The DTDIV tag can be used to create a visual separation between the definition terms. Each DTDIV tag
adds a vertical bar (plus display control attributes) to the Definition Term text.
Restrictions
The DTDIV tag can be coded before the first DT tag, between DT tags, or following the last DT tag (before
the DD tag definition).
Processing
None.
Examples
Here is an example that shows the use of the DTDIV tag in combination with the multiple DT tag function
and the DIVEND attribute of the DL tag. Figure 112 on page 287 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=dtdiv DEPTH=22 WIDTH=66>Employee Code Help
  <AREA depth=1 extend=on>
  <INFO width=*>
    <P>The following list defines the valid employee codes.
    <DL TSIZE='14 4' BREAK=none COMPACT DIVEND=yes>
      <DLDIV TYPE=solid>
      <DTHDIV>
      <DTHD>Code
      <DTHDIV>
      <DTHD>Flag
      <DTHDIV>
      <DDHD>Meaning
      <DLDIV TYPE=solid>
      <DTDIV>
      <DT NOSKIP>Full-time
      <DTDIV>
      <DT FORMAT=center>F
      <DTDIV>
      <DD>Indicates that the employee works a
      regular schedule of 40 hours or more weekly.
      <DLDIV TYPE=solid>
      <DTDIV>
      <DT>Part-time
      <DTDIV>
      <DT FORMAT=center>P
      <DTDIV>
      <DD>Indicates that the employee works a regular
      schedule of 20 to 40 hours weekly.
      <DLDIV TYPE=solid>
      <DTDIV>
      <DT>Supplemental
      <DTDIV>
      <DT FORMAT=center>S
      <DTDIV>
      <DD>Indicates that the employee works less than
      20 hours weekly.
DTDIV (Definition Term Divider)
286  z/OS: z/OS ISPF DTL Guide

## Page 319

No regular schedule is in place.
      <DLDIV TYPE=solid>
    </DL>
  </INFO>
  </AREA>
</HELP>
                        Employee Code Help
 The following list defines the valid employee codes.
 ----------------------------------------------------------------
 | Code           | Flag | Meaning                              |
 ----------------------------------------------------------------
 | Full-time      |  F   | Indicates that the employee works a  |
 |                |      | regular schedule of 40 hours or more |
 |                |      | weekly.                              |
 ----------------------------------------------------------------
 | Part-time      |  P   | Indicates that the employee works a  |
 |                |      | regular schedule of 20 to 40 hours   |
 |                |      | weekly.                              |
 ----------------------------------------------------------------
 | Supplemental   |  S   | Indicates that the employee works    |
 |                |      | less than 20 hours weekly. No        |
 |                |      | regular schedule is in place.        |
 ----------------------------------------------------------------
Figure 112. Definition  term divider
DTHD (Definition Term Header)
The DTHD tag defines the heading for the term column of a definition list.
Syntax
<DTHD> definition-term-header
</DTHD>
Parameters
definition-term-header
This is the text of the definition term header. The length of the text for the definition term header
should be less than the specified TSIZE value in the DL tag. A warning message is issued if the length
of the text exceeds the limit.
Comments
The DTHD tag defines the heading for the term column of a definition list. You can code multiple DTHD
tags within a definition list.
The conversion utility inserts a blank line between the header and the list items unless the COMPACT
attribute is specified on the DL tag.
Restrictions
• You must code the DTHD tag within a DL definition. See “DL (Definition List)” on page 261 for a
complete description of this tag.
• Each DTHD tag must be paired with and precede a definition description header (DDHD) tag.
DTHD
Chapter 12. Tag reference  287

## Page 320

Processing
Table 32. The tags you can code within a DTHD definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is help panel markup that contains a definition term header with the text “Prefix”. Figure 113 on
page 288 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=dthd DEPTH=18>Prefix Help
<AREA>
<INFO>
  <P>The following list defines each of the valid prefixes.
  <DL TSIZE=12>
    <DTHD>Prefix
    <DDHD>Meaning
    <DT>AU
    <DD>Automotive
    <DT>HB
    <DD>Health and beauty
    <DT>LG
    <DD>Lawn and garden
    <DT>SG
    <DD>Sporting goods
  </DL>
</INFO>
</AREA>
</HELP>
                   Prefix Help
 The following list defines each of the valid
 prefixes.
 Prefix      Meaning
 AU          Automotive
 HB          Health and beauty
 LG          Lawn and garden
 SG          Sporting goods
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 113. Definition  term header
DTHDIV (Definition Term Header Divider)
The DTHDIV tag defines a visible vertical divider (|) between multiple DTHD tags.
DTHDIV (Definition Term Header Divider)
288  z/OS: z/OS ISPF DTL Guide

## Page 321

Syntax
<DTHDIV>
</DTHDIV>
Comments
The DTHDIV tag can be used to create a visual separation between the definition term headings. Each
DTHDIV tag adds a vertical bar (plus display control attributes) to the Definition Term Header text.
Restrictions
The DTHDIV tag can be coded before the first DTHD tag, between DTHD tags, or following the last DTHD
tag (before the DDHD tag definition).
Processing
None.
Examples
Here is an example that shows the use of the DTHDIV tag in combination with the multiple DT tag function
and the DIVEND attribute of the DL tag. Figure 114 on page 290 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=dthdiv DEPTH=22 WIDTH=66>Employee Code Help
  <AREA depth=1 extend=on>
  <INFO width=*>
    <P>The following list defines the valid employee codes.
    <DL TSIZE='14 4' BREAK=none COMPACT DIVEND=yes>
      <DLDIV TYPE=solid>
      <DTHDIV>
      <DTHD>Code
      <DTHDIV>
      <DTHD>Flag
      <DTHDIV>
      <DDHD>Meaning
      <DLDIV TYPE=solid>
      <DTDIV>
      <DT NOSKIP>Full-time
      <DTDIV>
      <DT FORMAT=center>F
      <DTDIV>
      <DD>Indicates that the employee works a
      regular schedule of 40 hours or more weekly.
      <DLDIV TYPE=solid>
      <DTDIV>
      <DT>Part-time
      <DTDIV>
      <DT FORMAT=center>P
      <DTDIV>
      <DD>Indicates that the employee works a regular
      schedule of 20 to 40 hours weekly.
      <DLDIV TYPE=solid>
      <DTDIV>
      <DT>Supplemental
      <DTDIV>
      <DT FORMAT=center>S
      <DTDIV>
      <DD>Indicates that the employee works less than
      20 hours weekly.
      No regular schedule is in place.
      <DLDIV TYPE=solid>
    </DL>
  </INFO>
  </AREA>
</HELP>
DTHDIV (Definition Term Header Divider)
Chapter 12. Tag reference  289

## Page 322

Employee Code Help
 The following list defines the valid employee codes.
 ----------------------------------------------------------------
 | Code           | Flag | Meaning                              |
 ----------------------------------------------------------------
 | Full-time      |  F   | Indicates that the employee works a  |
 |                |      | regular schedule of 40 hours or more |
 |                |      | weekly.                              |
 ----------------------------------------------------------------
 | Part-time      |  P   | Indicates that the employee works a  |
 |                |      | regular schedule of 20 to 40 hours   |
 |                |      | weekly.                              |
 ----------------------------------------------------------------
 | Supplemental   |  S   | Indicates that the employee works    |
 |                |      | less than 20 hours weekly. No        |
 |                |      | regular schedule is in place.        |
 ----------------------------------------------------------------
Figure 114. Definition  term header divider
DTSEG (Definition Term Segment)
The DTSEG tag defines a segment of the definition term. It is used to provide vertical separation of the DT
tag text.
Syntax
<DTSEG>
</DTSEG>
Comments
The DTSEG tag is used to create a vertical separation within the definition term. The text following the
DTSEG tag is formatted directly under any previous definition term tag text. Multiple DTSEG tags create
additional DT text lines.
Use of the DTSEG tag affects the DL tag BREAK attribute. The first (or only) line of DT tag text is processed
according to the BREAK attribute of the DL tag. For additional lines, when TSIZE is large enough to
accommodate the text segments, the DTSEG text is formatted in front of the associated DD tag text. When
TSIZE is not large enough to accommodate the largest segment, all of the DT and DTSEG text is formatted
above the associated DD tag text.
Restrictions
• The DTSEG tag can be coded within the text following a DT tag.
• When a DTSEG tag is coded, then all remaining DT tag text for the current DT tag set must follow a
DTSEG tag.
• The DT nested tags RP and PS are not supported within DT tag text following any DTSEG tag in a DT/DD
tag set.
Processing
Table 33. The tag you can code within a DTSEG definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
DTSEG (Definition Term Segment)
290  z/OS: z/OS ISPF DTL Guide

## Page 323

Examples
Here is an example that shows the use of the DTSEG tag in combination with a multiple DT tag set. The
last DT tag includes the SPLIT=yes attribute to format the dash in front of the DD tag text. Figure 115 on
page 291 shows the formatted result.
<!DOCTYPE DM SYSTEM()>
<PANEL NAME=dtseg KEYLIST=ISRHELP APPLID=ISR WINDOW=no PADC=user
       TUTOR ZUP=ISP7R000>Traces - Primary Commands
<CMDAREA CAPS=on>
<AREA DEPTH=1 EXTEND=on>
  <INFO WIDTH=*>
    <P>
       Enter a <hp>Primary Command</hp> in the command input field.
       It is processed after all row modifications and all line commands
       are processed. The following primary commands are valid for the
       Traces options:
    <DL TSIZE="8 1" BREAK=fit INDENT=2>
      <DT>
          LOCATE
          function-name
          (Function Traces) or variable name (Variable Traces)
        <DTSEG>
          LOC or
        <DTSEG>
          L
      <DT SPLIT=yes>-
      <DD>The LOCATE command positions the scrollable display at the
          first (or next) row containing the function name (Function
          Traces option) or the variable name (Variable Traces option).
    </DL>
  </INFO>
</AREA>
</PANEL>
 Tutorial  ---------------  Traces - Primary Commands  --------------  Tutorial
 Command ===> _________________________________________________________________
 Enter a Primary Command in the command input field. It is processed after all
 row modifications and all line commands are processed. The following primary
 commands are valid for the Traces options:
   LOCATE function-name (Function Traces) or variable-name (Variable Traces)
   LOC or   - The LOCATE command positions the scrollable display at the first
   L          (or next) row containing the function name (Function Traces
              option) or the variable name (Variable Traces option).
Figure 115. Definition  term segment
FIG (Figure)
The FIG tag defines the format of text so that it is set off from other text on the panel and retains the
format of the enclosed text.
FIG
Chapter 12. Tag reference  291

## Page 324

Syntax
<FIG
FRAME=
RULE
NONE WIDTH=
PAGE
COL
NOSKIP
>
figure-content
</FIG>
Parameters
FRAME=RULE | NONE
This attribute specifies the type of frame to put around the figure.
RULE
Specifies dashed lines appears above and below the figure.
NONE
Specifies no frame; a blank line is left above and below the figure.
WIDTH=PAGE | COL
This attribute specifies where the figure should be aligned. The value PAGE (the default) formats the
figure on the original left margin. The value COL formats the figure on the current left margin. The
current left margin may be different than the original left margin of the panel if the FIG tag is nested
within another tag that causes indenting; the UL tag, for example.
NOSKIP
This attribute causes the blank line normally placed before the figure to be skipped.
figure-content
This is the text of the figure definition.
Comments
The FIG tag defines the format of text so that it is set off from other text on the panel and retains the
format of the enclosed text. Tags that normally cause word wrapping within an information region (such
as P, NOTE, or PARML) do not cause word-wrapping when nested within a FIG definition. In addition,
blank spaces and blank lines in the source are preserved in the figure.
If any DTL source text line is too long to fit in the remaining available formatting width, the data is
truncated. A warning message is issued when the first line within the figure is truncated.
A figure can also contain a figure caption, defined with the FIGCAP tag (see “FIGCAP (Figure Caption)” on
page 294).
Restrictions
• The FIG tag requires an end tag.
• You must code the FIG tag within an INFO definition. See “INFO (Information Region)” on page 317 for
a complete description of this tag.
Processing
Table 34. The tags you can code within a FIG definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
FIG
292  z/OS: z/OS ISPF DTL Guide

## Page 325

Table 34. The tags you can code within a FIG definition  (continued)
Tag Reference Usage Required
FIGCAP “FIGCAP (Figure Caption)” on page 294 Single No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains a figure definition with a ruled frame. The output of the text
within the figure definition is identical to the fig ur e - c ont ent . Figure 116 on page 294 shows the formatted
result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=fig DEPTH=20>ShelfBrowse Help
<AREA>
<INFO>
  <FIG>
    We're your local library...
           CHECK US OUT!
  </FIG>
</INFO>
</AREA>
</HELP>
FIG
Chapter 12. Tag reference  293

## Page 326

ShelfBrowse Help
 ------------------------------------------------
     We're your local library...
            CHECK US OUT!
 ------------------------------------------------
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 116. Figure
FIGCAP (Figure Caption)
The FIGCAP tag defines a caption for a figure defined with the FIG tag.
Syntax
<FIGCAP>
figure-caption-text </FIGCAP>
Parameters
figure-caption-text
This is the text of the figure caption.
Comments
The FIGCAP tag defines a caption for a figure defined with the FIG tag. The figure caption is formatted
below the frame of the figure when FRAME=RULE is specified on the FIG tag.
The conversion utility does not add any blank lines before or after the figure caption.
Restrictions
• You must code the FIGCAP tag within a FIG definition. See “FIG (Figure)” on page 291 for a complete
description of this tag.
• You can code only one FIGCAP within a FIG definition. Code the FIGCAP tag after the content of the
figure, before the FIG end tag.
Processing
Table 35. The tags you can code within a FIGCAP definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
FIGCAP
294  z/OS: z/OS ISPF DTL Guide

## Page 327

Table 35. The tags you can code within a FIGCAP definition  (continued)
Tag Reference Usage Required
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is help panel markup that contains a figure definition with an enclosed figure caption. Figure 117 on
page 295 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=figcap DEPTH=20>ShelfBrowse Help
<AREA>
<INFO>
  <FIG>
    We're your local library...
           CHECK US OUT!
  <FIGCAP>Our Motto
  </FIG>
</INFO>
</AREA>
</HELP>
                 ShelfBrowse Help
 ------------------------------------------------
     We're your local library...
            CHECK US OUT!
 ------------------------------------------------
 Our Motto
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 117. Figure caption
GA (Graphic Area)
The GA tag allows the creation of graphic areas on ISPF panels.
GA
Chapter 12. Tag reference  295

## Page 328

Syntax
<GA NAME=graphic-area-name
EXTEND=
OFF
ON
FORCE
DEPTH= n
*
WIDTH=n
DIV=
NONE
BLANK
SOLID
DASH
TEXT
DIV options
LVLINE=variable-name
>
</GA>
DIV options
FORMAT= START
CENTER
END
TEXT=divider-text
Parameters
NAME=graphic-area-name
This attribute defines the name of the graphic area. This name is the dialog variable specified by the
application that provides the data for the graphic area.
The NAME attribute must follow the standard naming convention described in “Rules for variable
names” on page 179.
EXTEND=OFF | ON | FORCE
This attribute defines the runtime display size of the graphic area. If EXTEND=ON is specified, the
graphic area definition is expanded to the size of the logical screen. If you intend to display the panel
in a pop-up window, use EXTEND=OFF (which is the default).
If EXTEND=FORCE is specified within a horizontal area or region, the EXTEND(ON) keyword is added
to the graphic area attribute statement in the )ATTR panel section. The conversion utility issues a
message to advise of a potential display error if other panel fields are formatted on or after the last
defined line of the graphic area.
DEPTH=n | *
This attribute specifies the number of lines reserved for the graphic area definition. The DEPTH
attribute value reserves space within the panel )BODY section. The minimum depth is one line. will
reserve the remaining available panel depth for the graphic area.
WIDTH=n
This attribute specifies the number of columns reserved for the graphic area definition. The minimum
width is the number of positions in the graphic area name plus 4 and the maximum is 2 positions less
than the panel width. The conversion utility places attribute bytes on both sides of the graphic area.
GA
296  z/OS: z/OS ISPF DTL Guide

## Page 329

DIV=NONE | BLANK | SOLID | DASH | TEXT
This attribute specifies the type of divider line to be placed before and after the graphic area. If
this attribute is not specified or has the value NONE, no divider line is generated. The value BLANK
produces a blank line. You must specify SOLID, DASH, or TEXT to produce a visible divider line. When
the GRAPHIC invocation option is specified, SOLID produces a solid line for host display and DASH
produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH produce a dashed line.
A visible divider line formats with a non-displayable attribute byte on each end of the line.
FORMAT=START | CENTER | END
This attribute specifies the position of the divider-text within the divider line. You must specify
both the FORMAT attribute and the TEXT attribute to create a divider line containing text.
TEXT=divider-text
This attribute specifies the text to be placed on the divider line. You must specify both the
FORMAT attribute and the TEXT attribute to create a divider line containing text.
LVLINE=variable-name
This attribute allows you to specify the name of a variable which contains the result of the ISPF
function LVLINE.
The LVLINE attribute must follow the standard naming convention described in “Rules for variable
names” on page 179.
Comments
The GA tag defines a graphic area in the panel )BODY section.
If you specify the CMDAREA tag within your DTL source file, it must appear before the GA tag when
DEPTH=* is specified. The GA tag DEPTH may have to be adjusted to allow for additional lines which
result from tags present within the panel definition following the end GA tag.
See z/OS ISPF Dialog Developer's Guide and Reference for a discussion of the graphic area in ISPF panels.
Restrictions
• You must code the GA tag within a PANEL, AREA, or REGION tag. If found anywhere else, an error is
logged and the output panel is not saved.
• If NAME is not valid or not specified, an error is logged and the output panel is not saved.
• You can use the EXTEND=ON attribute only once within a panel. If EXTEND is already active, from
another GA tag, or from an AREA, DA, SELFLD, or REGION tag, a warning message is logged and the
EXTEND attribute is ignored.
• You can code only one GA tag within a PANEL definition.
• You cannot code the GA tag within a scrollable area.
Processing
None.
Examples
GA
Chapter 12. Tag reference  297

## Page 330

<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=ga KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST> Type in patron's name and card number (if applicable)
<AREA>
  <DTACOL PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25 SELWIDTH=25>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8>Date
    <DTAFLD DATAVAR=cardno ENTWIDTH=7>Card No.
      <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name>Name
      <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address>Address
  </DTACOL>
  <DIVIDER>
  <GA NAME=garea DIV=solid DEPTH=6 WIDTH=40>
  </GA>
</AREA>
<CMDAREA>Enter a command
</PANEL>
GENERATE (Generate)
The GENERATE tag provides direct formatting for )BODY and )AREA panel sections.
Syntax
<GENERATE>
SUBSTITUTE=
NO
YES
</GENERATE>
Parameters
SUBSTITUTE=NO | YES
The SUBSTITUTE attribute specifies whether variable substitution is attempted within the pre-
formatted panel text.
Comments
The GENERATE tag is used to add pre-formatted displayable panel contents into the )BODY or )AREA
panel sections. These contents can contain any valid displayable information. It is the panel developer's
responsibility to provide valid displayable data.
The pre-formatted information is coded within a nested SOURCE tag. The SOURCE tag TYPE attribute is
automatically determined based on the position of the GENERATE tag within the DTL source file. When
panel attributes are required, the ATTR tag can be used to define the necessary )ATTR section entries.
Restrictions
• The GENERATE tag requires an end tag.
• You must code the GENERATE tag within an AREA, HELP or PANEL tag definition.
GENERATE
298  z/OS: z/OS ISPF DTL Guide

## Page 331

Processing
Table 36. The tags you can code within a GENERATE definition 
Tag Reference Usage Required
ATTR “ATTR (Attribute)” on page 200 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is markup that shows contains a GENERATE tag with nested ATTR and SOURCE tags. Figure 118 on
page 300 shows the generated panel file.
<!DOCTYPE DM SYSTEM>
<PANEL NAME=* KEYLIST=keylxmp applid=isr window=no>
            Generate Tag Example
<CMDAREA>
<pnlinst compact>
         Sample panel source to illustrate the GENERATE tag.
<divider type=solid gap=no>
<generate>
    <attr attrchar=! type=FP>
    <attr attrchar=_ type=NEF>
    <attr attrchar=+ type=NT>
<source>
!  Project ===>_PROJECT !
!  Group   ===>_GROUP1  !===>_GROUP2  !===>_GROUP3  !===>_GROUP4  +
!  Type    ===>_TYPE    !
!  Member  ===>_MEMBER  !
!  DS Name ===>_OTHERDSN                                           +
!  Volume  ===>_VOLUME+
   </source>
</generate>
</panel>
GENERATE
Chapter 12. Tag reference  299

## Page 332

)PANEL KEYLIST(KEYLXMP,ISR)
)ATTR DEFAULT(""") FORMAT(MIX)
 05 TYPE(PT)
 06 TYPE(PIN)
 09 TYPE(FP)
 0A TYPE(NT)
 13 TYPE(NEF)
 22 TYPE(WASL) SKIP(ON) GE(ON)
 !  TYPE(FP)
 _  TYPE(NEF)
 +  TYPE(NT)
)BODY  CMD(ZCMD)
                              Generate Tag Example
 Command ===> Z
 Sample panel source to illustrate the GENERATE tag.
 ------------------------------------------------------------------
!  Project ===>_PROJECT !
!  Group   ===>_GROUP1  !===>_GROUP2  !===>_GROUP3  !===>_GROUP4  +
!  Type    ===>_TYPE    !
!  Member  ===>_MEMBER  !
!  DS Name ===>_OTHERDSN                                          +
!  Volume  ===>_VOLUME+
)INIT
.ZVARS = '(ZCMD)'
&ZCMD = ' '
)PROC
)END
Figure 118. Generated panel
GRPHDR (Group Header)
The GRPHDR tag allows the creation of group headers on ISPF panels.
Syntax
<GRPHDR
FORMAT=
START
CENTER
END
NONE
WIDTH=n FMTWIDTH=n
INDENT=n
HEADLINE=
NO
YES DIV=
NONE
BLANK
SOLID
DASH
DIVLOC=
AFTER
BEFORE
BOTH
COMPACT STRIP
>
group-heading-text </GRPHDR>
GRPHDR
300  z/OS: z/OS ISPF DTL Guide

## Page 333

Parameters
FORMAT=START | CENTER | END | NONE
This attribute specifies the type of group header formatting.
When FORMAT=NONE, the lines of group-heading-text are placed in the panel )BODY section without
alteration. The processing is similar to the LINES tag.
When the values START, CENTER, or END are specified, the data is processed in a manner similar
to the P tag. The group-heading-text is read and flowed to fit within the width limit specified by
FMTWIDTH. Multiple lines may be added to the panel, depending on the length of the group-heading-
text.
WIDTH=n
This attribute specifies the number of columns reserved for the group heading. The minimum width
for a group heading is 4. The maximum value is the remaining panel width. If WIDTH is not specified,
the default value is set to the remaining panel width. The conversion utility uses 2 positions from the
specified or default WIDTH for attributes.
FMTWIDTH=n
This attribute specifies the number of columns to use for formatting the group-heading-text. The
minimum formatting width is 2. The maximum value is the value specified or defaulted for WIDTH. If
FMTWIDTH is not specified, the default value is set to the value of WIDTH.
INDENT=n
This attribute specifies that the group heading is to be indented from the current position.
HEADLINE=NO | YES
This attribute specifies whether dashes are added to span the width of the group heading not
occupied by text. This allows a visual indication of the width of the group heading.
DIV=NONE | BLANK | SOLID | DASH
This attribute specifies the type of divider line to be placed before and after the group heading. If
this attribute is not specified or has the value NONE, no divider line is generated. The value BLANK
produces a blank line. You must specify SOLID or DASH to produce a visible divider line. When
the GRAPHIC invocation option is specified, SOLID produces a solid line for host display and DASH
produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH produce a dashed line.
DIVLOC=AFTER | BEFORE | BOTH
This attribute specifies whether a divider line is to be added after the group heading, before the group
heading or both before and after the group heading.
COMPACT
This attribute causes the group heading to format without a blank before the heading.
STRIP
This attribute causes leading and trailing blanks to be removed from the heading.
group-heading-text
This is the text of the group header. If no group-heading-text is provided, a blank line is added to the
panel unless the COMPACT attribute is also specified.
Comments
The GRPHDR tag defines a group heading in the panel )BODY section.
The FMTWIDTH and HEADLINE attributes are not valid in combination with FORMAT=NONE. The DIVLOC
attribute is not valid in combination with DIV=NONE.
You use the FMTWIDTH attribute to control the width of flowed text within the number of columns
specified by WIDTH. The FORMAT attribute controls the placement of the resulting lines within the
heading WIDTH. The FMTWIDTH attribute has no effect if the length of the group-heading-text is less than
the value specified.
GRPHDR
Chapter 12. Tag reference  301

## Page 334

Because the group heading is formatted as text, a blank line is placed at the beginning of each group
heading unless the COMPACT attribute has been specified. However, when the group heading is the first
item in a scrollable region the blank line is not generated.
Restrictions
• You must code the GRPHDR tag within a PANEL, AREA, DTACOL, or REGION tag. If found anywhere else,
an error is logged and the output panel is not saved.
Processing
Table 37. The tags you can code within a GRPHDR definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
<!DOCTYPE DM SYSTEM>
  <!entity sampvar1 system>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=grphdr KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST> Type in patron's name and card number (if applicable)
<AREA>
  <GRPHDR FORMAT=center WIDTH=50 FMTWIDTH=30 DIV=solid COMPACT>
        Data Field Group Heading
  </GRPHDR>
  <DTACOL PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25 SELWIDTH=25>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8>Date
    <DTAFLD DATAVAR=cardno ENTWIDTH=7>Card No.
      <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name>Name
      <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address>Address
  </DTACOL>
</AREA>
<CMDAREA>Enter a command
</PANEL>
GRPHDR
302  z/OS: z/OS ISPF DTL Guide

## Page 335

File  Search  Help
 -------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number (if applicable)
             Data Field Group Heading
 -----------------------------------------------
 Date . . . : ________
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Enter a command ===> ____________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 119. Group heading
HELP (Help Panel)
The HELP tag defines a help panel.
HELP
Chapter 12. Tag reference  303

## Page 336

Syntax
<HELP NAME=help-panel-name
HELP= hhelp-panel-name
%varname
HELPDEF=helpdef-id
WIDTH=
50
n
FIT
DEPTH=
10
n
FIT
CCSID=n TUTOR
KEYLIST=key-list-name KEYLIST options EXPAND=xy
WINTITLE=window-title APPTITLE=application-title
MERGESAREA=
NO
YES MSGLINE=
YES
NO
IMAPNAME= image-name
%varname
IMAP group ZUP=zup-id
ZCONT=zcont-id
> help-panel-title </HELP>
KEYLIST options
KEYLTYPE=
PRIVATE
SHARED
APPLID=application-id
IMAP group
IMAPROW= n
%varname
IMAPCOL = n
%varname
Parameters
NAME=help-panel-name
This attribute specifies the name of the help panel. The help-panel-name must follow the standard
naming convention described in “Rules for variable names” on page 179.
In addition, the help-panel-name is limited to 7 characters when the DTL source you are converting
causes the conversion utility to build multiple panels. If you have specified an 8-position help name
and multiple panels are required, the help name is truncated to 7 positions. If you are not creating a
scrollable help panel, this allows additional panels to be built if the help text exceeds the limits of the
original help panel. Up to 36 additional help panels are built to contain additional help text.
HELP
304  z/OS: z/OS ISPF DTL Guide

## Page 337

If the number of generated panels required exceeds 37, a warning message is issued and all help
text after the 37th panel is discarded. The additional panel names are generated from the original
help-panel-name by these rules:
• The character ‘X’ pads the help-panel-name to 8 characters in length if the original help-panel-name
is less than 8 characters.
• The eighth character of the generated panel name increments from 0-9 and A-Z depending on the
number of panels required to be generated. For example, if the original help-panel-name is ‘HELP1’
and the help text extends beyond the original panel, the second generated panel name would be
‘HELP1XX0’, and the third would be ‘HELP1XX1’.
If you specify NAME=*, the help-panel-name is set to the input DTL source member name. If multiple
dialog element definitions have been combined within a single source file, then this notation should
be used for only one dialog element definition within the file. See “Dialog elements” on page 5 for a
description of dialog element types created by the conversion utility.
The help-panel-name is used to build the help panel output file name in which the conversion utility
stores the converted help panel. The default name is “userid.PANELS(help-panel-name)”.
The output panel file name can be specified on the invocation panel for the conversion utility. You can
specify the panel library of your choice. If the SCRIPT option was specified, the help-panel-name is
also used to build the file name in which the conversion utility stores the image of the help panel. The
default name is “userid.SCRIPT(help-panel-name)”.
See Chapter 10, “Using the conversion utility,” on page 151 for complete information on invocation
syntax.
The ISPF tutorial facility displays help panels. The user can scroll forward by pressing Enter or the
RIGHT (F11) key, or scroll backward by pressing the LEFT (F10) key. The scrolling indicators “More:   
+”, “More:  -”, and "More:  -+" are added to the displayed panel to indicate more help is available.
HELP=hhelp-panel-name | %varname
This attribute specifies the name of a defined help for help panel. It identifies the help text that is
associated with help processing.
The hhelp-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
Specification of the HELP attribute causes ISPDTLC to generate ".HHELP=hhelp-panel-name" (or
".HHELP=&varname") in the )INIT section during help panel generation.
If no value is provided for the HELP attribute, the conversion utility adds the default ".HHELP =
ISP00006" to the generated panel.
ISPF displays this panel when the application user requests help and the cursor is not on a panel field
that is defined as a reference phrase.
HELPDEF=helpdef-id
This attribute specifies a defined help default. The helpdef-id value is the identifier specified on the
HELPDEF tag. You can override any of the defaults from this HELPDEF tag by specifying that attribute
on the HELP tag. See the description of the HELPDEF tag for information on defining help defaults.
WIDTH=50 | n | FIT
This attribute specifies the width of the help panel. The default width is 50. When you specify this
attribute, it should be greater than or equal to the minimum width of 16 characters. The maximum is
156. Because there are set margins of 1 character on each side of the panel text to allow for 3270
attribute bytes, the effective width for text for a help panel defined as WIDTH=50 is 48 characters.
If you have specified WIDTH=FIT, the conversion utility formats the panel using the maximum
available width. When formatting is completed the WIDTH value is reset to the minimum width used
or to 16 if the formatted panel is less than 16 characters wide.
If the specified WIDTH exceeds the maximum minus 4 allowed by the display device, ISPF issues an
error message at run time.
HELP
Chapter 12. Tag reference  305

## Page 338

DEPTH=10 | n | FIT
This attribute specifies the depth of the HELP panel. The maximum depth is 60 and the minimum
depth is 6. When the panel body does not end with a scrollable area, four lines at the bottom of each
help panel are reserved for the function key area. Two lines are reserved at the top of the help panel
for the help-panel-title and a separator line. You must include provisions for these 6 lines in the depth
you specify.
The default help panel depth of 10 is used when the DEPTH attribute provided cannot be used or the
DEPTH attribute is not specified.
If you have specified DEPTH=FIT, the conversion utility formats the panel using a depth of 22. When
formatting is completed the DEPTH value is reset to the minimum depth used or to 6 if the formatted
panel contains less than 6 lines.
If the specified DEPTH exceeds the maximum, minus 2, allowed by the display device, ISPF issues an
error message at run time.
CCSID=n
CCSID specifies the coded-character-set identifier as defined by the Character Data Representation
Architecture. CCSID should be entered as a five-position numeric value. For more information on
using the CCSID attribute, refer to the z/OS ISPF Dialog Developer's Guide and Reference.
TUTOR
This attribute specifies that the panel title be formatted with the word Tutorial (or its translated
equivalent) on each end of the title line, similar to ISPF tutorial panels.
KEYLIST=key-list-name
KEYLIST is an ISPF extension to the Dialog Tag Language. This attribute specifies the name of the key
mapping list associated with the help panel. If you do not specify a key-list-name in a HELP definition,
the ISPF-provided key list (ISPHELP) is used. For information about defining key mapping list, see
“KEYL (Key List)” on page 322. For information about the ISPF-provided key list, refer to the z/OS ISPF
User's Guide Vol I.
KEYLTYPE= PRIVATE | SHARED
This attribute is used to add the SHARED keyword to the KEYLIST parameter of the )PANEL
statement. For information about the )PANEL statement, refer to the z/OS ISPF Dialog Developer's
Guide and Reference. The KEYLTYPE attribute is ignored if you have not provided the KEYLIST
attribute as part of the HELP tag definition or as part of an associated HELPDEF tag definition.
APPLID=application-id
This attribute is used to add the application ID to the )PANEL statement. The application-id
overrides the KEYLAPPL invocation option value. The APPLID attribute is ignored if you have not
provided the KEYLIST attribute as part of the HELP tag definition or as part of an associated
HELPDEF tag definition.
EXPAND=xy
This attribute adds the EXPAND(xy) attribute to the )BODY section of the panel. If only one character
is present, the second character is set to the same value. If the EXPAND attribute is present with
no value specified, the conversion utility uses a character from the range of low-order hex values
available for panel attributes. This removes an available character from possible use as a panel
attribute and may cause panel formatting errors.
WINTITLE=window-title
This attribute is used to add a title on the pop-up window border. The attribute value is placed in the
ISPF ZWINTTL variable. The maximum length of the window-title text is the panel width minus 1.
APPTITLE=application-title
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
MERGESAREA= NO | YES
This attribute controls an additional formatting step for panels with a single scrollable area. If the
entire contents of the scrollable area fit within a standard 24-line panel (allowing 4 lines for the
function keys display), the scrollable area content is moved into the panel body.
HELP
306  z/OS: z/OS ISPF DTL Guide

## Page 339

MSGLINE=YES | NO
This attribute controls the provision for a long message line in the generated panel. When
MSGLINE=NO, the blank line for the long message is not added to the panel )BODY section. It is
the panel designer's responsibility to ensure that critical panel areas are positioned so that the long
message does not inhibit use of the resulting panel.
IMAPNAME=image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPROW=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
IMAPCOL=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
ZUP=zup-id
This attribute provides the name of the Tutorial panel to be assigned to the ZUP variable. It is valid
only when the TUTOR attribute has also been specified.
ZCONT=zcont-id
This attribute provides the name of the Tutorial panel to be assigned to the ZCONT variable. It is valid
only when the TUTOR attribute has also been specified.
help-panel-title
This specifies the title that appears on the help panel.
The help-panel-title is centered within the specified help panel width in accordance to CUA rules. If
the title text is wider than the WIDTH specified, the title is truncated with an ellipsis (…) appended.
Two lines are reserved for the title and a separator which can include the scrolling indicator if there
are more panels.
Comments
The HELP tag defines a help panel. A help panel can contain multiple information areas, which you use the
INFO tag to define (see “INFO (Information Region)” on page 317).
ISPF always displays help panels defined with DTL in a pop-up window with a border. Therefore, the
maximum value you can specify for the WIDTH attribute is 4 less than the maximum allowed by
the display device. This allows for the left and right borders and their 3270 attribute characters. The
maximum value for the DEPTH attribute is 2 less than the maximum allowed by the display device to
allow for the top and bottom borders. Borders are added to the formatted help panel at run time.
If you are not creating a scrollable help panel and the text to be included in the )BODY section of the
ISPF panel exceeds the specified DEPTH value, up to 36 additional panels are generated to contain the
additional text. If the help text extends beyond the original help panel and 36 additional help panels, an
error message is issued and the excess text is truncated. If the error occurs, and the DEPTH and WIDTH
attributes are not set to their maximum values, the values should be increased or the amount of text to be
included in the help panel should be reduced.
For nonscrollable HELP panels or for scrollable HELP panels which end with a nonscrollable section, a
function key area of four lines is reserved at the bottom of the panel. The four lines are taken from the
value specified for the DEPTH attribute.
If you do not specify the KEYLIST attribute, ISPF automatically associates the ISPF-provided key list
"ISPHELP" with all DTL help panels.
This table shows the "ISPHELP" key list and assignments:
HELP
Chapter 12. Tag reference  307

## Page 340

Table 38. ISPHELP keylist and assignments
Key Command Key Label Format
F1 HELP Help Short
F2 SPLIT Split Long
F3 EXIT Exit Short
F4 RESIZE Resize Long
F5 EXHELP Exhelp Short
F6 KEYSHELP Keyshelp Short
F7 UP PrvTopic Short
F8 DOWN NxtTopic Short
F9 SWAP Swap Long
F10 LEFT PrvPage Short
F11 RIGHT NxtPage Short
F12 CANCEL Cancel Short
All ISPHELP function keys are active when the cursor is in the help panel. Display of keys in the function
key area is controlled by the user through the ISPF FKA command.
Because help panels are displayed by the ISPF tutorial processor, the commands assigned to the keys are
those supported by the ISPF tutorial. For more information on the ISPF tutorial, refer to the z/OS ISPF
User's Guide Vol I.
Since ISPDTLC generated panels are not normally used in a full Tutorial, the default ISPHELP keylist may
result in confusion in the use of the F7 and F8 keys for scrolling. An alternate approach is the ISPHLP2
keylist. To use this keylist, add the KEYLIST=ISPHLP2 attribute to your help panel definition.
Table 39. ISPHLP2 keylist and assignments
Key Command Key Label Format
F1 HELP Help Short
F2 SPLIT Split Long
F3 EXIT Exit Short
F4 RESIZE Resize Long
F5 EXHELP Exhelp Short
F6 KEYSHELP Keyshelp Short
F7 LEFT PrvPage Short
F8 RIGHT NxtPage Short
F9 SWAP Swap Long
F10 LEFT PrvPage Long
F11 RIGHT NxtPage Long
F12 CANCEL Cancel Short
HELP
308  z/OS: z/OS ISPF DTL Guide

## Page 341

Restrictions
• The HELP tag requires an end tag.
• You cannot code the HELP tag within any other tag definition.
• If the help panel does not have a panel body, the conversion utility issues an error message. The help
panel must contain at least one INFO (information region) definition to qualify as a panel body. See
“INFO (Information Region)” on page 317 for a complete description of this tag.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
Table 40. The tags you can code within a HELP definition 
Tag Reference Usage Required
AREA “AREA (Area)” on page 189 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
GENERATE “GENERATE (Generate)” on page 298 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple Yes
INFO “INFO (Information Region)” on page 317 Multiple Yes
REGION “REGION (Region)” on page 405 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
TEXTLINE “TEXTLINE (Text Line)” on page 439 Single No
Examples
Here is help panel markup that contains an information region that contains a paragraph, a definition
list, and two unordered lists nested within the definition list. Because all of the data does not fit in one
help panel, the conversion utility created three panels HELP, HELPXXX0, and HELPXXX1. The panels are
scrollable. Figures Figure 120 on page 310, Figure 121 on page 310, and Figure 122 on page 310 show
the formatted results with the function key area displayed in its short form.
<!DOCTYPE DM SYSTEM>
<HELP NAME=help WIDTH=46 DEPTH=16>ShelfBrowse for Kids
<AREA>
  <INFO>
    <P>ShelfBrowse can help you
    find any kind of book you are looking for.
    The two main categories for books are:
    <DL TSIZE=12>
      <DTHD>Book
      <DDHD>Description
      <DT>Fiction
      <DD>Fiction books are stories
      that never really happened.
      The writer made them up.
      For example:
        <UL>
          <LI>Fairy Tales
          <LI>Mysteries
          <LI>Science fiction stories
        </UL>
      <DT>Nonfiction
HELP
Chapter 12. Tag reference  309

## Page 342

<DD>Nonfiction books are about
      things that really exist.
      For example:
        <UL>
          <LI>History books
          <LI>Reference books
          <LI>How to books
        </UL>
    </DL>
  </INFO>
</AREA>
</HELP>
             ShelfBrowse for Kids
                                  More:     +
 ShelfBrowse can help you find any kind of
 book you are looking for. The two main
 categories for books are:
 Book        Description
 Fiction     Fiction books are stories that
             never really happened. The
             writer made them up. For
             example:
  F1=Help        F3=Exit        F5=Exhelp
  F6=Keyshelp    F7=PrvTopic    F8=NxtTopic
 F10=PrvPage    F11=NxtPage    F12=Cancel
Figure 120. Help panel (example 1 of 3)
             ShelfBrowse for Kids
                                  More:   - +
             o   Fairy Tales
             o   Mysteries
             o   Science fiction stories
 Nonfiction  Nonfiction books are about
             things that really exist. For
             example:
  F1=Help        F3=Exit        F5=Exhelp
  F6=Keyshelp    F7=PrvTopic    F8=NxtTopic
 F10=PrvPage    F11=NxtPage    F12=Cancel
Figure 121. Help panel (example 2 of 3)
             ShelfBrowse for Kids
                                  More:   -
             o   History books
             o   Reference books
             o   How to books
  F1=Help        F3=Exit        F5=Exhelp
  F6=Keyshelp    F7=PrvTopic    F8=NxtTopic
 F10=PrvPage    F11=NxtPage    F12=Cancel
Figure 122. Help panel (example 3 of 3)
HELP
310  z/OS: z/OS ISPF DTL Guide

## Page 343

HELPDEF (Help default)
The HELPDEF tag defines default values for help panels.
Syntax
<HELPDEF ID=helpdef-id
HELP= hhelp-panel-name
%varname
WIDTH= n
FIT
DEPTH= n
FIT
CCSID=n
KEYLIST=key-list-name KEYLIST options EXPAND=xy
WINTITLE=window-title APPTITLE=application-title
MERGESAREA=
NO
YES
IMAPNAME= image-name
%varname
IMAPROW= n
%varname
IMAPCOL= n
%varname
>
</HELPDEF>
KEYLIST options
KEYLTYPE=
PRIVATE
SHARED
APPLID=application-id
Parameters
ID=helpdef-id
This is the ID of the help panel default definition. The ID is used as the identifier of this set of default
definitions on the HELP tag.
The helpdef-id must follow the standard naming convention described in “Rules for variable names”
on page 179.
HELP=hhelp-panel-name | %varname
This attribute specifies the default name of a defined help for help panel. It identifies the help text
that is associated with help processing.
The hhelp-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
WIDTH=n | FIT
This attribute specifies a default width value for a help panel that refers to this help default.
HELPDEF
Chapter 12. Tag reference  311

## Page 344

DEPTH=n | FIT
This attribute specifies a default depth value for a help panel that refers to this help default.
CCSID=n
CCSID specifies the coded-character-set identifier as defined by the Character Data Representation
Architecture. CCSID should be entered as a five-position numeric value. For more information on
using the CCSID attribute, refer to the z/OS ISPF Dialog Developer's Guide and Reference.
KEYLIST=key-list-name
This attribute specifies the name of the key mapping list associated with the help panel. If you do
not specify a key-list-name in a HELP definition, the ISPF-provided key list (ISPHELP) is used. For
information about defining key mapping list, see “KEYL (Key List)” on page 322. For information on the
ISPF-provided key list, refer to the z/OS ISPF User's Guide Vol I.
KEYLTYPE=PRIVATE | SHARED
This attribute is used to add the SHARED keyword to the KEYLIST parameter of the )PANEL
statement. For information about the )PANEL statement, refer to the z/OS ISPF Dialog Developer's
Guide and Reference.
APPLID=application-id
This attribute is used to add the application ID to the )PANEL statement. The application-id
overrides the KEYLAPPL invocation option value.
EXPAND=xy
This attribute adds the EXPAND(xy) attribute to the )BODY section of the panel. If only one character
is provided, the second character is set to the same value. If the EXPAND attribute is present with
no value specified, the conversion utility uses a character from the range of low-order hex values
available for panel attributes. This removes an available character from possible use as a panel
attribute and may cause panel formatting errors.
WINTITLE=window-title
This attribute is used to add a title on the pop-up window border. The attribute value is placed in the
ISPF ZWINTTL variable. The maximum length of the window-title is the panel width minus 1.
APPTITLE=application-title
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
MERGESAREA= NO | YES
This attribute controls an additional formatting step for panels with a single scrollable area. If the
entire contents of the scrollable area fit within a standard 24-line panel (allowing 4 lines for the
function keys display), the scrollable area content is moved into the panel body.
IMAPNAME=image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPROW=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPCOL=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
Comments
The HELPDEF tag defines default values for help panels. When a HELP panel tag refers to a help panel
default, the values specified by the associated HELPDEF tag are used for the help panel unless overridden
by values specified in the HELP tag definition.
The HELP tag can override any of the HELPDEF values by specifying that value within its own definition.
Therefore, it is possible for a HELP tag to select certain default values from the help panel default and
override others.
HELPDEF
312  z/OS: z/OS ISPF DTL Guide

## Page 345

See “HELP (Help Panel)” on page 303 for more information.
You can code multiple HELPDEF definitions in a single application. Each help default must have a unique
helpdef-id.
Restrictions
• You cannot code the HELPDEF tag within any other tag definition.
• You must code the HELPDEF tag before you code any HELP tag that refers to it.
Processing
None.
Examples
Here is a source file example where the HELPDEF definition defines default DEPTH and WIDTH values.
The help panels "help15" and "help16" both reference the help default. "help15" uses both default values
and "help16" uses only the default WIDTH value, and overrides the default DEPTH value by specifying
its own DEPTH value. The help panel "help17" does not reference the help default, and defines its own
DEPTH and WIDTH values.
<!DOCTYPE DM SYSTEM>
<HELPDEF ID=helpdef1 DEPTH=10 WIDTH=40>
<HELP NAME=help15 HELPDEF=helpdef1>Help for This
⋮
</HELP>
<HELP NAME=help16 HELPDEF=helpdef1 DEPTH=15>Help for That
⋮
</HELP>
<HELP NAME=help17 DEPTH=15 WIDTH=25>Help for the Other
⋮
</HELP>
Hn (Heading)
The heading tags define main topics and subtopics of information within an information region.
Syntax
<Hn
COMPACT
>
heading-text </Hn>
Parameters
COMPACT
This attribute causes the heading-text to be formatted without creating a blank line before the
heading.
heading-text
This is the text of the heading.
Comments
Hn
Chapter 12. Tag reference  313

## Page 346

The heading tags define main topics and subtopics of information within an information region. You can
define up to four heading levels. The n in Hn indicates the heading level. The heading levels are formatted
in this fashion:
H1
Identifies a main topic of information. The text is centered on the panel.
H2, H3, H4
The text is formatted against the left margin of the panel body.
Headings are formatted with one blank line before them.
Restrictions
• The Hn tag must be coded within an INFO definition. See “INFO (Information Region)” on page 317 for a
complete description of this tag.
Processing
Table 41. The tags you can code only within an H2, H3, or H4 definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is help panel markup that contains two levels of headings. Figure 123 on page 315 shows the
formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=hn DEPTH=22>Department Descriptions Help
<AREA>
<INFO>
  <H1>Departments
  <H2>Entertainment
  <P>Our entertainment department carries the
  finest in home entertainment components.
  <H2>Exotic Pets
  <P>You can order from a wide variety of exotic
  pets and pet supplies in this department.
  <H2>Toys
  <P>Your kids will love our wide selection of
  toys, games, and play equipment.
</INFO>
</AREA>
</HELP>
Hn
314  z/OS: z/OS ISPF DTL Guide

## Page 347

Department Descriptions Help
                   Departments
 Entertainment
 Our entertainment department carries the finest
 in home entertainment components.
 Exotic Pets
 You can order from a wide variety of exotic pets
 and pet supplies in this department.
 Toys
 Your kids will love our wide selection of toys,
 games, and play equipment.
  F1=Help        F3=Exit        F5=Exhelp
  F6=Keyshelp    F7=PrvTopic    F8=NxtTopic
 F10=PrvPage    F11=NxtPage    F12=Cancel
Figure 123. Headings
HP (Highlighted Phrase)
The HP tag identifies text to be displayed with highlighted emphasis.
Syntax
<HP
TYPE=
ET
CH
CT
FP
LEF
LI
NT
PT
SAC
TEXT
WASL
WT
COLOR= WHITE
RED
BLUE
GREEN
PINK
YELLOW
TURQ
%varname
INTENS=
HIGH
LOW
NON
%varname
HILITE= USCORE
BLINK
REVERSE
%varname
INTENSE=varname phrase-to-be-highlighted </HP>
HP
Chapter 12. Tag reference  315

## Page 348

Parameters
TYPE= ET | CH | CT | FP | LEF | LI | NT | PT | SAC | TEXT | WASL | WT
This attribute defines the attribute type to be applied to the phrase-to-be-highlighted. Using a CUA
attribute causes the text to appear in the associated color.
When TYPE=TEXT, a non-CUA attribute is generated and you can specify the color, intensity, and
highlighting with the COLOR, INTENS, and HILITE attributes. These attributes are not valid for CUA
types.
COLOR= WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute specifies the color of the field. You can define this attribute as a variable name preceded
by a percent (%) sign.
INTENS= HIGH | LOW | NON | %varname
This attribute defines the intensity of a field. You can define this attribute as a variable name preceded
by a percent (%) sign.
HILITE= USCORE | BLINK | REVERSE | %varname
This attribute specifies the extended highlighting attribute of a field. You can define this attribute as a
variable name preceded by a percent (%) sign.
INTENSE=varname
This attribute supplies a variable name that must contain a valid value for the INTENS keyword. The
entire phrase is controlled by this value. For example, if the variable contains the value NON, the
phrase is not visible.
phrase-to-be-highlighted
This text displays with highlighted emphasis.
Comments
The HP identifies text to be displayed with highlighted emphasis by ISPF. The HP end tag restores normal
text.
Restrictions
• You can code the HP tag wherever the RP tag is valid.
• You can code the HP tag within the text following the CHDIV, CMDAREA, HELP, and PANEL tags.
• The HP tag requires an end tag.
Processing
None.
Examples
This markup shows the formatted result in Figure 124 on page 317.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=hp KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST> Type in <HP>patron's name</HP> and <HP>card number</HP>
          (if applicable)
<TOPINST> Then select an action bar choice.
<AREA>
HP
316  z/OS: z/OS ISPF DTL Guide

## Page 349

<DTACOL PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25 SELWIDTH=25>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8>Date
    <DTAFLD DATAVAR=cardno ENTWIDTH=7>Card No.
      <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name>Name
      <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address>Address
     </DTACOL>
  <DIVIDER>
  <REGION DIR=horiz>
  <SELFLD NAME=cardsel PMTWIDTH=30 SELWIDTH=38>Choose
  one of the following
    <CHOICE CHECKVAR=card MATCH=new>New
    <CHOICE CHECKVAR=card MATCH=renew>Renewal
    <CHOICE CHECKVAR=card MATCH=replace>Replacement
  </SELFLD>
  <SELFLD TYPE=multi PMTWIDTH=30 SELWIDTH=25>Check valid branches
    <CHOICE NAME=north HELP=nthhlp CHECKVAR=nth>North Branch
    <CHOICE NAME=south HELP=sthhlp CHECKVAR=sth>South Branch
    <CHOICE NAME=east HELP=esthlp CHECKVAR=est>East Branch
    <CHOICE NAME=west HELP=wsthlp CHECKVAR=wst>West Branch
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>Enter a command
</PANEL>
   File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number (if applicable).
 Then select an action bar choice.
 Date . . . :
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 124. HP (Highlighted Phrase)
INFO (Information Region)
The INFO tag defines an information region for a panel.
Syntax
<INFO
WIDTH= format-width
*
INDENT=n
> </INFO>
INFO
Chapter 12. Tag reference  317

## Page 350

Parameters
WIDTH=format-width | *
This attribute determines the width the conversion utility uses to format the text in the ISPF )BODY
section of the panel. If WIDTH is not the value is set to the remaining available panel (or region) width.
If specified, the WIDTH value cannot be larger than the defined width of the panel (or region) minus
2 characters. For example, a WIDTH value of 58 is acceptable for an information region within a panel
with a defined width of 60.
Note: You should code the WIDTH attribute if the information region is part of an application panel
definition that uses horizontal region capability. The actual width used in a horizontal region is 2
characters longer than the WIDTH attribute value to provide for attribute bytes that delimit the region.
INDENT=n
This attribute defines the number of columns to indent the current information region from the current
left boundary.
Comments
The INFO tag defines an information region for a panel. The information region is used to display text such
as paragraphs, lists, notes, examples, and figures. A typical use of the INFO tag is for the definition of text
within help panels.
Restrictions
• The INFO tag requires an end tag.
• You must code the INFO tag within an AREA, HELP, or PANEL definition. See “AREA (Area)” on page 189,
“HELP (Help Panel)” on page 303, and “PANEL (Panel)” on page 376 for descriptions of these tags.
Processing
Table 42. The tags you can code within an INFO definition 
Tag Reference Usage Required
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
Hn “Hn (Heading)” on page 313 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
INFO
318  z/OS: z/OS ISPF DTL Guide

## Page 351

Table 42. The tags you can code within an INFO definition  (continued)
Tag Reference Usage Required
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains an information region. The text of the information region is
defined using two P (paragraph) tags and an unordered list (UL) tag with three LI (list item) tags. Figure
125 on page 319 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=info WIDTH=60 DEPTH=22>ShelfBrowse Help
<AREA>
<INFO WIDTH=42>
  <P>When ShelfBrowse finds your book, it displays this
  information:
  <UL>
    <LI>Reference information about the book.
    <LI>The location of the book.
    <LI>If the book is in stock.
      <P>If the book is not in stock, see the librarian.
  </UL>
</INFO>
</AREA>
</HELP>
                      ShelfBrowse Help
 When ShelfBrowse finds your book, it
 displays this information:
 o   Reference information about the book.
 o   The location of the book.
 o   If the book is in stock.
     If the book is not in stock, see the
     librarian.
  F1=Help        F3=Exit        F5=Exhelp      F6=Keyshelp
  F7=PrvTopic    F8=NxtTopic   F10=PrvPage    F11=NxtPage
 F12=Cancel
Figure 125. Information region
KEYI (Key Item)
The KEYI tag defines a key assignment within a key mapping list.
KEYI
Chapter 12. Tag reference  319

## Page 352

Syntax
<KEYI KEY=virtual-key CMD=internal-command-name
CASE=
UPPER
MIXED
FKA=
NO
YES
LONG
SHORT
PARM=parm-string
>
FKA-text
</KEYI>
Parameters
KEY=virtual-key
This attribute specifies the name of the key to assign to the command. The conversion utility supports
F1-F24 only.
CMD=internal-command-name
This attribute specifies the command that ISPF runs when the user presses the key.
The internal-command-name must follow the standard naming convention described in “Rules for
variable names” on page 179.
As an extension to the Dialog Tag Language, the conversion utility supports special ISPF command
syntax for internal-command-name. In this case, the internal-command-name must have these
characteristics:
• 2-9 single-byte characters in length
• The first character must be a ‘>’, ‘:’, or ‘%’.
To code the > character you must use the &gtsym predefined entity. See “Predefined entities” on
page 23 for more information.
• The second character must be A-Z, a-z, @, #, or $.
• Remaining characters, if any, must be A-Z, a-z, @, #, $, or 0-9.
Lowercase characters are translated to their uppercase equivalents by default.
CASE=UPPER | MIXED
This attribute specifies whether the internal-command-name is converted to uppercase characters or
stored as entered in the tag definition.
FKA=NO | YES | LONG | SHORT
This attribute specifies whether the key assignment is to appear in the function key area of an
application panel. The default value NO indicates that the key is not to appear. You must specify
FKA=YES, FKA=LONG, or FKA=SHORT if you want the key to be displayed in the function key area.
When FKA=NO is specified, the key is active even if it is not displayed.
PARM=parm-string
This attribute allows a parameter to be added to the command specified by the CMD attribute. The
combined length of the command and the parameter is limited to 40 bytes. When the combined
length exceeds 40 bytes, truncation of the PARM occurs at the end of the last complete word in
the parm-string, for a parm-string containing multiple words. A parm-string which is a single word is
truncated at position 40.
KEYI
320  z/OS: z/OS ISPF DTL Guide

## Page 353

FKA-text
This is the text for the key which is to appear in the function key area of the panels that refer to the
key list. This text is appended to the string "Fn=" (with no intervening space) to create the displayed
format. Use initial caps for the FKA-text value.
If not specified, the FKA-text defaults to the internal-command-name specified for the key.
The function key area is formatted at run time based on the panel size. The maximum number of bytes
allowed for FKA-text is 64. If the text exceeds 64 bytes, it is truncated and a warning message is
issued. The conversion utility removes excess blanks from FKA-text. The first 8 bytes of the resulting
text are used by ISPF.
Comments
The KEYI tag defines a key assignment within a key mapping list. Key assignments provide a means of
associating commands with keys.
KEYI tags with the same assignment cause the conversion utility to issue a warning message and retain
only the first occurrence.
Restrictions
• You must code the KEYI tag within a KEYL definition. See “KEYL (Key List)” on page 322 for a complete
description of this tag.
• Each KEYI definition can only have one command assigned to it. Additionally, CUA requires these
conventions when assigning commands to certain keys:
– If KEY=F1 or F13, then CMD must be HELP.
– If KEY=F3 or F15, then CMD must be EXIT.
– If KEY=F12 or F24, then CMD must be CANCEL.
ISPF lets you provide the name of your own command on these keys.
If you code the command HELP, EXIT, or CANCEL as part of your KEYI definition, then HELP must be
assigned to key F1 or F13, EXIT must be assigned to F3 or F15, and CANCEL must be assigned to F12 or
F24.
Processing
None.
Examples
Here is source file markup that contains a key mapping list and an application panel that refers to the key
mapping list. The F7 and F8 keys do not appear on the panel because they both have an FKA value of NO.
Figure 126 on page 322 shows the formatted application panel with the displayed keys.
KEYI
Chapter 12. Tag reference  321

## Page 354

<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>
  <!entity sampbody system>)>
&sampvar1;
<KEYL NAME=keylxmp>
  <KEYI KEY=f1  CMD=help     FKA=yes>Help
  <KEYI KEY=f2  CMD=split    FKA=yes>Split
  <KEYI KEY=f3  CMD=exit     FKA=yes>Exit
  <KEYI KEY=f5  CMD=search   FKA=no>Display
  <KEYI KEY=f6  CMD=keyhlp   FKA=yes>Keyshelp
  <KEYI KEY=f7  CMD=backward FKA=no>Backward
  <KEYI KEY=f8  CMD=forward  FKA=no>Forward
  <KEYI KEY=f9  CMD=swap     FKA=yes>Swap
  <KEYI KEY=f10 CMD=actions  FKA=no>Actions
  <KEYI KEY=f12 CMD=cancel   FKA=yes>Cancel
</KEYL>
<PANEL NAME=keyi KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
&sampbody;
</PANEL>
   File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number if applicable.
 Then select an action bar choice.
 Date . . . :
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 126. Key Items
KEYL (Key List)
The KEYL tag defines a key mapping list where keys can be mapped to commands.
Syntax
<KEYL NAME=key-list-name
HELP=help-panel-name
ACTION=
UPDATE
DELETE
APPLID=application-id
> </KEYL>
KEYL
322  z/OS: z/OS ISPF DTL Guide

## Page 355

Parameters
NAME=key-list-name
This attribute specifies a name for a key list. The HELP, HELPDEF, PANEL, and PANDEF tag refer to the
key-list-name.
The key-list-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
The name of the keylist table is xxxxKEYS where xxxx represents the application identifier provided
to ISPDTLC with the KEYLAPPL keyword when invoked, in the "Keylist Application ID" field on the
invocation panel, or with the APPLID attribute of this tag.
The key-list-name is used to identify the entry in the keylist table. For example, if NAME=CONVLIST
and KEYLAPPL=XYZ, then CONVLIST is written as a table entry to member XYZKEYS in the table
library partitioned data set.
Keylists are updated using ISPF table services. Input is obtained from the ISPTLIB DDname allocation
and output is written to the ISPTABL DDname allocation. See the description of how to allocate
libraries before starting ISPF in the z/OS ISPF User's Guide Vol I for more information about the use of
ISPTLIB and ISPTABL.
See Chapter 10, “Using the conversion utility,” on page 151 for more information on invocation
parameters for the conversion utility.
HELP=help-panel-name
This attribute names a help panel that displays when the user requests help on a keylist display.
If a user requests help for a keylist and no help has been defined by the KEYL tag, the ZKEYHELP
variable is checked for a help panel name. If the application has not set ZKEYHELP, a message that
keyshelp is not available is displayed.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels.
ACTION=UPDATE | DELETE
This attribute specifies the type of action requested for the keylist specified by key-list-name.
When ACTION=DELETE is specified, it is not necessary to nest any KEYI tags within the KEYL tag
definition.
APPLID=application-id
This attribute provides the application ID used to build the keylist name. The application-id overrides
the KEYLAPPL invocation option value.
Comments
The KEYL tag defines a key mapping list where keys can be mapped to commands.
To display these keys on a panel requires that the PANEL or PANDEF tag refer to the key-list-name. ISPF
uses the specified key mapping list when building the display dependent on the user's setting by the FKA
command. For more information about displaying and formatting of the function key area, refer to the
appropriate section in the z/OS ISPF Dialog Developer's Guide and Reference.
Restrictions
• The KEYL tag requires an end tag.
• The KEYL tag cannot be nested within any other tag definition.
• When ACTION=UPDATE is specified (or defaulted), at least one KEYI tag must be included in the keylist
definition.
KEYL
Chapter 12. Tag reference  323

## Page 356

Processing
Table 43. The tags you can code within a KEYL definition 
Tag Reference Usage Required
KEYI “KEYI (Key Item)” on page 319 Multiple No
Examples
Here is source file markup that contains a key mapping list and an application panel that refers to the key
mapping list. Figure 127 on page 324 shows the formatted application panel with the displayed keys.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampabc system>
  <!entity sampbody system>)>
&sampvar1;
<KEYL NAME=keyltbl>
  <KEYI KEY=f1  CMD=help     FKA=yes>Help
  <KEYI KEY=f2  CMD=split    FKA=yes>Split
  <KEYI KEY=f3  CMD=exit     FKA=yes>Exit
  <KEYI KEY=f5  CMD=search   FKA=no>Display
  <KEYI KEY=f6  CMD=keyhlp   FKA=no>Keyshelp
  <KEYI KEY=f7  CMD=backward FKA=yes>Backward
  <KEYI KEY=f8  CMD=forward  FKA=yes>Forward
  <KEYI KEY=f9  CMD=swap     FKA=yes>Swap
  <KEYI KEY=f10 CMD=actions  FKA=no>Actions
  <KEYI KEY=f12 CMD=cancel   FKA=yes>Cancel
</KEYL>
<PANEL NAME=keyl KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
&sampbody;
</PANEL>
   File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number if applicable.
 Then select an action bar choice.
 Date . . . :
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 127. Function keys
KEYL
324  z/OS: z/OS ISPF DTL Guide

## Page 357

LI (List Item)
The LI tag defines a list item within a note list, ordered list, unordered list, or simple list.
Syntax
<LI
SPACE=
NO
YES
NOSKIP
>
item-text
</LI>
Parameters
SPACE=NO | YES
The SPACE attribute controls the indentation space for the list item. When the SPACE attribute is not
specified on the LI tag, the SPACE attribute from the enclosing list tag is used to set the indentation
space for the item-text.
When SPACE=YES, the indentation is set to 3 spaces. When SPACE=NO (or SPACE is not specified),
the indentation is set to 4 spaces.
The SPACE attribute can be used to control the alignment of list items when the first word of some list
items is a DBCS word preceded by a shift-out character and the first word of other list items is a SBCS
word.
NOSKIP
This attribute causes the list item to format without creating a blank line before the item.
item-text
This is the text of the list item.
Comments
The LI tag defines a list item within a note list, ordered list, unordered list, or simple list.
The formatting of the LI tag is dependent on the type of list you use it within and the level of nesting.
List
Formatting
Note
Formats with a 3-space or 4-space indentation (depending on the SPACE attribute) and is preceded by
sequential numbers.
Ordered
Formats with a 3-space or 4-space indentation (depending on the SPACE attribute) within the level of
the list in which it is defined and is preceded by sequential numbers or letters.
Simple
Formats with a 3-space or 4-space indentation (depending on the SPACE attribute) within the level of
the list it is defined within.
Unordered
Formats with a 3-space or 4-space indentation (depending on the SPACE attribute) within the level of
the list in which it is defined and is preceded by bullets or dashes.
The next list item implicitly ends the previous list item as do the NOTEL, OL, SL, and UL end tags.
If you do not specify text for a list item, a blank line is displayed for that item.
LI
Chapter 12. Tag reference  325

## Page 358

Restrictions
• You must code the LI tag within a NOTEL, OL, SL, or UL definition. See “NOTEL (Note List)” on page 361,
“OL (Ordered List)” on page 367, “SL (Simple List)” on page 433, and “UL (Unordered List)” on page 443
for descriptions of these tags.
Processing
Table 44. The tags you can code within an LI definition 
Tag Reference Usage Required
ATTENTION “ATTENTION (Attention)” on page 198 Single No
CAUTION “CAUTION (Caution)” on page 205 Single No
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
WARNING “WARNING (Warning)” on page 454 Single No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains an unordered list with three list items. The last list item contains
an additional paragraph of text. Figure 128 on page 327 shows the formatted result.
LI
326  z/OS: z/OS ISPF DTL Guide

## Page 359

<!DOCTYPE DM SYSTEM>
<HELP NAME=li DEPTH=20>ShelfBrowse Help
<AREA>
<INFO>
  <P>When ShelfBrowse finds your book,
  it displays this information:
  <UL>
    <LI>Reference information about the book.
    <LI>The location of the book.
    <LI>If the book is in stock.
      <P>If the book is not in stock, see the librarian.
  </UL>
  <P>Thank you for using ShelfBrowse.
</INFO>
</AREA>
</HELP>
                 ShelfBrowse Help
 When ShelfBrowse finds your book, it displays
 this information:
 o   Reference information about the book.
 o   The location of the book.
 o   If the book is in stock.
     If the book is not in stock, see the
     librarian.
 Thank you for using ShelfBrowse.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 128. List items
LINES (Lines)
The LINES tag defines unformatted text within an information region.
Syntax
< LINES
NOSKIP
>
text
</LINES>
Parameters
NOSKIP
This attribute causes the blank line normally placed before the lines to be skipped.
text
This is the unformatted text.
Comments
The LINES tag defines unformatted text within an information region. Tags that normally cause word-
wrapping (such as the P, LI, or CAUTION) do not cause wrapping when nested within a LINES definition.
LINES
Chapter 12. Tag reference  327

## Page 360

If the source text on any line is too long to fit in the remaining available formatting width, the conversion
utility truncates that line. The conversion utility issues a warning message the first time that truncation
occurs.
The formatting of the LINES tag is similar to that of the FIG tag, except that there is no border or caption
capability.
Restrictions
• The LINES tag requires an end tag.
• You must code the LINES tag within an INFO definition. See “INFO (Information Region)” on page 317
for a complete description of this tag.
Processing
Table 45. The tags you can code within a LINES definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is application panel markup that contains a LINES definition. The formatted output of the LINES
definition is identical to the input markup. Figure 129 on page 329 shows the formatted results.
<!DOCTYPE DM SYSTEM>
<PANEL NAME=lines DEPTH=22 WIDTH=40>Lines Tag Example
<AREA>
<INFO WIDTH=38>
<P>The following area shows how the LINES tag formats.
<LINES>
First line, just at it was entered.
  Second line, ditto.
Notice we skipped a line here?
  You
     can
        even
            do
LINES
328  z/OS: z/OS ISPF DTL Guide

## Page 361

this.
</LINES>
<P>The LINES tag formatting ends immediately above.
</INFO>
</AREA>
</PANEL>
           Lines Tag Example
 The following area shows how the LINES
 tag formats.
 First line, just at it was entered.
   Second line, ditto.
 Notice we skipped a line here?
   You
      can
         even
             do
               this.
 The LINES tag formatting ends
 immediately above.
  F1=Help    F3=Exit   F12=Cancel
Figure 129. LINES
LIT (Literal)
The LIT tag defines a string where all blanks are significant and included in the value.
Syntax
<LIT> literal-display-value </LIT>
Parameters
literal-display-value
This attribute specifies a string with all blanks preserved.
Comments
The LIT tag defines a string where all blanks are significant and included in the value. No stripping of
leading, trailing, or embedded blanks is performed.
This is the only way to specify trailing blanks or a value of all blanks in the XLATI displayed-value.
The LIT start and end tags must be on the same line as the literal-display-value to preserve the original
spacing of the value.
Restrictions
• The LIT tag requires an end tag.
• You must code the LIT tag only within an XLATI definition. See “XLATI (Translate Item)” on page 456 for
a complete description of this tag.
LIT
Chapter 12. Tag reference  329

## Page 362

• Multiple LIT tags may be coded within a single XLATI definition, as long as they are not nested within
each other. However, a better approach is to include the whole XLATI displayed-value within the LIT tag.
Processing
None.
Examples
Here is markup that contains a variable class definition with two translate lists. The last four translate
items in the second list contain LIT definitions that preserve trailing blanks in the displayed value of their
respective translate items.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=aa TYPE='char 2'>
<VARCLASS NAME=bb TYPE='char 9'>
<VARCLASS NAME=cc TYPE='char 9'>
  <XLATL FORMAT=upper>
  </XLATL>
  <XLATL>
    <XLATI VALUE=1>BIGCHARGE
    <XLATI VALUE=2><LIT>V I S T A</LIT>
    <XLATI VALUE=3><LIT>EZCARD   </LIT>
    <XLATI VALUE=4><LIT>CHECK    </LIT>
    <XLATI VALUE=5><LIT>     CASH</LIT>
</XLATL>
<VARLIST>
  <VARDCL NAME=dispva VARCLASS=cc>
  <VARDCL NAME=inptva VARCLASS=bb>
  <VARDCL NAME=chckva VARCLASS=aa>
</VARLIST>
<PANEL NAME=lit>LIT translation
  <TOPINST>You can display this panel with ISPF option 7.2
  <TOPINST>For this example, enter the word 'BIGCHARGE', 'V I S T A',
           'EZCARD', 'CHECK', or '     CASH' in the "literal value"
           field (no quotes).
  <TOPINST>The literal will be translated to the corresponding number
           defined in the XLATL tag, and will be displayed in the
           "translated value" field.
  <TOPINST>The literal you enter will be displayed (left justified) in
           the "original value" field.
  <DTACOL>
  <:-- assign "literal value" to "original value" -->
  <SOURCE>
  &inptva = &dispva
  </SOURCE>
  <DTAFLD DATAVAR=dispva ENTWIDTH=9 PMTWIDTH=20 ALIGN=center>Literal value
  <DTAFLD DATAVAR=chckva ENTWIDTH=2 PMTWIDTH=20 USAGE=out>Translated value
  <DTAFLD DATAVAR=inptva ENTWIDTH=9 PMTWIDTH=20 USAGE=out>Original value
  <:-- assign translated "literal value" to "translated value" -->
  <SOURCE>
  &chckva = &dispva
  </SOURCE>
  </DTACOL>
  <CMDAREA>
</PANEL>
LP (List Part)
The LP tag defines a comment or explanation within a note list, ordered list, unordered list, or simple list.
LP
330  z/OS: z/OS ISPF DTL Guide

## Page 363

Syntax
< LP
NOSKIP
>
implied-paragraph </LP>
Parameters
NOSKIP
This attribute causes the list part to format without creating a line before the list part.
implied-paragraph
This is the text of the list part.
Comments
The LP tag defines a comment or explanation within an ordered list, unordered list, or simple list. You can
code the LP tag anywhere within a list.
The text of the list part starts at the left margin of the current level of the list. It is not numbered
or lettered. When you use it within a NOTEL or OL definition, LP does not interrupt or increment the
sequence.
The next list item or the end of the list implicitly ends the list part.
Restrictions
• You must code the LP tag within a NOTEL, OL, SL, or UL definition. See “NOTEL (Note List)” on page 361,
“OL (Ordered List)” on page 367, “SL (Simple List)” on page 433, and “UL (Unordered List)” on page 443
for descriptions of these tags.
Processing
Table 46. The tags you can code within an LP definition 
Tag Reference Usage Required
ATTENTION “ATTENTION (Attention)” on page 198 Single No
CAUTION “CAUTION (Caution)” on page 205 Single No
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
LP
Chapter 12. Tag reference  331

## Page 364

Table 46. The tags you can code within an LP definition  (continued)
Tag Reference Usage Required
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
WARNING “WARNING (Warning)” on page 454 Single No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains an ordered list with a nested list part tag. WARNING and P tags
are nested within the list part definition. Figure 130 on page 332 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=lp WIDTH=50 DEPTH=20>Help For Changing a File
<AREA>
<INFO>
  <OL>
    <LI>Type over the existing data
    in the entry fields with the new data.
    <LP>
      <WARNING>
      Performing the next step will save all changes
      and delete the existing data.
      <P>To quit this function without
      deleting the existing data, press F12=Cancel.
      </WARNING>
    <LI>Press Enter to save the
    updated data.
  </OL>
</INFO>
</AREA>
</HELP>
             Help For Changing a File
 1.  Type over the existing data in the entry
     fields with the new data.
 Warning: Performing the next step will save all
 changes and delete the existing data.
 To quit this function without deleting the
 existing data, press F12=Cancel.
 2.  Press Enter to save the updated data.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 130. List part
LSTCOL (List Column)
The LSTCOL tag defines a column of data from an ISPF table displayed in the ISPF table display area of a
panel.
LSTCOL
332  z/OS: z/OS ISPF DTL Guide

## Page 365

Syntax
<LSTCOLDATAVAR=column-data
VARCLASS=variable-class-name
HELP=
NO
YES
help-panel-name
*help-message-id
%varname
*%varname
USAGE=
BOTH
IN
OUT
REQUIRED=
NO
YES
YES MSG=message-id
COLWIDTH=data-width
ALIGN=
START
CENTER
END
AUTOTAB=
NO
YES
LINE=n
CLEAR POSITION=n
FORMAT=
START
CENTER
END
TEXT=descriptive-text
TEXTLOC=
BEFORE
AFTER
TEXTFMT=
START
CENTER
END
TEXTLEN=n
TEXTSKIP=
NO
YES
NOENDATTR
PAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
PAS=
OFF
ON
%varname
CSRGRP=
NO
YES
n
ATTRCHANGE=
NO
YES
NEW
COLSPACE=n
COLTYPE=
CUA
ISPF
EE
VOI
LID
COLOR= WHITE
RED
BLUE
GREEN
PINK
YELLOW
TURQ
%varname
INTENS=
HIGH
LOW
NON
%varname
HILITE= USCORE
BLINK
REVERSE
%varname
CAPS=
OFF
ON
DISPLAY=
YES
NO VARDCL=
YES
NO
>
column-heading</LSTCOL>
LSTCOL
Chapter 12. Tag reference  333

## Page 366

Parameters
DATAVAR=column-data
This is the data which occupies the column. The column-data value must be an ISPF table variable
name (without a leading % sign).
VARCLASS=variable-class-name
This is the name of a variable class, defined with a VARCLASS tag, that overrides the default variable
class referred to by the VARDCL tag that declares the data variable for the list column.
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for this list column. This is
field-level help.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help for a list column and no help is defined, the extended help panel is displayed.
If an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
USAGE=BOTH | IN | OUT
This attribute indicates if this column is for input, output, or both.
REQUIRED=NO | YES
This attribute indicates if this column is required to have input for each modified row. The default is
NO. This attribute is valid only when USAGE=IN or BOTH.
If you specify REQUIRED=YES, a conditional VER(variable,NONBLANK) statement is built by the
conversion utility and placed in the )PROC section of the ISPF panel generated. This results in the
column variable being verified only when the row is selected or modified.
MSG=message-id
This attribute specifies the message that is displayed when the user does not complete a required
entry (defined with the REQUIRED attribute). If you do not specify a message-id, ISPF displays a
default message.
If you specify the MSG attribute and REQUIRED=YES, a VER(variable,NONBLANK,MSG=message-
identifier) statement is built by the conversion utility and placed in the )PROC section of the ISPF
panel generated. If you specify the MSG attribute and REQUIRED=NO (the default), the conversion
utility issues a warning message.
See “MSG (Message)” on page 352 for information about creating messages.
Note: You can specify messages pertaining to other validations using XLATL and CHECKL tags
within a VARCLASS definition. See the descriptions of these tags for additional information.
COLWIDTH=data-width
This attribute determines the data width to be used by the column. If you do not specify this
attribute, the data width and column formatting width are determined by the actual length of the
column-heading. If the width of the column-heading text is greater than the COLWIDTH, it is used as
the column formatting width. The minimum width is 1 and the maximum is the remaining available
panel (or region) width. If the column-heading and the COLWIDTH attribute are omitted, the data
width and column formatting width are determined by the TYPE value of the associated VARCLASS.
If a VARCLASS TYPE value is not available, the size of the column variable name (specified by the
DATAVAR attribute) determines the width.
You should code the COLWIDTH attribute with a value equal to the length of the table data variable.
LSTCOL
334  z/OS: z/OS ISPF DTL Guide

## Page 367

ALIGN=START | CENTER | END
This attribute specifies how the data value is to be displayed in the data field.
An attribute character is used for the field that specifies JUST(LEFT) if ALIGN=START, JUST(ASIS)
if ALIGN=CENTER or JUST(RIGHT) if ALIGN=END. When ALIGN=END, no underscore padding is
performed; blanks are used.
AUTOTAB=NO | YES
When AUTOTAB=YES, the cursor moves to the next field capable of input when the user enters the
last character in the list column field.
AUTOTAB=YES is valid only when the value for USAGE is either BOTH or IN.
LINE=n
This attribute provides the ability to place LSTCOL fields on different model lines. ISPF defines the
range of lines as 1 to 8. The default is 1. Column headings are generated on multiple lines to match
the LSTCOL field placement.
CLEAR
This attribute adds a CLEAR (variable, ...) statement to the )MODEL line. CLEAR should be specified for
table extension variables.
For more information about the )MODEL line, refer to the z/OS ISPF Dialog Developer's Guide and
Reference.
POSITION=n
This attribute specifies the starting position of the data column and related text or the column
heading, if the heading is longer than the data column. If this attribute is not specified or is not valid,
the conversion utility formats the column immediately to the right of the previous column on the
specified or default model line. This attribute allows you to position fields on different model lines
with vertical alignment. Column position is location of the attribute byte preceding the data column.
FORMAT=START | CENTER | END
This attribute specifies how the data column and its column heading are formatted. If you do not
specify this attribute, or if you specify the attribute value START, then the column formats as in ISPF
Version 3.1 and ISPF Version 3.2.
Formatting of the data in the column takes place within the column width, which is determined as
described in the COLWIDTH attribute section.
When you specify the attribute value CENTER, the conversion utility centers a column heading that
is shorter than the column width. If the column heading is longer than the column width, then the
data column is centered under the column heading. When either the heading or the data column is
centered, blank characters are added before and after the column heading or data column. The total
amount of space to be added is divided by 2 and the resulting whole number is the number of blanks
added in front of the column heading or data column. The difference between the total amount of
space and the amount placed in front of the column heading or data column is used at the end.
When you specify the attribute value END, a column heading that is shorter than the column width is
right-justified so it aligns with the end of the displayed data. If the column heading is longer than the
column width, the data column is right-justified so that the displayed data and the column heading
end at the same position.
If there is insufficient space available to format the column heading as requested, the conversion
utility issues a message that the FORMAT attribute is ignored.
The FORMAT attribute does not affect the display of the field contents within the data column, which
is determined by the ALIGN attribute.
TEXT=descriptive-text
This attribute specifies a short description of the data column. It can be placed before or after the
data column. Text containing special characters or embedded blanks must be enclosed in quotes.
TEXTLOC=BEFORE | AFTER
This attribute specifies the location of the TEXT relative to the data column. Text can be placed on
either side of the data column.
LSTCOL
Chapter 12. Tag reference  335

## Page 368

TEXTFMT=START | CENTER | END
This attribute specifies the format of the text within the length of the text area. The text can be
left-justified, centered, right-justified.
TEXTLEN=n
This attribute specifies the amount of space to reserve for formatting the descriptive text. This
attribute helps you line up text on different model lines, and if the space reserved is longer than the
descriptive text, it permits formatting within the reserved space with the TEXTFMT attribute. If the
descriptive text is longer than the space reserved by the TEXTLEN attribute, the descriptive text is not
formatted and a warning message is issued.
TEXTSKIP=NO | YES
This attribute provides for skipping past the descriptive text when either the TEXTLOC=BEFORE and
the previous LSTCOL tag includes the NOENDATTR attribute, or TEXTLOC=AFTER and the current
LSTCOL tag includes the NOENDATTR attribute. If there is no other input field on the panel, the cursor
moves to the first input field.
NOENDATTR
This attribute specifies that no ending attribute character is placed after the data column.
NOENDATTR is ignored for the last data column on each model line.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PADC=NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
PAS=OFF | ON | %varname
This attribute controls the availability of the point-and-shoot function for this table field. You can
define this attribute as a variable name preceded by "%"
CSRGRP=NO | YES | n
When CSRGRP=YES, the conversion utility generates a cursor group number to be used for this table
column. When CSRGRP=n, the number provided is used for this field. The PAS attribute must be
specified as ON or %varname.
The conversion utility accepts the CSRGRP attribute for any table field definition. The CRSGRP
attribute is used at runtime for output fields only.
ATTRCHANGE=NO | YES | NEW
When ATTRCHANGE=YES or ATTRCHANGE=NEW, the conversion utility formats an additional entry
in the panel )ATTR section (that can apply to multiple list columns) instead of creating a unique
".ATTR(field-name)" entry in the )INIT section for this field With this option, multiple LSTCOL tags
with the same characteristic require fewer panel logic statements. ATTRCHANGE=NEW creates a new
entry. ATTRCHANGE=YES uses an existing entry, if possible.
COLSPACE=n
The COLSPACE attribute specifies the total number of bytes for the column width, including the
leading and trailing attributes, and any trailing blank following an input field. The use of the COLSPACE
attribute causes column heading text longer than the COLSPACE value (minus the attribute bytes) to
be flowed into multiple lines.
COLTYPE=CUA | ISPF | EE | VOI | LID
This attribute defines the attribute type to be applied to the table field. TYPE=CUA, the default, causes
the field to display using the standard CUA attribute.
VOI and LID are valid only when USAGE=OUT.
EE is valid when USAGE=IN or USAGE=BOTH.
Using a CUA attribute causes the field to appear in the associated color.
LSTCOL
336  z/OS: z/OS ISPF DTL Guide

## Page 369

When COLTYPE=ISPF, a non-CUA attribute is generated and you can specify the color, intensity, and
highlighting of the field using the COLOR, INTENS, and HILITE attributes. These attributes are not
valid for CUA types.
COLOR=WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute specifies the color of the field. You can define this attribute as a variable name preceded
by a percent (%) sign.
INTENS=HIGH | LOW | NON | %varname
This attribute defines the intensity of the field. You can define this attribute as a variable name
preceded by a percent (%) sign.
HILITE=USCORE | BLINK | REVERSE | %varname
This attribute specifies the extended highlighting attribute of the field. You can define this attribute as
a variable name preceded by a percent (%) sign.
CAPS=OFF | ON
When CAPS=ON, the data in the field is displayed in uppercase characters.
DISPLAY=YES | NO
This attribute specifies whether the data for the field is visible when the panel is displayed. This
attribute is used to allow fields to contain information you do not want to appear on the screen.
VARDCL=YES | NO
When VARDCL=NO the list column name is not checked to the declared variable information provided
with the VARCLASS and VARDCL tags.
column-heading
This is the text of the list column heading. If the length of the column-heading and the COLWIDTH
values are not equal, the greater of the two is used to determine column formatting width. If
the column-heading and the COLWIDTH attributes are omitted, the column formatting width is
determined by the TYPE value of the associated VARCLASS. If a VARCLASS TYPE value is not
available, the size of the column variable name (specified by the DATAVAR attribute) determines the
width.
The column-heading text placement over the column is determined by the FORMAT attribute value.
Comments
In conjunction with the LSTFLD tag, LSTCOL tags provide a means of defining a vertically scrollable list
display area that is made up of columns of data coming from ISPF table data. One or more ISPF )MODEL
section statements is built to display the fields defined by the LSTCOL tags. The use of LSTCOL tags
requires the use of the TBDISPL service in the application program.
If the ISPF panel width is smaller than the total width of the group of columns, columns that exceed the
panel width are clipped from the right. A warning message is issued if this condition occurs.
You can use the LINE attribute to format your table to display on multiple lines.
If NOENDATTR is not specified, the conversion utility generates a beginning and ending attribute for each
column of the table display )MODEL line. An additional blank is also inserted for fields with USAGE=IN or
BOTH if AUTOTAB=NO. This characteristic results in these conditions:
• When USAGE=OUT, 2 extra spaces are added to the defined column formatting width.
• When AUTOTAB=YES and USAGE=IN or BOTH, 2 extra spaces are added to the defined column
formatting width.
• When AUTOTAB=NO (the default) and USAGE=IN or BOTH, 3 extra spaces are added to the defined
column formatting width.
It is important that you allow for this extra space when designing your panel. The extra space is added to
the width value for the field as defined in the description of the COLWIDTH attribute.
When the maximum number of requested attributes for a panel is exceeded, the conversion utility issues
error message ISPC804E. The number of requested attributes includes attribute override entries. These
LSTCOL
Chapter 12. Tag reference  337

## Page 370

are .ATTR entries that are added by the conversion utility for attributes that are specified on CHOFLD,
DTACOL, DTAFLD, LSTCOL, and LSTFLD tags. If the same set of attributes is specified on multiple tags,
duplicate .ATTR entries are added by default. Adding the parameter ATTRCHANGE=YES to the tags
causes the compiler to instead add a single entry in the panel )ATTR section for each unique set of
attributes specified. The entry for a set of attributes is then shared by all tags that specify that set of
attributes.
Restrictions
• You must code the LSTCOL tag within a LSTFLD or LSTGRP tag. See “LSTFLD (List Field)” on page 341
for a complete description of this tag.
• Each LSTCOL definition should have a VARDCL definition associated with the variable value specified
with the DATAVAR attribute. See “VARDCL (Variable Declaration)” on page 449 for a complete
description of this tag.
• Only MODEL lines that actually are formatted with fields are written in the panel body. Thus, if some
LSTCOL entries specify LINE=1 and others specify LINE=3, but there are no LSTCOL entries for LINE 2,
only two MODEL lines are created.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
Table 47. The tags you can code within a LSTCOL definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SCRFLD “SCRFLD (Scrollable Field)” on page 413 Single No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is source file markup where the application panel contains a list field with five list columns. The
first three columns are defined as output-only, and are coded within the Subscriber Name list group. The
Number column is an input/output column, and it is coded within the Phone list group. The last column is
input-only, and it is coded within the Approved list group. This column requires input, so if it is not filled in,
the error message MSGG886 is displayed. The variable declarations and classes associated with the list
columns are also shown. Figure 131 on page 339 shows the formatted result of the application panel.
LSTCOL
338  z/OS: z/OS ISPF DTL Guide

## Page 371

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=namecls TYPE='char 15'>
<VARCLASS NAME=midcls  TYPE='char 1'>
<VARCLASS NAME=phoncls TYPE='char 12'>
<VARCLASS NAME=appcls  TYPE='char 1'>
  <XLATL FORMAT=upper>
  </XLATL>
  <CHECKL>
    <CHECKI TYPE=values PARM1=EQ PARM2='Y N'>
  </CHECKL>
<VARLIST>
  <VARDCL NAME=xfname  VARCLASS=namecls>
  <VARDCL NAME=xlname  VARCLASS=namecls>
  <VARDCL NAME=xmid    VARCLASS=midcls>
  <VARDCL NAME=xphone  VARCLASS=phoncls>
  <VARDCL NAME=xapp    VARCLASS=appcls>
</VARLIST>
<PANEL NAME=lstcola KEYLIST=keyltbl>Subscriber List
<TOPINST>Enter phone number, if missing,
(format - nnn-nnn-nnnn) and approved
indicator (y or n) for each person.
<AREA>
  <LSTFLD>
    <LSTGRP HEADLINE=yes>Subscriber Name
      <LSTCOL DATAVAR=xfname USAGE=out COLWIDTH=15>First Name
      <LSTCOL DATAVAR=xlname USAGE=out COLWIDTH=15>Last Name
      <LSTCOL DATAVAR=xmid   USAGE=out COLWIDTH=1>MI
    </LSTGRP>
    <LSTGRP>Phone
      <LSTCOL DATAVAR=xphone COLWIDTH=12>Number
    </LSTGRP>
    <LSTGRP>Approved
      <LSTCOL DATAVAR=xapp USAGE=in REQUIRED=yes
        COLWIDTH=1 MSG=msgg886>(Y or N)
    </LSTGRP>
  </LSTFLD>
</AREA>
<CMDAREA>
</PANEL>
                              Subscriber List                ROW 1 TO 3 OF 3
 Enter phone number, if missing, (format - nnn-nnn-nnnn) and approved
 indicator (y or n) for each person.
 --------- Subscriber Name ----------  Phone          Approved
 First Name       Last Name        MI  Number         (Y or N)
 Pete             Moss             P   919-555-4444   _
 Sally            Forth            N   ____________   _
 Melba            Toast            T   919-555-8888   _
 ****************************** BOTTOM OF DATA *****************************
 Command ===> ______________________________________________________________
  F1=Help        F2=Split       F3=Exit        F7=Backward     F8=Forward
  F9=Swap       F12=Cancel
Figure 131. List columns
To display the same table in a different format, we can change the LSTCOL tags for name to include the
LINE attribute. The DTL changes are reflected in the this example.
LSTCOL
Chapter 12. Tag reference  339

## Page 372

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=namecls TYPE='char 15'>
<VARCLASS NAME=midcls  TYPE='char 1'>
<VARCLASS NAME=phoncls TYPE='char 12'>
<VARCLASS NAME=appcls  TYPE='char 1'>
  <XLATL FORMAT=upper>
  </XLATL>
  <CHECKL>
    <CHECKI TYPE=values PARM1=EQ PARM2='Y N'>
  </CHECKL>
<VARLIST>
  <VARDCL NAME=xfname  VARCLASS=namecls>
  <VARDCL NAME=xlname  VARCLASS=namecls>
  <VARDCL NAME=xmid    VARCLASS=midcls>
  <VARDCL NAME=xphone  VARCLASS=phoncls>
  <VARDCL NAME=xapp    VARCLASS=appcls>
</VARLIST>
<PANEL NAME=lstcolb KEYLIST=keyltbl>Subscriber List
<TOPINST>Enter phone number, if missing,
(format - nnn-nnn-nnnn) and approved
indicator (y or n) for each person.
<AREA>
  <LSTFLD DIV=solid>
    <LSTGRP HEADLINE=yes>Subscriber Name
      <LSTCOL DATAVAR=xfname USAGE=out LINE=1 COLWIDTH=15>First Name
      <LSTCOL DATAVAR=xlname USAGE=out LINE=2 COLWIDTH=15>Last Name
      <LSTCOL DATAVAR=xmid   USAGE=out LINE=3 COLWIDTH=1>MI
    </LSTGRP>
    <LSTGRP>Phone
      <LSTCOL DATAVAR=xphone COLWIDTH=12>Number
    </LSTGRP>
    <LSTGRP>Approved
      <LSTCOL DATAVAR=xapp USAGE=in REQUIRED=yes
        COLWIDTH=1 MSG=msgg886>(Y or N)
    </LSTGRP>
  </LSTFLD>
</AREA>
<CMDAREA>
</PANEL>
Figure 132 on page 340 shows the formatted result of the application panel.
                              Subscriber List                ROW 1 TO 3 OF 3
 Enter phone number, if missing, (format - nnn-nnn-nnnn) and approved
 indicator (y or n) for each person.
 Subscriber Name  Phone          Approved
 First Name       Number         (Y or N)
 Last Name
 MI
 Pete             919-555-4444   _
 Moss
 P
 --------------------------------------------------------------------------
 Sally            ____________   _
 Forth
 N
 --------------------------------------------------------------------------
 Melba            919-555-8888   _
 Toast
 Command ===> ______________________________________________________________
  F1=Help        F2=Split       F3=Exit        F7=Backward     F8=Forward
  F9=Swap       F12=Cancel
Figure 132. List columns
LSTCOL
340  z/OS: z/OS ISPF DTL Guide

## Page 373

LSTFLD (List Field)
The LSTFLD tag defines an ISPF table display area that is made up of columns of data coming from ISPF
tables.
Syntax
<LSTFLD
RULES=
NONE
HORIZ
VERT
BOTH
ROWS=
NOSCAN
SCAN
%varname
DIV=
NONE
BLANK
SOLID
DASH
char
SCROLLVAR=scroll-variable
SCRVHELP=
NO
YES
scroll-help-panel-name
*scroll-help-message-id
%varname
*%varname
SCROLLTAB=
NO
YES SCRCAPS=
OFF
ON
ATTRCHANGE=
NO
YES
NEW
VARDCL=
YES
NO
>
</LSTFLD>
Parameters
RULES=NONE | HORIZ | VERT | BOTH
This attribute specifies the type of interior rules that are drawn in the table display being defined
within the LSTFLD tag. This applies to all the list columns within the context of this tag.
This attribute is supported by using the ISPF outline (L|R|O|U|Box|None) statement on panel
definition statements. However, the lines around fields are only visible on double-byte character
support terminals.
Note: Any list column field within the list field defining OUTLINE overrides the LSTFLD RULES value.
LSTFLD
Chapter 12. Tag reference  341

## Page 374

ROWS=NOSCAN | SCAN | %varname
This attribute provides support by TBDISPL of rows previously selected by the TBSARG service. If you
specify ROWS=SCAN, the conversion utility adds ROWS(SCAN) to the )MODEL line statement in the
generated ISPF panel.
If you specify ROWS=%varname, ROWS(&varname) is added to the )MODEL line. The application must
set the variable name to ALL or SCAN before the panel is displayed.
DIV=NONE | BLANK | SOLID | DASH | char
This attribute specifies the type of divider line to be added as the last line of a model set. If this
attribute is omitted or specified as NONE, the divider line is not generated. If this attribute is specified
as BLANK, a blank divider line is generated. You may specify either SOLID or DASH to produce a
visible divider line. When the GRAPHIC invocation option is specified, SOLID produces a solid line for
host display and DASH produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH
produce a dashed line. Alternately, you can specify a character or a character string of your choice.
The character or characters provided are replicated to the available width of the panel (or region) to
create the divider line.
If you have defined LSTCOL tags for all 8 of the available model lines, then the conversion utility
issues a message and does not generate any divider line.
SCROLLVAR=scroll-variable
This attribute specifies the name of a variable that the application uses to obtain scrolling information.
The scroll-variable must follow the standard naming convention described in “Rules for variable
names” on page 179.
If the attribute is specified, the conversion utility creates a scroll entry on the command line, providing
that the resulting command area allows at least 8 bytes for a command entry.
SCRVHELP=NO | YES | scroll-help-panel-name | *scroll-help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for the field specified with
the SCROLLVAR attribute.
When SCRVHELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a choice and no help is defined, the extended help panel is displayed. If
an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The scroll-help-panel-name must follow the standard naming convention described in “Rules for
variable names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
SCROLLTAB=NO | YES
When SCROLLTAB=YES, the cursor moves to the next input field when the user enters the last
character in the scroll amount field. If there is no other input field on the panel, the cursor moves to
the beginning of the command line.
SCRCAPS=OFF | ON
When SCRCAPS=ON, the data in the scroll field is displayed in uppercase characters.
ATTRCHANGE=NO | YES | NEW
When ATTRCHANGE=YES or ATTRCHANGE=NEW, the conversion utility formats an additional entry
in the panel )ATTR section (that can apply to multiple list columns) instead of creating a unique
".ATTR(field-name)" entry in the )INIT section for each field. With this option, multiple LSTCOL tags
with the same characteristics require fewer panel logic statements. ATTRCHANGE=NEW creates a
new entry. ATTRCHANGE=YES uses an existing entry, if possible.
Note: Any list column field within the list field defining ATTRCHANGE overrides the LSTFLD
ATTRCHANGE value.
LSTFLD
342  z/OS: z/OS ISPF DTL Guide

## Page 375

VARDCL=YES | NO
When VARDCL=NO, the list field name is not checked to the declared variable information provided
with the VARCLASS and VARDCL tags.
Note: Any list column field within the list field defining VARDCL overrides the LSTFLD VARDCL value.
Comments
The LSTFLD tag defines a scrollable list display area that is made up of columns of data coming from ISPF
table data. The conversion utility creates a )MODEL line at the bottom of the )BODY section of the panel
the list field is coded within.
The use of the LSTFLD tag causes all other tags that generate panel data and that are coded after the
LSTFLD end tag to be moved before the )MODEL statement. This is because ISPF does not allow any panel
body definition after the )MODEL statement.
When the maximum number of requested attributes for a panel is exceeded, the conversion utility issues
error message ISPC804E. The number of requested attributes includes attribute override entries. These
are .ATTR entries that are added by the conversion utility for attributes that are specified on CHOFLD,
DTACOL, DTAFLD, LSTCOL, and LSTFLD tags. If the same set of attributes is specified on multiple tags,
duplicate .ATTR entries are added by default. Adding the parameter ATTRCHANGE=YES to the tags
causes the compiler to instead add a single entry in the panel )ATTR section for each unique set of
attributes specified. The entry for a set of attributes is then shared by all tags that specify that set of
attributes.
Restrictions
• The LSTFLD tag requires an end tag.
• You must code the LSTFLD tag within an AREA, REGION, or PANEL definition. See “AREA (Area)” on
page 189, “REGION (Region)” on page 405, and “PANEL (Panel)” on page 376 for descriptions of these
tags.
• You can code only one list field on an application panel.
• You should code a CMDAREA on any panel that contains a LSTFLD definition. If you do not include the
CMDAREA tag, the conversion utility inserts one and issues a message, unless the PANEL tag specifies
CMDLINE=NO.
• You can use the SCROLLVAR attribute only once within a panel.
• The resulting scroll entry on the command line must leave at least 8 positions for the command entry
field.
• If you specify the SCRVHELP attribute, you must also specify the SCROLLVAR attribute.
Processing
Table 48. The tags you can code within a LSTFLD definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
LSTCOL “LSTCOL (List Column)” on page 332 Multiple No
LSTGRP “LSTGRP (List Group)” on page 345 Multiple No
LSTVAR “LSTVAR (List Variable)” on page 348 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
LSTFLD
Chapter 12. Tag reference  343

## Page 376

Examples
Here is an application panel in a source file markup that contains a list field with five list columns of data.
In addition, three list groups are defined within the list field. The first three list columns are output-only
columns. The fourth list column uses the default value both, which allows it to handle both input and
output data. The last list column is an input-only column, and input by the user is required. Figure 133 on
page 345 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=namecls TYPE='char 15'>
<VARCLASS NAME=midcls  TYPE='char 1'>
<VARCLASS NAME=phoncls TYPE='char 12'>
<VARCLASS NAME=appcls  TYPE='char 1'>
  <XLATL FORMAT=upper>
  </XLATL>
  <CHECKL>
    <CHECKI TYPE=values PARM1=EQ PARM2='Y N'>
  </CHECKL>
<VARLIST>
  <VARDCL NAME=xfname  VARCLASS=namecls>
  <VARDCL NAME=xlname  VARCLASS=namecls>
  <VARDCL NAME=xmid    VARCLASS=midcls>
  <VARDCL NAME=xphone  VARCLASS=phoncls>
  <VARDCL NAME=xapp    VARCLASS=appcls>
</VARLIST>
<PANEL NAME=lstfld3 KEYLIST=keyltbl>Subscriber List
<TOPINST>Enter phone number, if missing,
(format - nnn-nnn-nnnn) and approved
indicator (y or n) for each person.
<AREA>
  <LSTFLD SCROLLVAR=scrlamt SCRVHELP=scrhelp>
    <LSTGRP HEADLINE=yes>Subscriber Name
      <LSTCOL DATAVAR=xfname USAGE=out COLWIDTH=15>First Name
      <LSTCOL DATAVAR=xlname USAGE=out COLWIDTH=15>Last Name
      <LSTCOL DATAVAR=xmid   USAGE=out COLWIDTH=1>MI
    </LSTGRP>
    <LSTGRP>Phone
      <LSTCOL DATAVAR=xphone COLWIDTH=12>Number
    </LSTGRP>
    <LSTGRP>Approved
      <LSTCOL DATAVAR=xapp USAGE=in REQUIRED=yes
        COLWIDTH=1 MSG=msgg886>(Y or N)
    </LSTGRP>
  </LSTFLD>
</AREA>
<CMDAREA>
</PANEL>
LSTFLD
344  z/OS: z/OS ISPF DTL Guide

## Page 377

Subscriber List
 Enter phone number, if missing, (format – nnn-nnn-nnnn) and approved
 indicator (y or n) for each person.
 --------- Subscriber Name ----------  Phone          Approved
 First Name       Last Name        MI  Number         (Y or N)
 Pete             Moss             P   919-555-4444   _
 Sally            Forth            N   ____________   _
 Melba            Toast            T   919-555-8888   _
 ****************************** BOTTOM OF DATA ****************************
 Command ===> ___________________________________________ Scroll ===> ____
  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward
  F9=Swap       F12=Cancel
Figure 133. List field 
LSTGRP (List Group)
The LSTGRP tag defines a heading for a single column or multiple columns within a list field.
Syntax
<LSTGRP
HEADLINE=
NO
YES
DASH
ALIGN=
CENTER
START
END
>
column-group-heading
</LSTGRP>
Parameters
HEADLINE=NO | YES | DASH
This attribute specifies whether the heading text is padded to span the width of the group heading not
occupied by the text. This provides a visual indication of the columns that belong to a group heading.
You must specify YES or DASH to produce the visible indicator. When the GRAPHIC invocation option
is specified, YES produces a solid line for host display and DASH produces a dashed line. When
NOGRAPHIC is specified, both YES and DASH produce a dashed line.
ALIGN=CENTER | START | END
This attribute specifies how the list group heading is formatted. If you do not specify this attribute, or
if you specify ALIGN=CENTER, then the heading is centered over multiple columns or a variable model
line, or left-justified over a single column.
When ALIGN=START, the list group heading is left-justified. When ALIGN=END, the list group heading
is right-justified.
LSTGRP
Chapter 12. Tag reference  345

## Page 378

column-group-heading
The heading is placed above the column group in the nonscrollable part of the list field. The heading
must fit on one line above the column or columns in the group. If column-group-heading text is longer
than the formatted width of the column or columns in the group, it is truncated. The column-group-
heading appears on the line immediately above the group of columns.
If you do not specify column-group-headings for any of the columns within the group, the conversion
utility reserves the area where the heading would be displayed and fill it with blanks. If the column-
group-heading is not specified but HEADLINE=YES is specified, the heading contains only a dashed
line.
Comments
The LSTGRP tag defines a heading for a single column or multiple columns within a list field. You can use
the LSTGRP tag to group columns in a list field together under a single heading that applies to all of the
columns. You create the columns using the LSTCOL or LSTVAR tag.
The list field can contain other columns that do not belong to the list column group. Only the LSTCOL or
LSTVAR definitions nested within the LSTGRP tag belong to the group.
There must be at least one LSTCOL tag, nested LSTGRP tag, or LSTVAR tag defined within a column group.
The column formatting widths, and the gutters between them, define how much space is allocated for the
group heading. If this space is less than the space needed for the group heading, the conversion utility
truncates the heading. If the LSTGRP definition contains only one LSTCOL tag, and the ALIGN attribute is
not specified, the group heading is left-justified over the column.
You can use the LSTGRP tag to specify multiple lines of single column headings or multiple lines of
multiple column headings.
Restrictions
• The LSTGRP tag requires an end tag.
• You must code the LSTGRP tag within a LSTFLD definition or another LSTGRP definition. See “LSTFLD
(List Field)” on page 341 for a complete description of the LSTFLD tag.
• You can code multiple LSTGRP tags within a LSTFLD definition.
• A LSTGRP definition must contain a nested LSTCOL, LSTVAR, or LSTGRP tag, otherwise the conversion
utility issues an error.
• The nested tags LSTCOL definitions must include at least one data column from the first displayable
model line.
Processing
Table 49. The tags you can code within a LSTGRP definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LSTCOL “LSTCOL (List Column)” on page 332 Multiple Yes
LSTGRP “LSTGRP (List Group)” on page 345 Multiple No
LSTVAR “LSTVAR (List Variable)” on page 348 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
LSTGRP
346  z/OS: z/OS ISPF DTL Guide

## Page 379

Table 49. The tags you can code within a LSTGRP definition  (continued)
Tag Reference Usage Required
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is source file markup where the application panel contains a list field with six list columns. The first
three columns are placed under a common group, as are the last two columns. Also, for each of the first
three columns, a second-level group heading is used in place of list column headings. This technique
provides a blank space between the group headings and the data columns. Figure 134 on page 348 shows
the formatted result of the application panel.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=namecls TYPE='char 12'>
<VARCLASS NAME=midcls  TYPE='char 1'>
<VARCLASS NAME=yearcls TYPE='char 9'>
<VARCLASS NAME=semcls  TYPE='char 2'>
<VARLIST>
  <VARDCL NAME=xfname  VARCLASS=namecls>
  <VARDCL NAME=xlname  VARCLASS=namecls>
  <VARDCL NAME=xmid    VARCLASS=midcls>
  <VARDCL NAME=xyear   VARCLASS=yearcls>
  <VARDCL NAME=sem1    VARCLASS=semcls>
  <VARDCL NAME=sem2    VARCLASS=semcls>
</VARLIST>
<PANEL NAME=lstgrp WIDTH=66 KEYLIST=keyltbl>Class Roster
<AREA>
  <LSTFLD>
    <LSTGRP HEADLINE=yes>Student Name
    <LSTGRP>Last
      <LSTCOL DATAVAR=xlname USAGE=out COLWIDTH=12>
    </LSTGRP>
    <LSTGRP>First
      <LSTCOL DATAVAR=xfname USAGE=out COLWIDTH=12>
    </LSTGRP>
    <LSTGRP>M
      <LSTCOL DATAVAR=xmid   USAGE=out COLWIDTH=1>
    </LSTGRP>
    </LSTGRP>
    <LSTGRP>Class
    <LSTGRP>Year
      <LSTCOL DATAVAR=xyear USAGE=out COLWIDTH=9>
    </LSTGRP>
    </LSTGRP>
    <LSTGRP HEADLINE=yes>Grade
      <LSTCOL DATAVAR=sem1 COLWIDTH=2>Sem 1
      <LSTCOL DATAVAR=sem2 COLWIDTH=2>Sem 2
    </LSTGRP>
  </LSTFLD>
</AREA>
<CMDAREA>
</PANEL>
LSTGRP
Chapter 12. Tag reference  347

## Page 380

Class Roster           ROW 1 TO 6 OF 6
 ------- Student Name --------  Class      -- Grade ---
 Last          First         M  Year
                                           Sem 1  Sem 2
 Scott         Dean          T  Junior     A      B+
 Lewis         Dana          L  Freshman   B+     B
 Roy           Sergio        J  Post-Grad  D      D
 Romero        Maria         C  Post-Grad  A      A
 Spencer       Alan          M  Freshman   A      B
 Zhou          Alex          B  Senior     C+     B
 ************************ BOTTOM OF DATA ************************
Command ===> ___________________________________________________
 F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward
 F9=Swap     F12=Cancel
Figure 134. List group
LSTVAR (List Variable)
The LSTVAR tag defines a )MODEL section variable model line displayed in the ISPF table display area of a
panel.
Syntax
<LSTVAR DATAVAR=variable-model-name
LINE=n
>
column-heading
</LSTVAR>
Parameters
DATAVAR=variable-model-name
This is the data that occupies the column. The variable-model-name value must be a variable model
line name (without a leading % sign).
LINE=n
This attribute provides the application the ability to place a LSTVAR model variable on different model
lines. ISPF defines the range of lines as 1 to 8. The default is 1. Headings are generated on multiple
lines to match the LSTVAR field placement.
column-heading
This is the text of the model variable heading.
Comments
In conjunction with the LSTFLD and LSTCOL tags, LSTVAR tags provide a means of defining a vertically
scrollable list display area that is made up of data coming from ISPF tables. One or more ISPF )MODEL
LSTVAR
348  z/OS: z/OS ISPF DTL Guide

## Page 381

section statements are built to display the fields defined by the LSTVAR tags. The use of LSTVAR tags
requires the use of the TBDISPL service in the application program.
The application must place valid data in the variable model line before the panel is displayed.
You can use the LINE attribute to format your table to display on multiple lines.
Restrictions
• You must code the LSTVAR tag within a LSTFLD tag. See “LSTFLD (List Field)” on page 341 for a
complete description of this tag.
• Only MODEL lines that are not blank fields are written in the panel body. Thus, if one LSTVAR entries
specifies LINE=1 and another specifies LINE=3, but there are no entries for LINE 2, only two MODEL
lines are created.
Processing
Table 50. Tags you can code within an LSTVAR definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is source file markup where the application panel contains a list field with five list columns and
2 variable model lines. The first three columns are defined as output-only, and are coded within the
Subscriber Name list group. The Number column is an input/output column, and it is coded within the
Phone list group. The last column is input-only, and it is coded within the Approved list group. This
column requires input, so if it is not filled in, the error message MSGG886 is displayed. The variable
declarations and classes associated with the list columns are also shown.
Note: The variable model lines are shown in the formatted output to illustrate the formatting process. The
application must provide valid values for these variables before the panel is displayed.
Figure 135 on page 350 shows the formatted result of the application panel.
LSTVAR
Chapter 12. Tag reference  349

## Page 382

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=namecls TYPE='char 15'>
<VARCLASS NAME=midcls  TYPE='char 1'>
<VARCLASS NAME=phoncls TYPE='char 12'>
<VARCLASS NAME=appcls  TYPE='char 1'>
 <XLATL FORMAT=upper>
 </XLATL>
<CHECKL>
 <CHECKI TYPE=values PARM1=EQ PARM2='Y N'>
  </CHECKL>
<VARLIST>
  <VARDCL NAME=xfname  VARCLASS=namecls>
  <VARDCL NAME=xlname  VARCLASS=namecls>
  <VARDCL NAME=xmid    VARCLASS=midcls>
  <VARDCL NAME=xphone  VARCLASS=phoncls>
  <VARDCL NAME=xapp    VARCLASS=appcls>
 </VARLIST>
<PANEL NAME=lstvar KEYLIST=keyltbl>Subscriber List
<TOPINST>Enter phone number, if missing,
(format - nnn-nnn-nnnn) and approved
indicator (y or n) for each person.
<AREA>
  <LSTFLD>
    <LSTVAR datavar=xmodelv1>Variable model line at top
 <LSTGRP HEADLINE=yes>Subscriber Name
      <LSTCOL DATAVAR=xfname USAGE=out line=2 COLWIDTH=15>First Name
      <LSTCOL DATAVAR=xlname USAGE=out line=2 COLWIDTH=15>Last Name
      <LSTCOL DATAVAR=xmid   USAGE=out line=2 COLWIDTH=1>MI     </LSTGRP>
 <LSTGRP>Phone
 <LSTCOL DATAVAR=xphone line=2 COLWIDTH=12>Number
    </LSTGRP>
    <LSTGRP>Approved
 <LSTCOL DATAVAR=xapp USAGE=in line=2 REQUIRED=yes
        COLWIDTH=1 MSG=msgf886>(Y or N)
    </LSTGRP>
<LSTVAR datavar=xmodelv2 line=3>Variable model line at bottom
 </LSTFLD>
</AREA>
<CMDAREA>
</PANEL>
                              Subscriber List
 Enter phone number, if missing, (format - nnn-nnn-nnnn)&cont.
 and approved
 indicator (y or n) for each person.
 --------- Subscriber Name ----------  Phone
Approved
 Variable model line at top
 First Name       Last Name        MI  Number         (Y or N)
 Variable model line at bottom
&XMODELV1
 _______________  _______________  _   ____________   _
&XMODELV2
 Command ===> ______________________________________________________________
  F1=Help        F2=Split       F3=Exit        F7=Backward     F8=Forward
  F9=Swap       F12=Cancel
Figure 135. List variable
LSTVAR
350  z/OS: z/OS ISPF DTL Guide

## Page 383

M (Mnemonic)
The M tag defines a single character to be used as a mnemonic selection for action bar choices.
Note: The M tag is accepted on pull-down choices in order to support existing DTL source files that use it.
However, it no longer affects the displayed panel.
Syntax
<M> mnemonic-character
</M>
Parameters
mnemonic-character
The single-byte character following the mnemonic start tag specifies the mnemonic for the action bar
choice. The mnemonic-character must be a single-byte alphabetic or numeric character; double-byte
characters are not allowed.
If you want the mnemonic to be a character that is not part of the normal choice text, follow the
choice text with the mnemonic character specified within parenthesis. This convention is particularly
useful when you have a large number of choices, which makes it difficult to choose a unique
mnemonic for each choice. For example, if you had the action bar choice Add, and the characters A
and d were already used on other choices in the same action bar, you could choose another character
for your mnemonic:
              <abc>Add(<m>B)
In this case B becomes the mnemonic for Add.
Comments
Unless you have specified MNEMGEN=NO on the AB tag, the conversion utility automatically selects a
mnemonic character for each action bar choice for SBCS conversions. The character selected as the
mnemonic is the first alphabetic or numeric character in the choice description not previously used as a
mnemonic for that set of choices.
Restrictions
When the conversion utility automatically generates mnemonics, the M tag selection is processed first,
and if the specified mnemonic is valid, the automatic mnemonic generation is not used for that choice.
If the specified mnemonic character is invalid, or a duplicate of a previously used mnemonic character
(either specified or automatically selected), a message is issued and an attempt is made to select a
different mnemonic character.
When processing DBCS conversions or when MNEMGEN=NO is coded on the AB tag, automatic mnemonic
character selection is disabled and mnemonic characters are only specified by the M tag. The use of
mnemonics should be consistent for all choices in an action bar:
• Code the M tag within the text following the ABC tags.
• Each mnemonic chosen must be unique. The conversion utility issues a message and discards duplicate
mnemonics.
• If mnemonics are used for any action bar choice, they should be used for all of the choices. The
conversion utility issues a message if any choice in a group does not have a mnemonic.
M
Chapter 12. Tag reference  351

## Page 384

Processing
None.
Examples
Here is an example where all of the action bar choices have been coded to show the use of the M tag.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 sysem>
  <!entity sampbody system>)>
&sampvar1;
<PANEL NAME=m1 KEYLIST=keylxmp>Library Card Registration
<AB>
<ABC><M>File
  <PDC>Add Entry
    <ACTION RUN=add>
  <PDC>Delete Entry
    <ACTION RUN=delete>
  <PDC>Update Entry
    <ACTION RUN=update>
  <PDC>Exit
    <ACTION RUN=exit>
<ABC><M>Search
  <PDC CHECKVAR=whchsrch MATCH=1>Search on name
    <ACTION SETVAR=whchsrch VALUE=1>
    <ACTION RUN=search>
  <PDC CHECKVAR=whchsrch MATCH=2>Search on card number
    <ACTION SETVAR=whchsrch VALUE=2>
    <ACTION RUN=search>
<ABC><M>Help
  <PDC>Extended Help...
    <ACTION RUN=exhelp>
  <PDC>Keys Help...
    <ACTION RUN=keyshelp>
</AB>
&sampbody;
</PANEL>
MSG (Message)
The MSG tag defines a message within a message member.
MSG
352  z/OS: z/OS ISPF DTL Guide

## Page 385

Syntax
<MSG SUFFIX=message-suffix-number
HELP= help-panel-name
%varname
*
MSGTYPE=
INFO
WARNING
ACTION
CRITICAL
%varname
LOCATION=
AREA
MODAL
MODAL(L)
MODELESS
MODELESS(L)
%varname
DISP= KANA
NOKANA ALARM=
NO
YES
%varname
ABBREV=
NONE
KEYWORD
VALUE
BOTH
FORMAT=
FLOW
ASIS
SMSG=short-message-text
>
message-text </MSG>
Parameters
SUFFIX=message-suffix-number
This attribute specifies the suffix of the message. The suffix consists of either 1 numeric character
(0-9) or a numeric character (0-9) and an optional alpha suffix character as defined for ISPF
messages, which is added to the MSGMBR message-member-name to form the ISPF message ID.
Each mes sage -suffix -number  within a message member must be unique. Attempts to define duplicate
suffixes result in a warning message and the duplicate MSG is ignored.
HELP=help-panel-name | %varname | *
Specifies the name of the help panel that is associated with this message and that is displayed if the
user requests help for the message.
If you specify a help panel, ISPDTLC generates ".HELP=help-panel-name" (or ".HELP=&varname" or
".HELP=*") in the ISPF message ID definition. If you don't specify a help panel, no help is available for
the message.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
MSGTYPE=INFO | WARNING | ACTION | CRITICAL | %varname
This attribute specifies the severity of the message. ISPF displays INFO messages without an alarm.
ISPF displays WARNING, ACTION, and CRITICAL messages with an alarm.
MSG
Chapter 12. Tag reference  353

## Page 386

ACTION and CRITICAL message types are used to identify the most severe errors. This level of error
usually requires a user response. ISPF always displays CRITICAL messages in a pop-up. ACTION
messages are displayed based on the value of the LOCATION attribute.
The %varname value specifies that the value INFO, WARNING, ACTION, or CRITICAL is provided in
the named variable by the application before issuing the message.
The conversion utility changes INFO to .TYPE=NOTIFY when formatting the message member.
ISPF recognizes message types (.TYPE=) of NOTIFY, WARNING, ACTION, and CRITICAL. ISPF
uses the TYPE value specified in conjunction with the value of .WINDOW to determine the display
characteristics of the message. The .WINDOW value is generated from the value specified for the
LOCATION attribute. For more information on ISPF messages, refer to z/OS ISPF Dialog Developer's
Guide and Reference.
LOCATION=AREA | MODAL | MODAL(L) | MODELESS | MODELESS(L) | %varname
This attribute specifies how the message is displayed.
LOCATION=AREA (the default) specifies that the message is to appear in the panel message area.
However, if the text of the message exceeds the length of the panel message area, ISPF displays the
message in a pop-up.
LOCATION=MODAL specifies that the message is to appear in a pop-up which requires a user
response. The conversion utility generates .WINDOW=RESP in the ISPF message definition.
LOCATION=MODAL(L) specifies that the long message is to appear in a pop-up which requires a user
response. The conversion utility generates .WINDOW=LRESP in the ISPF message definition.
LOCATION=MODELESS specifies that the message is to appear in a pop-up which does not require a
user response. The conversion utility generates .WINDOW=NORESP in the ISPF message definition.
LOCATION=MODELESS(L) specifies that the long message is to appear in a pop-up which does not
require a user response. The conversion utility generates .WINDOW=LNORESP in the ISPF message
definition.
LOCATION=%varname specifies that the value AREA, MODAL, or MODELESS is provided in
the named variable by the application before issuing the message. The conversion utility
generates .WINDOW=&VARNAME in the ISPF message definition.
DISP=KANA | NOKANA
This attribute specifies the addition of either the KANA or NOKANA keyword to the message control
information.
ALARM=NO | YES | %varname
This attribute controls the use of the alarm when the message is displayed.
ALARM=%varname specifies that the value YES or NO is provided in the named variable by the
application before issuing the message.
ABBREV=NONE | KEYWORD | VALUE | BOTH
This attribute specifies the format of the message control information. You may abbreviate the
message control keyword, the message control keyword value, or both.
FORMAT=FLOW | ASIS
This attribute specifies the formatting of the message-text.
The default of FLOW means to flow the message text continuously within the WIDTH of the MSGMBR.
When FORMAT=ASIS, the generated message preserves embedded blanks, but drops leading or
trailing blanks.
SMSG=short-message-text
You can provide a short message of up to 24 bytes which ISPF displays in the short message area of
the panel.
The VARSUB tag is not supported within the short-message-text. If a substitution variable is required,
you may code "&amp;variable" to place the variable name in the message. A short-message-text
MSG
354  z/OS: z/OS ISPF DTL Guide

## Page 387

consisting of more than one word must be enclosed within quotation marks (" "). If the short-
message-text contains a single apostrophe ('), the conversion utility generates double apostrophes as
it does for message-text, as described for the next parameter (message-text).
The short message is not recommended by the CUA Architecture definition.
A short message cannot be created unless the message-text is also provided.
message-text
This is the text of the message. The message-text is placed in the long-message area of a message
file. The message-text is limited to 512 characters. The conversion utility truncates all message-text
after 512 characters and issues a warning message. If no message-text is coded, then no message is
generated.
Several characters within the long message area have a special meaning to ISPF. If you use the
apostrophe within message-text, the conversion utility generates double apostrophes so the single
apostrophe is displayed when ISPF issues the message. If you use the ampersand (&) within the long
message, it must be coded as "&amp" followed by a blank or semicolon to be interpreted as a literal
ampersand character (through ENTITY substitution).
For ISPF substitution variables, you should code the VARSUB tag. ISPF does not perform output
translation (specified in the associated VARCLASS tag) on ISPF runtime substitution variables.
See z/OS ISPF Dialog Developer's Guide and Reference for a description of the syntax rules you should
use for defining consistent messages.
Comments
The MSG tag defines a message within a message member. Each MSG definition within a message
member must have a unique mes sage -suffix -number .
Restrictions
• You must code the MSG tag within a MSGMBR definition. See “MSGMBR (Message Member)” on page
356 for a complete description of this tag.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
Processing
Table 51. Tags you can code within a MSG definition 
Tag Reference Usage Required
VARSUB “VARSUB (Variable Substitution)” on page 452 Multiple No
Examples
Here is markup that contains the message member MSGG88, which contains nine MSG definitions. The
text of messages MSGG883 and MSGG888 contain variable substitutions. Figure 136 on page 356 shows
the generated ISPF message member.
MSG
Chapter 12. Tag reference  355

## Page 388

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=msgcls TYPE='char 20'>
<VARLIST>
  <VARDCL NAME=phoneno VARCLASS=msgcls>
  <VARDCL NAME=cnum    VARCLASS=msgcls>
</VARLIST>
<MSGMBR NAME=msgg88>
  <MSG SUFFIX=1 disp=kana abbrev=keyword>Name must be alphabetic.
  <MSG SUFFIX=2 disp=nokana abbrev=value>Enter only number of days.
  <MSG SUFFIX=3 MSGTYPE=critical>The only rooms we have available
      are either SINGLE or DOUBLE.  Please call the manager of the hotel
      who will arrange equivalent lodging at another
      hotel in the area.  This is our mistake, and we will, of course,
      pick up the bill.  Please call collect <VARSUB VAR=phoneno>.
  <MSG SUFFIX=4 MSGTYPE=action LOCATION=modal abbrev=both>
      Please enter either BIGCHARGE, V I S T A, EZCARD, CHECK, or CASH.
  <MSG SUFFIX=5 MSGTYPE=warning LOCATION=modeless>Please enter your name.
  <MSG SUFFIX=6>Please enter Y or N.
  <MSG SUFFIX=7>Card number is a seven-digit number.
  <MSG SUFFIX=8 MSGTYPE=warning>The card number you entered,
       <VARSUB VAR=cnum> is not valid.
  <MSG SUFFIX=9>Message '9' contains embedded quotes.
</MSGMBR>
MSGG881 .T=NOTIFY KANA
'Name must be alphabetic.'
MSGG882 .TYPE=N NOKANA
'Enter only number of days.'
MSGG883 .TYPE=CRITICAL
'The only rooms we have available are either SINGLE or DOUBLE. Please call th' +
'e manager of the hotel who will arrange equivalent lodging at another hotel ' +
'in the area. This is our mistake, and we will, of course, pick up the bill. ' +
'Please call collect &PHONENO.'
MSGG884 .T=A .W=R
'Please enter either BIGCHARGE, V I S T A, EZCARD, CHECK, or CASH.'
MSGG885 .TYPE=WARNING .WINDOW=NORESP
'Please enter your name.'
MSGG886 .TYPE=NOTIFY
'Please enter Y or N.'
MSGG887 .TYPE=NOTIFY
'Card number is a seven-digit number.'
MSGG888 .TYPE=WARNING .ALARM=YES
'The card number you entered, &CNUM is not valid.'
MSGG889 .TYPE=NOTIFY
'Message '9'' contains embedded quotes.'
Figure 136. Messages
MSGMBR (Message Member)
The MSGMBR tag defines a message member.
Syntax
<MSGMBR NAME=message-member-name
CCSID=n
WIDTH=
76
68
> </MSGMBR>
MSGMBR
356  z/OS: z/OS ISPF DTL Guide

## Page 389

Parameters
NAME=message-member-name
This specifies the name of the message member, which also serves as the prefix for all identifiers of
messages within the member.
The message-member-name can be specified as a 3-7 character name, conforming to ISPF message
member standard naming convention. The last two positions must be numeric. The preceding
characters can be A-Z, a-z, or #, $, @.
Lowercase characters are translated to their uppercase equivalents.
If you specify NAME=*, the message-member-name is set to the input DTL source member name. If
multiple dialog element definitions have been combined within a single source file, then this notation
should be used for only one dialog element definition within the file. See “Dialog elements” on page 5
for a description of dialog element types created by the conversion utility.
The message-member-name is also used to build the name used for storing messages. For example,
if NAME=MSGA12, the default name used to store the message members is userid.MSGS(MSGA12).
This can be changed by specifying a message file on the conversion utility invocation panel. See
Chapter 10, “Using the conversion utility,” on page 151 for more information about ISPDTLC syntax.
for information about allocating a message library at run time, refer to the z/OS ISPF User's Guide Vol
I.
CCSID=n
CCSID specifies the coded-character-set identifier as defined by the Character Data Representation
Architecture. CCSID should be entered as a five-position numeric value. For more information on
using the CCSID attribute, refer to the z/OS ISPF Dialog Developer's Guide and Reference.
WIDTH=76 | 68
This attribute specifies the width of the formatted messages. When WIDTH=68, the resulting
messages are formatted entirely within a normal Edit or View screen.
Comments
The MSGMBR tag defines a message member. You can code multiple message members for a single
application.
The message-member-name is an explicit part of the identifier for messages coded in the message
member. Each message member contains multiple messages. You use the MSG tag to define\ messages
within a message member.
Restrictions
• The MSGMBR tag requires an end tag.
• You cannot code the MSGMBR tag within any other tag definition.
Processing
Table 52. Tags you can code within an MSGMBR definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
MSG “MSG (Message)” on page 352 Multiple Yes
Examples
MSGMBR
Chapter 12. Tag reference  357

## Page 390

Here is markup that defines the message member MSGM88, which contains nine MSG definitions. Figure
137 on page 358 shows the generated ISPF message member.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=msgcls TYPE='char 20'>
<VARLIST>
  <VARDCL NAME=phoneno VARCLASS=msgcls>
  <VARDCL NAME=cnum    VARCLASS=msgcls>
</VARLIST>
<MSGMBR NAME=msgm88>
  <MSG SUFFIX=1>Name must be alphabetic.
  <MSG SUFFIX=2>Enter only number of days.
  <MSG SUFFIX=3 MSGTYPE=critical>The only rooms we have available
  are either SINGLE or DOUBLE.  Please call the manager of the hotel
  who will arrange equivalent lodging at another
  hotel in the area.  This is our mistake, and we will, of course,
  pick up the bill.  Please call collect <VARSUB VAR=phoneno>.
  <MSG SUFFIX=4 MSGTYPE=action LOCATION=modal>Please enter either
  BIGCHARGE, V I S T A, EZCARD, CHECK, or CASH.
  <MSG SUFFIX=5 MSGTYPE=warning LOCATION=modeless>Please enter your name.
  <MSG SUFFIX=6>Please enter Y or N.
  <MSG SUFFIX=7>Card number is a seven-digit number.
  <MSG SUFFIX=8 MSGTYPE=warning>The card number you
  entered, <VARSUB VAR=cnum> is not valid.
  <MSG SUFFIX=9>Message '9' contains embedded quotes.
</MSGMBR>
MSGM881 .TYPE=NOTIFY
'Name must be alphabetic.'
MSGM882 .TYPE=NOTIFY
'Enter only number of days.'
MSGM883 .TYPE=CRITICAL
'The only rooms we have available are either SINGLE or DOUBLE. Please call th' +
'e manager of the hotel who will arrange equivalent lodging at another hotel ' +
'in the area. This is our mistake, and we will, of course, pick up the bill. ' +
'Please call collect &PHONENO.'
MSGM884 .TYPE=ACTION .WINDOW=RESP
'Please enter either BIGCHARGE, V I S T A, EZCARD, CHECK, or CASH.'
MSGM885 .TYPE=WARNING .WINDOW=NORESP
'Please enter your name.'
MSGM886 .TYPE=NOTIFY
'Please enter Y or N.'
MSGM887 .TYPE=NOTIFY
'Card number is a seven-digit number.'
MSGM888 .TYPE=WARNING
'The card number you entered, &CNUM is not valid.'
MSGM889 .TYPE=NOTIFY
'Message '9'' contains embedded quotes.'
Figure 137. Message member
NOTE (Note)
The NOTE tag defines a single-paragraph note within an information region.
NOTE
358  z/OS: z/OS ISPF DTL Guide

## Page 391

Syntax
<NOTE
NOSKIP INDENT=n
TYPE=
ET
CH
CT
FP
LEF
LI
NT
PT
SAC
TEXT
WASL
WT
COLOR= WHITE
RED
BLUE
GREEN
PINK
YELLOW
TURQ
%varname
INTENS=
HIGH
LOW
NON
%varname
HILITE= USCORE
BLINK
REVERSE
%varname
TEXT=alternate-note-heading
>
note-text </NOTE>
Parameters
NOSKIP
This attribute causes the note to be formatted without creating a blank line before the note.
INDENT=n
This attribute specifies that the note be indented from the current left margin.
TYPE= ET | CH | CT | FP | LEF | LI | NT | PT | SAC | TEXT | WASL | WT
This attribute defines the attribute type to be applied to the note heading. Using a CUA attribute
causes the text to appear in the associated color.
When TYPE=TEXT, a non-CUA attribute is generated and you can specify the color, intensity, and
highlighting with the COLOR, INTENS, and HILITE attributes. These attributes are not valid for CUA
types.
NOTE
Chapter 12. Tag reference  359

## Page 392

COLOR= WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute specifies the color of the note heading. You can define this attribute as a variable name
preceded by a percent (%) sign.
INTENS= HIGH | LOW | NON | %varname
This attribute defines the intensity of the note heading. You can define this attribute as a variable
name preceded by a percent (%) sign.
HILITE= USCORE | BLINK | REVERSE | %varname
This attribute specifies the extended highlighting attribute for the note heading. You can define this
attribute as a variable name preceded by a percent (%) sign.
TEXT=alternate-note-heading
This attribute provides a text string to replace the standard "Note:" heading.
note-text
This is the text of the note.
Comments
The NOTE tag defines a single-paragraph note within an information region. You can code the NOTE tag
anywhere within an INFO tag.
The text of the note formats as an implied paragraph, at the current left margin. The text "Note:" (or its
translated equivalent), or the alternate note heading, begins the paragraph and is aligned with the text of
a list item when you use it within a list.
Restrictions
• You must code the NOTE tag within an INFO definition. See “INFO (Information Region)” on page 317
for a complete description of this tag.
• You cannot nest a NOTE tag within another NOTE definition.
Processing
Table 53. Tags you can code within a NOTE definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is help panel markup that contains a note. Figure 138 on page 361 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=note DEPTH=20>Book / Periodical Search Help
<AREA>
<INFO>
  <P>This entry screen allows you to locate a desired
  book or periodical by entering the title in the entry field.
  <NOTE>If the item you are trying to locate is not
  in stock and you would like to reserve it, please see the
  librarian at the front desk.
</INFO>
</AREA>
</HELP>
NOTE
360  z/OS: z/OS ISPF DTL Guide

## Page 393

Book / Periodical Search Help
 This entry screen allows you to locate a desired
 book or periodical by entering the title in the
 entry field.
 Note: If the item you are trying to locate is
 not in stock and you would like to reserve it,
 please see the librarian at the front desk.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 138. Note
NOTEL (Note List)
The NOTEL tag defines a list of notes within an information region.
NOTEL
Chapter 12. Tag reference  361

## Page 394

Syntax
<NOTEL
COMPACT NOSKIP
SPACE=
NO
YES
INDENT=n
TYPE=
ET
CH
CT
FP
LEF
LI
NT
PT
SAC
TEXT
WASL
WT
COLOR= WHITE
RED
BLUE
GREEN
PINK
YELLOW
TURQ
%varname
INTENS=
HIGH
LOW
NON
%varname
HILITE= USCORE
BLINK
REVERSE
%varname
TEXT=alternate-note-heading
>
</NOTEL>
Parameters
COMPACT
This attribute causes the list to be formatted without a blank line between the list items.
NOSKIP
This attribute causes the list to format without creating a blank line before the first line of the list.
SPACE=NO | YES
The SPACE attribute controls the indentation space for the list item. When the SPACE attribute is not
specified on the LI tag, the SPACE attribute from the NOTEL tag is used to set the indentation space
for the nested LI tag item-text.
NOTEL
362  z/OS: z/OS ISPF DTL Guide

## Page 395

When SPACE=YES, the indentation is set to 3 spaces.
When SPACE=NO (or SPACE is not specified), the indentation is set to 4 spaces.
The SPACE attribute can be used to control the alignment of list items when the first word of some
list items is a DBCS word preceded by a shift-out character and the first word of other list items is an
SBCS word.
INDENT=n
This attribute specifies that the note list be indented from the current left margin.
TYPE= ET | CH | CT | FP | LEF | LI | NT | PT | SAC | TEXT | WASL | WT
This attribute defines the attribute type to be applied to the note heading. Using a CUA attribute
causes the text to appear in the associated color.
When TYPE=TEXT, a non-CUA attribute is generated and you can specify the color, intensity, and
highlighting with the COLOR, INTENS, and HILITE attributes. These attributes are not valid for CUA
types.
COLOR= WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute specifies the color of the note heading. You can define this attribute as a variable name
preceded by a percent (%) sign.
INTENS= HIGH | LOW | NON | %varname
This attribute defines the intensity of the note heading. You can define this attribute as a variable
name preceded by a percent (%) sign.
HILITE= USCORE | BLINK | REVERSE | %varname
This attribute specifies the extended highlighting attribute of the note heading. You can define this
attribute as a variable name preceded by a percent (%) sign.
TEXT=alternate-note-heading
This attribute provides a text string to replace the standard "Notes:" heading.
Comments
The NOTEL tag defines a numbered list of notes. You can code the NOTEL tag anywhere within an INFO
tag.
The first line of the note list formats with the word "Notes:" (or its translated equivalent) or the alternate-
note-heading.
Use the LI tag to denote each list item. See “LI (List Item)” on page 325 for more information on the LI
tag.
Restrictions
• You must code the NOTEL tag within an INFO definition. See “INFO (Information Region)” on page 317
for a complete description of this tag.
• You cannot nest a NOTEL tag within a NOTEL definition.
Processing
Table 54. Tags you can code within a NOTEL definition 
Tag Reference Usage Required
LI “LI (List Item)” on page 325 Multiple No
LP “LP (List Part)” on page 330 Multiple No
NOTEL
Chapter 12. Tag reference  363

## Page 396

Examples
Here is help panel markup that contains a multiple notes. Notice the numbered format for the content of
the notes, which is different from the format generated with the NOTE or NT tag. A P tag is nested within
the NOTEL definition to provide an additional paragraph of note text. Figure 139 on page 364 shows the
formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=notel DEPTH=20>Book / Periodical Search Help
<AREA>
<INFO>
  <P>This entry screen allows you to locate a desired
  book or periodical by entering the title in the entry field.
  <NOTEL>
    <LI>If the item you are trying to locate is not
        in stock and you would like to reserve it, please see the
        librarian at the front desk.
    <LI>If the librarian is not there, please do not yell for help.
    <P>This is a library!
  </NOTEL>
</INFO>
</AREA>
</HELP>
           Book / Periodical Search Help
 This entry screen allows you to locate a desired
 book or periodical by entering the title in the
 entry field.
 Notes:
 1. If the item you are trying to locate is
    not in stock and you would like to
    reserve it, please see the librarian at
    the front desk.
 2. If the librarian is not there, please do
    not yell for help.
    This is a library!
   F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 139. NOTEL
NT (Note)
The NT tag defines a single- or multiple-paragraph note within an information region.
NT
364  z/OS: z/OS ISPF DTL Guide

## Page 397

Syntax
<NT
NOSKIP INDENT=n
TYPE=
ET
CH
CT
FP
LEF
LI
NT
PT
SAC
TEXT
WASL
WT
COLOR= WHITE
RED
BLUE
GREEN
PINK
YELLOW
TURQ
%varname
INTENS=
HIGH
LOW
NON
%varname
HILITE= USCORE
BLINK
REVERSE
%varname
TEXT=alternate-note-heading
>
note-text
</NT>
Parameters
NOSKIP
This attribute causes the note to be formatted without creating a blank line before the note.
INDENT=n
This attribute specifies that the note be indented from the current left margin.
TYPE= ET | CH | CT | FP | LEF | LI | NT | PT | SAC | TEXT | WASL | WT
This attribute defines the attribute type to be applied to the note heading. Using a CUA attribute
causes the text to appear in the associated color.
When TYPE=TEXT, a non-CUA attribute is generated and you can specify the color, intensity, and
highlighting with the COLOR, INTENS, and HILITE attributes. These attributes are not valid for CUA
types.
NT
Chapter 12. Tag reference  365

## Page 398

COLOR= WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute specifies the color of the note heading. You can define this attribute as a variable name
preceded by a percent (%) sign.
INTENS= HIGH | LOW | NON | %varname
This attribute defines the intensity of the note heading. You can define this attribute as a variable
name preceded by a percent (%) sign.
HILITE= USCORE | BLINK | REVERSE | %varname
This attribute specifies the extended highlighting attribute of the note heading. You can define this
attribute as a variable name preceded by a percent (%) sign.
TEXT=alternate-note-heading
This attribute provides a text string to replace the standard "Note:" heading.
note-text
This is the text of the note. You can use the P tag to code additional paragraphs of text.
Comments
The NT tag defines a single- or multiple-paragraph note within an information region. You can code the NT
tag anywhere within an INFO definition.
The text of the note formats as an indented block. The block of text is indented seven spaces from the
current left margin. The text "Note:" (or its translated equivalent), or the alternate note heading, begins
the paragraph. The note aligns with the text of a list item when you code it within a list.
Restrictions
• The NT tag requires an end tag.
• You must code the NT tag within an INFO definition. See “INFO (Information Region)” on page 317 for a
complete description of this tag.
• You can nest text tags such as paragraphs and lists within a note, but you cannot nest NT and NOTE
tags.
Processing
Table 55. Tags you can code within an NT definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
NT
366  z/OS: z/OS ISPF DTL Guide

## Page 399

Table 55. Tags you can code within an NT definition  (continued)
Tag Reference Usage Required
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains a multiple-paragraph note. Notice the indented format for the
content of the note, which is different from the format generated with the NOTE tag. A P tag is nested
within the NT definition to provide an additional paragraph of note text. Figure 140 on page 367 shows the
formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=nt DEPTH=20>Book / Periodical Search Help
<AREA>
<INFO>
  <P>This entry screen allows you to locate a desired
  book or periodical by entering the title in the entry field.
  <NT>If the item you are trying to locate is not
  in stock and you would like to reserve it, please see the
  librarian at the front desk.
  <P>If the librarian is not there, please do not yell for help.
  This is a library!
  </NT>
</INFO>
</AREA>
</HELP>
           Book / Periodical Search Help
 This entry screen allows you to locate a desired
 book or periodical by entering the title in the
 entry field.
 Note: If the item you are trying to locate is
       not in stock and you would like to
       reserve it, please see the librarian at
       the front desk.
       If the librarian is not there, please do
       not yell for help.  This is a library!
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 140. NT
OL (Ordered List)
The OL tag defines an ordered list of items within an information region.
OL
Chapter 12. Tag reference  367

## Page 400

Syntax
<OL
COMPACT NOSKIP
SPACE=
NO
YES
INDENT=n TEXT=OL-heading-text
> </OL>
Parameters
COMPACT
This attribute causes the list to be formatted without a blank line between the list items.
NOSKIP
This attribute causes the list to format without creating a blank line before the first line of the list.
SPACE=NO | YES
The SPACE attribute controls the indentation space for the list item. When the SPACE attribute is not
specified on the LI tag, the SPACE attribute from the OL tag is used to set the indentation space for the
nested LI tag item-text.
When SPACE=YES, the indentation is set to 3 spaces. When SPACE=NO (or SPACE is not specified),
the indentation is set to 4 spaces.
The SPACE attribute can be used to control the alignment of list items when the first word of some list
items is a DBCS word preceded by a shift-out
INDENT=n
This attribute specifies that the list be indented from the current left margin.
TEXT=OL-heading-text
This attribute causes the list to format with a heading line containing the OL-heading-text.
Comments
The OL tag defines an ordered list of items within an information region. You use ordered lists to indicate a
set of sequential items or steps. You can code the OL tag anywhere within an information region.
Ordered lists are formatted as indented lists, with sequential numbers or letters at the left margin of the
list items. Nested lists (lists embedded within other lists) indent four spaces to the right of the left margin
of the list that contains them.
Note: The SPACE attribute does not affect the indentation of nested lists.
The conversion utility adds a blank line before the first item in the list.
Sequential numbers or letters, depending on the nesting level of the ordered list precede the list items.
The levels are:
1. Level 1: 1., 2., 3., . . .
2. Level 2: a., b., c., . . .
3. Level 3: 1), 2), 3), . . .
4. Level 4: a), b), c), . . .
Any additional levels repeat the sequence from level 1.
Panels formatted with the DBCS option use uppercase alphabetic characters for the even-numbered
nesting levels.
Use the LI tag to denote each list item. See “LI (List Item)” on page 325 for more information on the LI
tag.
OL
368  z/OS: z/OS ISPF DTL Guide

## Page 401

Restrictions
• The OL tag requires an end tag.
• You must code the OL tag within an INFO definition. See “INFO (Information Region)” on page 317 for a
complete description of this tag.
Processing
Table 56. Tags you can code within an OL definition 
Tag Reference Usage Required
LI “LI (List Item)” on page 325 Multiple No
LP “LP (List Part)” on page 330 Multiple No
Examples
Here is help panel markup that contains two ordered lists and a paragraph. The second ordered list and
the paragraph are nested within the first list. Figure 141 on page 370 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=ol DEPTH=22 WIDTH=60>Widget Assembly Help
<AREA>
<INFO>
  <P>To assemble your new Widget, you should:
  <OL>
    <LI>Attach the gizmo flexure component to the
    main steering mechanism of the doohickey.
      <OL COMPACT>
        <LI>If slot A fits snugly on retaining
        pin B, proceed to step 2.
        <LI>If slot A does not fit snugly on
        retaining pin B, throw the Widget away
        and buy a new one.
      </OL>
    <LI>Use a screwdriver to turn the power drive unit on.
    <LI>Stand back and watch the fun!
      <P>Wake up the kids and call the neighbors, they won't
      want to miss it!
  </OL>
</INFO>
</AREA>
</HELP>
OL
Chapter 12. Tag reference  369

## Page 402

Widget Assembly Help
 To assemble your new Widget, you should:
 1.  Attach the gizmo flexure component to the main
     steering mechanism of the doohickey.
     a.  If slot A fits snugly on retaining pin B, proceed
         to step 2.
     b.  If slot A does not fit snugly on retaining pin B,
         throw the Widget away and buy a new one.
 2.  Use a screwdriver to turn the power drive unit on.
 3.  Stand back and watch the fun!
     Wake up the kids and call the neighbors, they won't
     want to miss it!
  F1=Help        F3=Exit        F5=Exhelp      F6=Keyshelp
  F7=PrvTopic    F8=NxtTopic   F10=PrvPage    F11=NxtPage
 F12=Cancel
Figure 141. Ordered lists
P (Paragraph)
The P tag defines a paragraph of text within an information region.
Syntax
<P
COMPACT INTENSE=varname INDENT=n OFFSET=n
SPACE=
NO
YES
>
paragraph-text </P>
Parameters
COMPACT
This attribute causes the paragraph to format without a blank line before the paragraph.
INTENSE=varname
This attribute supplies a variable name that must contain a valid value for the INTENS keyword. The
entire paragraph is controlled by this value. For example, if the variable contains the value NON, the
paragraph is not visible.
INDENT=n
This attribute specifies that the paragraph be indented from the current left margin.
OFFSET=n
This attribute specifies that the formatted text following the first line of the paragraph should be
indented an additional n bytes.
SPACE= NO | YES
This attribute is used when processing <P> tags coded within ENTITY definitions. When the ENTITY
keyword SPACE is not specified, text following a paragraph tag within the ENTITY definition is
processed as coded by default. This may result in unwanted spaces between words in the paragraph,
which can be removed by specifying <p space=yes>.
paragraph-text
This is the text of the paragraph.
P
370  z/OS: z/OS ISPF DTL Guide

## Page 403

Comments
The P tag defines a paragraph of text within an information region. You can code the P tag anywhere
within an INFO definition.
Each paragraph formats as an unindented block of text. A blank line is added before the paragraph unless
the COMPACT attribute is specified.
Paragraphs within a list align with the text of the list item.
Restrictions
• You must code the P tag within an INFO definition. See “INFO (Information Region)” on page 317 for a
complete description of this tag.
Processing
Table 57. Tags you can code within a P definition 
Tag Reference Usage Required
ATTENTION “ATTENTION (Attention)” on page 198 Single No
CAUTION “CAUTION (Caution)” on page 205 Single No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
WARNING “WARNING (Warning)” on page 454 Single No
Examples
Here is help panel markup that contains four paragraphs. The first three paragraphs are coded within an
information region with a defined width of 40, so the text of the paragraphs is formatted according to this
width. The last paragraph is coded within an information region with no defined width, so the paragraph
text is formatted according to the width defined on the HELP tag. Figure 142 on page 372 shows the
formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=p DEPTH=22 WIDTH=60>P Tag Help
<AREA>
<INFO WIDTH=40>
  <P>Here's a paragraph.
  Lines are formatted to fill the width of the
  information region.
  <P>Here's another paragraph.
  Notice the line skip between the paragraphs.
  <P>Paragraphs are very versatile.
  You can use them within many other tags.
</INFO>
<INFO WIDTH=58>
  <P>The paragraphs above were formatted within an
  information region defined with a width of 40.
  This paragraph is formatted within the width specified
  for the panel.
</INFO>
</AREA>
</HELP>
P
Chapter 12. Tag reference  371

## Page 404

P Tag Help
 Here's a paragraph. Lines are formatted
 to fill the width of the information
 region.
 Here's another paragraph. Notice the
 line skip between the paragraphs.
 Paragraphs are very versatile. You can
 use them within many other tags.
 The paragraphs above were formatted within an information
 region defined with a width of 40. This paragraph is
 formatted within the width specified for the panel.
  F1=Help        F3=Exit        F5=Exhelp      F6=Keyshelp
  F7=PrvTopic    F8=NxtTopic   F10=PrvPage    F11=NxtPage
 F12=Cancel
Figure 142. Paragraphs
PANDEF (Panel Default)
The PANDEF tag defines default values for application panels.
PANDEF
372  z/OS: z/OS ISPF DTL Guide

## Page 405

Syntax
<PANDEF ID=pandef-id
HELP= help-panel-name
%varname
DEPTH= n
FIT
WIDTH= n
FIT
%varname
KEYLIST=key-list-name
KEYLTYPE=
PRIVATE
SHARED
APPLID=application-id
CCSID=n
WINDOW=
YES
NO
WINTITLE=window-title
APPTITLE=application-title PAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
EXPAND=xy
MERGESAREA=
NO
YES
ENTKEYTEXT=enter-key-text IMAPNAME= image-name
%varname
IMAPROW= n
%varname
IMAPCOL= n
%varname
TMARGIN=n BMARGIN=n
>
</PANDEF>
PANDEF
Chapter 12. Tag reference  373

## Page 406

Parameters
ID=pandef-id
This attribute defines the identifier for the panel default definition. The pandef-id is the value you
specify with the PANDEF attribute of PANEL tags that refer to the panel default.
The pandef-id must follow the standard naming convention described in “Rules for variable names”
on page 179.
HELP=help-panel-name | %varname
This attribute specifies the extended (panel help) help panel that displays when the user selects help
on an application panel that specifies the panel default.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
Specification of the HELP attribute cause ISPDTLC to generate ".HELP=help-panel-name" (or
".HELP=&varname") in the )INIT section during panel generation.
ISPF displays this panel when the application user requests help and the cursor is not on a panel field
that has its own field-level help specified. This help panel is also displayed when the user requests
extended help.
DEPTH=n | FIT
This attribute specifies a default depth value for an application panel that refers to this panel default.
See “PANEL (Panel)” on page 376, for more information.
WIDTH=n | FIT | %varname
This attribute specifies a default width value for an application panel that refers to this panel default.
See “PANEL (Panel)” on page 376, for more information.
KEYLIST=key-list-name
This attribute specifies the name of a key mapping list associated with panels that refer to this panel
default. See “KEYL (Key List)” on page 322 for more information.
KEYLTYPE= PRIVATE | SHARED
This attribute is used to add the SHARED keyword to the KEYLIST parameter of the )PANEL
statement. For more information about the )PANEL statement, refer to the z/OS ISPF Dialog
Developer's Guide and Reference.
APPLID=application-id
This attribute is used to add the application ID to the )PANEL statement. The application-id
overrides the KEYLAPPL invocation option value.
CCSID=n
This attribute specifies the default CCSID value for an application panel that refers to this panel
default. See “PANEL (Panel)” on page 376 for more information.
WINDOW=YES | NO
The WINDOW attribute is used to control the generation of the WINDOW keyword on the panel )BODY
section. The default is to create the WINDOW keyword. WINDOW=NO should be used when
WIDTH=%varname is also used to create a panel.
WINTITLE=window-title
This attribute is used to add a title on the pop-up window border. The attribute value is placed in the
ISPF ZWINTTL variable. The maximum length of the window-title text is the panel width minus 1.
APPTITLE=application-title
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PANDEF
374  z/OS: z/OS ISPF DTL Guide

## Page 407

PADC= NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
EXPAND=xy
This attribute adds the EXPAND(xy) attribute to the )BODY section of the panel. If only one character
is present, the second character is set to the same value. If the EXPAND attribute is present with
no value specified, the conversion utility uses a character from the range of low-order hex values
available for panel attributes. This removes an available character from possible use as a panel
attribute and may cause panel formatting errors.
MERGESAREA= NO | YES
This attribute controls an additional formatting step for panels with a single scrollable area. If the
entire contents of the scrollable area fit within a standard 24-line panel (allowing 2 lines for the
function keys display), and no input or output fields are found in the panel body following the location
of the scrollable area, the scrollable area content is moved into the panel body.
ENTKEYTEXT=enter-key-text
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAME=image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPROW=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPCOL=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
TMARGIN=n
This attribute provides the number of blank lines to format at the top of the panel as a top margin.
BMARGIN=n
This attribute provides the number of blank lines to format at the bottom of the panel as a bottom
margin.
Comments
The PANDEF tag defines default values for application panels.
PANEL tags refer to the panel default by specifying the pandef-id definition as the PANDEF attribute value.
When a PANEL tag refers to a panel default, the values specified by the associated PANDEF tag are used
for the panel unless overridden by values specified in the PANEL tag definition.
The PANEL tag can override any of the PANDEF values by specifying that value within its own definition.
Thus, it is possible for a PANEL tag to select certain default values from the panel default and override
others.
See “PANEL (Panel)” on page 376 for more information.
You can code multiple panel defaults for an application. Each panel default should have a unique pandef-
id.
Restrictions
• You cannot code the PANDEF tag within any other tag definition.
PANDEF
Chapter 12. Tag reference  375

## Page 408

• You must code the PANDEF tag before you code any PANEL tag that refers to it.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
• EXPAND can operate only when there are no trailing attributes on the line to be expanded. Panel lines
formatted as part of a horizontal region require the use of attributes for field alignment. Therefore, the
EXPAND feature is functional only for panel sections built within a vertical (or default) region that is not
part of any horizontal region.
Processing
None.
Examples
Here is source file markup that contains two panel default definitions. The application panels panel1 and
panel2 both refer to the panel default pandef1. The panel panel1 uses all of the defined default values
and panel2 uses only the default DEPTH and WIDTH values, and overrides the default HELP and KEYLIST
values by specifying those values in the PANEL definition. The third application panel, panel3 refers to all
of the default values specified in the panel default pandef2.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 sysem>
  <!entity sampbody system>)>
&sampvar1;
<PANDEF ID=panldef1 DEPTH=20 WIDTH=76 HELP=helpaaa KEYLIST=keylxmp>
<PANDEF ID=panldef2 DEPTH=22 WIDTH=70 HELP=morehlp>
<PANEL NAME=pandef1 PANDEF=panldef1>First Panel
&sampbody;
</PANEL>
<PANEL NAME=pandef2 PANDEF=panldef1
 HELP=morehlp KEYLIST=keyltbl>Second Panel
&sampbody;
</PANEL>
<PANEL NAME=pandef3 PANDEF=panldef2>Third Panel
&sampbody;
</PANEL>
<HELP NAME=helpaaa>Help panel "helpaaa"
<AREA>
<INFO WIDTH=48>
<P>This is PANDEF help panel "helpaaa"
</INFO>
</AREA>
</HELP>
<HELP NAME=morehlp>Help panel "morehlp"
<AREA>
<INFO WIDTH=48>
<P>This is PANDEF help panel "morehlp"
</INFO>
</AREA>
</HELP>
PANEL (Panel)
The PANEL tag defines an application panel.
PANEL
376  z/OS: z/OS ISPF DTL Guide

## Page 409

Syntax
<PANELNAME=panel-name
HELP= help-panel-name
%varname
PANDEF=pandef-id
DEPTH=
22
n
FIT
WIDTH=
76
n
FIT
%varname
KEYLIST=key-list-nameKEYLIST options
CURSOR=cursor-field
CSRINDEX=index-valueCSRPOS=position-value
CCSID=n MENU PRIME TUTOR
WINDOW=
YES
NO
WINTITLE=window-title
APPTITLE=application-titlePAD= NULLS
USER
char
%varname
PADC= NULLS
USER
char
%varname
OUTLINE=
NONE
L
R
O
U
BOX
%varname
EXPAND=xy
MSGLINE=
YES
NO
TITLINE=
YES
NO CMDLINE=
YES
NO
ATTRUSE=
NO
YES
ALL
ENDATTR=
DEFAULT
TEXT
TYPE=
NOGUI
GUI
BOTH
SMSG=short-msg-fieldname
LMSG=long-msg-fieldnameASIS ACTBAR
MERGESAREA=
NO
YES PANELSTMT=
YES
NO
ENTKEYTEXT=enter-key-text
IMAPNAME=image-name
%varname
IMAP options TMARGIN=n
BMARGIN=n
ERRORCHECK=
NO
YES
ZUP=zup-id
ZCONT=zcont-id
AUTONRET=
NO
YES
AUTOTCMD=
NO
YES
PROC
>
panel-title-text
</PANEL>
KEYLIST options
PANEL
Chapter 12. Tag reference  377

## Page 410

KEYLTYPE=
PRIVATE
SHARED
APPLID=application-id
IMAP options
IMAPROW= n
%varname
IMAPCOL= n
%varname
Parameters
NAME=panel-name
This attribute specifies the name of the panel. The panel-name is used in the ISPF DISPLAY or
TBDISPL service call. The panel-name is also used as the panel ID, which the user can display. The
panel-name must follow the standard naming convention described in “Rules for variable names” on
page 179.
If you specify NAME=*, the panel-name is set to the input DTL source member name. If multiple
dialog element definitions have been combined within a single source file, then this notation should
be used for only one dialog element definition within the file. See “Dialog elements” on page 5 for a
description of dialog element types created by the conversion utility.
The panel-name is used to build the panel output file name in which the conversion utility stores the
converted panel. The default is “userid.PANELS(panel-name)”.
You can specify the output panel library file name of your choice on the invocation panel for the
conversion utility, or in the conversion utility profile as DDname DTLPAN for batch (or command syntax
invocation) processing.
If the SCRIPT option has been specified, the panel-name is also used to build the file name in
which the conversion utility stores the image of the panel. The default name is “userid.SCRIPT(panel-
name)”.
You can specify the output SCRIPT library file name of your choice on the invocation panel for the
conversion utility, or in the conversion utility profile as DDname DTLSCR for batch (or command syntax
invocation) processing.
See Chapter 10, “Using the conversion utility,” on page 151 for complete information on invocation
syntax.
HELP=help-panel-name | %varname
This attribute specifies the name of a defined extended (panel help) help panel. It identifies the help
text that is associated with the panel definition.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
Specification of the HELP attribute causes ISPDTLC to generate ".HELP=help-panel-name" (or
".HELP=&varname") in the )INIT section during panel generation.
ISPF displays this panel when the application user requests help and the cursor is not on a panel field
that has its own field-level help specified. This help panel is also displayed when the user requests
extended help.
PANDEF=pandef-id
This attribute specifies a defined panel default. The pandef-id value is the identifier specified on the
PANDEF tag. You can override any of the defaults from this PANDEF tag by specifying that attribute
on the PANEL tag. See “PANDEF (Panel Default)” on page 372 for information about defining panel
defaults.
PANEL
378  z/OS: z/OS ISPF DTL Guide

## Page 411

DEPTH=22 | n | FIT
This attribute defines the depth of the panel. The default depth is 22 when WINDOW=YES or 24 when
WINDOW=NO. When the panel is displayed in a pop-up, ISPF adds two lines to the DEPTH value you
specify to accommodate the borders at the top and bottom of the pop-up.
The value specified for the depth is the depth of the entire panel including the panel title, the action
bar, the function key area, the message area, any scrollable areas, and the command area.
The maximum depth is 62 and the minimum depth is 5. If the DEPTH value is less than the
minimum value allowed or exceeds the maximum value allowed, the conversion utility issues a
warning message and sets the depth to the default.
The depth defined should be large enough to include all formatted text and input/output fields as well
as the function key area, message area, any scrollable areas, and the command area. If the depth
specified is not large enough to include these panel elements, ISPF overlays with the function keys if
the function key display is on, or with the message area if the message is not displayed in a pop-up.
If DEPTH=FIT, The conversion utility formats the panel using a depth of 22. When formatting is
completed the DEPTH value is reset to the minimum depth used or to 5 if the formatted panel
contains less than 5 lines.
If the DEPTH value exceeds the maximum allowed to display the panel on the device, ISPF issues an
error message at run time.
WIDTH=76 | n | FIT | %varname
This attribute defines the width (in characters) of the panel. The default width is 76 when
WINDOW=YES or 80 when WINDOW=NO. When the panel is displayed in a pop-up, ISPF adds 4
to the WIDTH value you specify to accommodate the left and right borders of the pop-up.
The value specified for the width is the width of the entire panel (or region), including the margins.
The maximum width is 160 and the minimum width is 16.
Because there is a minimum margin width of 1 character on each side of the panel text, the effective
width for text for a panel defined with WIDTH=76 is a maximum of 74 characters.
If the WIDTH value is less than the minimum value allowed or exceeds the maximum value allowed,
ISPDTLC issues a warning message and sets the width to the default.
If WIDTH=FIT or WIDTH=%varname, the conversion utility formats the panel using the maximum
available width as determined from the LRECL value of the output panel file.
If WIDTH=FIT, when formatting is completed the WIDTH value is reset to the minimum width used or
to 16 if the formatted panel is less than 16 characters wide.
If WIDTH=%varname, when formatting is completed the WIDTH keyword on the )BODY panel
statement is set to the variable name. WINDOW=NO must also be coded on the PANEL tag in order to
use %varname.
Note: Panels that have the width specified as a variable cannot be preprocessed.
If WIDTH value exceeds the maximum allowed to display the panel on the device, ISPF issues an error
message at run time.
KEYLIST=key-list-name
This attribute specifies the name of the key mapping list associated with the panel.
If you do not specify a key-list-name in a PANEL definition or an associated PANDEF definition, the
ISPF-provided key list (ISPKYLST) is used. For information about defining key mapping lists, see
“KEYL (Key List)” on page 322. For information about the ISPF-provided key list, refer to the z/OS ISPF
Dialog Developer's Guide and Reference.
KEYLTYPE=PRIVATE | SHARED
This attribute is used to add the SHARED keyword to the KEYLIST parameter of the )PANEL
statement. For information about the )PANEL statement, refer to the z/OS ISPF Dialog Developer's
Guide and Reference. The KEYLTYPE attribute is ignored if you have not provided the KEYLIST
attribute as part of the PANEL tag definition or as part of an associated PANDEF tag definition.
PANEL
Chapter 12. Tag reference  379

## Page 412

APPLID=application-id
This attribute is used to add the application ID to the )PANEL statement. The application-id
overrides the KEYLAPPL invocation option value. The APPLID attribute is ignored if you have not
provided the KEYLIST attribute as part of the PANEL tag definition or as part of an associated
PANDEF tag definition.
CURSOR=cursor-field
This attribute, together with CSRINDEX and CSRPOS, controls the initial placement of the cursor when
the ISPF displays the panel. You can specify cursor -field  as the value of:
• The NAME attribute of a CHOICE tag (for multiple-choice selection fields)
• The DATAVAR attribute of the CHOFLD tag.
• The DATAVAR attribute of a DTAFLD tag
• The DATAVAR attribute of a LSTCOL tag
• The NAME attribute of a SELFLD tag (for single-choice selection fields).
The cursor can also be placed on the command area, when it is defined for a panel with the CMDAREA
tag. Use the ISPF-reserved name cmdarea as the value for cursor -field  to place the cursor on the
command area.
CSRINDEX=index-value
This attribute, together with CURSOR and CSRPOS, controls the placement of the cursor when
ISPF displays a table display panel. This attribute may be specified only when the CURSOR
attribute refers to a list column.
CSRINDEX specifies the row in the )MODEL section where ISPF places the cursor when it displays
the panel.
CSRPOS=position-value
This attribute, together with CURSOR and CSRINDEX, controls the placement of the cursor when
ISPF displays the panel. This attribute may be specified only when the CURSOR attribute refers to
a data field, list column, or the command area.
CSRPOS specifies the number of byte positions into the entry field that ISPF places the cursor
when it displays the panel.
The first position of a field is denoted by 1. The maximum position that you can specify is the
length of the underlying data.
If the value specified for this attribute is not valid, the default (1) is used.
CCSID=n
This attribute specifies the coded-character-set identifier as defined by the Character Data
Representation Architecture. CCSID should be entered as a five-position numeric value. For more
information about using the CCSID attribute, refer to the z/OS ISPF Dialog Developer's Guide and
Reference.
MENU
This attribute specifies that the panel is an ISPF menu selection or edit model selection panel. This
type of panel does not allow a table display.
PRIME
This attribute is used together with MENU to specify a primary option menu.
TUTOR
This attribute specifies that the panel title be formatted with the word Tutorial (or its translated
equivalent) on each end of the title line, similar to ISPF tutorial panels.
WINDOW=YES | NO
The WINDOW attribute is used to control the generation of the WINDOW keyword on the panel )BODY
section. The default is to create the WINDOW keyword. WINDOW=NO should be used when
WIDTH=%varname is also used to create a panel.
PANEL
380  z/OS: z/OS ISPF DTL Guide

## Page 413

WINTITLE=window-title
This attribute is used to add a title on the pop-up window border. The attribute value is placed in the
ISPF ZWINTTL variable. The maximum length of the window-title text is the panel width minus 1.
APPTITLE=application-title
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PADC= NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
EXPAND=xy
This attribute adds the EXPAND(xy) attribute to the )BODY section of the panel. If only one character
is present, the second character is set to the same value. If the EXPAND attribute is present with
no value specified, the conversion utility uses a character from the range of low-order hex values
available for panel attributes. This removes an available character from possible use as a panel
attribute and may cause panel formatting errors.
MSGLINE=YES | NO
This attribute controls the provision for a long message line in the generated panel. When
MSGLINE=NO, the blank line for the long message is not added to the panel )BODY section. It is
the panel designer's responsibility to ensure that critical panel areas are positioned so that the long
message does not inhibit use of the resulting panel.
TITLINE=YES | NO
This attribute controls the generation of the panel title line. When TITLINE=NO, the panel title is not
added to the generated panel. This option is provided for applications that format a panel title as part
of a dynamic area. It is the panel designer's responsibility to ensure that the resulting panel meets
CUA requirements.
CMDLINE=YES | NO
This attribute controls the automatic generation of the command area on option menu panels and
table display panels. When CMDLINE=NO, the command area is not automatically added to panels
that do not include a CMDAREA tag within the panel definition.
ATTRUSE=NO | YES | ALL
This attribute controls the assignment of panel attributes within the range of x‘01’ through x‘3F’.
When ATTRUSE=YES or ATTRUSE=ALL, attributes for use in dynamic areas supplied by the ATTR tag
can be assigned low-order hex values normally used by the conversion utility.
When ATTRUSE=YES, all of the attributes specified by the ATTR tag plus the required attributes used
by the conversion utility must fit in the defined range of x‘01’ through x‘2F’.
When ATTRUSE=ALL, all of the attributes specified by the ATTR tag plus the required attributes used
by the conversion utility must fit in the defined range of x‘01’ through x‘3F’.
ENDATTR=DEFAULT | TEXT
This attribute specifies that when the last attribute on any panel body line is "normal text" (CUA), it is
replaced by the default "text" (ISPF) attribute. The effect is to force any text on subsequent lines not
preceded by another attribute from the normal text color to blue.
TYPE=NOGUI | GUI | BOTH
This attribute specifies that the panel is used for either host display, by a client that is using the
JSON API, or both. When NOGUI is specified, for example, the panel language control statements that
enable check boxes and radio buttons are not added to the generated panel.
PANEL
Chapter 12. Tag reference  381

## Page 414

SMSG=short-msg-fieldname
This attribute provides the name of the field where the short message is to be placed. The shor t -ms g- 
fieldname  must follow the standard naming convention described in “Rules for variable names” on
page 179.
LMSG=long-msg-fieldname
This attribute provides the name of the field where the long message is to be placed. The long-ms g- 
fieldname  must follow the standard naming convention described in “Rules for variable names” on
page 179.
ASIS
This attribute specifies that the command and long message fields are to appear on the display as
specified in the generated panel definition. When ASIS is specified, any user request specified on the
Settings panel, or by setting the system variable ZPLACE is ignored.
ACTBAR
This attribute causes the action bar information for the panel to be generated, overriding the
NOACTBAR invocation option.
MERGESAREA=NO | YES
This attribute controls an additional formatting step for panels with a single scrollable area. If the
entire contents of the scrollable area fits within a standard 24-line panel (allowing two lines for the
function keys display), and no input or output fields are found in the panel body following the location
of the scrollable area, the scrollable area content is moved into the panel body.
PANELSTMT=YES | NO
This attribute controls the creation of the )PANEL statement. You can use this attribute to create a
panel without keylist interaction.
ENTKEYTEXT=enter-key-text
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAME=image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPROW=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
IMAPCOL=n | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
TMARGIN=n
This attribute provides the number of blank lines to format at the top of the panel as a top margin.
BMARGIN=n
This attribute provides the number of blank lines to format at the bottom of the panel as a bottom
margin.
ERRORCHECK=NO | YES
This attribute specifies whether error checking logic is added to the end of the )PROC section. The
extra logic prevents exit from the panel if any errors are present.
IF (.MSG ¬= ' ')
   &ZVERB = ' '
   .RESP = ENTER
ZUP=zup-id
This attribute provides the name of the Tutorial panel to be assigned to the ZUP variable. It is valid
only when the TUTOR attribute has been specified.
ZCONT=zcontid
This attribute provides the name of the Tutorial panel to be assigned to the ZCONT variable. It is valid
only when the TUTOR attribute has been specified.
PANEL
382  z/OS: z/OS ISPF DTL Guide

## Page 415

AUTONRET=NO | YES
This attribute specifies whether the .NRET = OFF panel statement is added to the )PROC section as
part of the AUTOTYPE logic. When YES is specified, '.NRET = OFF' is the first AUTOTYPE panel logic
statement created in the )PROC section.
AUTOTCMD=NO | YES | PROC
This attribute specifies whether the command field is refreshed during AUTOTYPE processing. When
YES is specified, the command field name (normally ZCMD) is included with the AUTOTYPE variables
added to the REFRESH statement in the )REINIT section of the panel. When PROC is specified, a
REFRESH statement that references the command field name is included in the )PROC section of the
panel. The REFRESH statement is inserted after the PANEXIT statement that invokes the AUTOTYPE
panel exit.
panel-title-text
This is the text of the panel title.
Panel titles should be used when an application can display more than one panel. The panel-title-text
is centered within the width defined for the panel in accordance with CUA rules. If the panel-title-text
is wider than the WIDTH specified, the title is truncated from the right and an ellipsis (...) is appended.
Two lines are reserved for the panel title and for a blank line between the panel title and the rest of
the panel body.
Comments
The PANEL tag defines an application panel.
Tags coded within a PANEL definition (between the PANEL start tag and end tag) define the content of the
panel.
Restrictions
• When the MENU attribute is specified, the LSTFLD tag cannot be nested under the PANEL tag.
• The PANEL tag requires an end tag.
• You cannot code a PANEL tag within any other tag definition.
• The PANEL definition must contain at least one of these tags:
– BOTINST (See “BOTINST (Bottom Instruction)” on page 204)
– DA (See “DA (Dynamic Area)” on page 250)
– DTAFLD (See “DTAFLD (Data Field)” on page 275)
– GA (See “GA (Graphic Area)” on page 295)
– INFO (See “INFO (Information Region)” on page 317)
– LSTFLD (See “LSTFLD (List Field)” on page 341)
– PNLINST (See “PNLINST (Panel Instruction)” on page 396)
– SELFLD (See “SELFLD (Selection Field)” on page 421)
– TOPINST (See “TOPINST (Top Instruction)” on page 441)
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
• EXPAND can operate only when there are no trailing attributes on the line to be expanded. Panel lines
formatted as part of a horizontal region require the use of attributes for field alignment. Therefore, the
EXPAND feature is functional only for panel sections built with a vertical (or default) region that is not
part of any horizontal region.
PANEL
Chapter 12. Tag reference  383

## Page 416

Processing
Table 58. Tags you can code within a PANEL definition 
Tag Reference Usage Required
AB “AB (Action Bar)” on page 179 Single No
AREA “AREA (Area)” on page 189 Multiple No
BOTINST “BOTINST (Bottom Instruction)” on page 204 Multiple No
CMDAREA “CMDAREA (Command Area)” on page 237 Single No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
DA “DA (Dynamic Area)” on page 250 Multiple No
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
DTACOL “DTACOL (Data Column)” on page 269 Multiple No
DTAFLD “DTAFLD (Data Field)” on page 275 Multiple No
GA “GA (Graphic Area)” on page 295 Single No
GENERATE “GENERATE (Generate)” on page 298 Multiple No
GRPHDR “GRPHDR (Group Header)” on page 300 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
INFO “INFO (Information Region)” on page 317 Multiple No
LSTFLD * “LSTFLD (List Field)” on page 341 Single No
PNLINST “PNLINST (Panel Instruction)” on page 396 Multiple No
REGION “REGION (Region)” on page 405 Multiple No
SELFLD “SELFLD (Selection Field)” on page 421 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
TEXTLINE “TEXTLINE (Text Line)” on page 439 Single No
TOPINST “TOPINST (Top Instruction)” on page 441 Multiple No
Note: Tags marked with * are not valid within an ISPF selection menu panel.
Examples
Here is application panel markup that contains an action bar, a top instruction, two selection fields, and a
command area. The PANEL KEYLIST attribute specifies a key mapping list, which is displayed below the
command area. Figure 143 on page 386 shows the formatted result.
PANEL
384  z/OS: z/OS ISPF DTL Guide

## Page 417

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=selcls TYPE='CHAR 2'>
<VARLIST>
  <VARDCL NAME=loc  VARCLASS=selcls>
  <VARDCL NAME=mode VARCLASS=selcls>
</VARLIST>
<PANEL NAME=panel HELP=trvlhlp KEYLIST=keylxmp
  DEPTH=22 WIDTH=60>Dream Vacation Guide
<AB>
  <ABC>File
    <PDC>Add Entry
        <ACTION RUN=add>
    <PDC>Delete Entry
        <ACTION RUN=delete>
    <PDC>Update Entry
        <ACTION RUN=update>
    <PDC>Exit
        <ACTION RUN=exit>
  <ABC>Help
    <PDC>Extended Help...
        <ACTION RUN=exhelp>
    <PDC>Keys Help...
        <ACTION RUN=keyshelp>
</AB>
<TOPINST>Choose one of the following exotic locations and
your preferred mode of travel, then press Enter.
<AREA>
  <REGION DIR=horiz>
  <SELFLD NAME=loc PMTWIDTH=23 SELWIDTH=25>Exotic Location:
    <CHOICE>Athens, GA
    <CHOICE>Berlin, CT
    <CHOICE>Cairo, IL
    <CHOICE>Lizard Lick, NC
    <CHOICE>Paris, TX
    <CHOICE>Rome, NY
    <CHOICE>Venice, FL
  </SELFLD>
  <DIVIDER>
  <SELFLD NAME=mode PMTWIDTH=25 SELWIDTH=25>Travel Mode:
    <CHOICE>Boxcar
    <CHOICE>Hitchhike
    <CHOICE>Mule
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>
</PANEL>
<HELP NAME=trvlhlp>Sample help panel "trvlhelp"
<AREA>
<INFO WIDTH=48>
<P>This is help panel "trvlhlp"
</INFO>
</AREA>
</HELP>
PANEL
Chapter 12. Tag reference  385

## Page 418

File  Help
 ---------------------------------------------------------
                    Dream Vacation Guide
 Choose one of the following exotic locations and your
 preferred mode of travel, then press Enter.
 Exotic Location:            Travel Mode:
 __  1.  Athens, GA          __  1.  Boxcar
     2.  Berlin, CT              2.  Hitchhike
     3.  Cairo, IL               3.  Mule
     4.  Lizard Lick, NC
     5.  Paris, TX
     6.  Rome, NY
     7.  Venice, FL
 Command ===> ______________________________________________
  F1=Help     F2=Split      F3=Exit     F6=Keyshelp
  F9=Swap    F12=Cancel
Figure 143. Application panel
PARML (Parameter List)
The PARML tag defines a parameter list within an information region.
Syntax
<PARML
TSIZE=
10
S1 S2 ... Sn BREAK=
ALL
FIT
NONE
COMPACT SKIP INDENT=n
FORMAT=
START
CENTER
END
DIVEND=
NO
YES
SPLIT=
NO
YES
> </PARML>
Parameters
TSIZE=10 | S1 S2... Sn
This attribute defines the space allocated for the parameter term. The default is 10 characters. The
minimum TSIZE value is 0 and the maximum is 40.
When multiple TSIZE values are specified, a PT tag must be coded for each value. The sizes are
applied to the PT tags in the order the tags are encountered in the DTL source file.
BREAK=ALL | FIT | NONE
This attribute controls the formatting of the parameter terms and descriptions. If BREAK=ALL (the
default), every description is on the line below the term. If BREAK=FIT, the description is on the line
PARML
386  z/OS: z/OS ISPF DTL Guide

## Page 419

below the term if the term is longer than the TSIZE value. If BREAK=NONE, the term is on the same
line as the description, spilling into the description area if the length exceeds the TSIZE value.
COMPACT
This attribute causes the conversion utility to format the list without a blank line between the items.
SKIP
This attribute causes a blank line to be formatted before the first parameter term when COMPACT is
also specified.
INDENT=n
This attribute specifies that the parameter list be indented from the current left margin.
FORMAT=START | CENTER | END
This attribute specifies the placement of the PT tag text within the space specified by TSIZE. The
PARML tag FORMAT setting applies to all of the PT tags within the parameter list.
DIVEND=NO | YES
This attribute specifies whether a divider character is formatted following the PD tag text. When
DIVEND=YES, the formatting width of the PD text is reduced to allow space for the divider character.
SPLIT=NO | YES
This attribute controls the format of the last PT tag in a multiple PT tag group. It is used only
when BREAK=ALL or when BREAK=FIT and the PT tag text length exceeds the TSIZE value. When
SPLIT=YES, the text following the last PT tag in the PT group (typically one or two dashes) is placed in
front of the first line of the formatted PD tag text. The SPLIT setting on a PARML tag applies to all of
the PT tag groups within the parameter list.
Comments
The PARML tag defines a parameter list within an information region.
Parameter lists are similar to definition lists. They involve three tags: PARML (parameter list) and a
matching end tag, PT (parameter term), and PD (parameter description). As in definition lists, the term tag
defines a term, and the definition tag defines the description associated with the term. The PD tag must
immediately follow the PT tag that it is associated with.
Parameter lists can occur anywhere in an information region; you can nest them within other lists, and you
can nest other lists within parameter lists.
Restrictions
• The PARML tag requires an end tag.
• You must code the PARML tag within an INFO definition. See “INFO (Information Region)” on page 317
for a complete description of this tag.
Processing
Table 59. Tags you can code within a PARML definition 
Tag Reference Usage Required
PD “PD (Parameter Description)” on page 388 Multiple No
PLDIV “PLDIV (Parameter List Divider)” on page 394 Multiple No
PT “PT (Parameter Term)” on page 400 Multiple No
PTDIV “PTDIV (Parameter Term Divider)” on page 402 Multiple No
PARML
Chapter 12. Tag reference  387

## Page 420

Examples
Here is help panel markup that contains two parameter lists. The second parameter list is nested within
the second parameter description of the first list. Figure 144 on page 388 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=parmls DEPTH=22>Part Number Code Help
<AREA>
<INFO>
  <P>Valid part numbers consist of a three-digit
  number followed by a 2-character suffix.
  <PARML TSIZE=6>
    <PT>123
    <PD>The first three digits represent
    the lot number of the part.
    <PT>AA
    <PD>The 2-character suffix represents the
    department the part originated from.
    The valid suffixes are:
      <PARML BREAK=none COMPACT>
        <PT>TO
        <PD>Tools
        <PT>EL
        <PD>Electrical
        <PT>ME
        <PD>Mechanical
      </PARML>
  </PARML>
</INFO>
</AREA>
</HELP>
              Part Number Code Help
 Valid part numbers consist of a three-digit
 number followed by a 2-character suffix.
 123
       The first three digits represent the lot
       number of the part.
 AA
       The 2-character suffix represents the
       department the part originated from. The
       valid suffixes are:
       TO        Tools
       EL        Electrical
       ME        Mechanical
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 144. Parameter list
PD (Parameter Description)
The PD tag defines a parameter description in a parameter list.
Syntax
<PD>
parameter-description </PD>
PD
388  z/OS: z/OS ISPF DTL Guide

## Page 421

Parameters
parameter-description
This is the text of the parameter description.
Comments
The PD tag defines a parameter description in a parameter list.
Restrictions
• You must code the PD tag within a PARML definition. See “PARML (Parameter List)” on page 386 for a
complete description of this tag.
• Each PD tag must be paired with a PT tag. You can specify only one PD tag for each PT tag within a
parameter list. The PD tag must immediately follow the PT tag it is associated with.
Processing
Table 60. Tags you can code within a PD definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains a parameter list with three PD definitions. Figure 145 on page
390 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=pd DEPTH=20>Help for Ordering Parts
<AREA>
<INFO>
  <P>Use one of the following codes when ordering
PD
Chapter 12. Tag reference  389

## Page 422

a part number from inventory:
  <PARML TSIZE=5>
    <PT>ST
    <PD>Indicates that the part
    order is for stock replenishment.
    <PT>CU
    <PD>Indicates that the part
    order is for immediate customer shipment.
    <PT>EL
    <PD>Indicates that the part
    order is for shipment to an external location.
  </PARML>
</INFO>
</AREA>
</HELP>
             Help for Ordering Parts
 Use one of the following codes when ordering a
 part number from inventory:
 ST
      Indicates that the part order is for stock
      replenishment.
 CU
      Indicates that the part order is for
      immediate customer shipment.
 EL
      Indicates that the part order is for
      shipment to an external location.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 145. Parameter descriptions
PDC (Pull-Down Choice)
The PDC tag defines a pull-down choice for an action bar pull-down.
PDC
390  z/OS: z/OS ISPF DTL Guide

## Page 423

Syntax
<PDC
HELP=
NO
YES
help-panel-name
*help-message-id
%varname
*%varname
UNAVAIL=unavail-variable-name
CHECKVAR=check-variable-name
MATCH=
1
match-string
ACC1=key1 ACC2=key2 ACC3=key3
>
pull-down-description-text
</PDC>
Parameters
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for a pull-down choice
selection.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a choice and no help is defined, the extended help panel is displayed. If
an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
Note: This attribute is valid only when the SELFLD tag has been specified with TYPE=MULTI.
UNAVAIL=unavail-variable-name
This attribute specifies the name of a variable that is used by ISPF to determine the availability of the
pull-down choice. When the variable value is 1, the pull-down choice is unavailable.
The unavail-variable-name must follow the standard naming convention described in “Rules for
variable names” on page 179.
CHECKVAR=check-variable-name
This attribute specifies a variable whose value indicates whether or not the pull-down choice is
preselected when the pull-down is displayed. If the value of the variable is equivalent to the match-
string you specify with the MATCH attribute, the pull-down choice appears preselected. Otherwise, it
does not. The check-variable-name must follow the standard naming convention described in “Rules
for variable names” on page 179.
PDC
Chapter 12. Tag reference  391

## Page 424

Note: Unlike selection fields, ISPF does not reset the check-variable-name to indicate the pull-down
choice the user selects. Therefore, you should code the SETVAR attribute in an ACTION tag associated
with the pull-down choices when the application needs to know which pull-down choice was selected.
MATCH=1 | match-string
This attribute defines the value that causes the pull-down choice to be preselected. The value of
variable specified by the CHECKVAR attribute is compared to the match-string value, and if they
are equal, the pull-down choice appears preselected.
ACC1=key1
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
ACC2=key2
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
ACC3=key3
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
pull-down-description-text
This is the text for the pull-down choice. The maximum length of the text is 64 bytes.
Each pull-down-description-text is prefixed with a sequential number beginning with 1 to allow
selection by number.
Comments
The PDC tag defines a pull-down choice for an action bar pull-down. If you do not code any PDC tags
within an ABC tag, that action bar choice does not appear on the action bar.
To provide for a pull-down selection, an input field is generated prior to the first pull-down-description-text
that allows entry of the number of the selected pull-down choice. Since field names are being generated,
the application developer should not use field names beginning with Z.
Restrictions
• You must code the PDC tag within an ABC definition. See “ABC (Action Bar Choice)” on page 182 for a
complete description of this tag.
• The maximum number of pull-down choices that is generated is 60. However, the depth specified on the
enclosing PANEL tag can further reduce this maximum number.
Processing
Table 61. Tags you can code within a PDC definition 
Tag Reference Usage Required
ACTION “ACTION (Action)” on page 184 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
M “M (Mnemonic)” on page 351 Single No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
PDC
392  z/OS: z/OS ISPF DTL Guide

## Page 425

Here is application panel markup that produces the action bar and pull-down shown in Figure 146 on
page 393.
In this example, when the action bar choice Search is chosen, the variable whchsrch is tested to see if
one of the pull-down choices should be preselected. If whchsrch=1 then the pull-down choice Search on
name is preselected with a 1 in the pull-down selection entry field. If whchsrch=2 then the pull-down
choice Search on card number is preselected with a 2 in the pull-down selection entry field. If whchsrch
is not equal to 1 or 2, then neither pull-down choice is preselected. The example shows the Search on
name choice preselected. If srch2=1, then the UNAVAIL attribute on the pull-down choice Search on card
number would cause that choice to be unavailable. The example shows the result.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 system>
  <!entity sampbody system>)>
&sampvar1;
<PANEL NAME=pdc2 KEYLIST=keylxmp>Library Card Registration
<AB>
<ABC>File
  <PDC>Add Entry
    <ACTION RUN=add>
  <PDC>Delete Entry
    <ACTION RUN=delete>
  <PDC>Update Entry
    <ACTION RUN=update>
  <PDC>Exit
    <ACTION RUN=exit>
<ABC>Search
  PDC CHECKVAR=whchsrch MATCH=1 UNAVAIL=srch1>Search on name
    <ACTION SETVAR=whchsrch VALUE=1>
    <ACTION RUN=search>
  PDC CHECKVAR=whchsrch MATCH=2 UNAVAIL=srch2>Search on card number
    <ACTION SETVAR=whchsrch VALUE=2>
    <ACTION RUN=search>
<ABC>Help
  <PDC>Extended Help...
    <ACTION RUN=exhelp>
  <PDC>Keys Help...
    <ACTION RUN=keyshelp>
</AB>
&sampbody;
</PANEL>
Figure 146. Pull-down choices
PDC
Chapter 12. Tag reference  393

## Page 426

PDSEP (Pull-Down Separator)
The PDSEP tag is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
PLDIV (Parameter List Divider)
The Parameter List Divider tag creates a blank or visible divider within the text portion of a parameter list.
Syntax
<PLDIV
TYPE=
NONE
SOLID
DASH
TEXT
GAP=
YES
NO
GUTTER=
1
n
FORMAT= START
CENTER
END
>
divider-text </PLDIV>
Parameters
TYPE=NONE | SOLID | DASH | TEXT
This attribute specifies the type of parameter list divider line.
The default value is NONE, which produces a blank line. You must specify SOLID, DASH, or TEXT to
produce a visible divider line. When the GRAPHIC invocation option is specified, SOLID produces a
solid line for host display and DASH produces a dashed line. When NOGRAPHIC is specified, both
SOLID and DASH produce a dashed line.
GAP=YES | NO
When GAP=NO, the divider line completely crosses from one side of the text area to the other. When
GAP=YES, a 1-character gap remains at each end of the divider line.
GUTTER=1 | n
This attribute specifies the total width of the parm list divider. If the GUTTER value is an even number,
the conversion utility increases the number by 1 so that the divider is centered within the defined
width.
The minimum GUTTER value, and the default, is 1.
FORMAT=START | CENTER | END
This attribute specifies the position of the divider text within the width of the divider line.
divider-text
This is the text of the area divider line.
Comments
PDSEP
394  z/OS: z/OS ISPF DTL Guide

## Page 427

The PLDIV tag creates a blank or solid divider within the text portion of an application panel. A
horizontally formatted visible divider is created when you specify the TYPE attribute value as SOLID or
DASH. When the GRAPHIC invocation option is specified, SOLID produces a solid line for host display and
DASH produces a dashed line. When NOGRAPHIC is specified, both SOLID and DASH produce a dashed
line.
The divider line can be formatted with descriptive text. When this feature is used, the FORMAT attribute
must be specified. If FORMAT is not specified, the tag text is ignored. You control the text padding
with the TYPE attribute. If TYPE=TEXT, the divider-text is padded with blanks. When TYPE=SOLID or
TYPE=DASH, the divider-text is padded with the specified character.
Restrictions
• You must code the PLDIV tag within a PARML tag definition.
Processing
Table 62. Tags you can code within a PLDIV definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
Examples
Here is an example that uses the PLDIV tag. Figure 147 on page 396 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=pldiv DEPTH=22 WIDTH=60>Part Number Code Help
<AREA>
<INFO>
  <P>Valid part numbers consist of a three-digit
  number followed by a 2-character suffix.
  <DIVIDER>
  <PARML TSIZE=6 compact>
    <PLDIV TYPE=solid>
    <PT>123
    <PD>The first three digits represent
    the lot number of the part.
    <PLDIV TYPE=solid>
    <PT>AA
    <PD>The 2-character suffix represents the
    department the part originated from.
    The valid suffixes are:
      <PARML BREAK=none COMPACT SKIP>
        <PT>TO
        <PD>Tools
        <PT>EL
        <PD>Electrical
        <PT>ME
        <PD>Mechanical
      </PARML>
  </PARML>
</INFO>
</AREA>
</HELP>
PLDIV
Chapter 12. Tag reference  395

## Page 428

Part Number Code Help
 Valid part numbers consist of a three-digit number
 followed by a 2-character suffix.
 ----------------------------------------------------------
 123
       The first three digits represent the lot number of
       the part.
 ----------------------------------------------------------
 AA
       The 2-character suffix represents the department the
       part originated from. The valid suffixes are:
       TO        Tools
       EL        Electrical
       ME        Mechanical
Figure 147. Parameter list divider
PNLINST (Panel Instruction)
The PNLINST tag defines panel instructions for an application panel.
Syntax
<PNLINST
COMPACT
>
instruction-text </PNLINST>
Parameters
COMPACT
This attribute causes the panel instruction to format without a blank line before the text.
instruction-text
This is the text of the panel instruction. The instruction-text must fit in the remaining panel depth.
Comments
The PNLINST tag defines panel instructions for an application panel. The instruction-text formats as a
paragraph based on the width of the application panel, area, or region. You can code multiple paragraphs
of instruction text by using a new panel instruction tag for each new paragraph.
If the COMPACT attribute is not specified, the conversion utility inserts a blank line before the panel
instruction text.
Restrictions
• You must code the PNLINST within a PANEL, AREA, or REGION definition.
PNLINST
396  z/OS: z/OS ISPF DTL Guide

## Page 429

Processing
Table 63. Tags you can code within a PNLINST definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is application panel markup that contains one panel instruction. Figure 148 on page 398 shows the
formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=selcls TYPE='char 2'>
<VARLIST>
   <VARDCL NAME=loc VARCLASS=selcls>
   <VARDCL NAME=mode VARCLASS=selcls>
</VARLIST>
<PANEL NAME=pnlinst HELP=trvlhlp WIDTH=60 DEPTH=22 KEYLIST=keylxmp>
Dream Vacation Guide
<AB>
  <ABC>File
    <PDC>Add Entry
             <ACTION RUN=add>
     <PDC>Delete Entry
             <ACTION RUN=delete>
     <PDC>Update Entry
             <ACTION RUN=update>
     <PDC>Exit
             <ACTION RUN=exit>
   <ABC>Help
      <PDC>Extended Help...
             <ACTION RUN=exhelp>
      <PDC>Keys Help...
             <ACTION RUN=keyshelp>
</AB>
<AREA>
  <PNLINST>Choose one of the following exotic locations and
  your preferred mode of travel, then press Enter.
  <DIVIDER>
  <REGION DIR=horiz>
  <SELFLD NAME=loc PMTWIDTH=23 SELWIDTH=25>Exotic Location:
        <CHOICE>Athens, GA
        <CHOICE>Berlin, CT
        <CHOICE>Cairo, IL
        <CHOICE>Lizard Lick, NC
        <CHOICE>Paris, TX
        <CHOICE>Rome, NY
        <CHOICE>Venice, FL
  </SELFLD>
  <DIVIDER>
  <SELFLD NAME=mode PMTWIDTH=25 SELWIDTH=25>Travel Mode:
        <CHOICE>Boxcar
        <CHOICE>Hitchhike
        <CHOICE>Mule
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>
</PANEL>
PNLINST
Chapter 12. Tag reference  397

## Page 430

Dream Vacation Guide
 Command ===> ____________________________________________
 Choose one of the following exotic locations and your
 preferred mode of travel, then press Enter.
 Exotic Location:            Travel Mode:
 __  1.  Athens, GA          __  1.  Boxcar
     2.  Berlin, CT              2.  Hitchhike
     3.  Cairo, IL               3.  Mule
     4.  Lizard Lick, NC
     5.  Paris, TX
     6.  Rome, NY
     7.  Venice, FL
Figure 148. Panel instructions
PS (Point-and-Shoot)
The PS tag defines a text string that is to be enabled for point-and-shoot.
Syntax
<PS VAR= point-and-shoot-variable-name
%varname
VALUE= point-and-shoot-value
%varname
* CSRGRP=
NO
YES
n
DEPTH= n
%varname
IMAP options
IMAP options
IMAPNAME= image-name
%varname
IMAP group
IMAP group
IMAPNAMEP= image-namep
%varname PLACE=
ABOVE
BELOW
LEFT
RIGHT
%varname
> point-and-shoot-text </PS>
PS
398  z/OS: z/OS ISPF DTL Guide

## Page 431

Parameters
VAR=point-and-shoot-variable-name | %varname
This attribute provides the name of a variable which is to be set when a point-and-shoot phrase is
clicked on for selection. You can define this attribute as a variable name preceded by a “%”.
The point-and-shoot-variable-name must follow the standard naming convention described in “Rules
for variable names” on page 179.
VALUE=point-and-shoot-value | %varname | *
This attribute provides the value to be placed in the field specified by the VAR attribute. You can
define this attribute as a variable name preceded by a “%”. To specify a blank value, the "' '"
(quotation mark, apostrophe, blank, apostrophe, quotation mark) coding notation should be used.
When the PS tag is used with the CHOICE tag, VALUE=* can be used to automatically use the current
choice number (or SELCHAR value) as the point-and-shoot selection value.
CSRGRP=NO | YES | n
When CSRGRP=YES, the conversion utility generates a cursor group number to be used for this
point-and-shoot text field. When CSRGRP=n, the number provided is used for this field.
DEPTH=n | %varname
This attribute defines the depth reserved for the point-and-shoot field. The minimum value is 1 and
the maximum value is the remaining panel depth. This attribute is accepted in order to support
existing DTL source files that use it. However, although the space is reserved, point-and-shoot does
not function in the additional reserved space..
IMAPNAME=image-name | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAMEP=image-namep | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
PLACE=ABOVE | BELOW | LEFT | RIGHT | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
point-and-shoot-text
This is the text of a point-and-shoot entry.
Comments
The PS tag is valid as part of the text following these tags:
INFO TAGS
ATTENTION, CAUTION, DD, DDHD, DT, DTHD, FIG, FIGCAP, H2, H3, H4, LI, LINES, LP, NOTE, NT, P, PD,
PT, WARNING, and XMP.
PANEL TAGS
BOTINST, CHOFLD, CHOICE, DTAFLD, DTAFLDD, GRPHDR, LSTCOL, LSTGRP, PNLINST, SELFLD, and
TOPINST.
The point-and-shoot-text is color emphasized within the text of the panel. The user places the cursor on
the point-and-shoot-text and presses ENTER to select the option.
Restrictions
• The PS tag requires an end tag.
• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
PS
Chapter 12. Tag reference  399

## Page 432

Processing
None.
Examples
Here is an example that shows the use of point-and-shoot selection for a sample option menu. Figure 149
on page 400 shows the formatted result.
<!doctype dm system ()>
<!-- Sample selection menu with point-and-shoot -->
<panel name=ps1 menu keylist=keylxmp>Sample Point-and-Shoot
  <topinst>This is a selection panel.
  <selfld type=menu   pmtloc=before
          selwidth=40 pmtwidth=10>Select an option
    <choice checkvar=xtest1 match=a>
        <PS VAR=zcmd VALUE=1>Selection #1 (Command Tstch1)
</PS>
      <action run=tstch1 parm='1 2 3 4'
       passlib newpool suspend>
    <choice checkvar=xtest1 match=b>
        <PS VAR=zcmd VALUE=2>Selection #2 (Command Tstch2)
</PS>
      <action run=tstch2 parm=1234>
    <choice checkvar=xtest1 match=c>
        <PS VAR=zcmd VALUE=3>Selection #3 (Command Tstch3)
</PS>
      <action run=tstch3 parm=abcd>
    <choice checkvar=xtest1 match=d>
        <PS VAR=zcmd VALUE=4>Selection #4 (Command Tstch4)
</PS>
      <action run=tstch4 parm='a b c d'>
  </selfld>
  <cmdarea>
</panel>
                         Sample Point-and-Shoot
 This is a selection panel.
 Select an
 option . . 1  Selection #1 (Command Tstch1)
            2  Selection #2 (Command Tstch2)
            3  Selection #3 (Command Tstch3)
            4  Selection #4 (Command Tstch4)
 Option ===> _____________________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 149. Point-and-shoot fields 
PT (Parameter Term)
The PT tag defines a term in a parameter list.
PT
400  z/OS: z/OS ISPF DTL Guide

## Page 433

Syntax
<PT
FORMAT=
START
CENTER
END
NOSKIP
SPLIT=
NO
YES
>
parameter-term </PT>
Parameters
FORMAT = START | CENTER | END
This attribute specifies the placement of the PT tag text within the space provided by TSIZE. The PT
tag FORMAT setting overrides the FORMAT setting of the enclosing PARML tag.
NOSKIP
This attribute causes the definition term to be formatted without a blank line before the term. It is
used to control the formatting of the parameter term when COMPACT has not been specified on the
enclosing PARML tag. When the PARML tag TSIZE attribute specifies that multiple PT tags are to be
formatted for each PD tag, NOSKIP should be coded on the first PT tag. It is ignored for the second
and subsequent PT tags.
SPLIT=NO | YES
This attribute controls the format of the last PT tag in a multiple PT tag group. It is used only
when BREAK=ALL or when BREAK=FIT and the PT tag text length exceeds the TSIZE value. When
SPLIT=YES, the text following the last PT tag in the PT group (typically one or two dashes) is placed
in front of the first line of the formatted PD tag text. The PT tag SPLIT setting overrides the SPLIT
specified in the enclosing PARML tag.
parameter-term
This is the text of the parameter term.
Comments
The PT tag defines a parameter term in a parameter list.
Restrictions
• You must code the PT tag within a PARML definition. See “PARML (Parameter List)” on page 386 for a
complete description of this tag.
• Each PT tag must be paired with an associated PD tag. You can specify only one PT tag for each PD tag
within a parameter list. The PT tag must immediately precede the PD tag it is associated with.
Processing
Table 64. Tags you can code within a PT definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
PTSEG “PTSEG (Parameter Term Segment)” on page 404 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
PT
Chapter 12. Tag reference  401

## Page 434

Examples
Here is help panel markup that contains a parameter list with two parameter terms. Figure 150 on page
402 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=pt WIDTH=40 DEPTH=18>Help for the Duplex Function
<AREA>
<INFO>
  <P>The two options associated with
  the DUPLEX function are:
  <PARML TSIZE=5>
    <PT>DCopies
    <PD>Which prints one-sided copies that
    are prepared for future duplex copying.
    <PT>DPrint
    <PD>Which prints two-sided copies.
  </PARML>
</INFO>
</AREA>
</HELP>
      Help for the Duplex Function
 The two options associated with the
 DUPLEX function are:
 DCopies
      Which prints one_sided copies
      that are prepared for future
      duplex copying.
 DPrint
      Which prints two_sided copies.
  F1=Help      F3=Exit      F5=Exhelp
  F6=Keyshelp  F7=PrvTopic  F8=NxtTopic
 F10=PrvPage  F11=NxtPage  F12=Cancel
Figure 150. Parameter terms
PTDIV (Parameter Term Divider)
The PTDIV tag defines a visible vertical divider (|) between multiple PT tags.
Syntax
<PTDIV
</PTDIV>
Comments
The PTDIV tag can be used to create a visual separation between the parameter terms. Each PTDIV tag
adds a vertical bar (plus display control attributes) to the parameter list.
Restrictions
The PTDIV tag can be coded before the first PT tag, between PT tags, or following the last PT tag (before
the PD tag definition).
PTDIV
402  z/OS: z/OS ISPF DTL Guide

## Page 435

Processing
None.
Examples
Here is an example that shows the PTDIV tag in combination with the DIVEND attribute of the PARML tag.
Figure 151 on page 403 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=ptdiv DEPTH=22 WIDTH=60>Part Number Code Help
<AREA>
<INFO>
  <P>Valid part numbers consist of a three-digit
  number followed by a 2-character suffix.
  <DIVIDER>
  <PARML TSIZE=6 compact>
    <PLDIV TYPE=solid>
    <PT>123
    <PD>The first three digits represent
    the lot number of the part.
    <PLDIV TYPE=solid>
    <PT>AA
    <PD>The 2-character suffix represents the
    department the part originated from.
    The valid suffixes are:
      <PARML BREAK=none COMPACT SKIP DIVEND=yes>
        <PLDIV TYPE=solid>
        <PTDIV>
        <PT>TO
        <PTDIV>
        <PD>Tools
        <PTDIV>
        <PT>EL
        <PTDIV>
        <PD>Electrical
        <PTDIV>
        <PT>ME
        <PTDIV>
        <PD>Mechanical
      </PARML>
  </PARML>
</INFO>
</AREA>
</HELP>
                  Part Number Code Help
 Valid part numbers consist of a three-digit number
 followed by a 2-character suffix.
 ----------------------------------------------------------
 123
       The first three digits represent the lot number of
       the part.
 ----------------------------------------------------------
 AA
       The 2-character suffix represents the department the
       part originated from. The valid suffixes are:
       ----------------------------------------------------
       | TO         | Tools                               |
       | EL         | Electrical                          |
       | ME         | Mechanical                          |
Figure 151. Parameter term divider
PTDIV
Chapter 12. Tag reference  403

## Page 436

PTSEG (Parameter Term Segment)
The PTSEG tag defines a segment of the parameter term. It is used to provide vertical separation of the PT
tag text.
Syntax
<PTSEG>
</PTSEG>
Comments
The PTSEG tag is used to create a vertical separation within the parameter term. The text following the
PTSEG tag is formatted directly under any previous parameter term tag text. Multiple PTSEG tags create
additional PT text lines.
Use of the PTSEG tag affects the PARML tag BREAK attribute. The first (or only) line of PT tag text is
processed according to the BREAK attribute of the PARML tag. For additional lines, when TSIZE is large
enough to accommodate the text segments, the PTSEG text is formatted in front of the associated PD tag
text. When TSIZE is not large enough to accommodate the largest segment, all of the PT and PTSEG text is
formatted above the associated PD tag text.
Restrictions
• The PTSEG tag can be coded within the text following a PT tag.
• When a PTSEG tag is coded, then all remaining PT tag text for the current PT tag set must follow a
PTSEG tag.
• The PT nested tags RP and PS are not supported within PT tag text following any PTSEG tag in a PT/PD
tag set.
Processing
Table 65. Tags you can code within a PTSEG definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
Examples
Here is an example that shows the PTSEG tag in combination with a multiple PT tag set. The last PT tag
includes the SPLIT=yes attribute to format the dash in front of the PD tag text. Figure 152 on page 405
shows the formatted result.
<!DOCTYPE DM SYSTEM()>
<PANEL NAME=ptseg KEYLIST=ISRHELP APPLID=ISR WINDOW=no PADC=user
       TUTOR ZUP=ISP7R000>Traces - Primary Commands
<CMDAREA CAPS=on>
<AREA DEPTH=1 EXTEND=on>
  <INFO WIDTH=*>
    <P>
       Enter a <hp>Primary Command</hp> in the command input field.
       It is processed after all row modifications and all line commands
       are processed. The following primary commands are valid for the
       Traces options:
    <PARML TSIZE="8 1" INDENT=2>
PTSEG (Parameter Term Segment)
404  z/OS: z/OS ISPF DTL Guide

## Page 437

<PT>
          LOCATE
          function-name
          (Function Traces) or variable name (Variable Traces)
        <PTSEG>
          LOC or
        <PTSEG>
          L
      <PT SPLIT=yes>-
      <PD>The LOCATE command positions the scrollable display at the
          first (or next) row containing the function name (Function
          Traces option) or the variable name (Variable Traces option).
    </PARML>
  </INFO>
</AREA>
</PANEL>
 Tutorial  ---------------  Traces - Primary Commands  --------------  Tutorial
 Command ===> _________________________________________________________________
 Enter a Primary Command in the command input field. It is processed after all
 row modifications and all line commands are processed. The following primary
 commands are valid for the Traces options:
   LOCATE function-name (Function Traces) or variable-name (Variable Traces)
   LOC or   - The LOCATE command positions the scrollable display at the first
   L          (or next) row containing the function name (Function Traces
              option) or the variable name (Variable Traces option).
Figure 152. Parameter term segment
REGION (Region)
The REGION tag defines the characteristics of a panel section including the direction in which fields on an
application panel are arranged.
Syntax
<REGION
DIR=
VERT
HORIZ
INDENT=n WIDTH= n
*
DEPTH= n
*
EXTEND=
OFF
ON
FORCE
ALIGN=
YES
NO
GRPBOX group
LOCATION=
DEFAULT
TITLE
group-box-title
</REGION>
GRPBOX group
REGION
Chapter 12. Tag reference  405

## Page 438

GRPBOX=
NO
YES
GRPWIDTH=n GRPBXVAR=variable-name
GRPBXMAT=
1
string
Parameters
DIR=VERT | HORIZ
This attribute specifies in which direction the contents of a region is arranged. The default value is
VERT, which formats the contents of the region in a vertical direction; that is, top to bottom. If you
specify the HORIZ value for DIR, the contents of the region are formatted horizontally; that is, left to
right within the region.
INDENT=n
This attribute defines the number of columns to indent the current region from the current left region
boundary.
WIDTH=n | *
This attribute defines the width of a panel region. If WIDTH is not specified or WIDTH=*, the default
value is the remaining available panel width.
DEPTH=n | *
This attribute defines the size of a scrollable region. When EXTEND=OFF, the minimum value is 2 and
the maximum value is the remaining panel depth. When EXTEND=ON, the minimum value is 1. If the
DEPTH value is specified as “*”, the conversion utility reserves the remaining available panel depth for
the scrollable region.
If DEPTH is not specified the region is not scrollable.
EXTEND=OFF | ON | FORCE
This attribute defines the runtime display size for the scrollable region. If EXTEND=ON is
specified, the panel definition is expanded from the minimum DEPTH to the size of the logical
screen. Only one EXTEND=ON attribute value is allowed on a panel. The first tag (AREA, DA, GA,
REGION, SELFLD) with EXTEND=ON is accepted; the EXTEND attribute on any subsequent tag is
ignored.
If you intend to display the panels in a pop-up window, it is recommended that you code
EXTEND=OFF.
If the EXTEND attribute is specified without the DEPTH attribute, a warning message is issued and
the EXTEND attribute is ignored.
If EXTEND=FORCE is specified within a horizontal area or region, the EXTEND(ON) keyword is
added to the scrollable area attribute statement in the )ATTR panel section. The conversion utility
issues a message to advise of a potential display error if other panel fields are formatted on or
after the last defined line of the scrollable area.
ALIGN=YES | NO
This attribute controls the horizontal alignment of the first fields in horizontal regions. The default is to
align the fields to facilitate cursor movement by tabbing. This attribute is valid only when DIR=HORIZ.
GRPBOX=NO | YES
This attribute is used to specify a region title. The default value is NO.
When GRPBOX=YES is specified on the same REGION tag that defines a scrollable region, the region
title is formatted as the first line within the )AREA panel section.
GRPWIDTH=n
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
REGION
406  z/OS: z/OS ISPF DTL Guide

## Page 439

GRPBXVAR=variable-name
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
GRPBXMAT=1 | string
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
LOCATION=DEFAULT | TITLE
This attribute is used to build a panel ‘title’ which requires data fields in specific column positions.
A single line may be formatted to be placed in the panel title position by enclosing the appropriate
tags within a horizontal region specifying LOCATION=TITLE. The resulting line displays with the colors
associated with the tags used to format the line. This attribute is valid only when DIR=HORIZ.
group-box-title
This is the title for the region. The group-box-title should be supplied only when GRPBOX=YES. In
other cases a warning message is issued.
Comments
The REGION tag defines the characteristics of a panel section. You can code multiple regions within an
application panel.
Nonscrollable horizontal regions are normally aligned left-to-right using the first input field from each
region. If a panel consists of both scrollable and nonscrollable regions formatted horizontally, scrollable
regions are normally aligned with the first input fields of nonscrollable regions.
Regions containing data formatted from INFO tags or from the GRPHDR tag normally start with a blank
line when formatted in the )BODY panel section. The blank line is omitted when these tags are formatted
at the beginning of a scrollable area.
If you specify the CMDAREA tag within your DTL source file, it must appear before the REGION tag when
DEPTH=* is specified. The REGION tag DEPTH may have to be adjusted to allow for additional lines which
result from tags present within the panel definition following the end REGION tag.
Restrictions
• The REGION tag requires an end tag.
• You must code the REGION tag within an AREA or PANEL definition. See “AREA (Area)” on page 189 and
“PANEL (Panel)” on page 376 for descriptions of these tags.
• You can also nest regions within other regions.
• You can code only one LSTFLD tag within a REGION definition.
Processing
Table 66. Tags you can code within a REGION definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
DA “DA (Dynamic Area)” on page 250 Multiple No
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
DTACOL “DTACOL (Data Column)” on page 269 Multiple No
DTAFLD “DTAFLD (Data Field)” on page 275 Multiple No
GA “GA (Graphic Area)” on page 295 Single No
REGION
Chapter 12. Tag reference  407

## Page 440

Table 66. Tags you can code within a REGION definition  (continued)
Tag Reference Usage Required
GENERATE “GENERATE (Generate)” on page 298 Multiple No
GRPHDR “GRPHDR (Group Header)” on page 300 Multiple No
INFO “INFO (Information Region)” on page 317 Multiple No
LSTFLD “LSTFLD (List Field)” on page 341 Single No
PNLINST “PNLINST (Panel Instruction)” on page 396 Multiple No
REGION “LSTFLD (List Field)” on page 341 Multiple No
SELFLD “SELFLD (Selection Field)” on page 421 Multiple No
Help panel
Table 67. Tags you can code within a REGION tag on a help panel
Tag Reference Usage Required
DIVIDER “DIVIDER (Area Divider)” on page 258 Multiple No
INFO “INFO (Information Region)” on page 317 Multiple No
REGION “REGION (Region)” on page 405 Multiple No
Examples
Here is application panel markup that contains horizontal and vertical regions. The first two horizontal
regions arrange the fields coded within them in a horizontal format. The third horizontal region arranges
the selection field and the contents of the vertical region nested within it in a horizontal format. In this
example, the INDENT attribute has been used to indent all fields formatted within a region 2 positions
under the previous text. The ALIGN attribute has adjusted the default placement of fields in the last
vertical region. Figure 153 on page 409 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=chr25 TYPE='char 25'>
<VARCLASS NAME=chr12 TYPE='char 12'>
<VARCLASS NAME=chr10 TYPE='char 10'>
<VARCLASS NAME=chr9  TYPE='char 9'>
<VARCLASS NAME=chr8  TYPE='char 8'>
<VARCLASS NAME=chr2  TYPE='char 2'>
<VARLIST>
  <VARDCL NAME=name   VARCLASS=chr25>
  <VARDCL NAME=date   VARCLASS=chr8>
  <VARDCL NAME=addr   VARCLASS=chr25>
  <VARDCL NAME=city   VARCLASS=chr10>
  <VARDCL NAME=state  VARCLASS=chr9>
  <VARDCL NAME=zip    VARCLASS=chr12>
  <VARDCL NAME=level  VARCLASS=chr2>
  <VARDCL NAME=graddate VARCLASS=chr2>
  <VARDCL NAME=major  VARCLASS=chr10>
</VARLIST>
<PANEL NAME=region1 keylist=keylxmp>Application Form
<TOPINST>Complete all of the fields below, then press Enter.
<AREA>
  <REGION INDENT=2>
    <REGION DIR=horiz>
    <DTACOL PMTWIDTH=10>
      <DTAFLD DATAVAR=name ENTWIDTH=25>Name
      <DTAFLD DATAVAR=date ENTWIDTH=8 DESWIDTH=10>Date
        <DTAFLDD>(mm/dd/yy)
    </DTACOL>
    </REGION>
REGION
408  z/OS: z/OS ISPF DTL Guide

## Page 441

<DTAFLD DATAVAR=addr ENTWIDTH=25 PMTWIDTH=10>Address
    <REGION DIR=horiz>
      <DTAFLD DATAVAR=city PMTWIDTH=10 ENTWIDTH=25>City
      <DTAFLD DATAVAR=state PMTWIDTH=9 ENTWIDTH=2>State
      <DTAFLD DATAVAR=zip PMTWIDTH=12 ENTWIDTH=5>Zip code
    </REGION>
  </REGION>
  <DIVIDER TYPE=solid GUTTER=3>
  <REGION DIR=horiz INDENT=2 ALIGN=no>
    <SELFLD NAME=level SELWIDTH=35 PMTWIDTH=25>Highest education level:
      <CHOICE>Some high school
      <CHOICE>High school graduate
      <CHOICE>Some college
      <CHOICE>College graduate
      <CHOICE>Some post-graduate work
      <CHOICE>Post-graduate degree
    </SELFLD>
    <DIVIDER TYPE=solid>
    <REGION>
      <GRPHDR FORMAT=none COMPACT STRIP>
          For applicants who are
          high school or college
          graduates:
      <REGION INDENT=2>
        <DTACOL PMTWIDTH=20>
          <DTAFLD DATAVAR=graddate ENTWIDTH=2>Year of graduation
          <DTAFLD DATAVAR=major ENTWIDTH=10>Field of study
        </DTACOL>
      </REGION>
    </REGION>
  </REGION>
</AREA>
<CMDAREA>Enter a command
</PANEL>
                              Application Form
 Complete all of the fields below, then press Enter.
   Name . . . _________________________  Date . . . ________  (mm/dd/yy)
   Address    _________________________
   City . . . _________________________  State . . __  Zip code . . _____
  -----------------------------------------------------------------------
   Highest education level:           |  For applicants who are
   __  1.  Some high school           |  high school or college
       2.  High school graduate       |  graduates:
       3.  Some college               |    Year of graduation   __
       4.  College graduate           |    Field of study . . . __________
       5.  Some post-graduate work    |
       6.  Post-graduate degree       |
 Enter a command ===> ____________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 153. Regions
Here is an example that shows the WIDTH and DEPTH attributes. The first vertical region width reserves
the space required for the second vertical region, which is also scrollable. Figure 154 on page 411 shows
the formatted result.
REGION
Chapter 12. Tag reference  409

## Page 442

<!DOCTYPE DM SYSTEM(
  <!entity sampvar2 sysem>
  <!entity sampabc system>)>
&sampvar2;
<PANEL NAME=region3 KEYLIST=keylxmp>File-A-Case
<AB>
&sampabc;
</AB>
<CMDAREA>Enter a command
<TOPINST COMPACT>
         Type in client's name and case number (if applicable).
<TOPINST>Then select an action bar choice.
<REGION DIR=horiz>
  <REGION WIDTH=50>
    <DTAFLD DATAVAR=caseno PMTWIDTH=12 ENTWIDTH=7 DESWIDTH=21>Case No
       <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=8>Name
       <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address PMTWIDTH=12 ENTWIDTH=25>Address
    <DIVIDER>
    <SELFLD NAME=casesel PMTWIDTH=11 PMTLOC=before SELWIDTH=38>Choose
    one of the following
       <CHOICE CHECKVAR=case MATCH=civ>Civil
       <CHOICE CHECKVAR=case MATCH=estate>Real Estate
       <CHOICE CHECKVAR=case MATCH=environ>Environmental
    </SELFLD>
  </REGION>
  <REGION DEPTH=10>
    <SELFLD TYPE=multi PMTWIDTH=24 SELWIDTH=26>
      Check type of offense
       <CHOICE NAME=patin HELP=patin CHECKVAR=val>Patent Infringement
       <CHOICE NAME=defa HELP=defame CHECKVAR=def>Defamation
       <CHOICE NAME=cont HELP=cont CHECKVAR=con>Breach of Valid Contract
       <CHOICE NAME=priv HELP=priv CHECKVAR=pri>Invasion of Privacy
       <CHOICE NAME=incr HELP=incr CHECKVAR=icr>Interference with
               Contractual Relations
       <CHOICE NAME=disp HELP=disp CHECKVAR=dis>Improper Disposal of
               Medical By-Products
       <CHOICE NAME=fraud HELP=fraud CHECKVAR=fra>Fraud
    </SELFLD>
  </REGION>
</REGION>
</PANEL>
REGION
410  z/OS: z/OS ISPF DTL Guide

## Page 443

File  Search  Help
 -------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
                                                  #SAREA37                 #
 Case No  . . _______  (A 7-digit number)         #                        #
 Name . . . . _________________________  (Last,   #                        #
                                         First,   #                        #
                                         M.I.)    #                        #
 Address  . . _________________________           #                        #
                                                  #                        #
 Choose one                                       #                        #
 of the                                           #                        #
 following   __  1.  Civil                        #                        #
                 2.  Real Estate
                 3.  Environmental
 Enter a command ===> ____________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Here are the contents of the scrollable area:
     )AREA SAREA37
 Check type of offense
 _  Patent Infringement
 _  Defamation
 _  Breach of Valid
    Contract
 _  Invasion of Privacy
 _  Interference with
    Contractual Relations
 _  Improper Disposal of
    Medical By-Products
 _  Fraud
     )AREA SAREA37
Figure 154. Using WIDTH and DEPTH attributes
RP (Reference Phrase)
The RP tag specifies a word or phrase within panel text that has additional help information associated
with it.
Syntax
<RP HELP= help-panel-name
*help-message-id
%varname
*%varname
> reference-phrase </RP>
RP
Chapter 12. Tag reference  411

## Page 444

Parameters
HELP= help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the name of a panel that displays when the user requests help for the
reference-phrase.
You can specify either a help panel or a message identifier. If a message identifier is used, it must be
prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a choice and no help is defined, the extended help panel is displayed. If
an extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
reference-phrase
This is the text of the phrase.
Comments
The RP tag specifies a word or phrase within panel text that has additional information associated with it.
The RP tag is valid as part of the text following these tags:
INFO tags
ATTENTION, CAUTION, DD, DDHD, DT, DTHD, FIG, FIGCAP, H2, H3, H4, LI, LINES, LP, NOTE, NT, P, PD,
PT, WARNING, and XMP.
PANEL tags
BOTINST, CHOFLD, CHOICE, DTAFLD, DTAFLDD, GRPHDR, LSTCOL, LSTGRP, PNLINST, SELFLD, and
TOPINST.
The reference-phrase is emphasized within the text of the panel to inform the user that additional
information is available. The user positions the cursor on the reference phrase and presses F1=Help
to obtain help on the phrase.
Each reference phrase is related to additional help panels in a manner similar to field-level help. The
panel that appears when you request help from a reference phrase can also contain reference phrases.
Each reference-phrase results in one or more entries in the )HELP panel section. Multiple entries are
required for phrases that span lines; a separate entry is created for each panel line used by the reference-
phrase.
Restrictions
• The RP tag requires an end tag.
Processing
None.
Examples
Here is help panel markup that contains a reference phrase definition for the phrase, “lifetime warranty”.
Figure 155 on page 413 shows the formatted result.
RP
412  z/OS: z/OS ISPF DTL Guide

## Page 445

<!DOCTYPE DM SYSTEM>
<HELP NAME=rp>HELP for Appliances
<AREA>
<INFO>
<p>In addition to our free delivery and installation program, we also
offer an exclusive <rp help=warrtyh>lifetime warranty</rp> on all
of our appliances.
</INFO>
</AREA>
</HELP>
<help name=warrtyh>Help for Lifetime Warranty
<AREA>
<INFO>
<p>Lifetime warranty covers the replacement of any part that breaks
or becomes non-functional while this product is used by the original
owner.
</INFO>
</AREA>
</HELP>
                  Help for Appliances
 In addition to our free delivery and
 installation program, we also offer an exclusive
 lifetime warranty on all of our appliances.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 155. Reference phrase example
Accordingly, when the user selects the reference phrase lifetime warranty, the help panel specified by
the HELP attribute (help=warrtyh) is displayed. Figure 156 on page 413 shows the formatted result.
            Help for Lifetime Warranty
 Lifetime warranty covers the replacement of any
 part that breaks or becomes non-functional while
 this product is used by the original owner.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 156. Reference phrase example
SCRFLD (Scrollable Field)
The SCRFLD tag defines a field on an application panel as being scrollable. The panel field is defined using
either the DTAFLD or LSTCOL tag. The SCRFLD tag must be nested within either a DTAFLD or LSTCOL tag.
Using the SCRFLD tag causes the conversion utility to format an entry in the )FIELD section of the
generated panel. See the z/OS ISPF Dialog Developer's Guide and Reference for a description of the )FIELD
section.
SCRFLD
Chapter 12. Tag reference  413

## Page 446

Syntax
<SCRFLD
DISPLEN= n
%varname
INDVAR=ind-var
INDVAL='ind-chars'
LINDVAR=lind-var
LINDVAL='lind-char'
RINDVAR=rind-var
RINDVAL='rind-char'
SINDVAR=sind-var
SINDVAL='sind-chars'
LCOLIND=lcol-var
LCOLDISP=
NO
YES
RCOLIND=rcol-var
RCOLDISP=
NO
YES
SCALE=scale-var
SCROLL=
ON
OFF
NOLR
%varname
FLDSPOS= 1
BELOW
SAME
ABOVE
>
</SCRFLD>
Notes:
1 When the SCRFLD tag is nested in a DTAFLD tag, FLDSPOS can be BELOW or SAME. When the
SCRFLD tag is nested in a LSTCOL tag, FLDSPOS can be BELOW or ABOVE.
Parameters
DISPLEN=n | %varname
This attribute is used to specify a length for the variable displayed in the scrollable field.
When DISPLEN=n is used, n specifies the initial length of the variable. n must be a value between 1
and 32 767.
%varname is a dialog variable that can contain a value between 1 and 32 767 to specify the initial
length of the variable displayed in the scrollable field. After the panel is displayed %varname contains
SCRFLD
414  z/OS: z/OS ISPF DTL Guide

## Page 447

the maximum of the length of the dialog variable displayed and the initial length specified. When the
scrollable field has been defined using the LSTCOL tag, the length of the dialog variable displayed is
the maximum of all instances on the current display for that variable.
INDVAR=ind-var
This attribute specifies the name of a dialog variable that contains the left and right scroll indicator.
ind-var is a 2-byte scroll indicator dialog variable that is updated with 1-byte indicators showing
whether left and right scrolling can be performed.
INDVAL='ind-chars'
This attribute is used to override the default scroll indicator values of '-' and '+' where:
-+
indicates you can scroll left or right
-
indicates you can only scroll left
+
indicates you can only scroll right
ind-chars must be a 2-byte literal enclosed in quotes.
The INDVAL attribute can only be specified together with the INDVAR attribute.
LINDVAR=lind-var
This attribute specifies the name of a dialog variable that contains the left scroll indicator.
lind-var is a 1-byte left-scroll-indicator dialog variable that is updated with an indicator showing
whether left scrolling can be performed. The LINDVAR attribute cannot be defined together with the
INDVAR attribute.
LINDVAL='lind-char'
This attribute is used to override the default left-scroll-indicator value of '-'.
lind-char must be a 1-byte literal enclosed in quotes.
The LINDVAL attribute can only be specified together with the LINDVAR attribute.
RINDVAR=rind-var
This attribute specifies the name of a dialog variable that contains the right scroll indicator.
rind-var is a 1-byte right-scroll-indicator dialog variable that is updated with an indicator showing
whether right scrolling can be performed. The RINDVAR attribute cannot be defined together with the
INDVAR attribute.
RINDVAL='rind-char'
This attribute is used to override the default right-scroll-indicator value of '+'.
rind-char must be a 1-byte literal enclosed in quotes.
The RINDVAL attribute can only be specified together with the RINDVAR attribute.
SINDVAR=sind-var
This attribute specifies the name of a dialog variable that contains the separator scroll indicator.
sind-var is a separator scroll indicator dialog variable that is initialized with the value repeated for the
length of the scrollable field displayed on the panel. If the field is scrollable to the left, the leftmost
byte is the value of the left indicator (default: '<'). If the field is scrollable to the right, the rightmost
byte is the value of the right indicator ('>').
SINDVAL='sind-chars'
This attribute is used to override the default separator scroll-indicator value of '<->'.
sind-chars must be a 3-byte literal enclosed in quotes.
The SINDVAL attribute can only be specified together with the SINDVAR attribute.
SCRFLD
Chapter 12. Tag reference  415

## Page 448

LCOLIND=lcol-var
This attribute specifies the name of a dialog variable that contains the value of the left column
position for the displayed scrollable field.
lcol-var is a dialog variable that is updated when the field is scrolled to contain the value of the left
column position. This dialog variable can be used to specify an initial left column position for the
scrollable field.
Note: If the same lcol-var is specified on multiple SCRFLD tags the associated panel fields scroll
simultaneously. When the same lcol-var is associated with multiple panel fields, the conversion utility
only defines lcol-var as a left column position indicator panel field for the first of those panel fields.
LCOLDISP=NO | YES
This attribute is used to specify whether the left column position indicator defined using the LCOLIND
attribute is displayed on the panel.
When LCOLDISP=NO, the left column indicator is not generated as a panel field.
RCOLIND=rcol-var
This attribute specifies the name of a dialog variable that contains the value of the right column
position for the displayed scrollable field.
rcol-var is a dialog variable that is updated when the field is scrolled to contain the value of the right
column position.
Note: If the same rcol-var is specified on multiple SCRFLD tags the associated panel fields scroll
simultaneously. When the same rcol-var is associated with multiple panel fields, the conversion utility
only defines rcol-var as a right column position indicator panel field for the first of those panel fields.
RCOLDISP=NO | YES
This attribute is used to specify whether the right column position indicator defined using the
LCOLIND attribute is displayed on the panel.
When RCOLDISP=NO, the right column indicator is not generated as a panel field.
SCALE=scale-var
This attribute specifies the name of a dialog variable that contains the scale indicator.
scale-var is a dialog variable that is updated with a scale line reflecting the current columns being
displayed for the scrollable field.
SCROLL=ON | OFF | NOLR | %varname
This attribute is used to specify whether the field is scrollable or not.
When SCROLL=OFF, the field is not scrollable.
When SCROLL=NOLR, LEFT and RIGHT scrolling of the scrollable field is disabled.
%varname is used to specify the name of a scroll control dialog variable. This can be set to a value
of ON or OFF to turn scrolling for the field either on or off. When SCROLL=NOLR, LEFT and RIGHT
scrolling of the scrollable field is disabled.
FLDSPOS=BELOW | ABOVE | SAME
This attribute is used to specify where the scroll indicator panel fields are positioned in relation to the
heading text for a table display field defined using the LSTCOL tag or in relation to the display field
defined using the DTAFLD tag.
With FLDSPOS=BELOW, the conversion utility defines all scroll indicator panel fields for the scrollable
table display field below the heading text or below the data field defined by the DTAFLD tag.
With FLDSPOS=SAME, the conversion utility attempts to define ind-var, lind-var, and rind-var for the
data field on the same line as the data field. This option is not valid when the SCRFLD tag is nested
within a LSTCOL tag.
With FLDSPOS=ABOVE, the conversion utility defines all scroll indicator panel fields for the scrollable
table display field above the heading text. This option is not valid when the SCRFLD tag is nested
within a DTAFLD tag.
SCRFLD
416  z/OS: z/OS ISPF DTL Guide

## Page 449

Comments
The SCRFLD tag defines a field on an application panel as being scrollable. The panel field is defined using
either the DTAFLD or LSTCOL tag. The SCRFLD tag must be nested within either a DTAFLD or LSTCOL tag.
Using the SCRFLD tag causes the conversion utility to format an entry in the )FIELD section of the
generated panel. See the z/OS ISPF Dialog Developer's Guide and Reference for a description of the )FIELD
section.
Scroll indicator fields
The conversion utility implicitly defines ind-var, lind-var, rind-var, and sind-var as scroll-indicator
panel fields, and scale-var as a scale-indicator panel field. This topic describes where the scroll and
scale indicator fields appear on the panel. Their position depends on whether the SCRFLD tag is
nested within a DTAFLD or LSTCOL tag, and on the attributes specified on the SCRFLD tag.
Here is the order in which the scroll indicator fields are created by the conversion utility:
1. lcol_var
2. rcol_var
3. ind_var | lind_var
4. rind_var
Position of scroll indicator fields under the LSTCOL tag
Depending on the attributes specified on the SCRFLD tag, the conversion utility can create below
or above the column heading text up to four panel lines containing scroll indicator fields. Here is
a table that identifies the order in which the scroll indicator fields are created by the conversion
utility, assuming FLDSPOS=BELOW is specified. 
Table 68. Order in which scroll indicator fields  are created when FLDSPOS=BELO is specified 
Relative Line
from Column
Heading  1 
Scroll Indicator Dialog
Variables
Comments
+1 ind-var |
lind-var and rind-var
Displays either the left/right scroll indicator
variable OR the left and right scroll indicator
variables.
The scroll indicator variables are positioned
left-justified relative to the column.
+2 lcol-var and rcol-var The left and right column position indicators
are positioned left-justified relative to the
column.
The number of characters used for the left
and right column indicators is one more than
the larger of the dimension of the initial field
display length or the dimension of the column
width.
+3 sind-var Separator scroll indicator field spans the width
of the column.
+4 scale-var Scale indicator field spans the width of the
column.
 1 If the associated scroll indicator dialog variables are not specified, the conversion utility uses
the line for the next scroll indicator field.
SCRFLD
Chapter 12. Tag reference  417

## Page 450

Position of scroll indicator fields under the DTAFLD tag
Depending on the attributes specified on the SCRFLD tag, the conversion utility can create below
or on the same line as the data field up to four panel lines containing scroll indicator fields.
The following table identifies the order in which the scroll indicator fields are created by the
conversion utility, assuming FLDSPOS=BELOW is specified.
The conversion utility defines, on the following panel lines, output fields for the scroll indicator
variables specified using the SCRFLD tag attributes. Here is a table that identifies the order in
which the scroll indicator fields are created by the conversion utility:
Table 69. Order in which scroll indicator fields  are created
Relative Line
from DTAFLD
Field  1 
Scroll Indicator Dialog
Variables
Comments
+1 scale-var Scale indicator field spans the width of the
field.
+2 sind-var Separator scroll indicator field spans the width
of the field.
+3 lcol-var and rcol-var The left and right column position indicators
are positioned left-justified relative to the
column.
The number of characters used for the left and
right column indicators is one more than the
initial field display length.
+4 ind-var |
lind-var and rind-var
Displays either the left/right scroll indicator
variable OR the left and right scroll indicator
variables.
The scroll indicator variables are positioned
left-justified relative to the field.
 1 If the associated scroll indicator dialog variables are not specified, the conversion utility uses
the line for the next scroll indicator field.
When the SCRFLD tag is associated with a DTAFLD tag that is immediately within a vertical region,
scale and separator scroll indicators are not permitted.
Restrictions
• You must code the SCRFLD tag within a LSTCOL or DTAFLD tag.
Processing
Table 70. Tags you can code within a SCRFLD definition 
Tag Reference Usage Required
COMMENT “COMMENT (Comment)” on page 245 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
SCRFLD
418  z/OS: z/OS ISPF DTL Guide

## Page 451

Here is source file markup where the application panel contains two scrollable fields, the Address field
and the Comments field. A scroll separator is displayed with the Address field and a scale line is displayed
with the Comments field. Figure 157 on page 420 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=date TYPE='char 8'>
<VARCLASS NAME=name TYPE='char 20'>
<VARCLASS NAME=addr TYPE='char 40'>
<VARCLASS NAME=prod TYPE='char 25'>
<VARCLASS NAME=comm TYPE='char 55'>
<VARLIST>
  <VARDCL NAME=curdate VARCLASS=date>
  <VARDCL NAME=snamvar VARCLASS=name>
  <VARDCL NAME=fnamvar VARCLASS=name>
  <VARDCL NAME=sindvar VARCLASS=addr>
  <VARDCL NAME=addrvar VARCLASS=addr>
  <VARDCL NAME=prodvar VARCLASS=prod>
  <VARDCL NAME=scalvar VARCLASS=comm>
  <VARDCL NAME=commvar VARCLASS=comm>
</VARLIST>
<PANEL NAME=scrfld0 HELP=loghelp>Customer Feedback
<TOPINST>Complete the following fields, then press Enter.
<AREA>
  <DTACOL PMTWIDTH=15>
    <DIVIDER>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8 FLDSPACE=27>Date
  <DTAFLDD>(Current Date)
    <DIVIDER>
    <DTAFLD DATAVAR=snamvar ENTWIDTH=20>Surname
    <DIVIDER>
    <DTAFLD DATAVAR=fnamvar ENTWIDTH=20>First Names
    <DIVIDER>
    <DTAFLD DATAVAR=addrvar ENTWIDTH=40 DESWIDTH=15>Address
  <DTAFLDD>(Optional)
      <SCRFLD DISPLEN=80 SINDVAR=sindvar>
    <DIVIDER>
    <DTAFLD DATAVAR=prodvar ENTWIDTH=25 DESWIDTH=25>Product
  <DTAFLDD>(Product Purchased)
    <DIVIDER>
    <DTAFLD DATAVAR=commvar ENTWIDTH=55>Comments
      <SCRFLD DISPLEN=110 SCALE=scalvar>
  </DTACOL>
</AREA>
<CMDAREA scrollvar=scrvar>Command
</PANEL>
SCRFLD
Chapter 12. Tag reference  419

## Page 452

Customer Feedback
 Complete the following fields, then press Enter.
 Date  . . . . : 02/10/21                   (Current Date)
 Surname . . . . Smith               
 First Names . . John Joseph         
 Address . . . . Apartment 10a, 100 Happiness Street, Ple  (Optional)
                 --------------------------------------->
 Product . . . . Hammer                     (Product Purchased)
 Comments  . . . An implement that has proved very useful for driving na
                 ----+----1----+----2----+----3----+----4----+----5----+
 Command ===>                                             Scroll ===> CSR 
  F1=Help    F3=Exit    F7=Up     F8=Down    F10=Left   F11=Right  F12=Cancel
Figure 157. List field 
Here is source file markup that uses the LSTFLD and LSTCOL tags to display the data in an ISPF table. The
SCRFLD tag is used to display the Customer and Comments data in scrollable fields. Left and right column
indicators are displayed in the column headings for the Customer and Comments data. A separator scroll
indicator is also displayed in the heading for the Customer column. A scale indicator is displayed in the
heading for the Comments column. Figure 158 on page 421 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=date TYPE='char 8'>
<VARCLASS NAME=cust TYPE='char 15'>
<VARCLASS NAME=prod TYPE='char 15'>
<VARCLASS NAME=comm TYPE='char 30'>
<VARLIST>
  <VARDCL NAME=datevar VARCLASS=date>
  <VARDCL NAME=custvar VARCLASS=cust>
  <VARDCL NAME=prodvar VARCLASS=prod>
  <VARDCL NAME=commvar VARCLASS=comm>
</VARLIST>
<PANEL NAME=scrfld1 HELP=loghelp>Customer Feedback Display
<AREA>
<LSTFLD SCROLLVAR=scrlamt SCRVHELP=scrhelp>
  <LSTCOL DATAVAR=datevar USAGE=out COLWIDTH=8>Date
  <LSTCOL DATAVAR=custvar USAGE=out COLWIDTH=15>Customer
   <SCRFLD DISPLEN=50 SINDVAR=sindvar LCOLIND=cuslcol LCOLDISP=YES
      RCOLIND=cusrcol RCOLDISP=YES>
  <LSTCOL DATAVAR=prodvar USAGE=out COLWIDTH=15>Product
  <LSTCOL DATAVAR=commvar USAGE=out COLWIDTH=30>Comments
   <SCRFLD DISPLEN=110 SCALE=scalvar LCOLIND=comlcol LCOLDISP=YES
      RCOLIND=comrcol RCOLDISP=YES>
 </LSTFLD>
</AREA>
<CMDAREA>Command
</PANEL>
SCRFLD
420  z/OS: z/OS ISPF DTL Guide

## Page 453

Customer Feedback Display           Row 1 to 6 of 6
 Date      Customer         Product          Comments
           1   15                            1    30
           -------------->                   ----+----1----+----2----+----3
 03/02/25  Big Bang Demoli  Jackhammer       We've used this piece of equip
 03/02/25  Carpenter, Scot  Chisel           I'm able to shape pieces of wo
 03/02/26  Acme Building C  Cement mixer     Our bricklayers make heavy use
 03/02/28  Smith, John Jos  Hammer           An implement that has proved v
 03/02/28  Hole, Doug       Shovel           Is ideally suited to our holis
 03/03/03  Picker, Sherry   Folding ladder   This tool is highly recommende
 ****************************** Bottom of data *******************************
 Command ===>                                             Scroll ===> CSR 
  F1=Help    F3=Exit    F7=Up      F8=Down    F10=Left   F11=Right  F12=Cancel
 F13=Help   F15=Exit    F24=Cancel
Figure 158. List variable
SELFLD (Selection Field)
The SELFLD tag defines a field that includes a list of choices.
SELFLD
Chapter 12. Tag reference  421

## Page 454

Syntax
<SELFLDNAME=field-nameHELP= NOYEShelp-panel-name
*help-message-id
%varname*%varname
TYPE=SINGLEMULTIMENUMODELTUTOR
PMTLOC=ABOVEBEFORE
PMTWIDTH=n***
SELWIDTH=n*
ENTWIDTH=2n'e1 e2...en'
REQUIRED= NOYESYESMSG=message-identifier
FCHOICE=10 AUTOTAB=YESNO
DEPTH=n* EXTEND=OFFONFORCE
TRAIL='trail-var-1 trail-var-2 ... trail-var-n'CHOICECOLS=1n
CHOICEDEPTH=n* CWIDTHS='w1 w2...wn'
PAD= NULLSUSERchar%varname
PADC=NULLSUSERchar%varname
OUTLINE=NONELROUBOX%varname
SELMSG=selfld-msg-identifier
SELMSGU=selfld-msg-unavailableINIT= YESNOinit-value
VERIFY=YESNO REFRESH=YESNO
SELFMT=STARTEND CHKBOX=YESNO
ZGUI=YESNO CSRGRP=NOYESn
TSIZE='s1 s2...sn'
LISTTYPE=RADIOLISTBOXDDLISTCOMBO
LISTREF=list-nameLISTDEPTH=n
DBALIGN=YESNOFIELDFORCE
NOSEL=no-selection-value
SELDEFAULT=xPMTSKIP=NOYES
FLDTYPE=CUAISPF COLOR=WHITEREDBLUEGREENPINKYELLOWTURQ
%varname
INTENS=HIGHLOWNON%varname
HILITE=USCOREBLINKREVERSE%varname
SELCHECK=NOYES VARDCL=YESNO
>
field-prompt-text</SELFLD>
SELFLD
422  z/OS: z/OS ISPF DTL Guide

## Page 455

Parameters
NAME=field-name
This attribute specifies the name for the selection field. The field -name  must follow the standard
naming convention described in “Rules for variable names” on page 179.
The NAME field is required if TYPE=SINGLE because the selection field name is used as the input field
for single-choice selection fields. The NAME field is ignored if TYPE=MULTI.
The NAME field is optional for TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR. If present, it is used in
place of the command field name in the construction of the option selection statement. However,
because the input field is the command line, you must provide panel logic using the SOURCE tag to
ensure that the selection choice is placed in the NAME field.
For single-choice selection fields, the field -name  can be used to position the cursor on the field using
the CURSOR attribute of the enclosing PANEL tag or the CURSOR parameter of the DISPLAY service
call. In addition, you can use the field -name  to position a pop-up using the POPLOC parameter of the
ADDPOP service call.
HELP=NO | YES | help-panel-name | *help-message-id | %varname | *%varname
This attribute specifies the help action taken when the user requests help for a selection field. This is
field-level help.
When HELP=YES, control is returned to the application. You can specify either a help panel or a
message identifier. If a message identifier is used, it must be prefixed with an asterisk (*).
The help attribute value can be specified as a variable name. When %varname is coded, a panel
variable name is created. When *%varname is coded, a message variable name is created.
If the user requests help on a field and no help is defined, the extended help panel is displayed. If an
extended help panel is not defined for the panel, the application or ISPF tutorial is invoked.
The help-panel-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
See “HELP (Help Panel)” on page 303 for information about creating help panels. For information
about creating messages, see “MSG (Message)” on page 352.
Note: This attribute is valid only when TYPE=SINGLE.
TYPE=SINGLE | MULTI | MENU | MODEL | TUTOR
This attribute specifies whether the selection field is single-choice, multiple-choice, an ISPF selection
menu, an edit model selection menu, or a tutorial selection menu.
Single-choice selection fields allow the user to select only one choice from the selection list. Choices
in a single-choice selection field appear with sequential numbers before each choice. An input field
precedes the first choice in the selection field.
Multiple-choice selection fields allow the user to select one or more choices from the selection list.
Choices in a multiple-choice selection field appear with a single character input field in front of each
choice.
The use of TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR is allowed only when the MENU keyword has
been specified on the PANEL tag. ISPF selection menu, edit model, or tutorial selection menu fields
are formatted in a manner similar to single-choice selection fields. Choices appear with sequential
numbers in front of each choice and the user may select only one choice from the selection list. With
these options, the command line is used as the entry choice field. Because the HELP attribute on the
SELFLD tag is not valid when TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR, help for selection menu or
edit model menu choices must be entered on the CMDAREA tag.
Note: Because the selection menu, edit model menu, or tutorial menu panel uses the command line
for choice selection, a command area is required. The conversion utility automatically generates a
command area if no CMDAREA tag is provided.
PMTLOC=ABOVE | BEFORE
This attribute specifies whether the field -pr omp t - t e xt  appears above or in front of the selection field.
SELFLD
Chapter 12. Tag reference  423

## Page 456

PMTWIDTH=n | * | **
This attribute specifies the number of bytes to be used by the prompt for the selection field. When you
specify PMTWIDTH=*, the conversion utility uses the length of the prompt text as the prompt width.
When you specify PMTWIDTH=**, the conversion utility uses the maximum available space as the
prompt width. If any prompt is longer than this value, the prompt is word-wrapped to fit on multiple
lines. The minimum value is 0 and the maximum is the remaining available panel (or region) value.
This value overrides the PMTWIDTH value on an enclosing DTACOL tag.
SELWIDTH=n | *
This attribute specifies the number of bytes used for the choices in the selection field. It is useful
for defining a consistent appearance for the selection choices. If you do not specify the SELWIDTH
parameter on the SELFLD tag, the SELWIDTH parameter on any enclosing DTACOL tag is used. If you
do not specify a SELWIDTH value and SELWIDTH is not specified on an enclosing DTACOL tag, then
the remaining available width of the panel (or current region) determines the width used to format
the choice text. If the SELWIDTH value is specified as “*”, the conversion utility uses the remaining
available width.
If the width required by the choice-description-text and its entry-field exceeds the value specified for
SELWIDTH, the text is word-wrapped to multiple lines.
Note: Because all of the remaining space is used if no SELWIDTH attribute is provided or if
SELWIDTH=“*” is coded, you should specify a SELWIDTH value for fields defined:
• With PMTLOC=BEFORE, because PMTWIDTH is not part of the space reserved by SELWIDTH.
• Within a horizontal region if additional fields are to be formatted to the right of the SELFLD section.
SELWIDTH for selection fields defined within a horizontal region if additional fields are to be formatted
to the right of the SELFLD section.
The width specified for a single-choice selection field should include all or a portion of the choice-
description-text plus 8-13 positions, determined is this way:
• The choice selection entry-field (1-3 characters)
• The entry-field 3270 attributes (2 characters)
• The choice-number inserted by the conversion utility (3-5 characters)
• The 3270 attributes that enclose the choice-description-text (2 characters).
The width of a multiple-choice selection field should include all or a portion of the choice-description-
text plus 5 positions, determined in this way:
• The choice selection entry-fields (1 character)
• The entry-field 3270 attributes (2 characters)
• The 3270 attributes that enclose the choice-description-text (2 characters).
The width specified for a menu-choice, model-choice, or tutorial-choice selection field should include
all or a portion of the choice-description-text plus 4-19 positions, determined in this way:
• The choice selection entry-field (1-16 characters)
• The entry-field 3270 attribute (1 character)
• The 3270 attributes that enclose the choice-description-text (2 characters).
ENTWIDTH=2 | n | 'e1 e2...en'
This attribute is valid only when TYPE=SINGLE, TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR.
Multiple ENTWIDTH values can be used when TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR. For these
types of selection lists, the ENTWIDTH is used only to format the amount of space used by the
selection character(s). The multiple width format is used when CHOICECOLS is greater than 1 to
customize the width required for each column of choices. If the number of ENTWIDTH values is less
than the number of columns, the last (or only) ENTWIDTH value is used for the remaining columns. If
more ENTWIDTH values are supplied than there are columns of choices, the excess ENTWIDTH values
are ignored.
SELFLD
424  z/OS: z/OS ISPF DTL Guide

## Page 457

When TYPE=SINGLE and the value of LISTTYPE is not COMBO, ENTWIDTH specifies the number of
bytes used for both the entry field and the space between the selection identifier and the selection
text. The default width value is 2. The minimum width value is 1, which can be specified for any
single-choice selection list. The maximum width value (when LISTTYPE is not COMBO) is 3, which can
be specified for selection lists within a scrollable panel area. The width of 3 is provided for use when
the number of CHOICE tags exceeds 99.
When LISTTYPE=COMBO, the maximum ENTWIDTH value is 2 bytes less than the SELWIDTH value.
Note: A width of 1 should only be used when the total number of CHOICE tags is less than 10. The
conversion utility discards choices which cannot be selected with the specified entry width.
When TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR, the command area is used as the input field and
the ENTWIDTH value is used only to determine the spacing between the selection identifier and the
selection text. The maximum ENTWIDTH value for these types is 16.
REQUIRED=NO | YES
This attribute indicates if the field requires input.
If REQUIRED=YES is coded, a VER(variable,NONBLANK) statement is built by ISPDTLC and placed in
the )PROC section of the generated ISPF panel.
Note: This attribute is valid only when TYPE=SINGLE.
MSG=message-identifier
This attribute specifies the message that is displayed when the user does not choose a selection
(defined with the REQUIRED attribute). If you do not specify a mes sage -identifier , ISPF displays a
default message.
If you specify the MSG attribute and REQUIRED=YES, a VER(variable,NONBLANK,MSG=message-
identifier) statement is built by ISPDTLC and placed in the )PROC section of the generated ISPF
panel. If you specify the MSG attribute and REQUIRED=NO (the default), the conversion utility
issues a warning message.
FCHOICE=1 | 0
The FCHOICE attribute controls the starting choice number for TYPE=SINGLE, TYPE=MENU,
TYPE=MODEL or TYPE=TUTOR. When FCHOICE=0, the first choice is the number 0.
Note: This attribute is valid only when TYPE=SINGLE, TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR.
AUTOTAB=YES | NO
When AUTOTAB=YES, the cursor moves to the next field capable of input when the user enters the
last character in this field. If no other field capable of user input exists on the panel, the cursor returns
to the beginning of this field.
Note: This attribute is valid only when TYPE=SINGLE.
DEPTH=n | *
This attribute defines the minimum size of a scrollable selection list. If DEPTH is not specified, the
selection list is not scrollable. If the DEPTH value is specified as “*”, the conversion utility reserves the
remaining available panel depth. When EXTEND=OFF, the minimum depth is 2. When EXTEND=ON,
the minimum depth is 1. The DEPTH attribute is ignored when LISTTYPE=COMBO.
EXTEND=OFF | ON | FORCE
This attribute defines the runtime display size for the scrollable list area. If EXTEND=ON is
specified, the panel definition is expanded from the minimum DEPTH to the size of the logical
screen. Only one EXTEND=ON attribute value is allowed on a panel. The first tag (AREA, DA, GA,
REGION, SELFLD) with EXTEND=ON is accepted; the EXTEND attribute on any subsequent tag is
ignored.
If the EXTEND attribute is specified without the DEPTH attribute, a warning message is issued and
the EXTEND attribute is ignored. The EXTEND attribute is ignored when LISTTYPE=COMBO.
If you intend to display the panels in a pop-up window, it is recommended that you code
EXTEND=OFF.
SELFLD
Chapter 12. Tag reference  425

## Page 458

If EXTEND=FORCE is specified within a horizontal area or region, the EXTEND(ON) keyword is
added to the scrollable area attribute statement in the )ATTR panel section. The conversion utility
issues a message to advise of a potential display error if other panel fields are formatted on or
after the last defined line of the scrollable area.
TRAIL='trail-var-1 trail-var-2 ... trail-var-n'
This attribute specifies variable name(s) that the application uses to obtain the TRAIL information
created by menu or model selection processing.
Each trail variable specified must follow the standard naming convention described in “Rules for
variable names” on page 179.
Note: This attribute is valid only when TYPE=MENU or TYPE=MODEL.
CHOICECOLS=1 | n
This attribute specifies the number of columns to format with the CHOICE items. The default is 1. The
CHOICECOLS attribute is ignored when LISTTYPE=COMBO.
CHOICEDEPTH=n | *
This attribute specifies the number of CHOICE entries to be placed in each column. The minimum
CHOICEDEPTH value is 1. The normal maximum and default is the remaining panel depth. If the
DEPTH attribute has been specified on the SELFLD tag, or an enclosing REGION or AREA tag, (and the
corresponding tag attribute value for EXTEND is OFF) the most recently specified depth value is used
as the maximum and default value. You may specify CHOICEDEPTH=“*” which tells the conversion
utility to calculate the column depth based on the total number of CHOICE tags and the number of
columns specified by the CHOICECOLS attribute.
If more CHOICE entries are specified than can be formatted in the available number of columns
specified by the CHOICECOLS attribute, the remaining CHOICE entries are placed in the rightmost
(or only) available column for the current SELFLD tag. The CHOICEDEPTH attribute is ignored when
LISTTYPE=COMBO.
CWIDTHS=‘w1 w2...wn’
This attribute specifies the number of bytes to be allocated for each column of CHOICE entries. The
‘w1 w2…wn’ notation provides the number of bytes for each column. You may use an asterisk or a
number combined with an asterisk to specify a proportional allocation of column space. For example,
the specification of ‘2* * 3*’ for 3 columns would result in a space calculation based on 6 units,
with 2 units allocated to column 1, 1 unit allocated to column 2, and 3 units allocated to column
3. If more columns have been specified by CHOICECOLS than are accounted for by CWIDTHS, the
remaining space is divided evenly between the remaining columns. If CWIDTHS is not specified, the
available formatting space is divided evenly based on the CHOICECOLS value. The CWIDTHS attribute
is ignored when LISTTYPE=COMBO.
PAD=NULLS | USER | char | %varname
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
Note: This attribute is valid only when TYPE=SINGLE.
PADC=NULLS | USER | char | %varname
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
Note: This attribute is valid only when TYPE=SINGLE.
OUTLINE=NONE | L | R | O | U | BOX | %varname
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
Note: This attribute is valid only when TYPE=SINGLE.
SELMSG=selfld-msg-identifier
This attribute specifies the message that is displayed when an invalid single-choice entry is selected.
SELFLD
426  z/OS: z/OS ISPF DTL Guide

## Page 459

SELMSGU=selfld-msg-unavailable
This attribute specifies the message that is displayed when an unavailable single-choice entry is
selected.
INIT=YES | NO | init-value
This attribute controls the single-choice and multi-choice selection field variables initialization in the
panel )INIT section. When INIT = NO, the variables are not initialized to blank. When TYPE = SINGLE,
you can alternatively provide a valid choice selection by specifying INIT = init-value.
CHOICE tag CHECKVAR processing can override the INIT value.
VERIFY=YES | NO
This attribute controls the single-choice verification and menu-choice, model-choice, or tutor-
choice selection logic generation in the panel )PROC section. When TYPE=MENU, TYPE=MODEL,
or TYPE=TUTOR, VERIFY=NO bypasses the creation of the ZSEL statement. You can provide a
replacement ZSEL statement with the <SOURCE> tag.
REFRESH=YES | NO
This attribute controls the creation of the REFRESH statement in the panel )REINIT section for multi-
choice selection lists.
SELFMT=START | END
This attribute controls the placement of the choice selection character(s) within the width specified by
ENTWIDTH. The default is to left justify the choice selection character(s).
Note: This attribute is valid only when TYPE=SINGLE, TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR.
CHKBOX=YES | NO
This attribute controls the creation of panel keywords that enable check boxes on a client that is using
the JSON API. The default value is YES.
The CHKBOX attribute is not valid and is ignored for single-choice, menu-choice, and model-choice
selection lists.
If the conversion utility has been invoked with the NOGUI option, specifying CHKBOX=YES on the
SELFLD tag overrides the invocation option so that check-box controls are generated.
ZGUI=YES | NO
This attribute controls the creation of the "VGET (ZGUI)" statement in the panel )INIT section for
multi-choice selection lists that specify the "&multipmt" ENTITY as field -pr omp t - t e xt .
CSRGRP=NO | YES | n
The CSRGRP attribute is valid only when TYPE=MULTI and CHKBOX=YES (either specified or
defaulted). When CSRGRP=YES, the conversion utility generates a cursor group number to be used
for this selection list. When CSRGRP=n, the number provided is used for the CHOICE fields within this
SELFLD tag.
TSIZE='s1 s2...sn'
The TSIZE attribute provides the number of bytes to indent multiple lines of CHOICE text. Multiple
TSIZE values can be used to provide unique indentation amounts for multiple column lists (when
CHOICECOLS is greater than 1). If the number of TSIZE values is less than the number of columns, the
last (or only) TSIZE value is used for the remaining columns. If more TSIZE values are supplied than
there are columns of choices, the extra TSIZE values are ignored.
LISTTYPE=RADIO | LISTBOX | DDLIST | COMBO
This attribute controls the creation of panel keywords that enable single-choice selection lists to be
displayed with radio buttons on a client that is using the JSON API.
LISTTYPE=LISTBOX, LISTTYPE=DDLIST, and LISTTYPE=COMBO are accepted in order to support
existing DTL source files that use them. However, they no longer affect the displayed panel.
Note: This attribute is valid only when TYPE=SINGLE.
LISTREF=list-name
This attribute is accepted in order to support existing DTL source files that use it. However, it no
longer affects the displayed panel.
SELFLD
Chapter 12. Tag reference  427

## Page 460

LISTDEPTH=n
The LISTDEPTH attribute is accepted in order to support existing DTL source files that use it.
However, it no longer affects the displayed panel.
DBALIGN=YES | NO | FIELD | FORCE
This attribute defines the DBALIGN value. DBALIGN is used only for DBCS language conversions when
PMTLOC=ABOVE and the DBALIGN invocation option is specified.
When DBALIGN=YES, and the field-prompt-text starts with a DBCS character or a single-choice or
multi-choice selection list definition does not include field-prompt-text, the entry field for the choice
is shifted 1 position to the right.
When DBALIGN=NO, no alignment adjustment is made.
When DBALIGN=FIELD, the entry field is shifted but no adjustment is done for the prompt. The FORCE
and FIELD values are useful when alignment is required with other SELFLD or DTAFLD tags.
When DBALIGN=FORCE, the entry field is shifted and the field-prompt-text is also adjusted to match
even if the field-prompt-text starts with a single byte character.
NOSEL=no-selection-value
This attribute provides a value to be placed the CHECKVAR variable (specified by the CHOICE tag)
when no selection is chosen from the available list.
If REQUIRED=YES is specified, a message is issued and NOSEL is ignored.
If no CHOICE tag specifies a CHECKVAR attribute, the NOSEL attribute is ignored.
Note: This attribute is valid only when TYPE=SINGLE.
SELDEFAULT=x
This attribute is used to provide a default choice selection when TYPE=SINGLE, MENU, MODEL, or
TUTOR. The value x must be a valid choice selection. If no selection is made by the user, the default
value is returned to the application.
PMTSKIP=NO | YES
This attribute is used for horizontal formatting of input fields. When PMTSKIP=YES, and the previous
DTAFLD definition includes the NOENDATTR attribute, the cursor moves past the prompt text to the
input field when the user enters the last character in the previous field. If there is no other input field
on the panel, the cursor returns to the first input field on the panel.
FLDTYPE=CUA | ISPF
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
COLOR=WHITE | RED | BLUE | GREEN | PINK | YELLOW | TURQ | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
INTENS=HIGH | LOW | NON | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
HILITE=USCORE | BLINK | REVERSE | %varname
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
SELCHECK = NO | YES
This attribute is used with menu-choice selection to specify that panel logic is to be included in
selection processing to check for valid selection entries. For example, a message is issued if a period
(.) or a period followed by data (.xxx) is entered as a selection choice.
Note: This attribute is valid only when TYPE=MENU.
VARDCL=YES | NO
When VARDCL=NO the field name is not checked to the declared variable information provided with
the VARCLASS and VARDCL tags.
SELFLD
428  z/OS: z/OS ISPF DTL Guide

## Page 461

Note: This attribute is only valid when TYPE=SINGLE.
field-prompt-text
This is the prompt text for the selection field. The prompt text can appear in front of or above the field,
based on the value assigned to the PMTLOC attribute.
Multi-choice selections can be displayed as check boxes on a client that is using the JSON API.
To support both host and JSON API client forms of multi-choice prompt text, a special pre-defined
ENTITY name of "&multipmt" may be specified as the field -pr omp t - t e xt . When the panel is displayed,
the field -pr omp t - t e xt  is
Enter "/" to select option
(or its translated equivalent) for host display or
Check box to select option
(or its translated equivalent) for JSON API client display. The panel definition should specify a
PMTWIDTH value large enough to format the prompt as a single line. If there is insufficient space
to present the entire field -pr omp t - t e xt , it is truncated to fit the available space.
Comments
The SELFLD tag defines a selection field that includes a list of choices. CHOICE tags coded within the
SELFLD definition define the choices for the selection field.
The TYPE attribute of the SELFLD tag determines how the choices appear. If TYPE=SINGLE, the SELFLD
NAME attribute is used as the selection input field. If TYPE=MULTI, the CHOICE NAME attribute is used
as the selection input field for each choice. If TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR, the command
line is used as the selection input field.
When a selection list is formatted as a scrollable list:
• The multi-choice list entry field scrolls with the choice descriptions.
• The single-choice entry field is formatted beside the choice list and remains visible when the choice
descriptions scroll.
• Choice descriptions that are formatted in multiple columns (CHOICECOLS and CHOICEDEPTH attributes
specified) result in a separate scrollable area for each column.
Note: If you specify the CMDAREA tag within your DTL source file, it must appear before the SELFLD tag
when TYPE=MENU, TYPE=MODEL, or TYPE=TUTOR and CHECKVAR or UNAVAIL attributes are specified
on nested CHOICE tags.
If you specify the CMDAREA tag within your DTL source file, it must appear before the SELFLD tag when
DEPTH=* is specified. The SELFLD tag DEPTH may have to be adjusted to allow for additional lines which
result from tags present within the panel definition following the end SELFLD tag.
Restrictions
• The SELFLD tag requires an end tag.
• You must code the SELFLD tag within an AREA, DTACOL, REGION, or PANEL definition. See “AREA
(Area)” on page 189, “DTACOL (Data Column)” on page 269, “REGION (Region)” on page 405, and
“PANEL (Panel)” on page 376 for descriptions of these tags.
• Single-choice selection fields (the default TYPE value) should have an associated VARDCL definition for
the field -name  specified with the NAME attribute. See “VARDCL (Variable Declaration)” on page 449 for
a complete description of this tag.
• If both PAD and PADC have been specified, PAD is ignored and PADC is used.
SELFLD
Chapter 12. Tag reference  429

## Page 462

• When a "%varname" notation is found on any of the attributes that allow a variable name, the
"%varname" entry must follow the standard naming convention described in “Rules for “%variable”
names” on page 179.
• You should code a CMDAREA on any panel that contains a SELFLD definition that specifies TYPE=MENU,
TYPE=MODEL, or TYPE=TUTOR. If you do not include the CMDAREA tag, the conversion utility inserts
one and issues a message, unless the PANEL tag specifies CMDLINE=NO.
• Only one menu-choice or model-choice list is formatted for any panel. If multiple menu-choice or
model-choice lists are specified, the first one is formatted as a menu; subsequent menu-choice or
model-choice lists are formatted as single-choice lists.
Processing
Table 71. Tags you can code within a SELFLD definition 
Tag Reference Usage Required
CHDIV “CHDIV (Choice Divider)” on page 207 Multiple No
CHOICE “CHOICE (Selection Choice)” on page 226 Multiple No
COMMENT “COMMENT (Comment)” on page 245 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SOURCE “SOURCE (Source)” on page 435 Multiple No
Examples
Here is application panel markup that contains two selection fields. The first selection field is a single-
choice selection field with the prompt text located in front of the selection field. The single-choice
selection field can be preselected depending on the value assigned to the variable card.
The second selection field is a multiple-choice selection field with the prompt text located above the
selection field. Choices within this field may be preselected depending on the value assigned to the
CHECKVAR attribute variable specified on the respective CHOICE tags.
SELFLD
430  z/OS: z/OS ISPF DTL Guide

## Page 463

<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 sysem>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=selfld3 KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST>Type in patron's name and card number (if applicable).
<TOPINST>Then select an action bar choice.
<AREA>
  <DTAFLD DATAVAR=curdate PMTWIDTH=12 ENTWIDTH=8 USAGE=out>Date
  <DTAFLD DATAVAR=cardno PMTWIDTH=12 ENTWIDTH=7 DESWIDTH=25>Card No
    <DTAFLDD>(A 7-digit number)
  <DTAFLD DATAVAR=name PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25>Name
    <DTAFLDD>(Last, First, M.I.)
  <DTAFLD DATAVAR=address PMTWIDTH=12 ENTWIDTH=25>Address
  <DIVIDER>
  <REGION DIR=horiz>
  <SELFLD NAME=cardsel PMTWIDTH=30 SELWIDTH=40
          entwidth=1 required=yes autotab=yes>
          Choose one of the following
    <CHOICE CHECKVAR=card MATCH=new>New
    <CHOICE CHECKVAR=card MATCH=renew>Renewal
    <CHOICE CHECKVAR=card MATCH=replace>Replacement
  </SELFLD>
  <SELFLD TYPE=multi PMTWIDTH=30 SELWIDTH=36
          depth=5 init=no>
          Check valid branches
    <CHOICE NAME=north HELP=nthhlp CHECKVAR=nth>North Branch
    <CHOICE NAME=south HELP=sthhlp CHECKVAR=sth>South Branch
    <CHOICE NAME=east HELP=esthlp CHECKVAR=est>East Branch
    <CHOICE NAME=west HELP=wsthlp CHECKVAR=wst>West Branch
    <CHOICE NAME=city HELP=ctyhlp CHECKVAR=cty>City Branch
    <CHOICE NAME=cnty HELP=cnthlp CHECKVAR=cnt>County Branch
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>Enter a command
</PANEL>
Figure 159 on page 432 shows the formatted result.
SELFLD
Chapter 12. Tag reference  431

## Page 464

File  Search  Help
 -------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number (if applicable).
 Then select an action bar choice.
 Date . . . : ________
 Card No  . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
                                         Check valid branches
 Choose one of the following            #SAREA37                           #
 _  1. New                              #                                  #
    2. Renewal                          #                                  #
    3. Replacement                      #                                  #
                                        #                                  #
 Enter a command ===> ____________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Here are the contents of the scrollable area:
          )AREA SAREA37
 _  North Branch
 _  South Branch
 _  East Branch
 _  West Branch
 _  City Branch
 _  County Branch
          )AREA SAREA37
Figure 159. Selection fields 
Here is an example that shows the creation of an ISPF selection menu. The FCHOICE attribute specifies
that the first selection number is 0. The choice selection for Exit is specified on the CHOICE tag. The
ACTION tag for the Exit choice selection specifies both the RUN and TYPE attributes because RUN is
required on the ACTION tag and TYPE is necessary to specify the ISPF selection for the generated ZSEL
panel statement.
SELFLD
432  z/OS: z/OS ISPF DTL Guide

## Page 465

<!doctype dm system ()>
<!--  Sample selection menu -->
<varclass name=vc1 type='char 80'>
  <xlatl format=upper>
  </xlatl>
<varlist>
  <vardcl name=zcmd varclass=vc1>
</varlist>
<panel name=selfld2 menu keylist=keylxmp>Sample Selection Panel
  <topinst>This is a selection panel.
  <selfld type=menu   pmtloc=before fchoice=0 trail=nextsel
          selwidth=40 pmtwidth=10>Select an option
    <choice checkvar=xtest1 match=a>
            Selection #0 (Command Selch0)
      <action run=Selch0>
    <choice checkvar=xtest1 match=b>
            Selection #1 (Command Selch1)
      <action run=Selch1 parm='1 2 3 4'
       passlib newpool suspend>
    <choice checkvar=xtest1 match=c>
            Selection #2 (Command Selch2)
      <action run=Selch2 parm=1234>
    <choice checkvar=xtest1 match=d>
            Selection #3 (Command Selch3)
      <action run=Selch3 parm=abcd>
    <choice checkvar=xtest1 match=e>
            Selection #4 (Command Selch4)
      <action run=Selch4 parm='a b c d'>
    <choice selchar=X>
            Exit
      <action run=exit type=exit>
  </selfld>
  <cmdarea>
</panel>
Figure 160 on page 433 shows the formatted result.
                           Sample Selection Panel
 This is a selection panel.
 Select an
 option . . 0  Selection #0 (Command Selch0)
            1  Selection #1 (Command Selch1)
            2  Selection #2 (Command Selch2)
            3  Selection #3 (Command Selch3)
            4  Selection #4 (Command Selch4)
            X  Exit
 Option ===> _____________________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 160. Selection menu
SL (Simple List)
The SL tag defines a simple list of items within an information region.
SL
Chapter 12. Tag reference  433

## Page 466

Syntax
<SL
COMPACT NOSKIP
SPACE=
NO
YES
INDENT=n TEXT=SL-heading-text
> </SL>
Parameters
COMPACT
This attribute causes the list to be formatted without a blank line between the list items.
NOSKIP
This attribute causes the list to format without creating a blank line before the first line of the list.
SPACE=NO | YES
The SPACE attribute controls the indentation space for the list item. When the SPACE attribute is not
specified on the LI tag, the SPACE attribute from the SL tag is used to set the indentation space for the
nested LI tag item-text.
When SPACE=YES, the indentation is set to 3 spaces. When SPACE=NO (or SPACE is not specified),
the indentation is set to 4 spaces.
The SPACE attribute can be used to control the alignment of list items when the first word of some list
items is a DBCS word preceded by a shift-out character and the first word of other list items is a SBCS
word.
INDENT=n
This attribute specifies that the list be indented from the current left margin.
TEXT=SL-heading-text
This attribute causes the list to format with a heading line containing the SL-heading-text.
Comments
The SL tag defines a simple list of items within an information region.
Simple lists are indented lists, with no bullets, dashes, or hyphens preceding the list items. Nested lists
indent four spaces to the right of the left margin of the list that contains them.
Note: The SPACE attribute does not affect the indentation of nested lists.
The conversion utility adds a blank line before the first item in the list.
Use the LI tag to denote each list item. See “LI (List Item)” on page 325 for more information on the LI
tag.
Restrictions
• The SL tag requires an end tag.
• You must code the SL tag within an INFO definition. See “INFO (Information Region)” on page 317 for a
complete description of this tag.
SL
434  z/OS: z/OS ISPF DTL Guide

## Page 467

Processing
Table 72. Tags you can code within an SL definition 
Tag Reference Usage Required
LI “LI (List Item)” on page 325 Multiple No
LP “LP (List Part)” on page 330 Multiple No
Examples
Here is help panel markup that contains two simple lists. The second simple list is compact, and is nested
within the first list. Figure 161 on page 435 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=sl WIDTH=40 DEPTH=22>Help for ShelfBrowse
<AREA>
<INFO>
  <P>Using ShelfBrowse, you can locate the following items:
  <SL>
    <LI>Audiotapes
    <LI>Books
    <LI>Periodicals
      <SL COMPACT>
        <LI>Newspapers
        <LI>Magazines
      </SL>
    <LI>Reference material
    <LI>Videotapes
</SL>
</INFO>
</AREA>
</HELP>
          Help for ShelfBrowse
 Using ShelfBrowse, you can locate the
 following items:
     Audiotapes
     Books
     Periodicals
         Newspapers
         Magazines
     Reference material
     Videotapes
  F1=Help      F3=Exit      F5=Exhelp
  F6=Keyshelp  F7=PrvTopic  F8=NxtTopic
 F10=PrvPage  F11=NxtPage  F12=Cancel
Figure 161. Simple list
SOURCE (Source)
The SOURCE tag defines ISPF panel logic statements within an application panel.
SOURCE
Chapter 12. Tag reference  435

## Page 468

Syntax
<SOURCE
TYPE=
PROC
REINIT
INIT
ABCINIT
ABCPROC
> text </SOURCE>
Parameters
TYPE=PROC | REINIT | INIT | ABCINIT | ABCPROC
This attribute specifies the panel section that is updated with the SOURCE tag text.
text
This is the unformatted ISPF panel statement.
Comments
The SOURCE tag defines ISPF panel statements within an application panel.
Lines of text from a SOURCE tag that follows an AREA, CHOICE, DA, DTACOL, DTAFLD, HELP, LSTCOL,
LSTFLD, LSTGRP, PANEL, REGION, or SELFLD tag are added to the )INIT, )REINIT, or )PROC panel section
when encountered in the DTL source file.
For example, if a SOURCE tag follows the DTAFLD tag, any logic or other entries normally generated by
DTAFLD would be completed before the lines within SOURCE are added.
The use of a SOURCE tag within a SELFLD tag results in the placement of the SOURCE tag lines after
any logic created by the previous CHOICE tag. Additional )INIT, )REINIT, or )PROC section entries may be
added when the end SELFLD tag is processed. You can control the placement of the SOURCE tag entries
by nesting the SELFLD tag definition within a DTACOL tag, and placing the SOURCE tag definition either
before or after the SELFLD tag definition.
Lines of text from a SOURCE tag within an action bar definition are added to:
• )ABCINIT following all other generated statements for that PDC tag.
• )ABCPROC before any other generated statements for that PDC tag.
SOURCE tags within an action bar definition must specify the TYPE as ABCINIT or ABCPROC. SOURCE
tags that follow the other listed tags cannot specify TYPE as ABCINIT or ABCPROC.
When the SOURCE tag is coded within a GENERATE tag, the TYPE attribute is ignored. TYPE is
automatically determined from the placement of the GENERATE tag within the DTL source file.
If the length of any line exceeds the record length of the output panel file, the conversion utility truncates
the line and issues a warning message.
Text found between the SOURCE and SOURCE end tags is placed in the specified panel section as coded;
that is, no formatting except entity substitution is performed. To refer to an entity within <SOURCE> tag
text, the entity name is preceded by a percent (%) instead of an ampersand (&). Using the percent (%)
sign avoids conflict with variable names. A valid percent sign can be specified as "%amp;" to avoid an
"entity not found" message. For example, you would refer to the TSO command "%xyz" as “%amp;xyz”.
Restrictions
• The SOURCE tag requires an end tag.
SOURCE
436  z/OS: z/OS ISPF DTL Guide

## Page 469

• You must code the SOURCE tag within an ABC, AREA, CHOICE, DA, DTACOL, DTAFLD, GENERATE, HELP,
INFO, LSTCOL, LSTFLD, LSTGRP, PANEL, PDC, REGION, or SELFLD tag definition.
Processing
None.
Examples
<!DOCTYPE DM SYSTEM(
  <!entity sampvar1 sysem>
  <!entity sampabc system>)>
&sampvar1;
<PANEL NAME=source1 KEYLIST=keylxmp>Library Card Registration
<AB>
&sampabc;
</AB>
<TOPINST>Type in patron's name and card number (if applicable)
<TOPINST>Then select an action bar choice.
<AREA>
    <DTACOL PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25 SELWIDTH=25>
    <DTAFLD DATAVAR=curdate USAGE=out ENTWIDTH=8>Date
    <DTAFLD DATAVAR=cardno ENTWIDTH=7>Card No.
      <DTAFLDD>(A 7-digit number)
    <DTAFLD DATAVAR=name>Name
      <DTAFLDD>(Last, First, M.I.)
    <DTAFLD DATAVAR=address>Address
   </DTACOL>
  <DIVIDER>
  <REGION DIR=horiz>
  <SELFLD NAME=cardsel PMTWIDTH=30 SELWIDTH=38>Choose
  one of the following
    <CHOICE CHECKVAR=card MATCH=new>New
    <CHOICE CHECKVAR=card MATCH=renew>Renewal
    <CHOICE CHECKVAR=card MATCH=replace>Replacement
    <SOURCE TYPE=proc>
    if (&cardsel = 1)
      VER (&name,nb)
      VER (&address,nb)
    </SOURCE>
  </SELFLD>
  <SELFLD TYPE=multi PMTWIDTH=30 SELWIDTH=25>Check valid branches
    <CHOICE NAME=north HELP=nthhlp CHECKVAR=nth>North Branch
    <CHOICE NAME=south HELP=sthhlp CHECKVAR=sth>South Branch
    <CHOICE NAME=east HELP=esthlp CHECKVAR=est>East Branch
    <CHOICE NAME=west HELP=wsthlp CHECKVAR=wst>West Branch
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>Enter a command
</PANEL>
T (Truncation)
The T tag designates the minimum command name that the user must enter to issue a command.
Syntax
<T>
</T>
Comments
T
Chapter 12. Tag reference  437

## Page 470

You must code the T tag within the external-command-name of the CMD tag. For example, imagine this
command is coded in an application command table:
<cmd name=compare>com<t>pare
Then you can enter com, comp, compa, compar, or compare to run the command.
The command name must be at least 2 bytes.
At run time, ISPF runs the first valid command in the command table that matches the character string
entered in the command area.
You should be careful to avoid specifying values that conflict with other commands. For example:
<cmd name=compare>co<t>mpare
<cmd name=copy>co<t>py
In this situation, if the user enters co as a command, ISPF runs the COMPARE command.
Restrictions
• You must code the T tag within the external-command-name of a CMD definition. See “CMD (Command
Definition)” on page 233 for a complete description of this tag.
Processing
None.
Examples
Here is source file markup that contains a command table. The commands DELETE and UPDATE have
truncation definitions that allow the user to enter "del" and “upd”, respectively, as the minimum
command name.
<!DOCTYPE DM SYSTEM>
<CMDTBL APPLID=conv>
  <CMD NAME=update>Upd<T>ate
    <CMDACT ACTION='alias add'>
  <CMD NAME=add>Add
    <CMDACT ACTION=setverb>
  <CMD NAME=delete>Del<T>ete
    <CMDACT ACTION=passthru>
  <CMD NAME=search>Search
    <CMDACT ACTION=passthru>
</CMDTBL>
Here is a table that shows the resultant ISPF application command table.
Table 73. ISPF application command table
ZCTVERB ZCTTRUNC ZCTACT
UPDATE 3 ALIAS ADD
ADD 0 SETVERB
DELETE 3 PASSTHRU
SEARCH 0 PASSTHRU
T
438  z/OS: z/OS ISPF DTL Guide

## Page 471

TEXTLINE (Text Line)
The TEXTLINE tag generates a single line of text to replace the regular tag text for the HELP and PANEL
tags.
Syntax
<TEXTLINE> </TEXTLINE>
Comments
The TEXTLINE tag encloses one or more TEXTSEG tags, used to define the parts or segments of the
replacement text. Text defined by the TEXTSEG tag(s) is accumulated in a left to right order. The resulting
text is used to create or replace the text portion of the HELP or PANEL tag definition.
Restrictions
• The TEXTLINE tag requires an end tag.
• You must code the TEXTLINE tag within a HELP or PANEL tag definition.
Processing
Table 74. Tags you can code within a TEXTLINE definition 
Tag Reference Usage Required
DTAFLD “DTAFLD (Data Field)” on page 275 Multiple No
TEXTSEG “TEXTSEG (Text Segment)” on page 439 Multiple Yes
Examples
See the example for “TEXTSEG (Text Segment)” on page 439.
TEXTSEG (Text Segment)
The TEXTSEG tag creates a text segment to be accumulated for the replacement text line created by the
TEXTLINE tag.
Syntax
<TEXTSEG
EXPAND= AFTER
BEFORE
BOTH
WIDTH= n
> text
</TEXTSEG>
TEXTLINE
Chapter 12. Tag reference  439

## Page 472

Parameters
EXPAND=ABOVE | BEFORE | BOTH
This attribute specifies whether expand control is added to the provided text. Expand characters are
obtained from the HELP or PANEL tag definition, if available. If no expand character(s) have been
specified on those tags, the conversion utility generates the necessary character. You may place the
expand control before, after, or both before and after the text.
WIDTH=n
This attribute specifies the number of bytes to reserve for the text. The default is to not allow space
beyond the actual text length.
Text
This is the text of the segment.
Comments
The TEXTSEG tag defines a part or segment of a replacement text line. When multiple TEXTSEG tags are
present within the TEXTLINE definition, the replacement text line is created from left to right in the order
the TEXTSEG tags are coded.
Restrictions
• You must code the TEXTSEG tag within a TEXTLINE tag definition.
• If the EXPAND attribute is not specified and the resulting replacement text is less than the panel width,
the text is centered as the panel title.
Processing
Table 75. Tags you can code within a TEXTSEG definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
Examples
Here is an example that uses the TEXTLINE and TEXTSEG tags to create a special panel title that includes
the system time and date. Because the EXPAND attribute is specified in the second TEXTSEG tag, the
resulting title replacement text has the time and date fields placed at the left and right panel border.
TEXTSEG
440  z/OS: z/OS ISPF DTL Guide

## Page 473

<!doctype dm system ()>
<!--   Sample selection menu -->
<varclass name=vc1 type='char 80'>
   <xlatl format=upper>
   </xlatl>
<varlist>
  <vardcl name=zcmd varclass=vc1>
</varlist>
<panel name=textseg1 menu keylist=keylxmp>
   <textline>
     <textseg>&ztime
     <textseg expand=both>
           Sample Selection Panel with TEXTLINE tag
     <textseg>&zdate(8)
   </textline>
  <topinst>This is a selection panel.
  <selfld type=menu   pmtloc=before fchoice=0 trail=nextsel
          selwidth=40 pmtwidth=10>Select an option
    <choice checkvar=xtest1 match=a>
                Selection #0 (Command Selch0)
      <action run=Selch0>
    <choice checkvar=xtext1 match=b>
                Selection #1 (Command Selch1)
      <action run=Selch1 parm='1 2 3 4'>
       passlib newpool suspend
    <choice checkvar=xtest1 match=c>
                Selection #2 (Command Selch2)
      <action run=Selch2 parm=1234>
    <choice checkvar=xtest1 match=d>
                Selection #3 (Command Selch3)
      <action run=Selch3 parm=abcd>
    <choice checkvar=xtest1 match=e>
                Selection #4 (Command Selch4)
      <action run=Selch4 parm='a b c d'>
    <chdiv>
    <choice selchar=x>
            Exit
      <action run=exit type=exit>
  </selfld>
  <cmdarea>
</panel>
 07:30           Sample Selection Panel with TEXTLINE tag           99/12/15
 Option ===> _
 
 This is a selection panel.
 
 Select an
 option . . 0  Selection #0 (Command Selch0)
            1  Selection #1 (Command Selch1)
            2  Selection #2 (Command Selch2)
            3  Selection #3 (Command Selch3)
            4  Selection #4 (Command Selch4)
 
            X  Exit
 
 
 
 
 
 
 
  
TOPINST (Top Instruction)
The TOPINST tag defines top instructions for an application panel.
TOPINST
Chapter 12. Tag reference  441

## Page 474

Syntax
<TOPINST
COMPACT
>
instruction-text </TOPINST>
Parameters
COMPACT
This attribute causes the top instruction to format without a blank line after the text.
instruction-text
This is the text of the top instruction. The instruction-text must fit in the remaining panel depth.
Comments
The TOPINST tag defines top instructions for an application panel. The instruction-text formats as a
paragraph based on the width of the application panel. You can code multiple paragraphs of instruction
text by using a new top instruction tag for each new paragraph.
If the COMPACT attribute is not specified, the conversion utility inserts a blank line after the top
instruction text.
Restrictions
• You must code the TOPINST within a PANEL definition. See “PANEL (Panel)” on page 376 for a complete
description of this tag.
• You cannot code a TOPINST tag within an AREA definition. If you define an area for the panel, code the
TOPINST tag before the AREA start tag.
Processing
Table 76. Tags you can code within a TOPINST definition 
Tag Reference Usage Required
HP “HP (Highlighted Phrase)” on page 315 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
Examples
Here is application panel markup that contains top instructions. Figure 162 on page 443 shows the
formatted result.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=selcls TYPE='CHAR 2'>
<VARLIST>
  <VARDCL NAME=loc  VARCLASS=selcls>
  <VARDCL NAME=mode VARCLASS=selcls>
</VARLIST>
<PANEL NAME=topinst HELP=trvlhlp WIDTH=60 DEPTH=22 KEYLIST=keylxmp>
Dream Vacation Guide
<AB>
  <ABC>File
    <PDC>Add Entry
        <ACTION RUN=add>
    <PDC>Delete Entry
TOPINST
442  z/OS: z/OS ISPF DTL Guide

## Page 475

<ACTION RUN=delete>
    <PDC>Update Entry
        <ACTION RUN=update>
    <PDC>Exit
        <ACTION RUN=exit>
  <ABC>Help
    <PDC>Extended Help...
        <ACTION RUN=exhelp>
    <PDC>Keys Help...
        <ACTION RUN=keyshelp>
</AB>
<TOPINST>Choose one of the following exotic locations and
your preferred mode of travel, then press Enter.
<AREA>
  <REGION DIR=horiz>
  <SELFLD NAME=loc PMTWIDTH=23 SELWIDTH=25>Exotic Location:
    <CHOICE>Athens, GA
    <CHOICE>Berlin, CT
    <CHOICE>Cairo, IL
    <CHOICE>Lizard Lick, NC
    <CHOICE>Paris, TX
    <CHOICE>Rome, NY
    <CHOICE>Venice, FL
  </SELFLD>
  <DIVIDER>
  <SELFLD NAME=mode PMTWIDTH=25 SELWIDTH=25>Travel Mode:
    <CHOICE>Boxcar
    <CHOICE>Hitchhike
    <CHOICE>Mule
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>
</PANEL>
   File  Help
 ---------------------------------------------------------
                    Dream Vacation Guide
 Choose one of the following exotic locations and your
 preferred mode of travel, then press Enter.
 Exotic Location:            Travel Mode:
 __  1.  Athens, GA          __  1.  Boxcar
     2.  Berlin, CT              2.  Hitchhike
     3.  Cairo, IL               3.  Mule
     4.  Lizard Lick, NC
     5.  Paris, TX
     6.  Rome, NY
     7.  Venice, FL
 Command ===>_______________________________________________
  F1=Help     F2=Split       F3=Exit     F6=Keyshelp
  F9=Swap    F12=Cancel
Figure 162. Top instructions
UL (Unordered List)
The UL tag defines an unordered list of items within an information region.
UL
Chapter 12. Tag reference  443

## Page 476

Syntax
<UL
COMPACT NOSKIP
SPACE=
NO
YES
INDENT=n TEXT=UL-heading-text
> </UL>
Parameters
COMPACT
This attribute causes the list to be formatted without a blank line between the list items.
NOSKIP
This attribute causes the list to format without creating a blank line before the first line of the list.
SPACE=NO | YES
The SPACE attribute controls the indentation space for the list item. When the SPACE attribute is not
specified on the LI tag, the SPACE attribute from the UL tag is used to set the indentation space for the
nested LI tag item-text.
When SPACE=YES, the indentation is set to 3 spaces. When SPACE=NO (or SPACE is not specified),
the indentation is set to 4 spaces.
The SPACE attribute can be used to control the alignment of list items when the first word of some list
items is a DBCS word preceded by a shift-out character and the first word of other list items is a SBCS
word.
INDENT=n
This attribute specifies that the list be indented from the current left margin.
TEXT=UL-heading-text
This attribute causes the list to format with a heading line containing the UL-heading-text.
Comments
The UL tag defines an unordered list of items within an information region. Unordered lists format as
indented lists, with the list item identifier at the left margin. Nested lists indent four spaces to the right of
the left margin of the list that contains them.
Note: The SPACE attribute does not affect the indentation of nested lists.
The conversion utility adds a blank line before the first item in the list. There are three levels of item
identifiers: bullets (o), hyphens (-), and dashes (--). Each level is used successively when you nest
unordered lists.
Panels formatted with the DBCS option use an uppercase ‘O’ as the bullet character.
Use the LI tag to denote each list item. See “LI (List Item)” on page 325 for more information on the LI
tag.
Restrictions
• The UL tag requires an end tag.
• You must code the UL tag within an INFO definition. See “INFO (Information Region)” on page 317 for a
complete description of this tag.
UL
444  z/OS: z/OS ISPF DTL Guide

## Page 477

Processing
Table 77. Tags you can code within a UL definition 
Tag Reference Usage Required
LI “LI (List Item)” on page 325 Multiple No
LP “LP (List Part)” on page 330 Multiple No
Examples
Here is help panel markup that contains two unordered lists. The second unordered list is nested within
the second list item of the first unordered list. Figure 163 on page 445 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=ul DEPTH=22>Help for Reference Section
<AREA>
<INFO>
  <P>Learn everything about anything,
  and more, in our Reference section.
  Our Reference section includes:
  <UL>
    <LI>Atlases
    <LI>Dictionaries
      <UL COMPACT>
        <LI>English
        <LI>Other languages
      </UL>
    <LI>Encyclopedias
    <LI>How-to books
    <LI>Magazines and periodicals
  </UL>
</INFO>
</AREA>
</HELP>
            Help for Reference Section
 Learn everything about anything, and more, in
 our Reference section. Our Reference section
 includes:
 o   Atlases
 o   Dictionaries
     -   English
     -   Other languages
 o   Encyclopedias
 o   How-to books
 o   Magazines and periodicals
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 163. Unordered list
VARCLASS (Variable Class)
The VARCLASS tag defines information related to a class of variables.
VARCLASS
Chapter 12. Tag reference  445

## Page 478

Syntax
<VARCLASS NAME=variable-class-name
TYPE= 'CHAR maximum-length'
'DBCS maximum length'
'MIXED maximum-length'
'ANY maximum-length'
'EBCDIC maximum-length'
'%varname maximum-length'
ITIME
STDTIME
IDATE
STDDATE
JDATE
JSTD
'VMASK maximum-length'
'NUMERIC total-digits
0
fractional-digits '
MSG=message-identifier
>
</VARCLASS>
Parameters
NAME=variable-class-name
This attribute specifies the name of this variable class.
The variable-class-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
TYPE=type of data
This attribute specifies the data type and display length characteristics for variables that refer to the
variable class.
For data fields and list columns, the conversion utility uses the lengths specified in this attribute when
CHOFLD or DTAFLD ENTWIDTH or LSTCOL COLWIDTH attributes cannot otherwise be determined.
The lengths specified control the width of the data field in the panel.
The allowable TYPE values are:
'CHAR maximum-length'
This specifies a character string for which the maximum length, in bytes, is given by maximum-
length.
'DBCS maximum-length'
This is a double-byte character string for which the maximum length, in bytes, is given by
maximum-length. The maximum length must an even number.
'MIXED maximum-length'
This specifies a character string containing single-byte characters, double-byte characters, or
both for which the maximum length, in bytes, is given by maximum-length. Strings of DBCS
characters are delimited by shift-out (SO) and shift-in (SI) codes.
'ANY maximum-length'
This attribute is processed by the conversion utility as TYPE=MIXED.
VARCLASS
446  z/OS: z/OS ISPF DTL Guide

## Page 479

'EBCDIC maximum-length'
This specifies a character string containing only single-byte characters for which the maximum
length, in bytes, is given by maximum-length.
'%varname maximum-length'
This specifies that a variable name is used to define the type of character string. The maximum
length, in bytes, is given by maximum-length. It is the responsibility of the application developer to
ensure that %varname contains a valid TYPE value before attempting to display the panel.
ITIME
The conversion utility adds a "VEDIT (variable)" statement to the )PROC section of the panel for
variables which are related to this VARCLASS. The default length value of ITIME is set by the
conversion utility to 5.
STDTIME
The conversion utility adds a "VEDIT (variable)" statement to the )PROC section of the panel for
variables which are related to this VARCLASS. The default length value of STDTIME is set by the
conversion utility to 8.
IDATE
The conversion utility adds a "VEDIT (variable)" statement to the )PROC section of the panel for
variables which are related to this VARCLASS. The default length value of IDATE is set by the
conversion utility to 8.
STDDATE
The conversion utility adds a "VEDIT (variable)" statement to the )PROC section of the panel for
variables which are related to this VARCLASS. The default length value of STDDATE is set by the
conversion utility to 10.
JDATE
This attribute is supported as an ISPF extension to the Dialog Tag Language. The conversion utility
adds a "VEDIT (variable)" statement to the )PROC section of the panel for variables which are
related to this VARCLASS. The default length value of JDATE is set by the conversion utility to 6.
JSTD
This attribute is supported as an ISPF extension to the Dialog Tag Language. The conversion utility
adds a "VEDIT (variable)" statement to the )PROC section of the panel for variables which are
related to this VARCLASS. The default length value of JSTD is set by the conversion utility to 8.
'VMASK maximum-length'
This attribute is supported as an ISPF extension to the Dialog Tag Language. The VMASK attribute
is provided to support the user mask option the ISPF VMASK service. The maximum-length value
is limited to the ISPF maximum of 20. The conversion utility adds a "VEDIT (variable)" statement
to the )PROC section of the panel for variables which are related to this VARCLASS.
'NUMERIC total-digits 0 | fractional-digits'
This attribute allows you to check to see if the user has entered a valid number. A valid number
can include thousands separators, a decimal separator, and a sign. The conversion utility builds
the VER(variable ENUM) statement to perform numeric validation. The value specified for total-
digits must not be greater than 16.
The total-digits and fractional-digits are used to determine a maximum-length value which is used
for field entry width, if necessary, in DTAFLD and LSTCOL processing. For example, ‘NUMERIC 8 2’
defines a width of 11, composed of 8 possible digits, a decimal point, a thousands separator, and a
leading sign.
Note: ISPF does not check to verify proper positioning of the decimal point. See the discussion
on VER(variable ENUM) in the z/OS ISPF Dialog Developer's Guide and Reference for more
information.
MSG=message-identifier
This attribute indicates the default message to be displayed if the variable fails any of the enclosed
checks. See “MSG (Message)” on page 352 for information on creating messages.
VARCLASS
Chapter 12. Tag reference  447

## Page 480

Comments
The VARCLASS tag defines information related to a class of variables. You can group validation and
translation checks you want ISPF to perform within one VARCLASS definition. You point to the VARCLASS
definition from one or more VARDCL tags you code within the VARLIST definition.
Note: The ISPF Dialog Tag Language conversion utility does not require that you code the VARCLASS,
VARDCL, or VARLIST tags for a successful generation of a panel, command table, or message member
that includes variables. If the conversion utility finds a variable that does not have an associated VARDCL
definition, it issues a warning message.
The use of the VARCLASS, VARDCL, and VARLIST tags is required if you want to use the facilities provided
by the CHECKL and XLATL tags.
Restrictions
• You cannot code the VARCLASS tag within any other tag definition.
• You must code the VARCLASS tag before any other tag within the source file that refers to it.
• Within the variable class definition, you must code any and all XLATL tags before any CHECKL tags.
Processing
Table 78. Tags you can code within a VARCLASS definition 
Tag Reference Usage Required
CHECKL “CHECKL (Validity Check List)” on page 218 Multiple No
XLATL “XLATL (Translate List)” on page 458 Multiple No
Examples
Here is an example that contains two variable classes. The first variable class provides an alphabetic
validity check. The second variable class provides input translation to uppercase and validates that the
input is one of the listed values. Also shown in the markup are two input data fields (within a PANEL
definition) that refer to the variable declarations associated with the variable classes.
VARCLASS
448  z/OS: z/OS ISPF DTL Guide

## Page 481

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=namec TYPE='char 25'>
   <CHECKL>
      <CHECKI TYPE=alpha>
   </CHECKL>
<VARCLASS NAME=officec TYPE='char 4'>
   <XLATL FORMAT=upper>
   </XLATL>
   <CHECKL>
      <CHECKI TYPE=values PARM1=EQ PARM2='A101 A108 B210 B214'>
   </CHECKL>
<VARLIST>
   <VARDCL NAME=reserver VARCLASS=namec>
   <VARDCL NAME=office VARCLASS=officec>
</VARLIST>
<PANEL NAME=varclass>Conference Room
   <TOPINST>Enter the required information to reserve a conference room.
   <AREA>
      <DTACOL PMTWIDTH=20>
        <DTAFLD DATAVAR=reserver USAGE=in ENTWIDTH=25>Name
        <DTAFLD DATAVAR=office USAGE=in ENTWIDTH=4>Office number
      </DTACOL>
   </AREA>
</PANEL>
VARDCL (Variable Declaration)
The VARDCL tag declares variables referred to in dialog element definitions.
Syntax
<VARDCL NAME=name VARCLASS=variable-class-name >
</VARDCL>
Parameters
NAME=name
This attribute specifies the name of a variable used elsewhere in the DTL source file. The name must
follow the standard naming convention described in “Rules for variable names” on page 179.
VARCLASS=variable-class-name
This attribute specifies the default variable class associated with the variable. If you want to perform
a different set of checks or translations on any data field or list column, you can specify an overriding
variable class in the DTAFLD or LSTCOL tags.
Comments
The VARDCL tag declares variables referred to in dialog element definitions.
Note: The ISPF Dialog Tag Language conversion utility does not require that you code the VARCLASS,
VARDCL, or VARLIST tags for successful generation of a panel, command table, or message member that
includes variables. If the conversion utility finds a variable that does not have an associated VARDCL
definition, it issues a warning message.
The use of the VARCLASS, VARDCL, and VARLIST tags is required if you want to use the facilities provided
by the CHECKL and XLATL tags.
VARDCL
Chapter 12. Tag reference  449

## Page 482

Restrictions
• You must code the VARDCL tag within a VARLIST tag. See “VARLIST (Variable List)” on page 450 for a
complete description of this tag.
Processing
None.
Examples
Here is source file markup that contains variable declarations for all of the variables defined in the panel
definition. The declared variables include:
• The variable whchsrch specified in the CHECKVAR attributes associated with the pull-down choices of
the Search action bar choice.
• The data field variables curdate, cardno, name, and address.
• The variable cardsel, which is the entry-field of the single-choice selection field.
• The variables north, south, east, and west, which are the entry-fields associated with the multiple-
choice selection field.
• The variables defined as the check variables (CHECKVAR attribute) for the selection fields.
<!DOCTYPE DM SYSTEM(
  <!entity sampabc sysem>
  <!entity sampbody system>)>
<VARCLASS NAME=date    TYPE='char 8'>
<VARCLASS NAME=numcls  TYPE='numeric 7'>
<VARCLASS NAME=namecls TYPE='char 25'>
<VARCLASS NAME=char1cls TYPE='char 1'>
<VARCLASS NAME=char7cls TYPE='char 7'>
<VARLIST>
  <VARDCL NAME=whchsrch VARCLASS=char1cls>
  <VARDCL NAME=curdate VARCLASS=date>
  <VARDCL NAME=cardno  VARCLASS=numcls>
  <VARDCL NAME=name    VARCLASS=namecls>
  <VARDCL NAME=address VARCLASS=namecls>
  <VARDCL NAME=cardsel VARCLASS=char1cls>
  <VARDCL NAME=card VARCLASS=char7cls>
  <VARDCL NAME=north VARCLASS=char1cls>
  <VARDCL NAME=south VARCLASS=char1cls>
  <VARDCL NAME=east VARCLASS=char1cls>
  <VARDCL NAME=west VARCLASS=char1cls>
  <VARDCL NAME=nth VARCLASS=char1cls>
  <VARDCL NAME=sth VARCLASS=char1cls>
  <VARDCL NAME=est VARCLASS=char1cls>
  <VARDCL NAME=wst VARCLASS=char1cls>
</VARLIST>
<PANEL NAME=vardcl>Library Card Registration
<AB>
&sampabc;
</AB>
&sampbody;
</PANEL>
VARLIST (Variable List)
The VARLIST tag provides the means to code VARDCL tags in a single list.
VARLIST
450  z/OS: z/OS ISPF DTL Guide

## Page 483

Syntax
<VARLIST> </VARLIST>
Comments
The VARLIST tag provides the means to code VARDCL tags in a single list. The VARDCL tags coded within
a VARLIST definition declare variables that are referred to in the dialog element definitions within a DTL
source file.
Note: The ISPF Dialog Tag Language conversion utility does not require that you code the VARCLASS,
VARDCL, or VARLIST tags for a successful generation of a panel, command table, or message member
that includes variables. If the conversion utility finds a variable that does not have an associated VARDCL
definition, it issues a warning message.
The use of the VARCLASS, VARDCL, and VARLIST tags is required if you want to use the facilities provided
by the CHECKL and XLATL tags.
Restrictions
• The VARLIST tag requires an end tag.
• You cannot code the VARLIST tag within any other tag definition.
• You can code the VARLIST tag immediately after all VARCLASS tags within the DTL source file and
before any tag definitions that refer to the variables declared in the variable list.
Processing
Table 79. Tags you can code within a VARLIST definition 
Tag Reference Usage Required
VARDCL “VARLIST (Variable List)” on page 450 Multiple No
Examples
Here is source file markup that contains a variable list. The variable declarations within the list define
variables for the fields within the PANEL definitions that refer to them.
VARLIST
Chapter 12. Tag reference  451

## Page 484

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=char8 TYPE='char 8'>
<VARCLASS NAME=name  TYPE='char 25'>
<VARCLASS NAME=phoncls TYPE='char 12'>
<VARCLASS NAME=appcls  TYPE='char 1'>
  <XLATL FORMAT=upper>
  </XLATL>
  <CHECKL>
    <CHECKI TYPE=values PARM1=EQ PARM2='Y N'>
  </CHECKL>
<VARLIST>
  <VARDCL NAME=curdate VARCLASS=char8>
  <VARDCL NAME=namevar VARCLASS=name>
  <VARDCL NAME=passvar VARCLASS=char8>
  <VARDCL NAME=xlname  VARCLASS=name>
  <VARDCL NAME=xphone  VARCLASS=phoncls>
  <VARDCL NAME=xapp    VARCLASS=appcls>
</VARLIST>
<PANEL NAME=varlist1 KEYLIST=keylxmp>System Log On
<TOPINST>Complete the following fields, then press Enter.
<AREA>
  <DTACOL PMTWIDTH=12>
    <DTAFLD DATAVAR=curdate ENTWIDTH=8 USAGE=out>Date
    <DTAFLD DATAVAR=namevar ENTWIDTH=25 DESWIDTH=15>Name
    <DTAFLD DATAVAR=passvar ENTWIDTH=8 DISPLAY=no>Password
  </DTACOL>
</AREA>
</PANEL>
<PANEL NAME=varlist2 DEPTH=14 KEYLIST=keyltbl>Subscriber List
<TOPINST>Enter phone number, if missing,
(format - nnn-nnn-nnnn) and approved
indicator (y or n) for each person.
<AREA>
  <LSTFLD>
      <LSTCOL DATAVAR=xlname USAGE=out COLWIDTH=25>Last Name
      <LSTCOL DATAVAR=xphone COLWIDTH=12>Phone Number
    <LSTGRP>Approved
      <LSTCOL DATAVAR=xapp USAGE=in REQUIRED=yes
        COLWIDTH=1 MSG=msgv886>(Y or N)
    </LSTGRP>
  </LSTFLD>
</AREA>
<CMDAREA>Enter a command
</PANEL>
VARSUB (Variable Substitution)
The VARSUB tag specifies a variable to substitute in message text.
Syntax
<VARSUB VAR=variable-name >
</VARSUB>
Parameters
VAR=variable-name
This attribute specifies the variable whose value is substituted within the message.
The variable-name should be declared with a VARDCL tag.
The variable-name must follow the standard naming convention described in “Rules for variable
names” on page 179.
VARSUB
452  z/OS: z/OS ISPF DTL Guide

## Page 485

Comments
The VARSUB tag specifies a variable to substitute in message text. You use the required VAR attribute
to specify the variable whose value is resolved and inserted into the message text when the message is
displayed. The value coded must be a variable name without leading % notation.
You can code the VARSUB tag in the message-text of a MSG tag. The variable value is inserted by ISPF at
run time at the position in the message text where the VARSUB tag is coded.
For example, assume this VARSUB tag was coded within the text of this message:
<msgmbr name=abca00>
<msg suffix=1 msgtype=warning>Invalid name,
“<VARSUB VAR=LASTN>”, specified.
The name may contain only alphabetic characters.
</msgmbr>
When a dialog refers to a message abca001 (with a GETMSG, SETMSG, DISPLAY, or TBDISPL service call)
or the message is displayed by ISPF during panel validation, the value of lastn is retrieved and inserted
into the message text. Here is the message after substitution:
Invalid name, “Jones1”, specified.
The name may contain only alphabetic characters.
Restrictions
• You must code the VARSUB tag within the text of a MSG definition. See “MSG (Message)” on page 352
for a complete description of this tag.
• The value specified by the VAR attribute should be declared with a VARDCL tag. See “VARDCL (Variable
Declaration)” on page 449 for a complete description of this tag.
Processing
None.
Examples
Here is markup that contains a message member which contains nine MSG definitions. The text of
messages MSGV883 and MSGV888 contain variable substitutions. Figure 164 on page 454 shows the
generated message member.
VARSUB
Chapter 12. Tag reference  453

## Page 486

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=msgcls TYPE='char 20'>
<VARLIST>
  <VARDCL NAME=phoneno VARCLASS=msgcls>
  <VARDCL NAME=cnum    VARCLASS=msgcls>
</VARLIST>
<MSGMBR NAME=msgv88>
  <MSG SUFFIX=1>Name must be alphabetic.
  <MSG SUFFIX=2>Enter only number of days.
  <MSG SUFFIX=3 MSGTYPE=critical>The only rooms we have available
  are either SINGLE or DOUBLE.  Please call the manager of the hotel
  who will arrange equivalent lodging at another
  hotel in the area.  This is our mistake, and we will, of course,
  pick up the bill.  Please call collect <VARSUB VAR=phoneno>.
  <MSG SUFFIX=4 MSGTYPE=action LOCATION=modal>Please enter either
  BIGCHARGE, V I S T A, EZCARD, CHECK, or CASH.
  <MSG SUFFIX=5 MSGTYPE=warning LOCATION=modeless>Please enter your name.
  <MSG SUFFIX=6>Please enter Y or N.
  <MSG SUFFIX=7>Card number is a seven-digit number.
  <MSG SUFFIX=8 MSGTYPE=warning>The card number you
  entered, <VARSUB VAR=cnum> is not valid.
  <MSG SUFFIX=9>Message '9' contains embedded quotes.
</MSGMBR>
MSGV881 .TYPE=NOTIFY
'Name must be alphabetic.'
MSGV882 .TYPE=NOTIFY
'Enter only number of days.'
MSGV883 .TYPE=CRITICAL
'The only rooms we have available are either SINGLE or DOUBLE. Please call th' +
'e manager of the hotel who will arrange equivalent lodging at another hotel ' +
'in the area. This is our mistake, and we will, of course, pick up the bill. ' +
'Please call collect &PHONENO.'
MSGV884 .TYPE=ACTION .WINDOW=RESP
'Please enter either BIGCHARGE, V I S T A, EZCARD, CHECK, or CASH.'
MSGV885 .TYPE=WARNING .WINDOW=NORESP
'Please enter your name.'
MSGV886 .TYPE=NOTIFY
'Please enter Y or N.'
MSGV887 .TYPE=NOTIFY
'Card number is a seven-digit number.'
MSGV888 .TYPE=WARNING
'The card number you entered, &CNUM is not valid.'
MSGV889 .TYPE=NOTIFY
'Message '9'' contains embedded quotes.'
Figure 164. Variable substitution
WARNING (Warning)
The WARNING tag defines text that alerts the user to a risk of possible error conditions in the system.
Syntax
<WARNING>
text
</WARNING>
Parameters
text
This is the text of the warning.
Comments
The WARNING tag defines text that alerts the user to a risk of possible error conditions in the system.
WARNING
454  z/OS: z/OS ISPF DTL Guide

## Page 487

The WARNING tag is one of the tags that alert the user of a possible risk; the others are the CAUTION tag
and the ATTENTION tag.
Code a warning statement before the text to which it pertains so that the user can read about the possible
risks before reading the text.
When a warning statement is displayed, the string "Warning:" (or its translated equivalent) appears on the
screen before the text of the warning statement.
You can code additional paragraphs of warning text by coding the P (paragraph) tag within a WARNING
definition. You can also code other tags allowed in an information area within a WARNING definition.
Restrictions
• The WARNING tag requires an end tag.
• You must code the WARNING tag within an INFO definition. See “INFO (Information Region)” on page
317 for a complete description of this tag.
• The WARNING tag must be immediately preceded by a P, LI, or LP tag. If the WARNING tag is coded
on the same line as one of these tags, there can be no intervening blanks. See “P (Paragraph)” on page
370, “LI (List Item)” on page 325, and “LP (List Part)” on page 330 for descriptions of these tags.
• You cannot nest WARNING, ATTENTION, or CAUTION tags within each other.
Processing
Table 80. Tags you can code within a WARNING definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
FIG “FIG (Figure)” on page 291 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
LINES “LINES (Lines)” on page 327 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
XMP “XMP (Example)” on page 460 Multiple No
Examples
Here is help panel markup that contains a warning statement. The warning statement starts at the left
margin because it is embedded in the LP tag.
WARNING
Chapter 12. Tag reference  455

## Page 488

<!DOCTYPE DM SYSTEM>
<HELP NAME=warning DEPTH=20>Help For Changing a File
<AREA>
<INFO>
  <OL>
    <LI>Type over the existing data
    in the entry fields with the new data.
      <LP><WARNING>Performing the next step will save
      all changes and delete the existing data.
      <P>To quit this function without
      deleting the existing data, press F12.
      </WARNING>
    <LI>Press Enter to save the
    updated data.
  </OL>
</INFO>
</AREA>
</HELP>
             Help For Changing a File
 1.  Type over the existing data in the entry
     fields with the new data.
 Warning: Performing the next step will save all
 changes and delete the existing data.
 To quit this function without deleting the
 existing data, press F12.
 2.  Press Enter to save the updated data.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 165. Warning statement
XLATI (Translate Item)
The XLATI tag defines an individual list element in a translate list.
Syntax
<XLATI
VALUE=internal-value
>
displayed-value </XLATI>
Parameters
VALUE=internal-value
ISPF saves this value in the variable pool when translating on input and retrieves it from the variable
pool when translating on output. If the internal-value contains characters other than A-Z, a-z, and 0-9,
you must enclose the value in quotes.
Omitting this attribute indicates that any value is acceptable. When translating on input, ISPF does not
translate the displayed-value before storing it in the pool. When translating on output, ISPF translates
to the displayed-value any value that is not already matched.
displayed-value
This attribute specifies the displayed value that must be matched when doing a translation on input
and the result when doing a translation on output. The test for a translation match is case-sensitive.
Any characters, including embedded blanks, are allowed in the displayed-value. If the value has
XLATI
456  z/OS: z/OS ISPF DTL Guide

## Page 489

blanks that you want preserved, or the value consists of only blanks, the value should be coded within
the LIT (Literal) tag. If the LIT tag is not used, all blanks are stripped and any value with only blanks
indicated that no value was specified.
Omitting this value indicates that any value is acceptable. When translating on output, this means that
the internal-value is not to be translated before being displayed. When translating on input, it means
that any value not already matched is to translate to the internal-value.
Comments
The XLATI tag defines an individual list element in a translate list. As many XLATI tags as are necessary
(up to a limit of 126) to accomplish the desired translation can be included within the translation list.
Each XLATI tag provides information necessary to translate a displayed-value to an internal-value and vice
versa. Translation is done in the order given by the tags. Translation stops when a match is found. An
XLATI tag that omits both internal-value and displayed-value has this effect: when translating on output
the variable value is displayed, and when translating on input the entered value is stored in the variable.
The ISPF TRANS() function is used for all translations. When translating on output, ISPF )INIT panel logic
translates the internal-value to the displayed-value. When translating on input, ISPF )PROC panel logic
translates the displayed-value to the internal-value. The test for a translation match is case-sensitive.
You can code an XLATL FORMAT=UPPER definition before an XLATL definition that contains XLATI tags to
convert user input to uppercase before the translate list is processed.
Restrictions
You must code the XLATI tag within an XLATL definition. See “XLATI (Translate Item)” on page 456 for a
complete description of this tag.
Processing
Table 81. Tags you can code within an XLATI definition 
Tag Reference Usage Required
LIT “LIT (Literal)” on page 329 Multiple No
Examples
Here is source file markup that contains a variable class with a translate list that performs input and
output translation on values assigned to the winter months. The associated variable declarations and
fields are also shown.
XLATI
Chapter 12. Tag reference  457

## Page 490

<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=monthcls type='char 3'>
   <XLATL FORMAT=upper>
   </XLATL>
   <XLATL MSG=abcd003>
       <XLATI VALUE=11>NOV
       <XLATI VALUE=12>DEC
       <XLATI VALUE=01>JAN
       <XLATI VALUE=02>FEB
       <XLATI VALUE=03>MAR
   </XLATL>
<VARCLASS NAME=costcls TYPE='numeric 6' MSG=abcd001>
<VARCLASS NAME=typecls TYPE='char 4'>
   <XLATL FORMAT=upper>
   </XLATL>
   <CHECKL MSG=abcd002>
      <CHECKI TYPE=values Parm1=EQ Parm2= 'GAS OIL ELEC'>
   </CHECKL>
<VARLIST>
   <VARDCL NAME=month VARCLASS=monthcls>
   <VARDCL NAME=cost VARCLASS=costcls>
   <VARDCL NAME=heat VARCLASS=typecls>
</VARLIST>
<PANEL NAME=xlati KEYLIST=keylxmp>Heating Cost Survey
   <TOPINST>Complete the information below and then press Enter.
   <AREA>
     <DTACOL PMTWIDTH=20 DESWIDTH=30>
        <DTAFLD DATAVAR=month ENTWIDTH=3>Winter month
          <DTAFLDD>Enter Nov, Dec, Jan, Feb, or Mar
        <DTAFLD DATAVAR=cost ENTWIDTH=6>Heating cost
        <DTAFLD DATAVAR=heat ENTWIDTH=4>Type of heating
          <DTAFLDD>Enter Oil, Gas, or Elec
     </DTACOL>
   </AREA>
   <BOTINST>Thank you for your participation.
</PANEL>
<MSGMBR NAME=abcd00>
  <MSG SUFFIX=1>Heating cost must be numeric
  <MSG SUFFIX=2>Type of heating must be "Gas", "Oil", or "Elec"
  <MSG SUFFIX=3>Winter month must be "Nov","Dec", "Jan", "Feb", or "Mar"
</MSGMBR>
XLATL (Translate List)
The XLATL tag defines a translate list for a variable class.
Syntax
<XLATL
FORMAT=
NONE
UPPER
NONE TRUNC = n
char
MSG=message-identifier
> </XLATL>
XLATL
458  z/OS: z/OS ISPF DTL Guide

## Page 491

Parameters
FORMAT=NONE | UPPER
This attribute defines the type of translation. NONE specifies that enclosed XLATI tags are to be used
to translate the value on an item for item basis. UPPER specifies that the variable value is translated
to uppercase.
TRUNC=n | char
This attribute defines the type of truncation to be performed on input values. It is valid only when
FORMAT=NONE. If a number is provided, truncation occurs at the length indicated. If a nonnumeric
character is provided, truncation occurs at the first occurrence of that character.
MSG=message-identifier
This attribute specifies the ID of a message to be issued for the error condition that results when an
input translation fails because the user entered a value not specified in the list. Specifying an XLATI
tag with no internal-value and no displayed-value ensures that any value not in the list is accepted
without error. If no message ID is specified and an error occurs, the mes sage -identifier  specified on
the VARCLASS tag is used. If no mes sage -identifier  is specified on the XLATL tag or the VARCLASS tag,
no message is displayed.
Note: This message is not used if translation on output fails. The variable value is displayed as is,
subject to whatever size restrictions apply to the field.
Comments
The XLATL tag defines a translate list for a variable class. XLATI tags, which define the elements of the
translation list, are coded within the XLATL tag. A translation list is defined within a VARCLASS tag.
If FORMAT=NONE is specified, it is expected that there are XLATI tags within the XLATL definition. If
FORMAT=UPPER is specified, no XLATI tags are accepted in the XLATL definition.
Translation lists are optional and provide a means of translating between a displayed value and the
internal value of the variable. Translation can occur on input (the translation result is stored in the variable
pool), on output (the value from the pool is translated before the user sees it), or both, depending on the
USAGE attribute of the DTAFLD tag that is associated with the variable. Translation for table display is not
supported by ISPF. See the z/OS ISPF Dialog Developer's Guide and Reference for additional information
about the TRANS function.
Restrictions
• The XLATL tag requires an end tag.
• You must code the XLATL tag within a VARCLASS definition. See “VARCLASS (Variable Class)” on page
445 for a complete description of this tag.
• You must code all XLATL tags before any CHECKL tags in the same variable class.
Processing
Table 82. Tags you can code within an XLATL definition 
Tag Reference Usage Required
XLATI “XLATI (Translate Item)” on page 456 Multiple Yes
Examples
Here is source file markup that includes translation of user input for monthcls to uppercase followed by
a translation list of the abbreviated month to an internal value. If no match is found, message abcd003
XLATL
Chapter 12. Tag reference  459

## Page 492

is issued. The example also shows the use of uppercase translation before a check for a list of values for
Type of heating.
<!DOCTYPE DM SYSTEM>
<VARCLASS NAME=monthcls type='char 3'>
   <XLATL FORMAT=upper>
   </XLATL>
   <XLATL MSG=abcd003>
       <XLATI VALUE=11>NOV
       <XLATI VALUE=12>DEC
       <XLATI VALUE=01>JAN
       <XLATI VALUE=02>FEB
       <XLATI VALUE=03>MAR
   </XLATL>
<VARCLASS NAME=costcls TYPE='numeric 6' MSG=abcd001>
<VARCLASS NAME=typecls TYPE='char 4'>
   <XLATL FORMAT=upper>
   </XLATL>
   <CHECKL MSG=abcd002>
      <CHECKI TYPE=values Parm1=EQ Parm2= 'GAS OIL ELEC'>
   </CHECKL>
<VARLIST>
   <VARDCL NAME=month VARCLASS=monthcls>
   <VARDCL NAME=cost VARCLASS=costcls>
   <VARDCL NAME=heat VARCLASS=typecls>
</VARLIST>
<PANEL NAME=xlatl KEYLIST=keylxmp>Heating Cost Survey
   <TOPINST>Complete the information below and then press Enter.
   <AREA>
     <DTACOL PMTWIDTH=20 DESWIDTH=30>
        <DTAFLD DATAVAR=month ENTWIDTH=3>Winter month
          <DTAFLDD>Enter Nov, Dec, Jan, Feb, or Mar
        <DTAFLD DATAVAR=cost ENTWIDTH=6>Heating cost
        <DTAFLD DATAVAR=heat ENTWIDTH=4>Type of heating
          <DTAFLDD>Enter Oil, Gas, or Elec
     </DTACOL>
   </AREA>
   <BOTINST>Thank you for your participation.
</PANEL>
<MSGMBR NAME=abcd00>
  <MSG SUFFIX=1>Heating cost must be numeric
  <MSG SUFFIX=2>Type of heating must be "Gas", "Oil", or "Elec"
  <MSG SUFFIX=3>Winter month must be "Nov","Dec", "Jan", "Feb", or "Mar"
</MSGMBR>
XMP (Example)
The XMP tag defines unformatted text within an information region.
Syntax
<XMP
NOSKIP
>
text
</XMP>
Parameters
NOSKIP
This attribute causes the blank line normally placed before the example to be skipped.
text
This is the text of the example.
XMP
460  z/OS: z/OS ISPF DTL Guide

## Page 493

Comments
The XMP tag defines unformatted text within an information region.
Text coded between the XMP start and end tags is indented two spaces and formats from the current left
margin. Tags which normally cause word-wrapping (for example, P and LI) do not cause word-wrapping
when nested within an XMP tag.
When defining text for an example in your source file, you should be careful not to exceed the width of
the information region it is coded within. If the source text on any line exceeds the formatting width, the
conversion utility truncates the line. A warning message is issued the first time truncation occurs.
Restrictions
• The XMP tag requires an end tag.
• You must code the XMP tag within an INFO definition. See “INFO (Information Region)” on page 317 for
a complete description of this tag.
• You can code multiple XMP tags within an INFO definition, as long as they are not nested within each
other.
Processing
Table 83. Tags you can code within an XMP definition 
Tag Reference Usage Required
DL “DL (Definition List)” on page 261 Multiple No
HP “HP (Highlighted Phrase)” on page 315 Multiple No
NOTE “NOTE (Note)” on page 358 Multiple No
NOTEL “NOTEL (Note List)” on page 361 Multiple No
NT “NT (Note)” on page 364 Multiple No
OL “OL (Ordered List)” on page 367 Multiple No
P “P (Paragraph)” on page 370 Multiple No
PARML “PARML (Parameter List)” on page 386 Multiple No
PS “PS (Point-and-Shoot)” on page 398 Multiple No
RP “RP (Reference Phrase)” on page 411 Multiple No
SL “SL (Simple List)” on page 433 Multiple No
UL “UL (Unordered List)” on page 443 Multiple No
Examples
Here is help panel markup that contains an example. Figure 166 on page 462 shows the formatted result.
<!DOCTYPE DM SYSTEM>
<HELP NAME=xmp WIDTH=40 DEPTH=20>Help for the Search Function
<AREA>
<INFO>
 <P>To locate a book, type the book
title in the "Title" field and press Enter.
<P>Example:
<XMP>
XMP
Chapter 12. Tag reference  461

## Page 494

Title:  THE JOY OF CODING
</XMP>
<P>You don't have to worry about using
upper or lowercase letters; all text is automatically
converted to uppercase for the search.
</INFO>
</AREA>
</HELP>
      Help for the Search Function
 To locate a book, type the book title
 in the "Title" field and press Enter.
 Example:
   Title:  THE JOY OF CODING
 You don't have to worry about using
 upper or lowercase letters; all text
 is automatically converted to
 uppercase for the search.
  F1=Help      F3=Exit      F5=Exhelp
  F6=Keyshelp  F7=PrvTopic  F8=NxtTopic
 F10=PrvPage  F11=NxtPage  F12=Cancel
Figure 166. Example
XMP
462  z/OS: z/OS ISPF DTL Guide
