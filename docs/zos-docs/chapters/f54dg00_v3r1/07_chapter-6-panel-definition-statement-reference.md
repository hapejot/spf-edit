# Chapter 6. Panel definition statement reference

Source file: f54dg00_v3r1.md
Start page: 161
Page span: 161-284

## Page 161

Chapter 6. Panel definition statement reference
The panel definition statement reference provides reference information to help you define the sections
of a panel. It covers the statements that can be coded in each section, and control variables, which you
can use to test conditions pertaining to the display of a panel or message.
• “Defining panel sections” on page 133
• “Formatting panel definition statements” on page 199
• “Using ISPF control variables” on page 244
The sections, statements, and control variables in this panel definition statement reference are arranged
in alphabetical order.
Defining panel sections
Table 8 on page 88 describes the panel sections in the order in which they must be defined .
For reference information for each of these panel sections, see:
)ABC—“Defining the action bar choice section” on page 133
)ABCINIT—“Defining the action bar choice initialization section” on page 137
)ABCPROC—“Defining the action bar choice processing section” on page 137
)AREA—“Defining the area section” on page 138
)ATTR—“Defining the attribute section” on page 143
)BODY—“Defining the body section” on page 169
)CCSID—“Defining the CCSID section” on page 175
)END—“Defining the END section” on page 176
)FIELD—“Defining the FIELD section” on page 176
)HELP—“Defining the HELP section” on page 182
)INEXIT—“Defining the INEXIT section” on page 183
)INIT—“Defining the initialization section” on page 191
)LIST—“Defining the LIST section” on page 192
)MODEL—“Defining the model section” on page 192
)PANEL—“Defining the panel section” on page 192
)PNTS—“Defining the point-and-shoot section” on page 194
)PROC—“Defining the processing section” on page 197
)REINIT—“Defining the reinitialization section” on page 197
Defining the action bar choice section
The )ABC (action bar choice) section defines an action bar choice for a panel and its associated pull-down
choices. An )ABC section must exist for each action bar choice displayed in the Action Bar area on a panel.
The maximum number of )ABC sections on a panel is 40.
)ABC DESC( ‘ choice-description-text ‘
choice-description-text
)
MNEM( ( number )
where:
DESC(choice-description-text)
Text displayed in the panel's action bar area for the action bar choice. The maximum length of the text
is 64 characters.
Defining the action bar choice section
© Copyright IBM Corp. 1980, 2025 133

## Page 162

The action bar choice-description-text must match the choice-description-text specified in the )BODY
section of the panel. ISPF does not translate the value to uppercase. If choice-description-text
contains any special characters or blanks, you must enclose it in quotes in the )ABC DESC parameter.
However, when it is specified in the )BODY section of the panel, you should not enclose it in quotes.
Each action bar choice should be unique.
MNEM(number)
Specifies the position of the character that will be the mnemonic for the action bar text. The letter is
designated by an underscore on the display. This keyword, if it exists, must follow the DESC keyword.
number is the position of the character (not byte position).
)ATTR
 # TYPE(AB)
 @ TYPE(NT)
 ? TYPE(PT)
 $ TYPE ABSL
⋮
)ABC DESC('Menu') MNEM(1)
⋮
)BODY CMD(ZCMD)
@# Menu# Utilities# Compilers# Options# Status# Help@
$--------------------------------------------------------------------
@                      ?ISPF Primary Option Menu+
⋮
For SBCS/DBCS mixed choice-description-text, number cannot be the position of a double-byte
character position. Shift-in/shift-out bytes are not considered characters. For action bar text
containing double-byte characters, add a single-byte character, enclosed in parentheses, to the end of
the double-byte text. The MNEM(number) is the position of this single-byte character. For example:
)ATTR
 # TYPE(AB)
 @ TYPE(NT)
 ? TYPE(PT)
 $ TYPE ABSL
⋮
)ABC DESC('OEDDOOUUBBLLEE0F(M)') MNEM(8)
⋮
)BODY CMD(ZCMD)
@# 0EDDOOUUBBLLEE0F(M)# Utilities# Compilers# Options# Status# Help@
$--------------------------------------------------------------------
@                      ?ISPF Primary Option Menu+
⋮
where DD, OO, UU, BB, LL, and EE represent double-byte characters, and 0E and 0F are shift-out
and shift-in characters. The single-byte character, M, enclosed in parentheses is the mnemonic letter.
MNEM(8) indicates the underscored mnemonic letter is in the eighth character position (not byte
position). Shift-out and shift-in characters are not considered as character positions.
In 3270 mode you access the action bar choice in one of these ways, where "x" is the mnemonic letter
that is underscored:
1. Enter "ACTIONS x" in the command field
2. Enter "x" in the command field and press the function key assigned to the ACTIONS command.
The pull-down menu for that action bar choice displays. If you enter a mnemonic letter, "x", that is not
found to be an underscored mnemonic letter on the panel, then the cursor is placed on the first action
bar choice.
Panels without a command line will not display mnemonic characters, because there is no command
line on which to enter the ACTIONS command and parameter. Terminals or emulators that do not
support extended highlighting will not display host mnemonics.
Note: For each separate action bar choice section, you must define a corresponding )ABCINIT (action bar
choice initialization) section. An )APCPROC (action bar choice processing) section is optional. You must
include these sections in the panel source definition in the proper order as shown in this example:
Defining the action bar choice section
134  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 163

)ABC
)ABCINIT
)ABCPROC
Specifying action bar choices in panel )BODY section
The specification of an action bar choice is included in the panel source immediately following the )BODY
panel definition statement header. The order in which the action bar choices are specified indicates to
ISPF how the choices will appear in the action bar area on the displayed panel. Internally, action bar
choices are numbered sequentially starting from left to right and from top to bottom. The first action bar
choice will be numbered one.
)ATTR
 @ TYPE(AB)
 # TYPE(NT)
⋮
)BODY
 @ choice1@ choice2@ choice3#
Note:
1. A blank must separate the choice-description-text and the AB attribute character. The attribute byte
for the first choice can be in any column except column 1. A text attribute character to delimit an action
bar line should be coded immediately following the last character of the last choice-description-text
on each action bar line.
2. A separator line should follow the last action bar line.
3. ISPF considers the panel line following the last action bar choice as part of the action bar area.
The action bar can consist of multiple lines by specifying action bar choices on more than one line in the
panel )BODY section.
)ATTR
 @ TYPE(AB)
 # TYPE(NT)
⋮
)BODY
 @ choice1@ choice2@ choice3#
 @ choice4@ choice5@ choice6#
Defining pull-down choices within the )ABC section
Within each action bar section, pull-down choices are defined with the PDC statement.
PDC DESC( ‘ choice-description-text ‘
choice-description-text
)
UNAVAIL(  unavail_variable_name )
MNEM ( number ) ACC (  key1
+ key2 + key3
)
PDSEP(
OFF
ON
where:
DESC(choice-description-text)
Actual text that is displayed for the pull-down choice it defines. Special characters or blanks must be
enclosed within quotes. The maximum length of the text is limited to 64 characters. ISPF numbers
each choice. Do not include choice numbers in your text. The pull-down choices defined in each )ABC
Defining the action bar choice section
Chapter 6. Panel definition statement reference  135

## Page 164

section are internally numbered sequentially starting with the number one (1,2,...,n) and the number
is prefixed to the pull-down choice-description-text.
UNAVAIL(unavail_variable_name)
Name of a variable that contains a value to indicate whether the pull-down choice is available for
selection when the panel is displayed. When the variable contains a value other than 0 (false,
therefore available) or 1 (true, therefore unavailable), the variable is ignored and the choice is
available. The choice is available even if the specified variable cannot be found.
Note: The current setting is shown as an unavailable choice; that is, it displays in blue (the default)
with an asterisk as the first digit of the selection number. ISPF issues an error message if you try to
select it. You can change the color, highlight, and intensity of an unavailable choice by using the CUA
Attribute Utility.
MNEM(number)
The MNEM keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
ACC(key1 +key2 +key3)
The ACC keyword is accepted in order to support existing panel definitions that use it. However, it no
longer affects the displayed panel.
PDSEP
The PDSEP keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
You must associate the pull-down choice entry field with a variable name. To do this, code a .ZVARS
statement in the )ABCINIT section. This variable is used as the pull-down entry field name of each
pull-down.
The PDC statement is paired with an optional ACTION statement. When some action is to be performed
for a pull-down choice, an ACTION statement must immediately follow the PDC statement defining the
pull-down choice.
ACTION RUN( command-name )
PARM(' command-parms ')
where:
RUN(command-name)
Required keyword. Specifies the name of a command to be run. The command name must be 2-8
characters. Coding the keyword ACTION RUN(x), where x is a 1-character command name, results in
an error condition. ISPF searches for the command in the application, user, site, and system command
tables, if they are defined.
You can use the ISRROUTE command, which is an ISPF command in ISPCMDS, to invoke the SELECT
service. The ACTION RUN statement is as follows:
ACTION RUN(ISRROUTE) PARM('SELECT your-select-command-parms')
where your-select-command-parms contains all the required parameters for the invocation of the
SELECT service. This allows your dialog not to have to create a separate command in the application
command table for every RUN statement coded within your dialog panels.
Here is an example of invoking the SELECT service from an ACTION RUN statement:
ACTION RUN(ISRROUTE) PARM('SELECT PGM(USERLIST) NEWAPPL(USR)')
PARM(command-parms)
Optional keyword. Specifies the parameters to use when processing the command in the application,
user, site, or system command table. Enclose the command-parms value in quotes.
Defining the action bar choice section
136  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 165

You can define only one ACTION statement per PDC statement in the )ABC panel section. You can specify
the RUN() or PARM() keywords in any order on an ACTION statement. Also, if the RUN() or PARM()
keywords are duplicated within an ACTION statement, ISPF will use the last occurrence of the keyword.
Figure 47 on page 137 shows an example of an action bar section definition.
)PANEL
)ATTR
 @ TYPE(AB)
 # TYPE(NT)
⋮
)ABC DESC(FILE) MNEM(1)
  PDC DESC(file-choice1) 
  ACTION RUN(command-name) PARM(command-parms)
  PDC DESC(file-choice2) UNAVAIL(&unvar2)
  ACTION RUN(command-name) PARM(command-parms)
  PDC DESC(file-choice3) 
  ACTION RUN(command-name) PARM(command-parms)
)ABCINIT
  .ZVARS = PDCHOICE
  &PDCHOICE = '
  &unvar2 = 1
⋮
)ABCPROC
  VER (&PDCHOICE,LIST,1,2,3)
⋮
)ABC DESC(HELP)
  PDC DESC(help-choice1) 
  ACTION RUN(command-name) PARM(command-parms)
  PDC DESC(help-choice2)
  ACTION RUN(command-name)
  PDC DESC(help-choice3)
  ACTION RUN(command-name) PARM(command-parms)
⋮
)ABCINIT
  .ZVARS = PDCHOICE
  &PDCHOICE = '
⋮
)ABCPROC
  VER (&PDCHOICE,LIST,1,2,3)
⋮
)BODY
 @ FILE@ HELP#
⋮
)END
Figure 47. Action bar section example
Defining the action bar choice initialization section
The )ABCINIT section header statement has no parameters. ISPF associates the first )ABCINIT section it
encounters before another panel definition statement header with the previous )ABC section.
)ABCINIT
The rules that apply to the )ABCINIT section and its contents are the same as those that apply to the
ISPF )INIT panel definition statements. However, the processing is limited to the action bar choice and its
pull-down.
The )ABCINIT section runs when the user selects that action bar choice.
At least one statement must be specified in the )ABCINIT section. The )ABCINIT section must contain
a .ZVARS control variable assignment statement to associate a field name with the pull-down entry field.
See “Formatting panel definition statements” on page 199 for additional information.
Defining the action bar choice processing section
The )ABCPROC section header statement has no parameters. ISPF associates the first )ABCPROC section
it encounters before another panel definition statement header with the previous )ABC section.
Defining the action bar choice initialization section
Chapter 6. Panel definition statement reference  137

## Page 166

)ABCPROC
The rules that apply to the )ABCPROC section and its contents are the same as those that apply to the
ISPF )PROC panel definition statement. However, the processing is limited to the action bar choice and its
pull-down.
The )ABCPROC section runs when the user completes interaction with the pull-down choice.
The )ABCPROC section is not required. ISPF verifies all valid pull-down choices for you.
When you manually position the cursor in the action bar area with the CANCEL, END, or RETURN
command on the command line, and you press ENTER, or if you manually position the cursor in the
action bar area and you press a function key to run the CANCEL, END, or RETURN commands, the cursor
is repositioned to the first input field in the body of the panel. If there is not an input field, the cursor is
repositioned under the action bar area. If the request is to run the EXIT command, the action taken is
controlled by the application.
When you use the ACTIONS command to position the cursor in the action bar area and you run the
CANCEL command, the cursor is returned to where it was before the ACTIONS command was run. A
CANCEL command executed from a pull-down removes the pull-down.
See “Formatting panel definition statements” on page 199 for additional information.
Defining the area section
The )AREA (scrollable area definition) section allows you to define scrollable areas on a panel. See
“Defining the attribute section” on page 143 for information about using the AREA(SCRL) keyword to
specify that you want a scrollable area. You can see and interact with the total content defined for the
panel area by scrolling the area.
Use the )AREA section header to describe the scrollable area.
)AREA name
DEPTH( depth)
name
Specifies the name of the scrollable area that is to be matched with the name specified in the )BODY
section. This name cannot be specified as a dialog variable.
DEPTH(depth)
Optional. Specifies the minimum number of lines in the scrollable area (not including the scroll
indicator line) when EXTEND(ON) has been specified. DEPTH has no effect when EXTEND(OFF) is
used. The top line is always reserved for the scroll information and is not considered part of the depth
value. DEPTH can be used to ensure that a required number of lines are displayed. The depth value
cannot be specified as a dialog variable. It must be greater than or equal to the number of lines
defined for the area in the )BODY section and less than or equal to the number of lines in the )AREA
definition.
A panel )AREA section defines the size and the contents of the information to be scrolled. The
contents of the )AREA section generally follow the same rules as the )BODY section. See “Panel
definition considerations” on page 139 for rules concerning the definition of a scrollable area. Multiple
scrollable areas can be defined. The name specified immediately following an AREA(SCRL) character
in the )BODY section is used to match each scrollable area to its corresponding )AREA section. If the
default EXTEND(OFF) is used, you designate the desired depth of the scrollable area by repeating
the AREA(SCRL) attribute. If EXTEND(ON) is specified, the minimum depth is the DEPTH specified in
the )AREA section.
The width of the scrollable area includes the special characters that designate the vertical sides. These
delimiter characters do not represent attribute characters.
The scrollable area is identified in the panel source with a new attribute defined in the )ATTR section. This
new attribute designates the borders of the scrollable area. For example:
Defining the area section
138  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 167

)ATTR
  # AREA(SCRL) EXTEND(ON)
)BODY
#myarea---------#
#               #
#               #
#               #
A single character, Z, can be used in the )AREA section, just as it can be used in the )BODY section, as a
place-holder for an input or output field. The actual name of the field is defined in the INIT section with
the control variable .ZVARS. The actual field names are in a name list, with all the actual field names for
the )BODY and )MODEL sections. The actual field names must appear in the name list in the order they
appear in the panel definition, not in the order they will appear when the panel is displayed. The names
must appear in the )BODY section, then )MODEL section, and then )AREA section order.
If you have defined several )AREA sections, the .ZVARS must be listed in order from top-to-bottom
left-to-right as they appear in the panel definition.
Cursor position determines how an area scrolls. This is called cursor-dependent scrolling. If scroll down is
requested, the line on which the cursor is placed is moved to the top line. If the cursor is currently on the
top line of the scrollable area, the section is scrolled as total visible lines minus one. On a panel with only
one scrollable area, if the cursor is not within the area and scrolling is requested, the area is scrolled by
the total visible lines minus one. If scrolling an area causes the last line of an area to not be the last visible
line in the area, the cursor is moved so that the last line of the area appears at the last visible line of the
scrollable area.
The top line of the scrollable area is reserved for the scroll indicators. Actual information from the )AREA
section is displayed beginning on the second line of the scrollable area. The scroll indicators are displayed
only if more data was defined in the )AREA section than fits in the panel area.
The scroll indicators are displayed as follows:
More:     +
You can only scroll forward.
More:   -
You can only scroll backward.
More:   - +
You can scroll forward or backward.
Forward and backward function keys should be defined in the keylist for any application panel that has
scrollable areas.
The )AREA section can contain any of the items that can be included in the )BODY section except for:
• Action Bar lines
• Graphics Area
• Model Section
• Command Line
• Alternate Message Locations
• Another scrollable area using AREA(SCRL)
• Dynamic Area using EXTEND(ON) or SCROLL(ON).
The )AREA section must fit within the literal table limit of 64K.
Panel definition considerations
When you are defining a scrollable area, a number of rules apply:
• The area cannot be specified by using a Z-variable place-holder within the panel body.
• To allow for the scroll information, the minimum width for a scrollable area is 20. The minimum depth of
the scrollable area is 2.
Defining the area section
Chapter 6. Panel definition statement reference  139

## Page 168

• If the width of the scrollable area is less than the screen size, you must place appropriate attribute
characters around this area so that the data within the area is not inadvertently affected. For example,
by using place fields with SKIP attributes following the right-most boundaries of the area, you can
ensure that the cursor will tab correctly to the next or continued input field within the area.
• You must terminate an input or output field preceding a scrollable area with an attribute character.
• A text field's attribute character is only processed if the start of the field is visible in the scrollable area.
This means that text fields defined to wrap in a scrollable area may not show their defined attribute
when they are only partially displayed. For example, if a field has the attribute HILITE(REVERSE), the
text will only appear in reverse video if the start of the field is visible in the scrollable area.
• The initialization of variables in the scrollable area has nothing to do with Z variables. The setting
of .ZVARS simply associates the name of a variable with a Z place holder. It does not initialize the
variable value.
An explicit setting of the variable in the )INIT section will initialize the variable whether it is in a
scrollable area or not. Normally, variables that are not explicitly defined are set to null by ISPF. This
occurs because ISPF tries to retrieve an existing value from the variable pool and finds that it is not
defined. ISPF then defines the variable and sets it to null.
For scrollable areas, ISPF does not retrieve the variable unless it is to be displayed. Therefore, a variable
in a scrollable area that is not visible on the screen does not get implicitly initialized. This is true for all
the variables. If the user wishes to initialize a variable it can be done by setting the variable to null in
the )INIT section.
If an EXTEND(ON) scrollable area is defined on a panel that does not have a )BODY definition that covers
the entire depth of the screen on which it is displayed, the )BODY line over which the last line of the
scrollable area is defined is repeated for the remaining depth of the screen, or for the remaining number
of lines of data in the scrollable area, whichever is larger.
It is good practice to frame a scrollable area or to allow enough blank space so that the definition
of the scrollable area is clear. You should consult you own usability standards to determine the best
implementation.
Help panels
When a help panel is defined with a scrollable area, the Left, Right, and Enter keys that currently scroll
through the tutorial panels also scroll the scrollable area. When running under tutorial and trying to scroll
past the end of the scrollable area, a message will be displayed indicating that no more information is
available in the scrollable area. If RIGHT or ENTER is pressed again, ISPF will follow the normal tutorial
flow and display the next help panel if one has been defined. The same is true when scrolling to the TOP
of the scrollable AREA; a message indicating that no more information is available will be displayed, and if
LEFT is pressed, the previous tutorial panel will be displayed if one has been defined.
Cursor positioning usually defines which scrollable area will be scrolled. However, when in tutorial, if the
cursor is not within a scrollable area, the first area defined in the )BODY section will be scrolled. The LEFT
and RIGHT commands should be included in any keylist specified for a scrollable help panel.
Panel processing
When a DISPLAY service is issued, the )INIT section is processed before the panel is displayed on
the screen. Each time you scroll and the panel is redisplayed, the )PROC and )REINIT sections are not
processed. The )PROC section is only processed when the panel is submitted for processing as when the
Enter or End key is pressed.
When panel processing is complete and ISPF returns control to the dialog, it is possible that required
fields were not displayed. Therefore, unless a VER NB was coded in the panel for a required field, it is
possible that the application user never scrolled the panel to see the field. It is your responsibility to
ensure that all required information is obtained.
When fields are displayed on a panel, their characteristics can change without the user interacting
with the fields. For example, when CAPS(ON) is set for a field, this only affects fields that actually are
Defining the area section
140  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 169

displayed. If a field is initialized with lowercase letters and it appears on a portion of the panel that is
never displayed, the data remains in lowercase even if CAPS(ON) was set for the field.
Scrollable area examples
Figure 48 on page 141 shows an invalid scrollable area definition. The last line of the extendable
scrollable area also contains a line of nonextendable text to its right.
)ATTR
   # AREA(SCRL)  EXTEND(ON)
   $ AREA(SCRL)
)BODY
%                       New Patient Information
%Command  ===>_ZCMD
%
+Name . . . . . . . . . ._pname                       %
+
#area1 ---------#           $area2 --------------$
#               #           $                    $
#               #           $                    $
#               #           $                    $
#               #           $                    $
#               #           $                    $
#               #           $                    $
                            $                    $
                            $                    $
+
+Please fill in all information.
+
)AREA AREA1 DEPTH(5)
⋮
)AREA AREA2 DEPTH(5)
⋮
Figure 48. Invalid scrollable area definition 
Here is a valid scrollable area definition. It is followed by the actual scrollable panel displays.
)ATTR
  # AREA(SCRL) EXTEND(ON)
)BODY
%
%Command ===>_ZCMD
%
+Patient name . . . . ._pname                       %
+
#myarea -----------------------------------------------------------#
+
+Please fill in all information.
+
)AREA MYAREA DEPTH(5)
+Personal information
+   Address . . . . . . ._address                     %
+   City, State . . . . ._ctyst                       %
+   Zip Code  . . . . . ._zip  %
+   Birth date  . . . . ._birth   %
+   Sex . . . . . . . . ._SX%  (M=Male or F=Female)
+   Marital Status  . . ._MS+1. Married
+                            2. Single
+                            3. Divorced
+                            4. Widowed
+
+   Home phone  . . . . ._hphone       %
+   Work phone  . . . . ._wphone       %
+
+Emergency Contact
+   Name  . . . . . . . ._ename                        %
+   Home phone  . . . . ._ehphone      %
+   Work phone  . . . . ._ewphone      %
+
+Insurance Coverage
+   Insurance Company . ._insure                        %
+   Group number  . . . ._gn%
+   ID number . . . . . ._ID   %
+   Cardholder's name . ._cname                         %
+   Relationship  . . . ._RL+1. Self
+                           +2. Spouse
Defining the area section
Chapter 6. Panel definition statement reference  141

## Page 170

+                           +3. Parent
+                           +4. Relative
+                           +5. Other
+  Signature on file  . ._SG+ (Y=Yes N=No)
)INIT
⋮
)PROC
⋮
)HELP
⋮
)END
Figure 49 on page 142 shows the initial panel display, which contains a scrollable area. More:   +
indicates that you can now scroll forward in the scrollable area.
  Command ===>
  Patient name . . . . . JOHN DOE
                                                    More:     +
  Personal information
     Address . . . . . . . 123 MAIN STREET
     City, State . . . . . ANYTOWN, NY
     Zip Code  . . . . . . 98765
     Birth date  . . . . . 01/23/45
     Sex . . . . . . . . . M    (M=Male or F=Female)
     Marital Status  . . . 1  1. Married
                              2. Single
                              3. Divorced
                              4. Widowed
     Home phone  . . . . . (123)456-7890
     Work phone  . . . . . (123)456-7890
  Please fill in all information.
Figure 49. Scrollable area screen display (part 1 of 2)
Figure 50 on page 142 shows the panel display after one scroll request has been processed. More:   -
 + indicates that you can now scroll forward or backward in the scrollable area.
  Command ===>
  Patient name . . . . . JOHN DOE
                                                    More:   - +
     Home phone  . . . . . (123)456-7890
     Work phone  . . . . . (123)456-7890
  Emergency Contact
     Name  . . . . . . . . JANE DOE
     Home phone  . . . . . (123)456-7890
     Work phone  . . . . . (123)456-7890
  Insurance Coverage
     Insurance Company . . MY INSURANCE COMPANY
     Group number  . . . . 123
     ID number . . . . . . 456789
     Cardholder's name . . JOHN DOE
     Relationship  . . . . 1  1. Self
                              2. Spouse
  Please fill in all information.
Figure 50. Scrollable area screen display (part 2 of 2)
After you have completely scrolled through the scrollable area, More:   -  indicates that you can now
only scroll backward.
Defining the area section
142  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 171

Defining the attribute section
The )ATTR (attribute) section of a panel contains the definitions for the special characters or two-digit
hexadecimal codes that are to be used in the definition of the body of the panel to represent attribute
(start-of-field/end-of-field) bytes. When the panel is displayed, these characters are replaced with the
appropriate hardware attribute bytes and appear on the screen as blanks. If you do not define attribute
characters, ISPF uses defaults.
If specified, the attribute section precedes the panel body. It begins with the )ATTR header statement.
)ATTR
DEFAULT(  def1def2def3 ) FORMAT( EBCDIC
DBCS
MIX
)
OUTLINE
(NONE)
(BOX)
(
L R O U
)
where:
DEFAULT(def1def2def3)
You can use the DEFAULT keyword to specify the characters that define a high-intensity text field, a
low-intensity text field, and a high-intensity input field, respectively. The value inside the parentheses
must consist of exactly three characters, not enclosed in single quotes and not separated by commas
or blanks.
The DEFAULT keyword can also be specified on the )BODY header statement.
FORMAT
Valid values:
• EBCDIC
• DBCS
• MIX
The default value for a TYPE(INPUT) and a TYPE(DATAIN) field is FORMAT(EBCDIC). These two
default values can be changed by using the )ATTR statement or the )BODY statement. These values, in
turn, can be overridden if explicitly specified on a subsequent statement. For example, the net result
of these two statements is FORMAT(DBCS):
)ATTR FORMAT(MIX)
 $ TYPE(INPUT) FORMAT(DBCS)
OUTLINE
Valid values:
• L
• R
• O
• U
• BOX
• NONE
Defining the attribute section
Chapter 6. Panel definition statement reference  143

## Page 172

The default value for OUTLINE is NONE. The default value for TYPE(INPUT) and TYPE(DATAIN) fields
can be specified on the )ATTR or )BODY statement and can be overridden by the OUTLINE keyword.
For example:
)ATTR OUTLINE(U)
  @ TYPE(INPUT) OUTLINE(BOX)
The attribute section ends with the )BODY header statement. The number of lines allowed in an )ATTR
section depends upon the storage size available.
Using default attribute characters
If not specified explicitly with the DEFAULT keyword, the default attribute characters are:
% (percent sign) — text (protected) field, high intensity
+ (plus sign)    — text (protected) field, low intensity
_ (underscore)   — input (unprotected) field, high intensity
These three defaults are the equivalent to specifying:
)ATTR
  % TYPE(TEXT) INTENS(HIGH)
  + TYPE(TEXT) INTENS(LOW)
  _ TYPE(INPUT) INTENS(HIGH)
The default values for the JUST (justification) and CAPS (uppercase and lowercase) keywords vary
according to how the field is used. JUST and CAPS are attribute statement keywords that are described in
“Formatting attribute section statements” on page 144.
You can change the default characters by using a keyword on either the )ATTR or )BODY header
statement. For example:
DEFAULT(abc)
where a, b, and c are the three characters that take the place of %, +, and _, respectively.
Typically, you use the DEFAULT keyword on the )ATTR header statement if the 3 default characters are to
be changed, and additional attribute characters are also to be defined. For example:
)ATTR  DEFAULT($ø_)
   ¬ TYPE(INPUT)  INTENS(NON)
   # TYPE(OUTPUT) INTENS(LOW) JUST(RIGHT) PAD(0)
In this example, the default characters for text fields are changed to $ for high intensity, and ø for
low intensity. The default character for high-intensity input fields is _, the same as the ISPF-supplied
default. The example defines two additional attribute characters: ¬ for nondisplay input fields and # for
low-intensity output fields. The output fields are to be right-justified and padded with zeros.
You could use DEFAULT on the )BODY header statement, with the entire attribute section omitted, if the
only change is to redefine the default characters. For example:
)BODY  DEFAULT($ø_)
If you use DEFAULT on both the )ATTR and the )BODY header statements, the )BODY specification takes
precedence.
Formatting attribute section statements
Each attribute statement defines the attribute character for a particular kind of field. You can define a
given attribute character only once. The remainder of the statement contains keyword parameters that
define the nature of the field.
Generally, you should choose special (non-alphanumeric) characters for attribute characters so that they
will not conflict with the panel text. An ampersand (&), blank (hexadecimal 40), shift-out (hexadecimal
0E), shift-in (hexadecimal 0F), or null (hexadecimal 00) cannot be used as an attribute character.
Defining the attribute section
144  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 173

Note: 
1. You can specify a maximum of 127 attribute characters. This limit includes the 3 default characters,
attribute overrides, and TBDISPL dual defaults. For action bar panels or panels with scrollable areas,
you can specify a maximum of 110 attribute characters. This is because ISPF uses some attribute
characters internally.
2. For the attribute keywords AREA, EXTEND, SCROLL, and REP, the keyword value must be expressed as
a literal.
3. For other attribute keywords the value can be expressed as a literal, or as a dialog variable name
preceded by an ampersand (&). For example:
INTENS(&A)
4. Variable substitution is done after the )INIT section has been processed. The current value of the
dialog variable must be valid for the particular keyword. For example, if the CAPS keyword is specified
as CAPS(&B), the value of dialog variable B must be ON, OFF, IN, or OUT.
Defining the attribute section
Chapter 6. Panel definition statement reference  145

## Page 174

attrchar
AREA(DYNAMIC) parameters
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
CKBOX(
OFF
ON )
COLOR( value)
COMBO(
OFF
ON
name
)
CSRGRP( x) CUADYN( value)
DDLIST(
OFF
ON
name
)
DEPTH( d)
FORMAT( EBCDIC
DBCS
MIX
)
GE(
OFF
ON )
HILITE( value)
INTENS(
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
WIDTH( w)
AREA(DYNAMIC) parameters
Defining the attribute section
146  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 175

AREA(DYNAMIC)
EXTEND(
OFF
ON SCROLL(
OFF
ON
USERMOD(  usermod-code ) DATAMOD(  datamod-code )
where:
attrchar
The single-character or two-digit hexadecimal code that is assigned to the attributes that follow.
AREA(DYNAMIC)
The value in attrchar specifies the special character or two-position hexadecimal value that is used
to define the dynamic area within the panel body section. In the panel body section, the name
immediately following this character identifies the dialog variable that contains the dynamically
formatted string to be displayed in the area. Subsequent lines of the dynamic area are defined in the
panel body by placing this character in the starting and ending columns of the dynamic area. Except
on the first line of the dynamic area, where the area name immediately follows the left delimiter
character, at least one blank must follow the delimiter characters on the left side of the dynamic area.
This is a special character, not an actual attribute character. Other fields must not be defined within or
overlapping a DYNAMIC area.
EXTEND
Specifies whether the depth of an area can be automatically increased.
ON
Specifies that the depth (number of lines) of an area can be automatically increased, if
required, so that the depth of the entire body of the panel matches the depth of the physical
screen on which it is being displayed. Accordingly, an extendable area can be designated in the
panel definition by a single line unless text or other fields are to appear along the graphic area.
Only one extendable area can be specified in a panel definition.
Note: Using EXTEND(ON) is not recommended if your dynamic area is displayed in a pop-up.
When EXTEND(ON) is used, the panel is extended to the size of the logical screen. If the panel
is then displayed in a pop-up, the panel may be truncated at the pop-up border.
The value for the EXTEND keyword cannot be specified as a dialog variable.
OFF
The default. Specifies that the depth (number of lines) of an area cannot be automatically
increased.
SCROLL
Specifies whether the area can be treated as a scrollable area.
ON
Specifies that the area can be treated as a scrollable area. When a panel containing a
scrollable area is displayed, the scrolling commands are automatically enabled. Only one
scrollable area can be specified in a panel definition.
The value for the SCROLL keyword cannot be specified as a dialog variable.
A panel cannot have more than one scrollable area or more than one extended area.
A panel displayed using TBDISPL cannot have a dynamic area defined by SCROLL ON.
Although the panel display service does not perform the scrolling, it does provide an
interpretation of the user's scroll request.
OFF
The default. Specifies that the area cannot be treated as a scrollable area.
Defining the attribute section
Chapter 6. Panel definition statement reference  147

## Page 176

USERMOD(usermod-code) and DATAMOD(datamod-code)
Specifies a character or two-position hexadecimal value to be substituted for attribute characters
in a dynamic area variable following a user interaction. The attribute characters used within the
dynamic area are intermixed with the data. These attribute characters designate the beginning
of a new data field within the area. When the dynamic area variable is returned to the dialog,
usermod-code and datamod-code are used to replace the attribute character of each field that has
been modified, according to these rules:
• USERMOD specified but DATAMOD not specified
If there has been any user entry into the field, even if the field was overtyped with identical
characters, the attribute byte for that field is replaced with usermod-code.
• DATAMOD specified but USERMOD not specified
If there has been any user entry into the field, and if the value in the field has changed, either by
the user entry or by ISPF capitalization or justification, the attribute byte for that field is replaced
with datamod-code.
• Both USERMOD and DATAMOD specified If there has been any user entry into the field but
the value in the field has not changed, the attribute byte for that field is replaced with usermod-
code.
If there has been any user entry into the field and the value in the field has changed, either by
the user entry or by ISPF capitalization or justification, then the attribute byte for that field is
replaced with datamod-code.
• Neither DATAMOD nor USERMOD specified
The attribute byte for the field is unchanged.
You can specify more than one dynamic area on a panel. The number of dynamic areas in a panel
definition is limited only by physical space limitations of the particular terminal being used for the
display.
Examples:
)ATTR
  # AREA(DYNAMIC) EXTEND(ON) USERMOD(!)
The character '!' replaces the attribute byte for each field in the dynamic area that has been touched,
not necessarily changed in value, by the user. All other attribute bytes remain as they are.
)ATTR
  # AREA(DYNAMIC) EXTEND(ON) DATAMOD(01)
The hexadecimal code '01' replaces the attribute byte for each field in the dynamic area that has been
touched by the user and has changed in value. All other attribute bytes remain as they are.
)ATTR
  # AREA(DYNAMIC) EXTEND(ON) USERMOD(0C) DATAMOD(03)
The hexadecimal code '0C' replaces the attribute byte for each field in the dynamic area that has been
touched by the user, but has not changed in value. The hexadecimal code '03' replaces the attribute
byte for each field in the dynamic area that has been touched by the user and has changed in value.
All other attribute bytes remain as they are.
If the datamod or usermod code is one of these special characters, it must be enclosed in single
quotes in the )ATTR section:
blank < ( + | ) ; ¬ - , > : =
If the desired character is a single quote, use four single quotes: DATAMOD(‘’‘’).
AREA(GRAPHIC)
The value in attrchar specifies a character or two-digit hexadecimal value, called the graphic attribute
character, to be used to define the graphic area (4 corners) within the panel body. If you use a
Defining the attribute section
148  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 177

graphics area, this character must be defined; there is no default value. A panel definition can contain
only one graphic area.
EXTEND
Specifies whether the depth of an area can be automatically increased.
ON
Specifies that the depth (number of lines) of an area can be automatically increased, if
required, so that the depth of the entire body of the panel matches the depth of the physical
screen on which it is being displayed. Accordingly, an extendable area can be designated in the
panel definition by a single line unless text or other fields are to appear along the graphic area.
Only one extendable area can be specified in a panel definition.
Note: Using EXTEND(ON) is not recommended if your graphic area is displayed in a pop-up.
When EXTEND(ON) is used, the panel is extended to the size of the logical screen. If the panel
is then displayed in a pop-up, the panel may be truncated at the pop-up border.
The value for the EXTEND keyword cannot be specified as a dialog variable.
OFF
The default. Specifies that the depth (number of lines) of an area cannot be automatically
increased.
A graphic attribute character cannot have any other attribute properties. For example, it cannot be
mixed with attributes such as INTENS, CAPS, JUST, or PAD.
The graphic attribute character is used to define the boundaries of the graphic area in the panel body,
as follows:
• The graphic area is defined on the panel as a rectangle. The graphic attribute character is used
to define the 4 corners plus the remaining characters of the vertical sides of this rectangle. You
delineate the top and bottom of the rectangle with the characters you use to complete the area
outline on the screen. For example, in Figure 51 on page 150, the 4 corners and vertical sides are
defined by the asterisk character in the )ATTR section. The top and bottom of the area have been
completed with dashes.
• A graphic area must be identified with a name that appears in the left top corner, immediately
following the first graphic attribute character of that area. The name of the graphic area must be
followed by a blank. This name is used when retrieving information about the area through the
PQUERY dialog service or the LVLINE panel built-in function. The PQUERY service is described in
z/OS ISPF Services Guide.
• A graphic area can contain ISPF-defined alphanumeric fields.
• ISPF-defined alphanumeric fields can partially overlap graphic areas.
• The first line of the graphic area in the panel definition must have the graphic attribute character
in the starting and ending columns of the area. If an alphanumeric field overlaps one of the
subsequent lines of the graphic area, it must be delimited by a graphic attribute character. See
Figure 53 on page 150 for an example.
• Any field preceding a graphic attribute character should be terminated by an ISPF attribute
character to prevent GDDM from overlaying the left-most boundary characters of the area. When
variable substitution occurs within a text field in the panel body, the field must be terminated by
an attribute character before a special character defining a graphic area. “Using variables and literal
expressions in text fields and panel sections” on page 96 provides additional information about
variable substitution in text fields.
• The width of the graphic area includes the graphic attribute character positions.
• The PQUERY service and the LVLINE panel built-in function can be used to obtain information about
the size of the graphic area.
These rules are applied in Figure 51 on page 150.
Defining the attribute section
Chapter 6. Panel definition statement reference  149

## Page 178

)ATTR
 *  AREA(GRAPHIC)
)BODY
%------------------- TITLE -------------------
%COMMAND ===>_ZCMD                           %
%
+  (Text or other fields that are part of the
+   normal panel body ... )
+ 
+   +*PICT1 ----------------------------*
     *                                  *
     *                                  *
     *                                  *
     *                                  *
     *                                  *
     *                                  *
     * ---------------------------------*
)END
Figure 51. Panel definition  illustrating a graphic area
In this example, a graphic area is defined. PICT1 is specified as the name of the area. An asterisk (*)
is the delimiter character for the vertical sides of the area, and hyphens (-) are the delimiter character
for the top and bottom. Note that a blank follows the area name and follows all asterisks (*) other than
the asterisk adjacent to PICT1.
Figure 52 on page 150 and Figure 53 on page 150 are examples of panel definitions with a graphic
area. In Figure 53 on page 150, note that the alphanumeric field INPUT1 starts at '_' and ends at '|'.
)ATTR
 * AREA(GRAPHIC)
)BODY
%             MY COMPANY OPTION PANEL
% Your selection ==>_ZCMD                          + 
+ 
+  1 Our application 1      +*LOGO ----------------*
+  2 Our application 2      +*                     *
+  3 Our application 3      +*                     *
+  4 Our application 4      +*                     *
+  5 Our application 5      +*                     *
+                           +*                     *
+  X Exit                   +* --------------------*
+  T Tutorial                   <--- Graphic  Area --->
)END
Figure 52. Panel definition  with graphic area
)ATTR
 | AREA(GRAPHIC)
)BODY
%       Panel with Overlapping text field
%
% Here is the data as a graph and with editorial text:
+ 
     +|PIC1 ------------|
      |                 |
      |                 |
      |                 |
      |                 |
  _INPUT1    |          |
      |                 |
      | ----------------|
%        <- graphic area  ->
)END
Figure 53. Definition  of panel graphic area with overlapping text field 
AREA(SCRL)
The value in attrchar specifies the special character or two-position hexadecimal value that is used to
define the borders of the scrollable area in the )BODY section.
Defining the attribute section
150  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 179

EXTEND
Specifies whether the depth of an area can be automatically increased.
ON
Specifies that the depth (number of lines) of an area can be automatically increased, if
required, so that the depth of the entire body of the panel matches the depth of the physical
screen on which it is being displayed. Accordingly, an extendable area can be designated in the
panel definition by a single line unless text or other fields are to appear along the graphic area.
Only one extendable area can be specified in a panel definition.
Note: Using EXTEND(ON) is not recommended if your scrollable area is displayed in a pop-up.
When EXTEND(ON) is used, the panel is extended to the size of the logical screen. If the panel
is then displayed in a pop-up, the panel may be truncated at the pop-up border.
The value for the EXTEND keyword cannot be specified as a dialog variable.
OFF
The default. Specifies that the depth (number of lines) of an area cannot be automatically
increased.
ATTN
Defines the attention-select attribute of the field; it is valid only for text fields.
ON
Specifies that the field can be selected by using the cursor select key.
OFF
The default. Specifies that the field cannot be selected in this manner.
Note: The panel designer must provide an adequate number of blank characters before and after the
attention attribute character, as required by the 3270 hardware.
CAPS
Specifies the uppercase or lowercase attribute of a field. CAPS is not valid for text fields. The CAPS
keyword can have these values:
ON
Data is translated to uppercase before being displayed and all input fields are translated to
uppercase before being stored.
OFF
Data is displayed as it appears in the variable pool and all input fields are stored as they appear on
the screen.
IN
Data is displayed as it appears in the variable pool, but all input fields on the screen are translated
to uppercase before being stored.
OUT
Data is translated to uppercase before being displayed. All input fields are stored as they appear
on the screen.
Unless you specify a CONTROL ASIS command procedure (CLIST) statement, the use of CAPS(OFF),
CAPS(IN), and CAPS(OUT) is negated if the dialog variable is referred to in the command procedure.
If you omit the CAPS parameter, the default is:
• CAPS(OFF) for input or output fields in the )MODEL section of a table display panel
• CAPS(OFF) for DATAIN and DATAOUT fields in dynamic areas
• CAPS(ON) for all other input or output fields.
CKBOX
Allows a 1-character input field followed by a protected (text or output) field to be processed as a
check box by a client that is using the JSON API. The input field is displayed as a check box and the
protected field is the check box description.
The CKBOX keyword can have one of these values:
Defining the attribute section
Chapter 6. Panel definition statement reference  151

## Page 180

ON
Process the input field as a check box.
OFF
Process the input field as non-check box field. This is the default setting.
If the check box input field is not blank, the check box is initialized as selected (checked). If the
check box is selected, a slash character (/) is placed in the check box input field when the panel is
processed.
The CKBOX keyword is ignored if the input field is greater than one character, or if the field following
the check box field is not a protected field. An error message is issued if the CKBOX keyword is used
on any fields other than input fields, or the selected choice (SC) output field.
)ATTR
@ TYPE(CEF) CKBOX(ON)
$ TYPE(SAC)
)BODY
% -------- CHECK BOX PANEL ---------- +
+  Select options:
   &INSTR+
   @Z$Check box #1 description+
   @Z$Check box #2 description+
   @Z$Check box #3 description+
   @Z$Check box #4 description+
)INIT
 .ZVARS = '(BOX1 BOX2 BOX3 BOX4)'
 IF (&ZGUI = ' ')
   &INSTR = 'Enter '/'' to select option.'
 ELSE
   &INSTR = 'Check box to select option.'
)END
Figure 54. Example of CKBOX keyword
COLOR(value)
For 3279-B terminals (or other ISPF-supported seven-color terminals), the COLOR keyword defines
the color of a field. The value can be: WHITE, RED, BLUE, GREEN, PINK, YELLOW, or TURQ (turquoise).
If a color has not been specified and the panel is displayed on a terminal, a default color is generated
based on the protection (TYPE) and intensity attributes of the field. Table 12 on page 152 shows
which defaults are the same as the hardware-generated colors for 3279-A (or other ISPF-supported
four-color terminals). 
Table 12. Color defaults
Field Type Intensity Default Color
Text/Output HIGH WHITE
Text/Output LOW BLUE
Input HIGH RED
Input LOW GREEN
If a color has been specified and the panel is displayed on a terminal other than one with features
such as those on the 3279-B, then:
• If an explicit intensity has also been specified for the field, the color specification is ignored. For
example:
)ATTR
  @ TYPE(INPUT) INTENS(HIGH) COLOR(YELLOW)
In this example, COLOR(YELLOW) is ignored except on terminals like the 3279-B. On a 3279-A
terminal, for example, the resulting color is red.
Defining the attribute section
152  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 181

• If an explicit intensity has not been specified for the field, the color is used to generate a default
intensity. Specification of blue, green, or turquoise defaults to low intensity. Specification of red,
yellow, pink, or white defaults to high intensity. For example:
)ATTR
  $ TYPE(OUTPUT) COLOR(GREEN)
In this example, a low-intensity output field results.
• If neither color nor intensity has been specified for a field, the default intensity is HIGH.
Note: You can make global changes to one or more of the ISPF-supported colors by using the COLOR
command or by selecting the Global Color Change choice from the Colors pull-down on the ISPF
Settings panel (Option 0).
COMBO
The COMBO keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
CSRGRP(x)
Enables you to determine which radio button and checkbox fields are grouped together for cursor
movement purposes. When radio buttons or checkboxes are grouped into cursor groups, the cursor up
and down keys move the focus through each of the fields within the group. The TAB key moves the
focus out of the group, to the next field that is not within this particular group.
To specify the CSRGRP(x) keyword for cursor groups use this syntax:
attribute-char TYPE(CEF) RADIO(ON) CSRGRP(x)
attribute-char TYPE(CEF) CKBOX(ON) CSRGRP(x)
where attribute-char is the special character or 2-position hexadecimal value that is used to define
the field within the panel body section. The x in CSRGRP(x) can be a number between 1 and 99.
The number is used to group all of the fields with the same value into cursor groups. If you specify
a CSRGRP on a field that is not displayed as a checkbox or radio button, the CSRGRP keyword is
ignored.
All radio buttons and checkbox fields that do not have a CSRGRP defined do not have a cursor group
set on a client that is using the JSON API, which has the same effect as having them all in the same
cursor group.
CUADYN(value)
Enables you to define dynamic area DATAIN and DATAOUT attributes with CUA attribute
characteristics. For more information, see “Specifying dynamic areas” on page 165.
DDLIST
The DDLIST keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
DEPTH(d)
The DEPTH keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
FORMAT
For DBCS terminals, the FORMAT keyword specifies the character format for a field.
EBCDIC
EBCDIC characters only
DBCS
DBCS characters only
MIX
EBCDIC and DBCS characters
In a FORMAT(MIX) field, any DBCS character string must be enclosed by a shift-out (hexadecimal 0E)
and a shift-in (hexadecimal 0F).
Defining the attribute section
Chapter 6. Panel definition statement reference  153

## Page 182

The default value for a TYPE(INPUT) and a TYPE(DATAIN) field is FORMAT(EBCDIC). These two
default values can be changed by using the )ATTR statement or the )BODY statement. These values, in
turn, can be overridden if explicitly specified on a subsequent statement. For example, the net result
of these two statements is FORMAT(DBCS):
)ATTR FORMAT(MIX) $ TYPE(INPUT) FORMAT(DBCS)
The default value for a TYPE(TEXT) and a TYPE(OUTPUT) field is FORMAT(MIX). The format of
a TYPE(TEXT) field cannot be overridden by the execution of an .ATTR or .ATTRCHAR statement.
Attempting to do so results in a dialog error.
The pad character for a DBCS field is converted to the corresponding 16-bit character and is then used
for padding. Other format fields are padded normally.
The CAPS attribute is meaningful only for EBCDIC and MIX fields. In addition, within a MIX field, the
CAPS attribute applies only to the EBCDIC subfields.
GE
The GE keyword indicates that a specific character attribute should be preceded in the order stream
by the graphic escape order, provided the terminal supports GE order. The GE order indicates that the
character comes from the APL/TEXT character set. This keyword is supported on TYPE(CHAR) within a
Dynamic Area, action bar separator lines (TYPE(ABSL)), work area separator lines (TYPE(WASL)), and
column headings (TYPE(CH)).
The GE keyword can have one of these values:
ON
Specifies that ISPF will place a graphic escape order before the attribute character when building
the order stream.
OFF
The default. Specifies that ISPF will not place a graphic escape order before the attribute
character.
If GE(ON) is specified on TYPE(ABSL), TYPE(WASL), or TYPE(CH), and if the characters following these
TYPE's in the panel definition are dashes (-) or vertical bars (|), then the appropriate APL character will
be used. This results in these panel elements displaying as solid horizontal or vertical lines, instead of
broken lines.
Note: If the terminal does not support graphic escape or if you are running under GDDM (i.e., GRINIT
service has been issued) then these panel elements will be displayed as coded in the panel definition.
For more information about the GE keyword support on TYPE(CHAR) within a dynamic area, see
“Specifying character attributes in a dynamic area” on page 124.
HILITE(value)
For ISPF-supported terminals with the extended highlighting feature, the HILITE keyword defines the
extended highlighting attribute for a field. The value can be:
USCORE
Underscore
BLINK
Blinking
REVERSE
Reverse video
No default is assumed if highlighting is not specified.
If highlighting is specified and the panel is displayed on a terminal without the extended highlighting
feature, then:
• If an explicit intensity has also been specified, the highlighting is ignored.
• If an explicit intensity has not been specified for the field, a high-intensity field results. On a 3279-A
terminal, there is also color provided by default, as described in Table 12 on page 152.
Defining the attribute section
154  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 183

Examples of Using COLOR and HILITE Keywords
@ TYPE(OUTPUT) INTENS(HIGH) COLOR(YELLOW) HILITE(BLINK)
the results are as follows:
3277,8  —  TYPE(OUTPUT) INTENS(HIGH)
3279-A  —  TYPE(OUTPUT) INTENS(HIGH) *
3279-B  —  TYPE(OUTPUT) COLOR(YELLOW) HILITE(BLINK)
3290    —  TYPE(OUTPUT) HILITE(BLINK)
* Results in white.
INTENS
Specifies the intensity of the field (HIGH is the default):
HIGH
High-intensity field
LOW
Low-intensity (normal) field
NON
Nondisplay field
You can specify these operands for the basic attribute types (TEXT|INPUT|OUTPUT). NEF is the only
CUA panel-element type that supports the INTENS(NON) operand. The remaining CUA panel-element
types do not allow the COLOR, INTENS, and HILITE keyword default values to be changed. The NON
operand allows you to optionally display comments or directive lines.
For a panel displayed on a color terminal, you can also use the INTENS keyword to generate a default
color for the field, as described for the COLOR keyword. INTENS(HIGH) and INTENS(LOW) are ignored
for a 3290 terminal.
JUST
Specifies how the contents of the field are to be justified when displayed. JUST is valid only for input
and output fields.
LEFT
Left justification
RIGHT
Right justification
ASIS
No justification
Justification occurs if the initial value of a field is shorter than the length of the field as described
in the panel body. Normally, right justification should be used only with output fields, since a right-
justified input field would be difficult to type over.
For LEFT or RIGHT, the justification applies only to how the field appears on the screen. Leading
blanks are automatically deleted when the field is processed. For ASIS, leading blanks are not deleted
when the field is processed, nor when it is initialized. Trailing blanks are automatically deleted when a
field is processed, regardless of its justification.
If you omit the JUST parameter, the default is:
• JUST(ASIS) for input or output fields in the )MODEL section of a table display panel
• JUST(ASIS) for DATAIN and DATAOUT fields in dynamic areas
• JUST(LEFT) for all other input or output fields.
LISTBOX
The LISTBOX keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
Defining the attribute section
Chapter 6. Panel definition statement reference  155

## Page 184

NOJUMP
Specifies whether the jump function is disabled for a specific input field. It is ignored on text and
output fields. NOJUMP(OFF), jump function enabled, is the default for fields with field prompts of ==>
and for fields with field prompts of leader dots (. . or ...), provided that jump from leader dots is set to
YES in the Configuration table or "jump from leader dots" is selected in the Settings panel.
ON
Specifies that the jump function is disabled and the data entered is passed to the dialog as it was
entered.
OFF
Specifies that the jump function is enabled for fields with field prompts of ==> and for fields with
field prompts of leader dots (. . or ...) provided that "jump from leader dots" is set to YES in the
Configuration table or selected in the Settings panel. This is the default.
Note: If the application developer defines the NOJUMP(ON) attribute keyword on a specific input
field, this disables the "jump from leader dots" setting for that field, and takes precedence over the
"jump from leader dots" setting on the Settings panel or the Configuration setting of YES for "jump
from leader dots".
NUMERIC
For terminals with the Numeric Lock feature, the NUMERIC attribute keyword allows users to be
alerted to certain keying errors. The NUMERIC attribute keyword is used to specify, for a panel field,
whether Numeric Lock is to be activated for data keyed into that field.
ON
Specifies that the Numeric Lock feature is to be activated. The terminal keyboard locks if the
operator presses any key other than 0 through 9, minus(-), period (.), or duplicate (DUP). ON is
valid only for unprotected fields.
OFF
Specifies that the Numeric Lock feature is not to be activated. The user can type in any characters.
NUMERIC(OFF) is the default value.
On a data-entry keyboard with the Numeric Lock feature, when the user moves the cursor into a
field defined by the NUMERIC(ON) attribute keyword, the display shifts to numeric mode. If the user
presses any key other than those allowed by the Numeric Lock feature, the DO NOT ENTER message
displays in the operator information area and the terminal is disabled. The user can continue by
pressing the reset key.
Note: On non-English keyboards with the Numeric Lock feature, the comma sometimes replaces the
period as a valid numeric character.
NUMERIC(ON) and SKIP(ON) attributes cannot be specified for the same field. If attempted, ISPF
issues an error message.
The NUMERIC(ON) attribute is not supported when GDDM is active.
OUTLINE
For DBCS terminals, the OUTLINE keyword lets you display lines around any type of field. The keyword
parameters specify where the line or lines are displayed.
L
Line to the left side of the field
R
Line to the right side of the field
O
Line over the field
U
Line under the field
BOX
Line surrounding the field (equivalent to LROU)
Defining the attribute section
156  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 185

NONE
No lines
You can specify any combination of the L, R, O, or U parameters in any order, without intervening
blanks.
The default value for OUTLINE is NONE. The default value for TYPE(INPUT) and TYPE(DATAIN) fields
can be specified on the )ATTR or )BODY statement, and can be overridden by the OUTLINE keyword.
For example:
)ATTR OUTLINE(U)
  @ TYPE(INPUT) OUTLINE(BOX)
PAD
Specifies the pad character for initializing the field. This is not valid for text fields. If PAD is omitted,
the default is PAD(' ') for output fields.
char
Any character, including blank (' '), can be specified as the padding character. If the character is
any of these, it must be enclosed in single quotes:
blank < ( + ) ; ¬ , > : =
If the desired pad character is a single quote, use four single quotes: PAD('''').
NULLS
Nulls are used for padding.
USER
Padding character is specified by a user through the ISPF Settings panel.
If the field is initialized to blanks or the corresponding dialog variable is blank, the entire field contains
the pad character when the panel is first displayed. If the field is initialized with a value, the remaining
field positions, if any, contain the pad character.
Padding and justification work together as follows. At initialization, unless you have specified ASIS,
the field is justified and then padded. For left-justified and ASIS fields, the padding extends to the
right. For right-justified fields, the padding extends to the left.
When ISPF processes an input field, it automatically deletes leading or trailing pad characters as
follows:
• For a left-justified field, ISPF deletes leading and trailing pad characters.
• For a right-justified field, ISPF deletes leading pad characters and stores trailing pad characters.
• For an ASIS field, ISPF deletes trailing pad characters and stores leading pad characters.
Regardless of the type of justification, ISPF deletes leading and trailing pad characters for command
fields.
In no case does ISPF delete embedded pad characters. It deletes only leading or trailing pad
characters.
PADC
Specifies conditional padding with the specified pad character. The pad character is used as a field
filler only if the value of the input or output field is initially blank. The pad character is not displayed
in the remaining unfilled character positions if the field has an initial value. Instead, the unfilled
positions contain nulls. Otherwise, ISPF treats the PADC keyword like the PAD keyword, including
justification and deletion of pad characters before storing variables in the pool.
char
Any character, including blank (' '), can be specified as the padding character. If the character is
any of these, it must be enclosed in single quotes:
blank < ( + ) ; ¬ , > : =
If the desired pad character is a single quote, use four single quotes: PADC('''').
Defining the attribute section
Chapter 6. Panel definition statement reference  157

## Page 186

NULLS
Nulls are used for padding.
USER
Specifies that a user-defined character be used for padding. You define the character by using the
ISPF Settings panel. PAD and PADC are incompatible. It is not valid to specify both PAD and PADC
for the same attribute character.
If PADC is omitted, the default is PADC(USER) for input fields.
PAS
PAS is valid for input and output fields only (not for text fields). The point-and-shoot keyword specifies
the field as a point-and-shoot field. The PAS keyword is used in conjunction with the )PNTS point-and-
shoot panel section. See “Defining the point-and-shoot section” on page 194 for more information.
For each field on the panel that has been designated as a point-and-shoot field, there must be a
corresponding entry in the )PNTS point-and-shoot panel section. If the cursor is placed on a point-
and-shoot panel field and the Enter key is pressed, the action associated with the field is performed.
In the example shown, if the cursor is placed on the point-and-shoot field, BLUE1, and the Enter key is
pressed, the variable RED1 is set to RED. The cursor only remains positioned on the point-and-shoot
field if no intermediate panel is displayed and if the dialog does not set the cursor position.
Note:
• You can use option 0 (Settings) to set the tab key to move the cursor point-and-shoot fields. This
changes output fields to input fields, but data is not altered. However, if a variable is used on an
output field that is changed to an input field by the tab to point-and-shoot option, and the variable
is VDEFINEd to the application, the variable will be truncated. In this case, the application developer
should have a temporary panel variable.
• If there is a command entered on the command line, the point-and-shoot field is ignored.
ON
The field is a point-and-shoot field.
OFF
The default. This field is not a point-and-shoot field.
Example:
)PANEL
)ATTR
  $ TYPE(PIN)
  } TYPE(PS)
  + TYPE(NT)
  | AREA(SCRL) EXTEND(ON)
  ! TYPE(OUTPUT) PAS(ON) COLOR(RED)
  * TYPE(OUTPUT) PAS(ON) COLOR(BLUE)
  @ TYPE(TEXT) INTENS(LOW) COLOR(RED) PAD(NULLS)
  ø TYPE(TEXT) INTENS(LOW) COLOR(BLUE) PAD(NULLS)
)BODY WINDOW(60,23)
$
%COMMAND ===>_ZCMD
$
$  Press }DEFAULTS$to reinstate defaults
$
+
|S1                                                |
)AREA S1
+                                        +
+                                        +
+     øBLUE  . . . .*BLUE1               +
+     @RED . . . . .!RED1                +
)INIT
 .cursor = blue1
    &blue1    = ' '
)PROC
 REFRESH(*)
)PNTS
  FIELD(BLUE1) VAR(RED1) VAL(RED)
  FIELD(ZPS00001) VAR(BLUE1) VAL(DEFAULT)
)END
Defining the attribute section
158  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 187

RADIO
Displays mutually exclusive textual settings choices. These fields must contain at least two choices,
one of which is usually selected. A single-choice selection list is the equivalent function on the host.
They can be displayed as radio button groups by a client that is using the JSON API.
To have a single-choice selection list display as a radio button group, use the RADIO(ON) keyword
with the CSRGRP(x) keyword on the CEF type (or other input type) field that is used to enter the
selection on the host.
Note: The RADIO keyword is supported for any input, output, or text field type. To keep the discussion
simple, CEF is used to mean any input field type, and SAC is used to mean any protected text or output
type.
For a list of possible selections, attribute type SAC (select available choice) or another text or output
field type must be used before the choice selection number. The attribute used for the choice
selection number also must have the RADIO(ON) keyword with the CSRGRP(x) keyword. The x on
the CSRGRP keyword is a number used to identify each radio button group. The CSRGRP number on
both the CEF type field and the SAC type field must match. (For more information about CSRGRP, see
CSRGRP(x).) The next field must be a text or output field, used as the radio button choice text.
ISPF initially sets the radio button in the group that corresponds to the value in the CEF field. If the
CEF field is blank or the value in the field does not correspond with any of the radio button selections,
then no radio button is set by default. ISPF then uses the characters following the SAC attribute to set
the value into the CEF field with the same CSRGRP(x) number.
The CEF field must be no more than 3 characters, because only 3 characters are checked and set
for the CEF fields processed as radio buttons. If the text following the SAC attribute is longer than
3 characters, or longer than the value in the CEF field, then the text is truncated to the size of the
CEF field or 3 characters, whichever is smaller when the radio button corresponding to that choice is
selected. Periods at the end of the string are ignored.
To specify the RADIO(ON/OFF) CSRGRP(xx) keyword for radio buttons, use this syntax:
attribute-char TYPE(CEF) RADIO(ON/OFF) CSRGRP(x)
attribute-char TYPE(SAC) RADIO(ON/OFF) CSRGRP(x)
attribute-char
the special character or 2-position hexadecimal value used to define the choice entry field, or
the SAC field within the panel body section. The radio button group is defined in the panel body
section by using the special character to define the radio button entry field and the radio button
choices that go with it.
TYPE(CEF)
field attribute overrides for the CEF fields can be used to set the RADIO(ON) and CSRGRP(x) value
for the CEF field.
TYPE(SAC)
or other text or output field type to be used before each of the choice selection numbers.
RADIO
ON if the radio button is implemented, OFF if it is not.
CSRGRP(x)
x can be any number from 1 to 99. The number refers to the number of the radio button group as a
whole, not the individual choices with the radio button group.
For example:
)ATTR
@ TYPE(CEF) RADIO(ON) CSRGRP(1)
$ TYPE(SAC) RADIO(ON) CSRGRP(1)
! TYPE(CEF) RADIO(ON) CSRGRP(2)
^ TYPE(SAC) RADIO(ON) CSRGRP(2)
#TYPE(SAC)
)BODY
% -------- Radio Button PANEL ---------- +
Defining the attribute section
Chapter 6. Panel definition statement reference  159

## Page 188

+Terminal Characteristics:
  +Screen format  @Z $1.#Data+  $2.#Std+   $3.#Max+   $4.#Part+
  +Terminal Type  !Z ^1.#3277+  ^3.#3278+  ^5.#3290A+ ^7.#3278CF+
                     ^2.#3277A+ ^4.#3278A+ ^6.#3278T+ ^8.#3277KN+
)END
 
Notes about syntax:
1. If a CEF field has the same CSRGRP(x) value as a previous CEF field, and both of them have
RADIO(ON), then the new CEF field is displayed as an input field.
2. If a CEF field has a RADIO(ON) and a CSRGRP(x) value that does not match an SAC with
RADIO(ON) and a CSRGRP(x) value that comes after it, then the CEF field is displayed as an input
field.
3. If an SAC field has a RADIO(ON) and a CSRGRP(x) value that does not match a previous CEF field
with RADIO(ON) and a CSRGRP(x) value, then the SAC field is displayed as an output field instead
of a radio button.
4. If an SAC field is not followed by an output field to be used as the radio button text, then the SAC
field is displayed as an output field.
5. If the radio button choice text wraps from one row to the next, then the text on the next line is not
displayed as part of the radio button choice text, but as normal text.
Restrictions on radio buttons and scrollable areas:
• Radio button groups can appear in a scrollable area, but choices that do not appear in the visible
portion of the area are not displayed.
• If a radio button group does appear in a scrollable area, and the panel cannot be scrolled to show
all of the choices and the CEF field, then it might not be possible to select some of the choices in
the radio button group.
• If the CEF field is scrolled out of the visible area of a scrollable area, the SAC field and the choice
text field that follow it are displayed in the panel body as text or output fields.
REP(character)
For DBCS terminals, the REP keyword allows users to view, on panel definitions, the displayable
replacements for nondisplayable attribute characters. This provides for the use of a wider range
of BODY record attribute characters that can be viewed on panel definitions. These replacement
characters are not visible on the actual panel displays.
You can specify any replacement character, but those that must be enclosed in single quotes are as
follows: < > ( ) + ; : , = blank.
Replacement characters are defined in the attribute section. Then, in the body section of the
panel definition, a record containing only the defined attribute replacement characters is inserted
immediately below any field defined by a corresponding statement in the attribute section. Each
replacement character must be in the same column position as the attribute character position in the
field above.
When the panel definition, for example, is viewed for editing, the data field and the characters that
replace the attribute positions are both displayed. However, when the panel is displayed, the record
containing the replacement characters is not displayed.
Any character immediately above an attribute replacement character in the panel definition is overlaid
by the attribute character's hexadecimal code, not by the displayable replacement character.
In the example shown, hexadecimal codes 38, 31, 32, and 34 are in the field attribute positions when
the panel is displayed. Because these codes are not visible on a display, replacement characters *, !,
$, and # are specified for viewing the panel definition.
When the panel is displayed, the attribute position above the asterisk (*) contains hexadecimal 38;
the one above the exclamation marks (!) contain hexadecimal 31; the one above the dollar sign ($)
Defining the attribute section
160  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 189

contains hexadecimal 32, and the one above the number sign (#) contains hexadecimal 34. None
of these attribute characters is visible on the display, and the panel definition record containing the
replacement characters is not displayed.
The field attribute positions on the panel definition can contain any character, illustrated as x in
the example shown, because they are overlaid by the replacement characters when the panel is
displayed.
Example:
)ATTR
  38 TYPE(INPUT) FORMAT(DBCS) REP(*)
  31 TYPE(INPUT) FORMAT(EBCDIC) REP(!)
  32 TYPE(TEXT) FORMAT(EBCDIC) REP($)
  34 TYPE(TEXT) FORMAT(MIX) REP(#)
)BODY
  + DBCS input field %===>x VARDBCS +
                          *
  [DBDBDBDBDBDBDBDBDB]===>x VAREBC  +
  #                  $    !
Any characters used to replace shift-out or shift-in characters must be less than hexadecimal 40 and
must not be hexadecimal 00, 0E, or 0F.
The EXPAND keyword cannot be used for records containing only those characters defined by the REP
keyword.
SKIP
The SKIP keyword defines the autoskip attribute of the field. It is valid only for text or output
(protected) fields (OFF is the default).
ON
Specifies that the cursor automatically skips the field. When a character is entered into the last
character location of the preceding unprotected data field, ISPF positions the cursor at the first
character location of the next unprotected field.
OFF
Specifies that the cursor does not automatically skip the field when the condition described for
SKIP(ON) occurs.
TYPE(value)
Specifies the TYPE category of the panel element. The default is TYPE(INPUT). The TYPE values
shown must be coded explicitly; it is not valid to assign any of these values to dialog variables: AB,
ABSL, CH, CHAR, CT, DATAIN, DATAOUT, DT, ET, FP, GRPBOX, NT, PIN, PT, RP, SAC, SI, SUC, TEXT,
WASL, and WT. For simplicity, the values in examples are shown as literals.
value may be:
Value
Description
AB
AB unselected choices
ABSL
AB separator line
CEF
Choice entry field
CH
Column heading
CHAR
Character attributes in a dynamic area
CT
Caution text
Defining the attribute section
Chapter 6. Panel definition statement reference  161

## Page 190

DATAIN
Input (unprotected) field in a dynamic area
DATAOUT
Output (protected) field in a dynamic area
DT
Descriptive text
EE
Error emphasis
ET
Emphasized text
FP
Field prompt
GRPBOX
The GRPBOX keyword is accepted in order to support existing panel definitions that use it.
However, it no longer affects the displayed panel.
INPUT
Input (unprotected) field
LEF
List entry field
LI
List items
LID
List item description
NEF
Normal entry field
NT
Normal text
OUTPUT
Output (protected) field
PIN
Panel instruction
PS
Point-and-shoot
PT
Panel title
RP
Reference phrase
SAC
Select available choices
SC
Selected choice
SI
Scroll information
SUC
Select unavailable choices
TEXT
Text (protected) field
VOI
Variable output information
Defining the attribute section
162  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 191

WASL
Work area separator line
WT
Warning text
Note: TYPE values are grouped into four categories:
• Basic attribute types (TEXT|INPUT|OUTPUT). See “Basic attribute types” on page 163.
• Dynamic area types (CHAR|DATAIN|DATAOUT). See “Specifying dynamic areas” on page 165.
• CUA panel-element types. See “CUA panel-element types” on page 166.
• Other attribute types. See “Other attribute types” on page 168.
UNAVAIL
The UNAVAIL attribute keyword is used to indicate the availability of a choice in conjunction with radio
button and checkboxes on a client that is using the JSON API.
ON
Specifies that the choice is not available. This means that the choice cannot be selected in the
current context.
OFF
Specifies the choice is available and can be selected. This is the default setting.
WIDTH(w)
The WIDTH keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
Basic attribute types
For text (protected) fields, the information in the body of the panel following the attribute character is
the data to be displayed. Text fields can contain substitutable variables which consist of a dialog variable
name preceded by an ampersand (&). The name and ampersand are replaced with the value of the
variable, with trailing blanks stripped, before the panel is displayed.
For input (unprotected) or output (protected) fields in the body of the panel, a dialog variable name
immediately follows the attribute character, with no intervening ampersand. The name is replaced with
the value of the variable before displaying the panel. For input fields, any user-entered information is
stored in the variable after the panel has been displayed.
An output field is different from a text field in that an output field has a variable name associated with the
field. It also permits padding, capitalization, justification, and refreshing of the data. There is no default
attribute character for output fields.
ISPF initializes input fields before displaying them. They can be entered (or typed over) by the user. ISPF
also initializes output fields before displaying them, but output fields cannot be changed by the user. Both
input and output fields can have associated caps, justification, and pad attributes. There is no default
attribute character for output fields.
The default values for the data-manipulation attribute keywords, by TYPE, are summarized in Table 13 on
page 163.
Table 13. Default values for data-manipulation keywords
TYPE CAPS JUST PADDING
TEXT N/A N/A N/A
INPUT ON LEFT PADC(USER)
OUTPUT ON LEFT PAD(' ')
Defining the attribute section
Chapter 6. Panel definition statement reference  163

## Page 192

The ISPF basic attribute type rules for field types (defined in Table 13 on page 163) determine which
attribute keywords can be used in conjunction with the basic attribute TYPE keywords.
Keyword
Valid For
CAPS
Not valid for text fields
PAD
Not valid for text fields
JUST
Valid only for input and output fields
ATTN
Valid only for text fields
SKIP
Valid only for text or output (protected) fields
NUMERIC
Valid only for input fields
PADC
Valid only for input or output fields
FORMAT
EBCDIC
Default value for input fields
MIX
Default value for text and output fields
DBCS
Valid for text, input, and output fields
Example of basic attribute types
Figure 55 on page 165 shows a panel definition in which an attribute section is included. As previously
mentioned, an attribute section is not required in a panel definition if only the default attributes are to be
used in the panel body.
Defining the attribute section
164  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 193

)ATTR
  * TYPE(TEXT)   INTENS(HIGH) COLOR(WHITE) CAPS(OFF)
  # TYPE(TEXT)   INTENS(HIGH) COLOR(BLUE)  CAPS(OFF)
  @ TYPE(TEXT)   INTENS(LOW)  COLOR(BLUE)  HILITE(REVERSE)
  ? TYPE(TEXT)   INTENS(LOW)  COLOR(TURQ)  CAPS(OFF)
  _ TYPE(INPUT)  INTENS(HIGH) COLOR(YELLOW)
  $ TYPE(INPUT)  INTENS(NON)
  ø TYPE(OUTPUT) INTENS(LOW)  COLOR(TURQ)  CAPS(OFF)
)BODY
* --------------------------@EMPLOYEE RECORD*--------------------------
# SERIAL NO.*===>_SERNUM  +&rbl                                          %
#
#
#   NAME:?&LAST, &FIRST
#
#   ADDRESS:øADDR1                      + 
#           øADDR2                      + 
#           øADDR3                      + 
#           øADDR4                      + 
#
#   POSITION:øPOSIT                     + 
#
#   YEARS EXPERIENCE:øYRS+ 
#
#   SALARY:øSALARY +       # PASSWORD*===>$PSW   + 
#                              (Password is required for salary)
#
#
* Enter#END*command to terminate application.
#
)PROC
   VER(&SERNUM,NB,NUM)
   .ATTR(.CURSOR) = ‘COLOR(RED) HILITE(BLINK)’
)END
 
Figure 55. Attribute section in a panel definition 
Specifying dynamic areas
TYPE(DATAIN|DATAOUT|CHAR) can be specified for dynamic areas. Use DATAIN and DATAOUT values
only for specifying unprotected or protected fields, respectively, within a dynamic area.
You can specify the ATTN, CAPS, COLOR, HILITE, INTENS, JUST, PAD, PADC, and SKIP keywords for
DATAIN and DATAOUT fields. You can specify NUMERIC for DATAIN fields. The defaults for CAPS, JUST,
and padding are different from those for other panel fields.
The default values for the DATAIN and DATAOUT attribute keywords, by TYPE, are summarized in Table 14
on page 165.
Table 14. Default values for DATAIN and DATAOUT keywords
TYPE CAPS JUST PADDING
DATAIN OFF ASIS PAD(' ')
DATAOUT OFF ASIS PADC(' ')
For more information about TYPE(CHAR) see “Character-level attribute support for dynamic areas” on
page 124.
CUA attribute characteristics in dynamic areas
You can define dynamic area DATAIN and DATAOUT attributes with CUA attribute characteristics. You
do this with the attribute keyword CUADYN(value) on the TYPE(DATAIN) or TYPE(DATAOUT) attribute
statements. DATAIN and DATAOUT fields that you define with the CUADYN(value) keyword are not true
CUA attribute fields, but are DATAIN and DATAOUT fields that have taken on CUA attribute characteristics.
The valid values of CUADYN for each TYPE keyword are:
Defining the attribute section
Chapter 6. Panel definition statement reference  165

## Page 194

Field Type
Valid Attribute Keyword
DATAIN
CEF, EE, LEF, NEF
DATAOUT
CH, CT, DT, ET, FP, LI, LID, NT, PIN, PT, SAC, SI, SUC, VOI, WASL, WT
The CUADYN(value) keyword is ignored on any type other than DATAIN or DATAOUT. The values allowed
on the TYPE(DATAOUT) statement are ignored if specified on the TYPE(DATAIN) statement, and the
reverse is also true.
After the DATAIN or DATAOUT attribute is defined with CUA attribute characteristics, the color, intensity,
and highlighting of the attribute can only be overridden using the CUA Attribute Color Change utility.
CUA panel-element types
The CUA guidelines define the default colors and emphasis techniques for individual panel elements.
The CUA guidelines also request that application users be allowed to change the color and emphasis
for individual panel elements. To conform with CUA principles, ISPF provides panel-element attributes.
The CUA Attribute Change Utility, which is invoked with the CUAATTR command or by selecting the "CUA
attributes" choice from the Colors pull-down on the ISPF Settings panel, allows you to change the color
and emphasis for individual panel elements.
You can define those panel-element attributes that have a TYPE keyword value in the panel attribute
section. The panel-element attributes without a TYPE keyword value are used internally by ISPF in
response to user interactions.
These field types of the CUA panel-element attributes play a major role in determining which attribute
keywords can be used with the CUA panel-element attribute values.
Field Type
Valid Attribute Keyword
Input, Unprotected
CEF, EE, LEF, NEF
Output, Protected
VOI, LID, LI
Text, Protected
ABSL, CH, CT, DT, ET, FP, NT, PIN, PS, PT, SAC, SI, SUC, WASL,WT
Text, Unprotected
AB, RP
The ISPF CUA attribute type rules for field types (defined in Table 15 on page 166) determine which
attribute keywords can be used in conjunction with the CUA panel-element TYPE keywords.
Table 15 on page 166 lists the CUA values for the TYPE keyword. With each TYPE keyword are listed
additional attribute keywords and their default values.
Table 15. CUA TYPE default keyword values
TYPE
Keyword
Value
COLOR * INTENS * HILITE * CAPS JUST PAD PADC SKIP NUM-
ERIC
FORMAT
AB WHITE HIGH NONE N/A N/A N/A N/A N/A N/A MIX
CEF TURQ LOW USCORE OFF LEFT   B N/A OFF EBCDIC
EE YELLOW HIGH REVERSE OFF LEFT   6D N/A OFF EBCDIC
LEF TURQ LOW USCORE OFF ASIS   B N/A OFF EBCDIC
NEF TURQ LOW1 USCORE OFF LEFT   B N/A OFF EBCDIC
Defining the attribute section
166  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 195

Table 15. CUA TYPE default keyword values (continued)
TYPE
Keyword
Value
COLOR * INTENS * HILITE * CAPS JUST PAD PADC SKIP NUM-
ERIC
FORMAT
RP WHITE HIGH NONE N/A N/A N/A N/A N/A N/A MIX
ABSL BLUE LOW NONE N/A N/A N/A N/A OFF N/A MIX
CH BLUE HIGH NONE N/A N/A N/A N/A OFF N/A MIX
CT YELLOW HIGH NONE N/A N/A N/A N/A OFF N/A MIX
DT GREEN LOW NONE N/A N/A N/A N/A OFF N/A MIX
ET TURQ HIGH NONE N/A N/A N/A N/A OFF N/A MIX
FP GREEN LOW NONE N/A N/A N/A N/A OFF N/A MIX
NT GREEN LOW NONE N/A N/A N/A N/A OFF N/A MIX
PIN GREEN LOW NONE N/A N/A N/A N/A OFF N/A MIX
PS TURQ HIGH NONE N/A LEFT B   OFF N/A MIX
PT BLUE LOW NONE N/A N/A N/A N/A OFF N/A MIX
SAC WHITE LOW NONE N/A N/A N/A N/A OFF N/A MIX
SI WHITE HIGH NONE N/A N/A N/A N/A OFF N/A MIX
SUC BLUE LOW NONE N/A N/A N/A N/A OFF N/A MIX
WASL BLUE LOW NONE N/A N/A N/A N/A OFF N/A MIX
WT RED HIGH NONE N/A N/A N/A N/A OFF N/A MIX
LI WHITE LOW NONE OFF ASIS B   OFF N/A MIX
LID GREEN LOW NONE OFF ASIS B   OFF N/A MIX
VOI TURQ LOW NONE OFF LEFT B   OFF N/A MIX
Note:
1. The attribute keywords whose value is denoted with N/A (not applicable) are not valid to use in
conjunction with the corresponding TYPE keyword value.
2. It is not valid to use the attribute keywords FORMAT, REP, and OUTLINE with TYPE(AB). If used, the
default values remain in effect.
3. You cannot change the keyword values for COLOR, INTENS, or HILITE. This is indicated with an * in the
preceding table. If you attempt to change these keyword values, you will get an error condition. The
exceptions are the CUA attribute types NEF, LEF, VOI, LID, and LI. NEF, LEF, VOI, LID, and LI support
the INTENS(NON) keyword value.
4. You can change the default values of COLOR, INTENS, and HIGHLIGHT by using the CUAATTR
command or by selecting the "CUA attributes" choice from the Colors pull-down on the ISPF Settings
panel.
5. The character B in the PAD column stands for blank. The PAD and PADC keywords are mutually
exclusive, so when PAD is set to B (blank, X'40') PADC cannot be set. The EE pad character is X'6D',
underscore.
1 You may specify the INTENS(NON) keyword with the CUA type NEF.
Defining the attribute section
Chapter 6. Panel definition statement reference  167

## Page 196

6. Three keywords not shown on this table are ATTN, REP, and OUTLINE. ATTN always is N/A, REP is
defined by the dialog, and OUTLINE is NONE.
7. Another keyword not shown on this table is CKBOX. CKBOX is only used with TYPE(CEF). This keyword
is ignored except by a client that is using the JSON API. For more information about using CKBOX, see
the CKBOX Keyword.
Table 16 on page 168 lists the CUA panel-element attributes that are used internally by ISPF in response
to user interactions. These panel-element attributes do not have a TYPE keyword, so you cannot code
them in the panel attribute section. They are considered as field-type text (that is, protected). The related
attribute keywords and their default values are shown for each.
Table 16. Internal attributes without TYPE keyword values
Panel Element Attribute COLOR INTENS HILITE
AB Selected Choices YELLOW LOW NONE
PD Choices BLUE LOW NONE
Function Keys BLUE LOW NONE
Informational Message Text WHITE HIGH NONE
Warning Message Text YELLOW HIGH NONE
Action Message Text RED HIGH NONE
Panel ID BLUE LOW NONE
You can change the default values of COLOR, INTENS, and HIGHLIGHT by using the CUAATTR command
or by selecting the "CUA attributes" choice from the Colors pull-down on the ISPF Settings panel.
Other attribute types
The other attribute types consist of the Group Box (GRPBOX) and Selected Choice (SC).
Group box
The GRPBOX keyword is accepted in order to support existing panel definitions that use it. However, it no
longer affects the displayed panel.
Selected choice
The Select Choice (SC) type is an output (protected) field to be used in conjunction with the UNAVAIL
attribute keyword.
When TYPE(SC) is coded with the UNAVAIL(OFF) attribute, the field has the color, intensity, and
highlighting characteristics of TYPE(SAC).
When TYPE(SC) is coded with the UNAVAIL(ON) attribute, the field has the color, intensity, and
highlighting characteristics of TYPE(SUC).
You can use field overrides on the choices.
Relationship to Control variables .ATTR and .ATTRCHAR
The appropriate and inappropriate override conditions for CUA and basic panel-element attributes are
described here. See “.ATTR and .ATTRCHAR” on page 247 for information on .ATTR and .ATTRCHAR.
• TYPE
CUA panel-element attribute TYPE keywords can be overridden by .ATTR or by .ATTRCHAR. You can
change the TYPE:
– From INPUT/CUA input types to OUTPUT/CUA output and input types
Defining the attribute section
168  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 197

– From OUTPUT/CUA output types to INPUT/CUA input and output types
– From TEXT/CUA text types to TEXT/CUA text types
Some exceptions are:
– Only TYPE keyword values that have a field type of input can be overridden with TYPE(EE)—error
emphasis.
– CUA attribute types AB, RP, ABSL, and PS cannot be overridden, nor can they be used to override text
fields.
• COLOR, INTENS, HILITE
If you change a basic attribute type to a CUA attribute type, the attribute takes on the characteristics
of that particular CUA type, including the default COLOR, HILITE, and INTENS keyword values. For
example, if you change a TYPE(INPUT) INTENS(HIGH) attribute to TYPE(NEF), the default color for the
attribute changes from red to turquoise, the default color of the NEF attribute type. Also, after you
change a basic attribute type into a CUA attribute type, the color, highlight, and intensity can only be
overridden by using the CUA Attribute Color Change utility.
For example, hoping to change the TYPE(INPUT) to CUA TYPE(NEF) with the color pink, you enter:
.ATTR(FIELD1) = 'TYPE(NEF) COLOR(PINK)'
The result is that the field is changed to CUA TYPE(NEF), but when the COLOR(PINK) keyword is
processed a dialog error message is given stating that the color of the CUA attribute cannot be
overridden.
If you try to enter:
.ATTR(FIELD1) = 'COLOR(PINK) TYPE(NEF)'
The COLOR(PINK) keyword is processed before the TYPE(NEF) keyword. Thus, no error message is
given concerning the changing of the color of a CUA attribute. However, when the TYPE(NEF) keyword is
processed, the attribute type is changed to the CUA default color, and subsequent attempts to change
the attribute's color, intensity, or highlighting result in a dialog error message.
If you change a CUA attribute type to a basic attribute type, only the type changes. The other
characteristics associated with the type do not change. For example, the color associated with the
CUA type does not change unless you specifically override the color using the COLOR keyword. If you
change the CUA type ET to basic type TEXT, the color remains turquoise unless you purposely override
it.
• CAPS, JUST, PAD, PADC, SKIP, ATTN, NUMERIC, FORMAT, REP, OUTLINE
If the keyword is applicable on the )ATTR statement, it can be overridden using the attribute override
statements. Those panel attribute keywords whose value is denoted as N/A (not applicable) are not
valid in attribute override statements.
• CUADYN(value) keyword
The CUADYN(value) attribute keyword can be used in .ATTRCHAR statements for DATAIN or DATAOUT
attribute characters. The keyword values listed in “CUA attribute characteristics in dynamic areas” on
page 165 for DATAOUT attributes can only override DATAOUT attribute characters. Those listed for
DATAIN attributes can only override DATAIN attribute characters.
Defining the body section
The )BODY (panel body) section of the panel definition specifies the format of the panel as the user sees
it. Each record in the body section corresponds to a line on the display.
The body section begins with the )BODY header statement, which can be omitted if there are no
preceding sections and no change to the default attribute characters. The )BODY header statement and
Defining the body section
Chapter 6. Panel definition statement reference  169

## Page 198

all associated keywords must be specified on the same line. The panel body ends with any of these
statements:
   )MODEL         )FIELD
   )AREA          )HELP
   )INIT          )PNTS
   )REINIT        )LIST
   )PROC          )END
)BODY
KANA WINDOW(  width, depth) CMD( field name )
SMSG( field name ) LMSG( field name ) ASIS
WIDTH(  width) EXPAND( xy) DEFAULT(  def1def2def3 )
FORMAT( EBCDIC
DBCS
MIX
)
OUTLINE
(NONE)
(BOX)
(
L R O U
)
Note:
1. There are system-defined (default) areas for the display of messages and the command field. You can
specify alternate locations using the WINDOW, CMD, SMSG, LMSG, and ASIS keywords on the )BODY
header statement.
2. The WIDTH and EXPAND keywords on the )BODY header statement control the width of a panel. Both
keywords are optional. You can specify either or both. However, if the panel definition width is greater
than 80 characters, the WIDTH keyword must be used. If the WIDTH keyword is used, the WIDTH
variable must be set in the variable pool before the panel is displayed.
3. DEFAULT, FORMAT, and OUTLINE can also be specified on the )ATTR section statement. The values
specified on the )BODY section statement take precedence.
where:
KANA
Include the KANA keyword when Katakana characters will appear within the panel and you have not
specified an extended code page using the )CCSID section.
WINDOW(width,depth)
Identifies the width and depth of the window that the Dialog Manager uses when displaying the panel
in a pop-up window. The values do not include the panel borders; the Dialog Manager adds them
outside of the dimension of the width and depth values.
For panels not displayed in a pop-up window, the depth is the minimum of the specified depth and the
actual number of )BODY records in the panel definition. Extendable areas are not truncated. That is,
the depth expands to the length of the logical screen.
Defining the body section
170  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 199

The width that you specify must be a numeric value greater than or equal to the minimum width of 8
characters. The depth that you specify must be a numeric value greater than 0.
Note: The width and depth cannot be specified by a dialog variable.
For panels that are not being displayed in a pop-up window (no active ADDPOP), ISPF validates the
width and depth values against the screen size and issues an error message if either:
• The width is greater than the current device width.
• The depth is greater than the current device depth.
For help panels and panels that are being displayed in a pop-up window (after ADDPOP service), ISPF
validates the width and depth values against the screen size minus the frame and issues an error
message if:
• The depth is greater than the screen depth minus 2.
• The depth is less than the screen depth minus 2 and the width is greater than the screen width
minus 3.
• The depth is equal to the screen depth minus 2 and the width is greater than the screen width minus
4.
The Dialog Manager recognizes the WINDOW keyword for panels displayed in a pop-up window (after
an ADDPOP service request has been issued). If the panel is not being displayed in a pop-up window,
ISPF validates the keyword, but ignores it. If the text on the panel you are defining exceeds the width
of the window, the panel fields do not wrap. All fields end at the window width.
Note: Text coded in column 1 of the panel body does not appear when a panel is displayed in a
pop-up window. This occurs because ISPF places a field attribute in the column following the pop-up
border character, due to hardware requirements. Without the field attribute after the border character,
subsequent panel text would have the attributes (color, intensity, and so on) of the window frame.
Therefore, your panel text should be coded so that it does not start in column 1 of the body if you are
going to display your panel in a pop-up window.
Attributes coded in column 1 of the panel body overlay the field attributes that ISPF generates
following the left side of the window frame. Therefore, an attribute coded in column 1 of the panel will
be in effect for subsequent text.
CMD(field -name )
Identifies the panel field (variable name) to be treated as the command field. The field type must be a
CUA input type. If the CMD keyword is omitted from a )BODY statement, ISPF uses the first input field
as a default command field. You can specify that you do not want a command field by using CMD(). Do
not use this option for a table display. You must have a command field for a table display.
SMSG(field -name )
Identifies the panel field (variable name) where the short message, if any, is to be placed. The
field type must be a CUA output type. If the message is longer than the length of this field, the
message is placed in a pop-up window. The SMSG keyword does not effect placement of the TOP-
ROW-DISPLAYED indicator which is right-justified on the top line of the display, or just below the
action bar separator line if an action bar is defined.
LMSG(field -name )
Identifies the panel field (variable name) where the long message, if any, is to be placed. The field
type must be a CUA output type. If the message is longer than the length of this field, the message is
placed in a pop-up window.
Note:
1. For CMD, SMSG, and LMSG the field-name must be within the )BODY section, not within a
scrollable area or table.
2. For long or short messages in pop-up windows, if the message originates from panel processing, as
in a verification error message, the message pop-up window is placed adjacent to the field that is
the object of the validation.
Defining the body section
Chapter 6. Panel definition statement reference  171

## Page 200

3. The format of the command, long-message, and short-message fields must not be FORMAT(DBCS).
Because a FORMAT(EBCDIC) field does not display DBCS characters correctly, FORMAT(MIX) is
recommended.
4. For additional information about the placement of the command and long message fields, see
about understanding ISPF panels in the z/OS ISPF User's Guide Vol I.
ASIS
Specifies that the command and long message fields are to appear on the display as specified in the
panel definition. When ASIS is specified, any user request, using SETTINGS option 0 or by setting
system variable ZPLACE, to reposition the command and long message fields is ignored.
WIDTH(width)
The number of columns to use in formatting the panel. width can be a constant or a dialog variable,
including the system variable &ZSCREENW The specified width must not be less than 80 or greater
than the width of the terminal on which the panel is to be displayed. If the WIDTH keyword is not
specified, the default is 80.
EXPAND(xy)
The repetition delimiter characters. The delimiters can be used on any line within the panel body
to enclose a single character that is repeated to expand the line to the required width. The starting
and ending delimiter can be the same character. If no delimiters are specified, or if any line does not
contain the delimiters, then the line is expanded to the required width by adding blanks on the right.
The delimiter characters cannot be specified with a dialog variable.
Before the panel is displayed, it is formatted according to the WIDTH and EXPAND keyword values as
if the expanded format of the body were originally coded in the panel definition. For example:
)BODY  WIDTH(&EDWIDTH) EXPAND(//)
+-- &TITLE ---------------------------------/-/----------
%COMMAND ===>_ZCMD     / /             +SCROLL%===>_SCRL +
+
%EMPLOYEE NUMBER:@EMPLN         / /                     @
 
In the title line, hyphens are repeated to expand the line to the width specified by &EDWIDTH The
command field and the employee number field would both be expanded with repeated blanks.
If more than one repetition character appears in a line of the panel body, each of the characters is
repeated an equal number of times. For example:
)BODY  EXPAND(#@)
TUTORIAL #-@ TITLE OF PAGE #-@ TUTORIAL
would become:
TUTORIAL ------------ TITLE OF PAGE ------------ TUTORIAL
ISPF treats as an error a request to display a panel that is wider than the physical screen or current
logical screen for a 3290 terminal. ISPF displays a box panel indicating the error. For the 3290, if a
panel that is wider than 80 characters is being displayed, and the user attempts to divide the screen
vertically (SPLITV command), ISPF denies the request and displays an error message. Remember that
the panel is displayed as though the expanded format of the body were originally coded in the panel
definition. Therefore, be careful when expanding text fields that contain substitutable variables, so
that meaningful text is not truncated. For example:
)BODY  EXPAND(//)
TUTORIAL /-/ &VAR1 /-/ TUTORIAL
would become:
TUTORIAL ---------------- &VAR1 ---------------- TUTORIAL
Then, if &VAR1 had the value ‘ABCDEFG’ when the screen was displayed, this line would result:
Defining the body section
172  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 201

TUTORIAL ---------------- ABCDEFG ---------------- TUTORI
To avoid this problem, provide a few blanks at the end of the text string, as follows:
TUTORIAL /-/ &VAR1 /-/ TUTORIAL      + 
Table 17 on page 173 and Table 18 on page 173 describe the display width, data expansion width
(resulting from EXPAND keyword on the )BODY statement), and the pop-up window width based on
various WINDOW/WIDTH keyword combinations. 
Table 17. Display in primary window
WINDOW/WIDTH
Combinations DISPLAY EXPANSION
no WINDOW, no WIDTH WIDTH (def. 80) WIDTH (def. 80)
WINDOW, no WIDTH WIDTH (def. 80) WINDOW value
no WINDOW, WIDTH WIDTH WIDTH value
WINDOW <= WIDTH WIDTH WINDOW value
WINDOW > WIDTH ERROR ERROR
Table 18. Display in pop-up window
WINDOW/WIDTH
Combinations DISPLAY EXPANSION WINDOW
no WINDOW, no WIDTH WIDTH (def. 80) WIDTH (def. 80) (76, 22)
WINDOW, no WIDTH WIDTH (def. 80) WINDOW value WINDOW (w, d)
no WINDOW, WIDTH WIDTH WIDTH value (76, 22)
WINDOW <= WIDTH WIDTH WINDOW value WINDOW (w, d)
WINDOW > WIDTH ERROR ERROR ERROR
Note: ISPF will issue an error message if you attempt to display a panel in a pop-up window where the
WINDOW width value is greater than the width of the underlying panel.
DEFAULT(def1def2def3)
You can use the DEFAULT keyword to specify the characters that define a high-intensity text field, a
low-intensity text field, and a high-intensity input field, respectively. The value inside the parentheses
must consist of exactly three characters, not enclosed in single quotes and not separated by commas
or blanks.
The DEFAULT keyword can also be specified on the )ATTR section statement.
FORMAT
Valid values:
• EBCDIC
• DBCS
• MIX
The default value for a TYPE(INPUT) and a TYPE(DATAIN) field is FORMAT(EBCDIC). These two
default values can be changed by using the )ATTR statement or the )BODY statement. These values, in
Defining the body section
Chapter 6. Panel definition statement reference  173

## Page 202

turn, can be overridden if explicitly specified on a subsequent statement. For example, the net result
of these two statements is FORMAT(DBCS):
)BODY FORMAT(MIX)
 $ TYPE(INPUT) FORMAT(DBCS)
OUTLINE
Valid values:
• L
• R
• O
• U
• BOX
• NONE
The default value for OUTLINE is NONE. The default value for TYPE(INPUT) and TYPE(DATAIN) fields
can be specified on the )ATTR or )BODY statement and can be overridden by the OUTLINE keyword.
For example:
)BODY OUTLINE(U)
  @ TYPE(INPUT) OUTLINE(BOX)
A sample panel body section
The sample panel definition, shown in Figure 56 on page 174, consists of a panel body followed by
an )END control statement. It has no attribute, initialization, reinitialization, or processing sections, and
uses the default attribute characters.
This data entry panel has 11 input fields (for example, ZCMD and TYPECHG) indicated with the
underscore attribute character. It also has a substitutable variable (EMPSER) within a text field. The
first two lines of the panel and the arrows preceding the input fields are all highlighted, as indicated by
the percent sign attribute characters. The other text fields are low intensity, as indicated by the plus sign
attribute characters.
)Body
%----------------------------  EMPLOYEE RECORDS  ------------------------------
%COMMAND ===>_ZCMD                                                            %
%
%EMPLOYEE SERIAL: &EMPSER
+ 
+   TYPE OF CHANGE%===>_TYPECHG +  (NEW, UPDATE, OR DELETE)
+ 
+   EMPLOYEE NAME:
+     LAST   %===>_LNAME         + 
+     FIRST  %===>_FNAME         + 
+     INITIAL%===>_I+ 
+ 
+   HOME ADDRESS:
+     LINE 1 %===>_ADDR1                                   + 
+     LINE 2 %===>_ADDR2                                   + 
+     LINE 3 %===>_ADDR3                                   + 
+     LINE 4 %===>_ADDR4                                   + 
+ 
+   HOME PHONE:
+     AREA CODE   %===>_PHA+ 
+     LOCAL NUMBER%===>_PHNUM   + 
+ 
)End
Figure 56. Sample panel definition 
Figure 57 on page 175 shows the panel as it appears when displayed, assuming that the current value of
EMPSER is 123456 and that the other variables are initially null.
Defining the body section
174  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 203

----------------------------  EMPLOYEE RECORDS  ------------------------------
 COMMAND ===>
 EMPLOYEE SERIAL: 123456
    TYPE OF CHANGE ===>            (NEW, UPDATE, OR DELETE)
    EMPLOYEE NAME:
      LAST    ===>
      FIRST   ===>
      INITIAL ===>
    HOME ADDRESS:
      LINE 1  ===>
      LINE 2  ===>
      LINE 3  ===>
      LINE 4  ===>
    HOME PHONE:
      AREA CODE    ===>
      LOCAL NUMBER ===>
Figure 57. Sample panel—when displayed
Defining the CCSID section
The )CCSID section identifies the Coded Character Set Identifier used in the panel definition.
)CCSID
NUMBER(  xxxxx)
where:
NUMBER(xxxxx)
The CCSID of the EXTENDED CODE PAGE as defined by Character Data Representation Architecture.
See “Supported CCSIDs” on page 303 for which CCSIDs are supported.
The )CCSID section must be the first section in the panel as illustrated in this example:
)CCSID NUMBER(00037)
)PANEL
)BODY
%---------------------- NAME OF PANEL -------------------------------
%COMMAND ===>__ZCMD
⋮
)END
If the CCSID section is used, the single-byte text characters in the )BODY, )AREA, or )MODEL section of
the panel are translated to the equivalent character (or a period if the character does not exist) in the
terminal code page for display. ISPF scans the panel for a text attribute, notes the position, and then
scans for a non-text attribute. When the non-text attribute is found, ISPF translates the text between the
text attribute and the non-text attribute. Thus you must have one text attribute defined before any text
you want translated. This translation occurs only if the code page indicated by the CCSID is different from
the code page of the terminal.
All characters in the panel source that are not in the )BODY text must be in the Syntactic Character Set:
• A-Z
• a-z
• 0-9
• + < = > % & * " '
• ( ) , _ - . / : ; ?
Note: Lowercase a-z can be used for any CCSID supported by ISPF except the Japanese (Katakana)
Extended CCSID 930.
Defining the CCSID section
Chapter 6. Panel definition statement reference  175

## Page 204

See Chapter 10, “Extended code page support,” on page 299
Defining the END section
The )END section identifies the end of the panel definition. It is a required section.
)END
The definition consists only of the )END statement. Any lines placed after the END statement are ignored.
)PANEL
)BODY
%---------------------- NAME OF PANEL -------------------------------
%COMMAND ===>__ZCMD
⋮
)END
Defining the FIELD section
The )FIELD section of a panel definition specifies what fields, if any, are scrollable fields. Defining a field
as scrollable provides the ability to display and input a variable larger than the display area that the
dialog variable occupies. The LEFT, RIGHT, and ZEXPAND primary commands are active when the cursor
is positioned within the variable on the display panel. These enable left and right scrolling and expansion
of the variable into a pop-up window.
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
Note:
1. Each entry in the )FIELD section must begin with the keyword FIELD.
2. With the exception of the LCOL parameter, all dialog variable names must be unique to each
parameter.
3. Scrollable field support is panel specific. A subsequent panel display that references the same variable
but does not define it as scrollable may cause data truncation (depending on the data lengths
involved).
4. The command field for a panel has a maximum length of 255 characters. The LEFT and RIGHT
commands cannot be used to scroll data in the command field.
Defining the END section
176  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 205

where:
FIELD(field -name )
The name of the field on the panel that this statement controls.
LEN(v alue|field -name )
Length of the displayed variable.
value: Specify a value between 1 and 32 767.
field -name : The length dialog variable can be used to specify an initial length if it contains a value
between 1 and 32 767. After the display, this variable will contain the calculated display length.
Calculated display length: The length of the variable will be the maximum value of the default
display variable length and the specified length.
Default: If the LEN parameter is not specified, the field will default to the length of the dialog
variable, if it exists. For variables referenced in a )MODEL section, the dialog variable length will be the
maximum of all instances on the current display for that variable.
IND(field -name, v alue )
Left and right scroll indicator dialog variable.
field -name : This must refer to a 2 byte scroll indicator dialog variable that will be updated on the panel
to indicate whether left and right scrolling can be performed.
value: (Default -+) Specify a 2 byte literal (enclosed in quotes) to override the default scroll indicator
values. Each byte must be nonblank.
Displays as:
-+
Indicates that you can scroll left and right
-
Indicates that you can only scroll left
 +
Indicates that you can only scroll right.
Panel definition:
)ATTR
|  TYPE(OUTPUT) CAPS(OFF) JUST(ASIS )
_  TYPE(INPUT)  CAPS(OFF) JUST(ASIS )
)BODY
+Scrollable Variable:_SCRFLD            |SCRIND+
)FIELD
 FIELD(SCRFLD) IND(SCRIND,'<>')   /* replace -+ with <> */
Panel display:
 Scrollable Variable: CDEFGHIJKLMNOPQRST <>
LIND(field -name, v alue )
Left scroll indicator dialog variable.
field -name : This must refer to a 1 byte left scroll indicator dialog variable that will be updated on the
panel to indicate whether left scrolling can be performed.
value: (Default -) Specify a 1 byte nonblank literal (enclosed in quotes) to override the default left
indicator value.
Displays as:
value
Indicates that you can scroll left
blank
Indicates you are positioned at the start of the field.
Defining the FIELD section
Chapter 6. Panel definition statement reference  177

## Page 206

Panel definition:
)FIELD
 FIELD(SCRFLD)  LIND(LSCRIND,'<')  /* replace - with <  */
RIND(field -name, v alue )
Right scroll indicator dialog variable.
field -name : This must refer to a 1 byte right scroll indicator dialog variable that will be updated on the
panel to indicate whether right scrolling can be performed.
value: (Default +) Specify a 1 byte nonblank literal (enclosed in quotes) to override the default right
indicator value.
Displays as:
value
Indicates that you can scroll right
blank
Indicates you are positioned at the end of the field.
Panel definition:
)FIELD
 FIELD(SCRFLD)  RIND(RSCRIND,'>')  /* replace - with >  */
SIND(field -name, v alue )
Separator scroll indicator dialog variable. This field will be initialized with the value repeated for the
length of the field on the panel. If the field is scrollable to the left, the leftmost byte will be the value
of the left indicator (default '-'). If the field is right scrollable, the rightmost byte will be the value of
the right indicator (default '+').
field -name : This must refer to a 3 byte scroll indicator dialog variable that will be updated on the panel
to indicate whether left and right scrolling can be performed.
value: (Default '<->') Specify a 3 byte literal (enclosed in quotes) to override the default separator
indicator values. The 3 bytes represent the left scroll indicator, the separator value and the right scroll
indicator respectively. Each byte must be nonblank.
Panel definition:
)ATTR
| TYPE(OUTPUT) CAPS(OFF) JUST(ASIS )
_ TYPE(INPUT)  CAPS(OFF) JUST(ASIS )
)BODY
+Separator  Variable:|SEPIND                           |
+Scrollable Variable:_SCRFLD                           |
)FIELD
 FIELD(SCRFLD) SIND(SEPIND)
Panel display:
Separator  Variable: <---------------->
Scrollable Variable: CDEFGHIJKLMNOPQRST
LCOL(field -name )
Left column dialog variable - to display current left position.
field _name : This must refer to a dialog variable that will be updated when the field is scrolled to
contain the left column value. You can use this to specify an initial left column position for the
scrollable field. It must be a numeric value greater than or equal to 1. Values greater than the
maximum left column position will be set to the maximum left column position.
Note: Fields with the same left column dialog variable will scroll simultaneously and will have the
same left column value up to the maximum for each field.
Defining the FIELD section
178  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 207

RCOL(field -name )
Right column dialog variable - to display current right position.
field _name : This must refer to a dialog variable that will be updated when the field is scrolled to
contain the right column value. It is an output field only. Any pre-existing values will be ignored and
will be replaced with the current right column value.
SCALE(field -name )
Scale indicator dialog variable. This field will be initialized with a scale line reflecting the current
columns within the field being displayed. The variable will occupy the display length on the panel with
the a value as follows:
----+----1----+----2----+----3... etc.
field _name : This must refer to the dialog variable that is placed on the panel in the position at which
the scale line is to be initialized.
Panel definition:
)ATTR
| TYPE(OUTPUT) CAPS(OFF) JUST(ASIS )
_ TYPE(INPUT)  CAPS(OFF) JUST(ASIS )
)BODY
+Scale Line         :|SCLIND                           |
+Scrollable Variable:_SCRFLD                           |
)FIELD
 FIELD(SCRFLD) SCALE(SCLIND)
Panel display:
Scale Line         : --+----1----+----2
Scrollable Variable: CDEFGHIJKLMNOPQRST
SCROLL(v alue|field -name )
Scroll control field.
value:
OFF
Field is not scrollable
ON
Field is scrollable
field _name :
Specifies a scroll control dialog variable which you can set to a value of OFF to turn scrolling off
from the application or from the panel.
NOLR:
LEFT/RIGHT scrolling is disabled for the scrollable field.
Default: If the SCROLL parameter is not specified, the default for the scroll control is ON.
Primary commands for scrollable fields
These commands apply when the cursor has been placed within a scrollable field:
LEFT
Scroll left the specified scroll amount.
RIGHT
Scroll right the specified scroll amount.
ZEXPAND
Display the variable in a dynamic area in a popup window. If the scrollable field is input then you will
be able to update the variable in the expand window.
Defining the FIELD section
Chapter 6. Panel definition statement reference  179

## Page 208

The expand panel displays the variable in a scrollable dynamic area. Standard up and down scrolling
is supported. You can display the variable in character and hexadecimal using the HEX primary
command.
HEX ON/OFF
Turn hexadecimal display on and off
The setting will be remembered for subsequent expand processing.
ZCLRSFLD
Clears the contents of the scrollable field to blanks.
If a scroll field is found on the current panel, then the scroll amount will be honored as for up and down
scrolling, where:
PAGE
is the equivalent of the length of the display field
DATA
is the equivalent of the length of the display field minus 1
HALF
is half the length of the display field
CSR
will scroll relative to the cursor position
You can enter M in the command line to scroll the maximum distance in the left or right direction. The
maximum right position is the field length minus the display length. The maximum left position is 1. You
can also enter a number in the command line to specify the number of characters to scroll to the left or
right.
Example
Panel source:
)ATTR
 | TYPE(OUTPUT) CAPS(OFF) JUST(ASIS )
 _ TYPE(INPUT)  CAPS(OFF) JUST(ASIS )
)BODY
%----------LEFT / RIGHT / Expand Example  1 -------------------------
%OPTION  ===>_ZCMD
%
+ Testcase 1
+
+ Field            Value        Scroll
+ -------------------------------------
+ Value          :_SCRFLD      |SFIND
+ Left & Right   :|SFLIND    |SFRIND
+ Left column    :_SFLCOL
+ Right column   :_SFRCOL
+ Length         :_SFLEN
)INIT
  .CURSOR = ZCMD
)FIELD
 FIELD(SCRFLD) LEN(SFLEN)
 LCOL(SFLCOL) RCOL(SFRCOL)
 IND(SFIND) LIND(SFLIND) RIND(SFRIND)
 SCROLL(SFCTL)
)END
REXX to display panel:
/* REXX - Example 1 FOR LEFT/RIGHT/EXPAND PANEL FUNCTIONS */
ARG SFCTL
SCRFLD = 'abcdefghijklmnopqrstuvwxyz'       /* initialize field */
SFLCOL = 3                                  /* initial left position */
SFLEN  = 84                                 /* initial length        */
DO UNTIL RC = 8
  ADDRESS ISPEXEC
     'DISPLAY PANEL(SFSAMP1)'               /* display panel */
END
Defining the FIELD section
180  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 209

Initial panel display:
----------LEFT / RIGHT / Expand Example  1 -------------------------
OPTION  ===>
 Testcase 1
 Field            Value        Scroll
 -------------------------------------
 Value          : cdefghijklmn -+
 Left & Right   : -          +
 Left column    : 3
 Right column   : 14
 Length         : 84
Changing the scroll indicators in the panel definition to:
)FIELD
 FIELD(SCRFLD) LEN(SFLEN)
 LCOL(SFLCOL) RCOL(SFRCOL)
 IND(SFIND,'<>') LIND(SFLIND,'<') RIND(SFRIND,'>')
 SCROLL(SFCTL)
)END
changes the panel display to:
----------LEFT / RIGHT / Expand Example  1 -------------------------
OPTION  ===>
 Testcase 1
 Field            Value        Scroll
 -------------------------------------
 Value          : cdefghijklmn <>
 Left & Right   : <          >
 Left column    : 3
 Right column   : 14
 Length         : 84
If PF4 is set to the value ZEXPAND and PF4 is pressed while the cursor is positioned within the scrollable
field, ISPF displays:
┌────────────────────────────────────────────── SCRFLD+0 ──────────────────────────────────┐
│                                                          Line   1 of      2 │
│ Command ===>                                             Scroll ===> CSR    │
│                                                                             │
│ abcdefghijklmnopqrstuvwxyz                                                  │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
⋘────────────────────────────────────────────────────────────────────────────────────────────┘
Defining the FIELD section
Chapter 6. Panel definition statement reference  181

## Page 210

If the HEX ON primary command is entered, ISPF displays:
┌────────────────────────────────────────── SCRFLD+X'0'(0) ───────────────────────────────┐
│                                                          Line   1 of      2 │
│ Command ===>                                             Scroll ===> CSR    │
│                                                                             │
│ abcdefghijklmnopqrstuvwxyz                                                  │
│ 888888888999999999AAAAAAAA444444444444444444444444444444444444444444444444  │
│ 12345678912345678923456789000000000000000000000000000000000000000000000000  │
│                                                                             │
│                                                                             │
│ 4444444444                                                                  │
│ 0000000000                                                                  │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
⋘────────────────────────────────────────────────────────────────────────────────────────────┘
Panel definition considerations
The LEFT, RIGHT, and ZEXPAND commands should be included in any keylist specified for a scrollable
field.
Defining the HELP section
The )HELP section of the panel definition specifies what help panel, if any, is displayed when help is
requested for a particular element defined on the panel. Help can be requested for a field, an action bar
choice, or a pull-down choice by including a statement in the source panel definition help section. See
“Reference phrase help” on page 83 for a discussion on requesting help for reference phrases.
)HELP FIELD( field-name )
PANEL( help-panel-name )
MSG( msg-name )
PASSTHRU
where:
FIELD(field -name )
The name of the source panel element (input selection field, action bar choice, dynamic area name,
and so on). When the PANEL keyword is used, a help panel is displayed when help is requested for
an element. When the MSG keyword is used, a message is displayed when help is requested for an
element. When the PASSTHRU keyword is used, control returns to the dialog when help is requested
for an element. Field-name can be a variable. If the field-name variable value is not found, the Tutorial
table of contents panel (ISR00003) is displayed.
PANEL(help-panel-name)
The name of the help panel associated with the field. Help-panel-name can be a variable.
MSG(msg-name)
The name of the message associated with the field. The msg-name can be a variable. When help is
requested on the field that specified MSG(msg-name) in the )HELP section, the message is displayed.
The short message appears in the upper right corner of the panel. The long message box is placed at
the field on the screen.
PASSTHRU
The PASSTHRU keyword is intended for use on dynamic-area fields. When help is requested on the
field, control returns to the dialog. No help panel or message is displayed.
Note:
Defining the HELP section
182  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 211

1. Using the PASSTHRU keyword on reference phrases within scrollable areas can cause
unpredictable results.
2. System variables ZCURFLD and ZCURPOS can be used to determine the cursor position. You must
define a )PANEL section for ZCURFLD and ZCURPOS to be set.
Specifying the value for the field-name and help-panel-name
When modifying or adding statements to the )HELP section of a new or existing source panel, you must
adhere to these rules to prevent unexpected results and errors when the source panel is processed.
The field-name and help-panel-name must have these characteristics:
• 1-8 characters in length
• The first (or only) character must be A-Z or a-z
• The remaining characters, if any, must be A-Z, a-z, or 0-9.
Lowercase characters are translated to their uppercase equivalents.
The action bar choice and pull-down choice elements have no associated field name. ISPF uses these
conventions when generating a field-name value for these panel elements:
• Action bar choice field-names have the format ZABCxx, where:
ZABC
The field-name prefix
xx
The number of the action bar choice
• Pull-down choice field-names have the format ZPDCxxyy, where:
ZPDC
The field-name prefix
xx
The number of the action bar choice
yy
The number of the pull-down choice within this action bar choice
See “Specifying action bar choices in panel )BODY section” on page 135 to determine the numbering
sequence ISPF uses for these panel elements.
Defining the INEXIT section
The )INEXIT section, which must be specified as the first statement in the panel source member,
identifies a program that is called by ISPF for each source record read for the panel. The program is
passed the panel source record and can change the record, delete the record, or insert a new record.
)INEXIT PGM exit-add
LOAD exit-mod CACHE
Where:
PGM
Keyword that indicates that the exit routine being invoked was loaded when ISPF loaded the
application dialog or was loaded from the application. The application passes ISPF the address of
the exit routine in exit-add.
exit-add
The name of a 4-byte, FIXED format dialog variable that contains the address of the exit routine,
which can reside above or below the 16Mb line. The exit routine receives control in AMODE=31 mode.
This parameter is used in conjunction with the keyword PGM.
Defining the INEXIT section
Chapter 6. Panel definition statement reference  183

## Page 212

LOAD
Keyword that indicates the exit routine is to be loaded dynamically. The application passes ISPF the
module name of the exit routine that is to be dynamically loaded. The module name is passed in the
exit-mod parm.
exit-mod
Identifies the name of the panel input exit routine module that is to be dynamically loaded by ISPF.
The panel input exit name can be passed as a literal or as a dialog variable that contains the panel
user exit name. This parameter is used in conjunction with the LOAD keyword.
CACHE
Keyword that requests ISPF to retain a copy of the panel in virtual storage and to use this copy for
subsequent displays of the panel. By default, a panel with a input exit is not retained in virtual storage.
How to LOAD the panel input exit routine
If the dialog function routine and the panel input exit routine are separate object modules, you can load
the panel input exit routine by any of these methods:
• Linking the exit routine object module to the dialog function object module containing the display
request for the panel containing the )INEXIT statement. Thus, when ISPF loads the application, it also
loads the exit routine.
• Loading the exit routine from the application and passing to ISPF the address of the exit routine.
• Letting ISPF load the exit routine dynamically.
Invoking the panel input exit routine
If the LOAD keyword is specified, ISPF issues an OS load to bring the load module into virtual storage.
ISPF then invokes the exit routine through a call (BASR 14,15). You must use standard OS linkage
conventions when invoking the panel user exit. The exit routine (called in AMODE 31) must support 31-bit
addressing.
Panel exits can be written in languages that use the Language Environment runtime environment.
However, a mixture of Language Environment-conforming main dialog code and service routine code
is not supported. Dialogs and service routines must either all be Language Environment-conforming or all
be Language Environment-nonconforming.
ISPF uses the standard parameter list format to pass parameters. Register one points to a list of
addresses; each address points to a different parameter as shown in Table 19 on page 184.
Table 19. Parameter list format used to pass parameters
Register Points at… Address points at…
R1 Address 1 Panel name
Address 2 Panel record buffer address
Address 3 Panel record buffer length
Address 4 Panel record length
Address 5 Flags
Address 6 Data area address
See “Parameters passed from ISPF to the panel input exit routine” on page 185.
The keyword, LOAD, on the )INEXIT panel statement provides the option of dynamically loading a panel
input exit routine. PGM and LOAD are the only valid keywords:
PGM
Indicates that a panel input exit is already loaded into virtual storage with the address passed in the
exit-add parameter.
Defining the INEXIT section
184  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 213

LOAD
Indicates that the panel user exit routine named by the exit-mod parameter is to be dynamically
loaded by ISPF.
ISPF checks the keyword to determine if the panel input exit routine is to be dynamically loaded. If it is,
ISPF issues an OS load to bring the load module into virtual storage. The search sequence for link libraries
is:
• job pack area
• ISPLLIB
• steplib
• link pack area
• linklib
See the z/OS ISPF Services Guide for further discussion of the search order using LIBDEF.
The panel input exit routine is loaded only once for each SELECT level the first time the panel is displayed.
The loaded panel input exit routine is not deleted until the SELECT (which first displayed the panel) is
terminated.
Parameters passed from ISPF to the panel input exit routine
Parameters passed to the panel input exit routine are (in the order passed):
1. Panel name
The name of the panel for which the panel input exit is being invoked. Its format is CHAR(8), left-
justified in the field. ISPF ignores any changes made to this parameter by the exit.
2. Panel record buffer address
The address of the buffer area containing the data for the latest record read from the panel member.
Its format is a fullword address value. ISPF ignores any changes made to this parameter by the exit.
3. Panel record buffer length
The length of the buffer area containing the data for the latest record read from the panel member. Its
format is a fullword fixed value. ISPF ignores any changes made to this parameter by the exit.
4. Panel record length
The length of the latest record read from the panel member. Its format is a fullword fixed value. ISPF
ignores any changes made to this parameter by the exit.
5. Flags
Four bytes of bit flags passed to the exit by ISPF and defined as:
0
End of file indicator:
0
End of file not reached for panel member.
1
End of file reached for panel member.
1–31
Reserved.
6. Data area address
A fullword that the exit can use to save the address of a data area obtained by the exit and used to
retain or pass information between invocations of the exit. ISPF sets the data area address to 0 on the
initial call to the input exit for a panel.
Return codes and error processing
Return codes, set in the panel input exit routine, recognized by ISPF are:
Defining the INEXIT section
Chapter 6. Panel definition statement reference  185

## Page 214

0
Process the current panel record (exit may have modified the record data).
2
Exit has inserted a new record. The current record will be passed on the next call to the exit.
4
Delete the current panel record.
8
Stop calling the panel input exit. ISPF continues to process the remaining records from the panel
member.
20 (or code unrecognized by ISPF)
Severe error in the exit routine. The DISPLAY service terminates with a severe error condition (return
code 20) and ISPF issues a message indicating that the exit routine issued an incorrect return code.
Panel input exit processing
ISPF calls the panel input exit for each record in the panel member after the initial record with
the )INEXIT statement. Calls to the exit continue until one of the following conditions occurs:
• The )END statement is processed. The panel record containing the )END record is passed to exit for
processing.
• The exit passes a return code of 8 back to ISPF to indicate calls to the exit are no longer required.
• The exit passes a return code of 20 or an unrecognized return code back to ISPF to indicate a severe
error in the exit routine.
• ISPF encounters a terminating or severe error condition during panel processing.
To modify a panel record, the panel input exit must:
• Make the required changes to the data in the buffer area which is addressed using the panel record
buffer address parameter and has a length as specified in the panel record buffer length parameter.
• Set the return code to a value of 0.
To insert a new panel record, the panel input exit must:
• Store the data for the new panel record in the buffer area which is addressed using the panel record
buffer address parameter and has a length as specified in the panel record buffer length parameter.
• Set the return code to a value of 2.
When the panel input exit inserts a new panel record, the record passed by ISPF on that call is again
passed on the subsequent call to the exit.
To delete a panel record, the panel input exit must:
• Set the return code to a value of 4.
The panel input exit can use the data area address parameter to pass the address of a data area between
calls to the exit. For example, on the initial call to the exit it can obtain the storage for the data area and
store the address in the data area address parameter. The data area can then be used to pass information
between calls to the exit. On the last call to the exit (that is, when the )END statement is passed) the exit
should free the storage for the data area.
On the initial call to the input exit for a panel, ISPF sets the data area address parameter to a value of
zero. A panel input exit can test the data area address parameter for a value of zero to identify the initial
call to the exit, provided the exit then sets the data area address parameter to a non-zero value.
Panel input exits cannot issue calls to any ISPF services apart from the VCOPY service. The VCOPY service
can be used by the exit to get the values for ISPF dialog variables.
To assist in debugging problems with panel input exits, the panel trace generated using the ISPDPTRC has
been enhanced to show the return codes and panel source records inserted, changed, and deleted by a
panel input exit.
Defining the INEXIT section
186  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 215

Examples of using panel input exits
This example shows how a panel input exit can be used to dynamically add options to a menu based on
the value of an ISPF variable.
The source for a menu panel contains special IF condition records of the form: <IF &varname=value>
When the input exit finds one of these records, it calls the VCOPY service to get the current value of the
ISPF variable varname and checks whether it matches value. If so, all panel records up to the next END-IF
record (</IF>) are included in the panel. Otherwise the exit deletes these records.
Figure 58 on page 187 shows the source for the menu panel. This source can be found in member
ISPPXMNP in the ISPF samples library ISP.SISPSAMP.
)INEXIT LOAD,ISPPXMNX
)PANEL KEYLIST(ISRSNAB,ISR)
)ATTR DEFAULT(%+_) FORMAT(MIX)
 ~ TYPE(PT)
 ˋ TYPE(FP)
 ! TYPE(NT)
 @ TYPE(SAC)
 \ TYPE(NEF) CAPS(ON) PADC(USER)
 * AREA(SCRL) EXTEND(ON)
 { TYPE(PS) CSRGRP(99)
)BODY  CMD(ZCMD)
!                             ~z/OS Utilities!                                !
ˋOption ===>\Z                                                                !
*SAREA39                                                         *
*                                                                *
*                                                                *
*                                                                *
*                                                                *
)AREA SAREA39
@1 {SDSF         !SDSF                                           !
@2 {DFSMSdfp     !DFSMSdfp/ISMF                                  !
@3 {Security     !Security Server                                !
<IF &USRTASK=UNIX>
@4 {Udlist       !z/OS UNIX Directory List                       !
@5 {UNIX Shell   !z/OS UNIX Shell                                !
</IF>
)INIT
.ZVARS = '(ZCMD)'
.HELP = ISR00003
)PROC
&ZCMDWRK = &Z
IF (&ZCMD = &Z)
  &ZCMDWRK = TRUNC(&ZCMD,'.')
  &ZTRAIL=.TRAIL
  IF (&ZCMDWRK = &Z)
    .MSG = ISRU000
&ZSEL = TRANS (TRUNC (&ZCMD,'.')
  1,'PGM(ISFISP) NOCHECK NEWAPPL(ISF) SCRNAME(SDSF)'
  2,'PGM(DGTFMD01) PARM(&ZCMD) NEWAPPL(DG) SCRNAME(DFSMSDFP) NOCHECK'
  3,'PANEL(ICHP00) SCRNAME(SECURITY)'
<IF &USRTASK=UNIX>
  4,'PGM(ISRUUDL) PARM(ISRUUDLP) SCRNAME(UDLIST)'
5,'CMD(ISHELL) SCRNAME(ISHELL)'
</IF>
  X,EXIT
 ' ',' '
   *,'?')
&ZTRAIL=.TRAIL
)PNTS
FIELD(ZPS01001) VAR(ZCMD) VAL(1)
FIELD(ZPS01002) VAR(ZCMD) VAL(2)
FIELD(ZPS01003) VAR(ZCMD) VAL(3)
<IF &USRTASK=UNIX>
FIELD(ZPS01004) VAR(ZCMD) VAL(4)
FIELD(ZPS01005) VAR(ZCMD) VAL(5)
</IF>
)END
Figure 58. Source in member ISPPXMNP in the ISPF samples library ISP.SISPSAMP
Defining the INEXIT section
Chapter 6. Panel definition statement reference  187

## Page 216

Here is the source for the panel input exit. This source can be found in member ISPPXMNX in the ISPF
samples library ISP.SISPSAMP.
         TITLE ' ISPPXMNX: Dynamic menu panel input exit'
*  Member: ISPPXMNX
*
*  Description: Sample panel input exit used to dynamically add
*               options to menu panel ISPPXMNP based on the value
*               of ISPF variable USRTASK.
*
* 5694-A01     COPYRIGHT IBM Corp. 2009
*              All Rights Reserved
*
ISPPXMNX CSECT ,
ISPPXMNX AMODE 31
ISPPXMNX RMODE ANY
@MAINENT DS    0H
         USING *,@15
         J     @PROLOG
         DC    AL1(18)
         DC    C'ISPPXMNX  2008.263'
         DROP  @15
@PROLOG  STM   @14,@12,12(@13)
         LR    @12,@15
@PSTART  EQU   ISPPXMNX
         USING @PSTART,@12
         LA    @15,0
         L     @00,@SIZDATD+4
         GETMAIN  RU,LV=(0),SP=(15)
         LR    @11,@01
         USING @DATD,@11
         ST    @13,4(,@11)
         ST    @11,8(,@13)
         LM    @15,@01,16(@13)
         LR    @13,@11
         MVC   @PC00001(24),0(@01)
         XR    @05_RC,@05_RC
*----------------------------------------------------------------------
* On initial entry, create the common processing control block used
* to share data between invocations of the exit.
*----------------------------------------------------------------------
         L     @04,@PA00138_exit_warea
         ICM   @06_@OV00005,15,EXIT_WAREA(@04)
         JNZ   @RF00012
         LHI   R0,21
@GS00016 DS    0H
         GETMAIN RU,LV=(0),LOC=ANY
@GE00016 DS    0H
         L     @04,@PA00138_exit_warea
         LR    @06_@OV00005,R1
         ST    @06_@OV00005,EXIT_WAREA(,@04)
         XC    COMM_AREA(21,@06_@OV00005),COMM_AREA(@06_@OV00005)
         MVC   CA_ID(4,@06_@OV00005),@CC00098
*----------------------------------------------------------------------
* If the current panel record passed by ISPF is one of our IF
* statements of the form:
* <IF &varname=varval>
* get the current value of ISPF variable varname and see if its
* value matches varval. If so set a flag to indicate the
* condition is met.
*----------------------------------------------------------------------
@RF00012 L     @04,@PA00130_panel_recp
         L     @07_@OV00009,PANEL_RECP(,@04)
         CLC   PANEL_REC(4,@07_@OV00009),@CC00100
         JNE   @RF00022
         OI    IN_IF(@06_@OV00005),B'10000000'
         NI    IN_IF(@06_@OV00005),B'10111111'
         CLI   PANEL_REC+4(@07_@OV00009),C'&&'
         JNE   @RF00026
         L     @15,@PA00132_panel_recl
         L     @10_@OV00015,PANEL_RECL(,@15)
         BCTR  @10_@OV00015,0
         XC    @TS00001(256),@TS00001
         XR    @08_I,@08_I
         IC    @08_I,@CC00108
         LA    @01,@TS00001(@08_I)
         XR    @02,@02
         MVI   0(@01),X'01'
         LA    @01,PANEL_REC(,@07_@OV00009)
         EX    @10_@OV00015,@SB00226
Defining the INEXIT section
188  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 217

ALR   @01,@02
         LA    @02,PANEL_REC(,@07_@OV00009)
         SLR   @01,@02
         LTR   @08_I,@01
         JNP   @RF00029
         MVI   @TS00001+1,C' '
         MVC   @TS00001+2(254),@TS00001+1
         LR    @02,@08_I
         AHI   @02,-7
         EX    @02,@SM00227
         MVC   VARNAME(8,@06_@OV00005),@TS00001
         XC    @TS00001(256),@TS00001
         XR    @09_J,@09_J
         IC    @09_J,@CC00111
         LA    @01,@TS00001(@09_J)
         XR    @02,@02
         MVI   0(@01),X'01'
         LA    @01,PANEL_REC(,@07_@OV00009)
         EX    @10_@OV00015,@SB00226
         ALR   @01,@02
         LA    @02,PANEL_REC(,@07_@OV00009)
         SLR   @01,@02
         LR    @09_J,@01
         CR    @09_J,@08_I
         JNH   @RF00033
         LR    @07,@09_J
         SLR   @07,@08_I
         BCTR  @07,0
         ST    @07,VARVALL(,@06_@OV00005)
         LA    @09,@CC00114
         ST    @09,@AL00001
         LA    @10,VARNAME(,@06_@OV00005)
         ST    @10,@AL00001+4
         LA    @07,VARVALL(,@06_@OV00005)
         ST    @07,@AL00001+8
         LA    @09,VARVALP(,@06_@OV00005)
         ST    @09,@AL00001+12
         LA    @10,@CC00115
         ST    @10,@AL00001+16
         OI    @AL00001+16,X'80'
         L     @15,@CV00162
         LA    @01,@AL00001
         BASR  @14,@15
         LTR   R15,R15
         JNZ   @RF00038
         L     @10,@PA00138_exit_warea
         L     @03_@OV00058,EXIT_WAREA(,@10)
         L     @10,@PA00130_panel_recp
         L     @09,VARVALP(,@03_@OV00058)
         L     @14,PANEL_RECP(,@10)
         L     @02,VARVALL(,@03_@OV00058)
         BCTR  @02,0
         ALR   @14,@08_I
         EX    @02,@SC00229
         JNE   @RF00038
         OI    IF_COND_MET(@03_@OV00058),B'01000000'
@RF00038 DS    0H
         LHI   @05_RC,4
*----------------------------------------------------------------------
* If the current panel record passed by ISPF is one of our END-IF
* statements of the form:
* </IF>
* terminate any existing IF condition processing.
*----------------------------------------------------------------------
         J     @RC00022
@RF00022 CLC   PANEL_REC(5,@07_@OV00009),@CC00118
         JNE   @RF00045
         NI    IF_COND_MET(@06_@OV00005),B'00111111'
         LHI   @05_RC,4
*----------------------------------------------------------------------
* If the current panel record passed by ISPF is the )END statement
* then cleanup exit processing.
*----------------------------------------------------------------------
         J     @RC00045
@RF00045 CLC   PANEL_REC(5,@07_@OV00009),@CC00119
         JNE   @RF00051
         LHI   R0,21
         LR    R1,@06_@OV00005
@GS00056 DS    0H
         FREEMAIN RU,LV=(0),A=(1)
@GE00056 DS    0H
*----------------------------------------------------------------------
Defining the INEXIT section
Chapter 6. Panel definition statement reference  189

## Page 218

* If the current panel record passed by ISPF is within one of our
* IF conditions, check if the condition was found to be true. If
* so allow the panel record to be processed by ISPF. If no tell
* ISPF to delete the panel record.
*----------------------------------------------------------------------
         J     @RC00051
@RF00051 TM    IN_IF(@06_@OV00005),B'10000000'
         JNO   @RF00059
         TM    IF_COND_MET(@06_@OV00005),B'01000000'
         JNZ   @RF00061
         LHI   @05_RC,4
@RF00061 DS    0H
         LR    @01,@11
         L     @13,4(,@13)
         LA    @15,0
         L     @00,@SIZDATD+4
         FREEMAIN RU,LV=(0),A=(1),SP=(15)
         LR    @15,@05
         L     @14,12(,@13)
         LM    @00,@12,20(@13)
         BR    @14
@DATA    DS    0F
@SIZDATD DS    0A
         DC    AL1(0)
         DC    AL3(@DYNSIZE)
         DC    A(@DYNSIZE)
@SB00226 TRT   PANEL_REC(0,@07_@OV00009),@TS00001
@SM00227 MVC   @TS00001(0),PANEL_REC+5(@07_@OV00009)
@SC00229 CLC   VARVAL(0,@09),PANEL_REC(@14)
@DATD    DSECT
         DS    0F
@SA00001 DS    18F
@PC00001 DS    6F
@AL00001 DS    5A
ISPPXMNX CSECT ,
         DS    0F
         DS    0D
@DATD    DSECT
         DS    0D
@TS00001 DS    CL256
ISPPXMNX CSECT ,
         LTORG
         DS    0D
@CV00162 DC    A(X'80000000'+ISPLINK)
@CC00115 DC    CL7'LOCATE '
@CC00114 DC    CL6'VCOPY '
@CC00118 DC    CL5'</IF>'
@CC00119 DC    CL5')END '
@CC00098 DC    CL4'IXCA'
@CC00100 DC    CL4'<IF '
@CC00108 DC    CL1'='
@CC00111 DC    CL1'>'
         DS    0D
@DATD    DSECT
         ORG   *+1-(*-@DATD)/(*-@DATD)
@ENDDATD DS    0X
@DYNSIZE EQU   ((@ENDDATD-@DATD+7)/8)*8
ISPPXMNX CSECT ,
         DS    0F
@00      EQU   0
@01      EQU   1
@02      EQU   2
@03      EQU   3
@04      EQU   4
@05      EQU   5
@06      EQU   6
@07      EQU   7
@08      EQU   8
@09      EQU   9
@10      EQU   10
@11      EQU   11
@12      EQU   12
@13      EQU   13
@14      EQU   14
@15      EQU   15
@STATNUM   EQU 0
@DATANUM   EQU 1
@DATAREG1  EQU @11
@DATALOC1  EQU @DATD
@05_RC   EQU   @05
@03_@OV00058 EQU @03
@06_@OV00005 EQU @06
Defining the INEXIT section
190  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 219

@07_@OV00009 EQU @07
@08_I    EQU   @08
@10_@OV00015 EQU @10
@09_J    EQU   @09
R0       EQU   @00
R1       EQU   @01
R15      EQU   @15
         EXTRN ISPLINK
PANEL_NAME EQU 0,8,C'C'
PANEL_RECP EQU 0,4,C'A'
PANEL_BUFL EQU 0,4,C'F'
PANEL_RECL EQU 0,4,C'F'
PFLAGS   EQU   0,4,C'B'
EXIT_WAREA EQU 0,4,C'A'
COMM_AREA EQU  0,21,C'C'
CA_ID    EQU   COMM_AREA,4,C'C'
VARNAME  EQU   COMM_AREA+4,8,C'C'
VARVALL  EQU   COMM_AREA+12,4,C'F'
VARVALP  EQU   COMM_AREA+16,4,C'A'
CA_FLAGS EQU   COMM_AREA+20,1,C'B'
IN_IF    EQU   CA_FLAGS,1,C'B'
IF_COND_MET EQU CA_FLAGS,1,C'B'
PANEL_REC EQU  0,,C'C'
VARVAL   EQU   0,,C'C'
@PA00138_exit_warea EQU @PC00001+20,4,C'F'
@PA00133_pflags EQU @PC00001+16,4,C'F'
@PA00132_panel_recl EQU @PC00001+12,4,C'F'
@PA00131_panel_bufl EQU @PC00001+8,4,C'F'
@PA00130_panel_recp EQU @PC00001+4,4,C'F'
@PA00129_panel_name EQU @PC00001,4,C'F'
@RF00033 EQU   @RF00038
@RF00059 EQU   @RF00061
@RF00029 EQU   @RF00033
@RC00051 EQU   @RF00059
@RF00026 EQU   @RF00029
@RC00045 EQU   @RC00051
@RC00022 EQU   @RC00045
         DS    0D
@ENDDATA EQU   *
@MODLEN  EQU   @ENDDATA-ISPPXMNX
         END   ,(PL/X-390,0203,08263)
Further panel input exit examples can be found in the ISPF samples library ISP.SISPSAMP.
Member ISPPXINP contains the source code for an ISPF menu panel. The source contains special
*INCLUDE statements which are recognized by a panel input exit and used to include panel code
from members in the ISPPLIB DD concatenation. The input exit handles processing of the *INCLUDE
statements, reading the records from the include members, and passing these records to ISPF for
inclusion in the panel code. The exit supports nested include members. The source code for this panel
exit is in samples member ISPPXINX.
Member ISPPXDAP contains the source for an ISPF panel that displays the values of static and dynamic
system symbols. A panel input exit is used to cause the panel to display either the static symbols, the
dynamic symbols, or both depending on the value found in ISPF dialog variable DISPREQ. The source
code for this panel exit is in samples member ISPPXDAX.
Defining the initialization section
The initialization section specifies the initial processing that is to occur before the panel is displayed.
)INIT
It begins with the )INIT header statement and ends with either the )REINIT, )PROC, )HELP, or )END
header statement. The number of lines allowed in an )INIT section depends upon the storage size
available for panel processing at execution time.
The variables that are displayed in the panel body reflect the contents of the corresponding dialog
variables after the )INIT section has been processed, just before the panel is displayed. The input fields
are automatically stored into the corresponding dialog variables immediately following display and before
processing the )PROC section.
See “Formatting panel definition statements” on page 199 for more information.
Defining the initialization section
Chapter 6. Panel definition statement reference  191

## Page 220

Defining the LIST section
The )LIST section is accepted in order to support existing panel definitions that use it. However, it no
longer affects the displayed panel.
Defining the model section
The )MODEL section defines how each table row is to be formatted. Because the model section is used
only for table display panels, it is discussed in Defining  table display panels—see “Requirements for model
section” on page 117.
Defining the panel section
The )PANEL section specifies the keylist that will be used for the panel, identifies where the keylist is to be
found, and controls specific CUA display characteristics of the panel.
)PANEL
KEYLIST ( keylist-name
, keylist-applid ,SHARED
)
IMAGE ( image-name , row, col)
where:
KEYLIST
keylist-name
Required when KEYLIST is specified. The keylist name must have these characteristics:
• 1-8 characters in length
• First, or only, character must be A-Z or a-z
• Remaining characters, if any, must be A-Z, a-z, or 0-9.
Lowercase characters are translated to their uppercase equivalents.
keylist-applid
Optional. Application ID used at run time to find the keylist. It has a maximum length of 4
characters, the first of which must be alphabetic. Any remaining characters can be alphabetic or
numeric.
SHARED
Optional. When specified, ISPF looks only at the shared keylist for the panel. If the user issues the
KEYLIST OFF or KEYLIST PRIVATE commands, they have no effect; the keylist in xxxxKEYS table
allocated to ISPTLIB is used.
IMAGE
The IMAGE keyword is accepted in order to support existing panel definitions that use it. However, it
no longer affects the displayed panel.
When a keylist-name is specified without a keylist-applid, ISPF searches for the named keylist in the:
• Keylists for the application ID that is currently running
• ISP applid (if not found in application ID that is currently running and the name of the application ID is
not ISP).
If the KEYLIST keyword is not found on the )PANEL statement, then the default keylist, ISPKYLST, is used.
Before runtime processing, any keylist (other than the default ISPKYLST) referenced in a panel's
definitions must have been created and stored. If you add or modify the )PANEL KEYLIST statement
in the definition of an existing source panel, you must create the keylist if it does not already exist. New
keylists can be created using ISPF option 0 or using the Dialog Tag Language.
Defining the LIST section
192  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 221

Keylist variables
These variables are used by the keylist function:
ZKLUSE
Y or N, this variable indicates whether the keylists are being used for an application ID or not. For
example, if KEYLIST OFF has been issued, &ZKLUSE is N. This variable is stored in the application
profile. The VPUT service can be used by your application to set this value. Putting a value of N in
&ZKLUSE to the profile pool is equivalent to issuing the KEYLIST OFF command. Putting a value of Y in
&ZKLUSE to the profile pool is equivalent to issuing the KEYLIST ON command.
ZKLNAME
contains the name of the keylist of the panel currently being displayed. If no keylist is defined for the
panel or the keylist is not being used, &ZKLNAME is blank.
ZKLAPPL
contains the application ID where the keylist of the panel currently being displayed is found. If no
keylist is defined for the panel or the keylist is not being used, &ZKLAPPL is blank.
ZKLTYPE
P or S, this variable indicates that the keylist for the panel currently being displayed is a private (P)
copy defined in the profile table, or a shared (S) copy defined in the xxxxKEYS table (where xxxx is the
application ID of the keylist (ZKLAPPL)).
ZKLPRIV
Y or N, this variable indicates that ISPF is to look at both the private and shared keylist (Y, the default)
or that it is to look at only the shared keylists (N). This variable is stored in the application profile. The
VPUT service can be used by the application to set this value. Putting a value of N in &ZKLPRIV to the
profile pool is equivalent to issuing the KEYLIST SHARED command. Putting a value of Y in &ZKLPRIV
to the profile pool is equivalent to issuing the KEYLIST PRIVATE command.
Note: This variable shows and determines where ISPF looks for a keylist. &ZKLTYPE is a non-
modifiable variable that shows where ISPF found the keylist.
CUA display characteristics
The )PANEL section controls specific CUA display characteristics of a panel. Specifying the )PANEL
statement in the panel source definition affects the same display characteristics controlled by selecting
the Panel display CUA mode option on the ISPF Settings panel (Option 0). See the z/OS ISPF User's Guide
Vol II for more information.
The )PANEL statement controls these CUA display characteristics:
• Display and placement of the command line and long message text
• Building and display of the named keylist in the Function Key Area (FKA)
• Handling of undefined or null function key definitions
• Execution of the CANCEL and EXIT commands
• Setting of three system control variables that relate to the position of the cursor after panel display.
Command lines and long messages
When the )PANEL section is used, the ISPF default command line placement is at the bottom of the panel
(above the function key area, if it is displayed). Long messages are displayed above the command line.
To override the ISPF default, go to the ISPF Settings panel and specify Command line placement - ASIS.
This setting places the command line and long message as they are specified in your panel definition
(usually at the top of the panel). See z/OS ISPF User's Guide Vol I. Changes to the )BODY section also
affect command line and long message placement. The ASIS keyword on the )BODY section overrides
ISPF defaults. The WINDOW keyword also affects the displaying of the command line and long messages.
See “Defining the body section” on page 169.
You can specify to not have a command line by including the keyword CMD() with no value on the )BODY
statement. This is valid only for displaying panels with the DISPLAY service. In this case, the default
Defining the panel section
Chapter 6. Panel definition statement reference  193

## Page 222

position of the long message is at the bottom of the panel above the FKA, if it is displayed. Panels (tables)
displayed with the TBDISPL service must specify a command area either by coding a CMD() with a value
or by coding the system control variable ZCMD in the panel body.
Because the )PANEL statement affects the same display characteristics as if you had selected the Panel
display CUA mode option on the ISPF Settings panel, the color and intensity of the short and long
messages is affected by the presence of the )PANEL statement. If you specify the LMSG or SMSG
keywords on the )BODY statement, you control the color and intensity in which both the short and long
messages are displayed, regardless of CUA mode or the presence of a ) PANEL statement. Table 26 on
page 270 illustrates default message placement.
Keylist building and display
The format and display of the named keylist or an ISPF default keylist for a panel containing the )PANEL
statement is as follows:
• The maximum number of function keys that can be formatted on each line is displayed.
• Each displayed function key definition appears as Fnn=label or Fn=label (where nn or n is the numeric
value of the function key).
ISPF attempts to build the FKA with the named keylist or an ISPF default keylist. However, the display of
the keylist in the FKA area depends upon the settings of the FKA or PFSHOW commands and the keylist
format (SHORT or LONG) specified for the function key definition. The number and set of function keys
displayed also varies.
Note: The system control variable ZPFCTL setting is ignored for panel source definitions that contain
the )PANEL statement.
Undefined or null function keys
When you press an undefined or null function key, ISPF displays an error message.
CANCEL and EXIT execution
When the CANCEL or EXIT commands (specified on a function key or entered in a command field) are
processed, ISPF returns the entered command in the system control variable ZVERB and sets a return
code of 8 from the display service.
If the panel contains an action bar and the cursor is on the action bar, CANCEL moves the cursor to the
panel body. ZVERB is not updated.
Setting system control variables
When panels with a )PANEL section specified are displayed, ISPF sets these system control variables:
ZCURFLD
Name of the field (or list column) containing the cursor when the user exits the panel.
ZCURPOS
Position of the cursor within the field specified by ZCURFLD when the user exits the panel.
ZCURINX
Current row number of the table row containing the cursor.
These system variables are stored in the function pool as output variables.
Defining the point-and-shoot section
The )PNTS (point-and-shoot) section of a panel definition specifies what fields, if any, are point-and-shoot
fields. Input and output fields are specified as point-and-shoot fields by the use of the attribute keyword,
PAS(ON). Text fields are specified as point-and-shoot fields by the attribute type keyword, TYPE(PS). For
each panel field specified as a point-and-shoot field, there must be a corresponding entry in the )PNTS
section. If a field specified as a point-and-shoot field has no corresponding entry in the )PNTS section,
Defining the point-and-shoot section
194  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 223

no action will be taken if the point-and-shoot field is selected. The examples show a )PNTS section
point-and-shoot phrase definition for input/output fields and for text fields.
Note:
• You can use option 0 (Settings) to set the tab key to move the cursor point-and-shoot fields. This
changes output fields to input fields, but data is not altered. However, if a variable is used on an output
field that is changed to an input field by the tab to point-and-shoot option, and the variable is VDEFINEd
to the application, the variable will be truncated. In this case, the application developer should have a
temporary panel variable.
• If there is a command entered on the command line, the point-and-shoot field is ignored.
)PNTS FIELD( field_name
ZPS xxyyy
) VAR( value) VAL( value)
DEPTH( depth) IMAGE( image-name ) IMAGEP(  image-name )
TEXT(' text') PLACE( a, b, l, r)
Note: Each entry in the )PNTS section must contain the keywords in this order: FIELD, VAR, VAL.
where:
FIELD
For point-and-shoot input/output fields, the format is:
FIELD(field _name )
where:
field_name
The name of the field on the panel that this statement controls.
For point-and-shoot text fields, the format is:
FIELD(ZPSxxyyy)
where:
xx
00 for a point-and-shoot field defined in the )BODY section and 01 to 99 for the number of the
scrollable area in which the point-and-shoot text field is defined.
Each scrollable area is assigned a sequential number based on its relative position within the
panel body. The scrollable area closest to the upper-left corner of the panel body is assigned
number 01. Each additional scrollable area, scanning left to right, top to bottom, is assigned
the next sequential number. A maximum of 99 scrollable areas in any given panel can contain
point-and-shoot text fields.
yyy
001 to 999 for the relative number of the point-and-shoot text field within the panel body or
within a particular scrollable area.
A point-and-shoot text field can wrap around multiple terminal lines in panels that are not
displayed in a window. A point-and-shoot text field that logically wraps in a pop-up window
requires the beginning of each wrapped line to contain a PS field attribute and an entry must
exist in the )PNTS section for each wrapped line. This is also true for panels containing the
WINDOW() keyword that are not displayed in a pop-up window. The additional )PNTS section
entries should result in the same action as the first line of the wrapped text field.
VAR(value)
The name, or a variable containing the name, of the variable to be set when the field named
in this )PNTS statement is selected. If the value is a variable, an ampersand (&) must be in the
Defining the point-and-shoot section
Chapter 6. Panel definition statement reference  195

## Page 224

first column following the left parenthesis of the VAR keyword, and it must follow dialog variable
naming conventions. If the value is a variable it is limited to the leading ampersand plus 7
characters.
VAL(value)
The value assigned to the variable named in this statement. The value can be a variable or text. If
the value is a variable, an ampersand (&) must be in the first column following the left parenthesis
of the VAL keyword. The length of the variable data is limited to 255 single-byte characters. If the
variable data is longer than 255 bytes, it is truncated. If the value is a variable it is limited to the
leading ampersand plus 7 characters.
VAL(&var)
If the value is a single word text string it is not necessary to enclose it in single quotation marks.
VAL(Batch)
If the value is more than a single word of text, the phrase must be enclosed in single quotation
marks.
VAL('List of products')
Literal values can be split between lines by coding a plus sign (+) as the last character on each line
that is to be continued. The plus sign is used as a continuation character.
VAL('This is an example of a continuation +
of the literal string')
DEPTH(depth)
The depth of the point-and-shoot field. ISPF allows depth values from zero to sixty-two (0 -
62). The maximum screen depth is 62. It is up to the dialog developer to define the depth such
that other items on the panel body will not be overlaid by the point-and-shoot field. If depth is
specified as 0, the default depth of two (2) is used. The depth can be a variable, whose value
is from 0-62. This attribute is accepted in order to support existing DTL source files that use
it. However, although the space is reserved, point-and-shoot does not function in the additional
reserved space.
IMAGE(image-name)
The IMAGE keyword is accepted in order to support existing panel definitions that use it. However,
it no longer affects the displayed panel.
IMAGEP(image-name)
The IMAGEP keyword is accepted in order to support existing panel definitions that use it.
However, it no longer affects the displayed panel.
TEXT('text')
The TEXT keyword is required for point-and-shoot text fields. The text ties the point-and-shoot
text field defined in the panel body with its point-and-shoot entry in the )PNTS section. The text
must match the text for the point-and-shoot field in the body. If the text in the body contains
variables, the text of the TEXT keyword must allow for the possible expansion once the variable
has been substituted, just as the point-and-shoot text field in the body should. If the text consists
of more than a single word of text, the phrase must be enclosed in single quotation marks.
PLACE
The PLACE keyword is accepted in order to support existing panel definitions that use it. However,
it no longer affects the displayed panel.
Defining the point-and-shoot section
196  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 225

Example:
)PANEL
)ATTR
  $ TYPE(PIN)
  } TYPE(PS)
  + TYPE(NT)
  | AREA(SCRL) EXTEND(ON)
  ! TYPE(OUTPUT) PAS(ON) COLOR(RED)
  * TYPE(OUTPUT) PAS(ON) COLOR(BLUE)
  @ TYPE(TEXT) INTENS(LOW) COLOR(RED) PAD(NULLS)
  ø TYPE(TEXT) INTENS(LOW) COLOR(BLUE) PAD(NULLS)
)BODY WINDOW(60,23)
$
%COMMAND ===>_ZCMD
$
$  Press }DEFAULTS$to reinstate defaults
$
+
|S1                                                |
)AREA S1
+                                        +
+                                        +
+     øBLUE  . . . .*BLUE1               +
+     @RED . . . . .!RED1                +
)INIT
 .CURSOR = blue1
)PROC
 REFRESH(*)
)PNTS
  FIELD(BLUE1) VAR(RED1) VAL(RED)
  FIELD(RED1) VAR(BLUE1) VAL(BLUE)
  FIELD(ZPS00001) VAR(BLUE1) VAL(DEFAULT)
)END
Figure 59. Sample point-and-shoot definition 
Defining the processing section
The processing section specifies additional processing that is to occur after the panel has been displayed.
In the case of a primary option menu panel, the processing section also executes before the initial display
of the panel. It begins with the )PROC header statement and ends with the )HELP or )END statement. The
number of lines allowed in a )PROC section depends upon the storage size available.
To support an initial command stack being provided in an ISPF variable to a primary option menu
specified using the PANEL parameter, ISPF puts the variable name (or "ZSTART DEFAULT" when the
default cmd_stack_var_name value ZSTART is used) into the ZCMD variable. Then, the )PROC section of
the first primary option menu displayed is executed before the initial display of the panel. The primary
option menu in this scenario must not perform verification of the ZCMD variable in the )PROC section
unless the verification allows for the initial command stack variable name (for example, ZSTART) to be
stored in ZCMD. See “Syntax for issuing the ISPSTART command” on page 8 for more information on
initial command stack processing by a primary option menu.
)PROC
A statement can be continued over as many lines as necessary as long as it is broken at the end of a word,
or a continuation symbol (+) is used within a literal. In menus, the processing section is required and must
be in a special format, as described in “Defining menus” on page 98.
See “Formatting panel definition statements” on page 199 for additional information.
Defining the reinitialization section
The reinitialization section specifies processing that is to occur prior to redisplay of a panel. If it is
present, it follows the initialization section and precedes the processing section.
)REINIT
Defining the processing section
Chapter 6. Panel definition statement reference  197

## Page 226

Panel redisplay occurs in either of these situations:
• Redisplay occurs automatically after the )PROC section has been processed if the .MSG control variable
is nonblank and the user has not requested END or RETURN. The .MSG control variable is set
automatically if a translation or verification error occurs. It can also be set explicitly by use of an
assignment statement in the )PROC section.
• Redisplay occurs if a dialog function invokes the DISPLAY or TBDISPL service with no panel name
specified (a blank).
Note: See z/OS ISPF Services Guide under the description of TBDISPL for a explanation of how redisplay
processing for the TBDISPL service differs from that for the DISPLAY service described here.
Processing of the )INIT section is intentionally bypassed when a redisplay occurs. Instead, the )REINIT
section is processed. The automatic fetching of variables to be displayed in the panel body is also
bypassed on a redisplay. Thus, the panel is redisplayed exactly as the user last saw it, except:
• An error message can appear on a redisplay.
• Field attribute overrides, assignment statements, or REFRESH statements can be used.
• A scrollable area can be scrolled to position the cursor or to verify failure.
Typically, a )REINIT section contains:
• Field attribute overrides, specified by the .ATTR control variable
• Changes to displayed panel fields, specified by assignment statements and the REFRESH statement.
See “Formatting panel definition statements” on page 199 for additional information.
Figure 60 on page 199 shows panel processing and the point at which attribute settings can be modified
for redisplay of a panel.
Defining the reinitialization section
198  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 227

Figure 60. Panel processing
Formatting panel definition statements
This topic describes panel definition statements:
• Assignment statements. See “The assignment statement” on page 200.
Note: You can use ten built-in functions in an assignment statement:
– TRUNC (truncate)
– TRANS (translate)
Defining the reinitialization section
Chapter 6. Panel definition statement reference  199

## Page 228

– PFK (function key)
– LENGTH (return length of variable)
– UPPER (return uppercase value of variable)
– LVLINE (last visible line)
– ADDSOSI (add shift-out character)
– DELSOSI (delete shift-out character)
– ONEBYTE (convert to a 1-byte code)
– TWOBYTE (convert to a 2-byte code)
• ELSE on page “The ELSE statement” on page 207
• EXIT on page “EXIT and GOTO statements” on page 208
• GOTO on page “EXIT and GOTO statements” on page 208
• IF on page “The IF statement” on page 210
• PANEXIT on page “The PANEXIT statement” on page 214
• REFRESH on page “The REFRESH statement” on page 220
• *REXX … *ENDREXX on page “The *REXX statement” on page 221
• TOG on page “The TOG statement” on page 228
• VEDIT on page Figure 66 on page 229
• VER on page “The VER statement” on page 230
• VGET on page “The VGET statement” on page 241
• VPUT on page “The VPUT statement” on page 243
These types of data references can appear within panel section statements:
Dialog variable
A name preceded by an ampersand (&)
Control variable
A name preceded by a period (.)
Literal value
A character string not beginning with an ampersand or period. A literal value can be enclosed in single
quotes (‘’). It must be enclosed in single quotes if it begins with a single ampersand or a period, or if it
contains any of these special characters:
Blank < ( + | ) ; ¬ - , > : =
A literal can contain substitutable variables, consisting of a dialog variable name preceded by an
ampersand (&). The name and ampersand are replaced with the value of the variable before processing
the statement. Trailing blanks are removed from the variable before the replacement. You can use a
double ampersand to specify a literal character string starting with, or containing, an ampersand.
In the description of statements and built-in functions that follows, a variable can be either a dialog
variable or a control variable. A value can be either type of variable or a literal value.
The assignment statement
Assignment statements can be used in the )INIT section to set the contents of dialog variables before the
automatic initialization of variables in the panel body. Also, they can be used in the )REINIT section before
redisplay of the panel body. Assignment statements can also be used in the )PROC section, typically to set
the contents of dialog variables that do not correspond to fields in the panel body.
variable = value
where:
assignment statement
200  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 229

value
Specifies the contents of the dialog variable.
Example:
&A     = ‘’
&COUNT = 5
&DSN   = ‘’‘SYS1.MACLIB’‘’
&BB    = &C
The first example sets variable A to blanks. The second example sets variable COUNT to a literal character
string (the number 5). The third example sets variable DSN to a character string that begins and ends with
a single quote. See Chapter 5, “Panel definition statement guide,” on page 87 for information about syntax
rules and restrictions. The fourth example sets variable BB to the contents of another variable, C.
The literal ' ' represents a single blank. To define a null, you must use the &Z literal.
The TRUNC built-in function
The TRUNC built-in function can occur on the right side of an assignment statement to cause truncation.
variable = TRUNC ( variable , value)
where:
variable
(Inside the parentheses). Specifies the variable to be truncated.
value
A numeric quantity indicating the length of the truncated result or any special character indicating
truncation at the first occurrence of that character.
Examples:
&A = TRUNC (&XYZ,3)
&INTEG = TRUNC (&NUMB,‘.’)
In the first example, the contents of variable XYZ are truncated to a length of 3 characters and stored
in variable A. Variable XYZ remains unchanged. In the second example, the contents of variable NUMB
are truncated at the first occurrence of a period and stored in variable INTEG. Variable NUMB remains
unchanged. If NUMB contains 3.2.4, INTEG contains 3.
The control variable .TRAIL contains the remainder following a TRUNC operation. When the contents
of a variable are truncated to a specified length, all remaining characters are stored in .TRAIL. If the
contents of a variable are truncated at the first occurrence of a special character, the remaining characters
following the special character are stored in .TRAIL. The special character is not stored, nor is it retained
in the assignment variable's value. For example:
)PROC
  &AAA = TRUNC (&ZCMD, ‘.’)
  &BBB = .TRAIL
If variable ZCMD contains 9.4.6, variable AAA contains 9. The .TRAIL control variable and variable BBB
contain 4.6. The value of ZCMD remains as 9.4.6.
Because the control variable .TRAIL is set to blanks before the truncation function is performed, it
should not be specified as the truncation variable in the TRUNC statement. For example: &ERROR =
TRUNC(.TRAIL,1) would always result in &ERROR being set to blank.
For the TRUNC built-in function, the source and destination variables can be the same. Figure 61 on page
203 shows an example in which it is assumed that variable TYPECHG was originally set (in the dialog
function) to a single character N, U, or D. In the )INIT section, TYPECHG is translated to NEW, UPDATE, or
assignment statement
Chapter 6. Panel definition statement reference  201

## Page 230

DELETE and stored into itself before the panel is displayed. In the )PROC section, TYPECHG is truncated
back to a single character.
Use of this technique allows you to change the valid options for TYPECHG by simply typing over the first
character.
The TRUNC and TRANS built-in functions can be nested. For example:
&XYZ = TRUNC( TRANS(&A ---),1 )
&ZSEL = TRANS( TRUNC(&ZCMD,‘.’) --- )
In the first example, the current value of variable A is translated. The translated value is then truncated to
a length of one, and the result is stored in variable XYZ. In the second example, the contents of variable
ZCMD are truncated at the first period, the truncated value is then translated, and the result is stored in
variable ZSEL.
The VSYM built-in function can be nested on the TRANS and TRUNC built-in functions. For example:
&B = TRANS(VSYM(A) A,1 B,2 *,3)
&B = TRANS(TRUNC(VSYM(A),1) A,1 B,2 *,3)
The TRANS built-in function
The TRANS built-in function can occur on the right side of an assignment statement to cause translation.
variable = TRANS ( variable  value, value
MSG= value
)
where:
variable
(Inside the parentheses). Specifies the variable to be translated.
value,value
Paired values. The maximum number of paired values allowed is 126. The first value in each pair
indicates a possible value of the variable, and the second indicates the translated result.
Example:
&REPL = TRANS (&MOD Y,YES N,NO)
The current value of variable MOD is translated, and the result is stored in variable REPL. Variable
MOD remains unchanged. The translation is as follows: if the current value of MOD is Y, it is translated
to YES. If the current value is N, it is translated to NO. If the current value is anything else (neither Y
nor N), it is translated to blank.
The anything-else condition can be specified by using an asterisk in the last set of paired values. For
example:
&REPL = TRANS (&MOD  ...  *,‘?’)
&REPL = TRANS (&MOD  ...  *,*)
In the first example, if the current value of MOD does not match any of the listed values, a question
mark is stored in variable REPL. In the second example, if the current value of MOD does not match
any of the listed values, the value of MOD is stored untranslated into REPL.
MSG=value
A message ID. Another option for the anything-else condition is to cause a message to be displayed to
the user. Typically, this technique is used in the processing section of the panel definition.
Example:
assignment statement
202  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 231

&DISP = TRANS (&D 1,SHR 2,NEW 3,MOD MSG=PQRS001)
The contents of variable D are translated as follows: 1 is translated to SHR, 2 is translated to NEW, and
3 is translated to MOD. If none of the listed values is encountered, message PQRS001 is displayed.
Message PQRS001 can be an error message indicating that the user has entered an invalid option.
For the TRANS built-in function, the source and destination variables can be the same. Figure 61 on page
203 shows an example in which it is assumed that variable TYPECHG was originally set (in the dialog
function) to a single character N, U, or D. In the )INIT section, TYPECHG is translated to NEW, UPDATE, or
DELETE and stored into itself before display of the panel. In the )PROC section, TYPECHG is truncated
back to a single character.
Use of this technique allows you to change the valid options for TYPECHG by simply typing over the first
character.
The TRANS and TRUNC built-in functions can be nested. For example:
&XYZ = TRUNC( TRANS(&A ---),1 )
&ZSEL = TRANS( TRUNC(&ZCMD,‘.’) --- )
In the first example, the current value of variable A is translated. The translated value is then truncated to
a length of one, and the result is stored in variable XYZ. In the second example, the contents of variable
ZCMD are truncated at the first period, the truncated value is then translated, and the result is stored in
variable ZSEL.
The VSYM built-in function can be nested on the TRANS and TRUNC built-in functions. For example:
&B = TRANS(VSYM(A) A,1 B,2 *,3)
&B = TRANS(TRUNC(VSYM(A),1) A,1 B,2 *,3)
)Body
%----------------------------  EMPLOYEE RECORDS  -------------------------------
%COMMAND===>_ZCMD                                                              %
+ 
%EMPLOYEE SERIAL: &EMPSER
+ 
+   TYPE OF CHANGE%===>_TYPECHG +  (NEW, UPDATE, OR DELETE)
+ 
+   EMPLOYEE NAME:
+     LAST   %===>_LNAME         + 
+     FIRST  %===>_FNAME         + 
+     INITIAL%===>_I+ 
+ 
+   HOME ADDRESS:
+     LINE 1 %===>_ADDR1                                   + 
+     LINE 2 %===>_ADDR2                                   + 
+     LINE 3 %===>_ADDR3                                   + 
+     LINE 4 %===>_ADDR4                                   + 
+ 
+   HOME PHONE:
+     AREA CODE   %===>_PHA+ 
+     LOCAL NUMBER%===>_PHNUM   + 
+ 
)Init
  &TYPECHG = Trans (&TYPECHG  N,NEW  U,UPDATE  D,DELETE)
)Proc
  &TYPECHG = Trunc (&TYPECHG,1)
)End
Figure 61. Sample panel definition  with TRANS and TRUNC
The PFK built-in function
The PFK built-in function provides function key assignment information by command or key number.
variable = PFK ( value)
assignment statement
Chapter 6. Panel definition statement reference  203

## Page 232

where:
value
Either a command or a key number.
Example:
&X = PFK (HELP)
&Y = PFK (2)
In the first example, the first function key that is assigned to the HELP command is returned in variable X
as a character string PFnn, where nn is the function key number. If CUA mode is set, or the panel has an
active keylist, the character string is Fnn, where nn is the function key number. If the HELP command is
not assigned to a function key, a blank value is returned.
In scanning the current function key definitions, the primary keys are scanned first, then the secondary
keys. If KEYLIST OFF has been issued, ISPF searches the ZPF variables. On a 24-key terminal, for
example, if both function keys 13 and 1 are assigned to HELP, the function returns F13.
In the second example, the command assigned to F2 is returned in variable Y. If no command is assigned
to the key requested, a blank value is returned.
The LENGTH built-in function
The LENGTH built-in function can occur on the right side of an assignment statement to evaluate the
length of a dialog variable. The variable length returned will be the maximum value of the actual length of
the variable if it exists and the length specified in the )FIELD section if any.
variable = LENGTH ( field-name )
where:
field -name 
Specifies the dialog variable name.
Here is an example:
&A = LENGTH(ABC)
The length of dialog variable ABC is stored in &A. If ABC does not exist, zero is returned. If we added this
section to the panel:
)FIELD
  FIELD(ABC) LEN(105)
then the length calculated for &A will be 105 if ABC does not exist or exists with a length less than 105.
The UPPER built-in function
The UPPER built-in function can occur on the right side of an assignment statement and will return the
uppercase value of a variable.
variable = UPPER ( field-name )
where:
field -name 
Specifies the dialog variable name.
Here is an example:
&A = UPPER(ABC)
The uppercase value of ABC dialog variable will be returned.
assignment statement
204  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 233

The LVLINE built-in function
The LVLINE built-in function (used on an assignment statement in the )INIT, )REINIT, or )PROC section)
provides the line number of the last visible line within a graphic or dynamic area of the currently displayed
panel.
variable = LVLINE ( value)
where:
value
Name of the GRAPHIC or DYNAMIC area. In split-screen mode, this value could be less than the
number of lines defined in the area.
This built-in function provides the line number of the last line within a graphic or dynamic area that is
visible to the user on the currently displayed panel. The value parameter is the name of the graphic or
dynamic area. In split-screen mode, this value could be less than the number of lines defined in the area.
If the area is defined within a scrollable area, the number returned is the last visible line when the user
submitted the panel, even if the user could have scrolled to see more.
Note: When coding the command line after the dynamic area on a non-TBDISPL panel, ISPF might not
be able to calculate the LVLINE value correctly based on the location of the command line following the
dynamic area, the number of lines after the dynamic area, the function key settings, SPLIT or SPLITV
command processing, or other ISPF commands that affect the screen size displayed. To achieve the
correct LVLINE value with the command line displayed at the bottom of the ISPF dynamic area panel, the
command line will have to be coded above the dynamic area on the panel, ZPLACE set to BOTTOM, and
CUA mode set to YES.
Example:
&L1 = LVLINE(AREA1)
The ADDSOSI and DELSOSI built-in functions
These built-in functions are used to add to or delete from a value-string the shift-out and shift-in
characters that mark the start and end of a DBCS field, without changing the value of the input string.
variable = ADDSOSI ( variable_name )
variable = DELSOSI ( variable_name
' DBCS_literal'
)
where:
variable_name
Name of the variable that the function will process.
Examples:
&VAR2 = ADDSOSI(&VAR1)
&VAR2; = DELSOSI(‘[DBDBDBDB]’)
The bracket characters [ and ] represent the shift-out and shift-in characters.
The target variable must not contain mixed (DBCS/EBCDIC) data. Only variables, not literals, can be
specified with the ADDSOSI function. Variables or literals can be specified with the DELSOSI function. An
odd input-value length is not permitted for either function. The input-value length does not include trailing
blanks or nulls. Nested built-in functions are not allowed on the DELSOSI function. The ADDSOSI function
allows nesting of the TWOBYTE built-in function (see “The ONEBYTE and TWOBYTE built-in functions” on
page 206).
assignment statement
Chapter 6. Panel definition statement reference  205

## Page 234

Example:
&VARB = ADDSOSI(TWOBYTE(&VARA))
Variable VARA is converted to a 2-byte character code and shift-out and shift-in characters are added to
the character string. Then, variable VARB is set to the resulting value.
The ONEBYTE and TWOBYTE built-in functions
The ONEBYTE function is used to convert a variable from a 1-byte character code to the corresponding
1-byte code without changing the value of the variable. The TWOBYTE function is used to convert a
variable from a 1-byte character code to the corresponding 2-byte code without changing the value of the
variable.
variable = ONEBYTE ( variable_name )
variable = TWOBYTE ( variable_name )
where:
variable name
Name of the variable the function will process.
assignment statement
206  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 235

Examples:
&VARA = ONEBYTE(&VARB)
&VARA = TWOBYTE(&VARB)
The variable being converted must not contain mixed (DBCS/EBCDIC) data. Only variables, not literals,
can be converted. An odd input value length is permitted for the TWOBYTE function, but is not permitted
for the ONEBYTE function. The input value length does not include trailing blanks or nulls. Literals
cannot be used as input parameters for either function. Nested built-in functions are not allowed on the
TWOBYTE function. The ONEBYTE function allows nesting of the DELSOSI built-in function.
Example:
&VARB = ONEBYTE(DELSOSI(&VARA))
The VSYM built-in function
The VSYM built-in function can appear on the right side of an assignment statement and returns the value
of a dialog variable found in the function pool with all the system symbols resolved.
variable = VSYM ( field-name )
where:
field-name
Specifies the dialog variable name.
Example:
&A = VSYM(ABC)
The ELSE statement
The ELSE statement specifies that alternate processing is to take place when the conditions of the
matching IF statement are not satisfied.
ELSE
The ELSE statement has no parameters. The ELSE statement must be column-aligned with the matching
IF statement. Only one ELSE statement is allowed on the same line, even though each can align with a
prior IF statement. You can nest IF statements within ELSE statements. The only limitation on the number
of nested IF statements is the maximum number of columns available for indented statements due to the
panel record length.
The ELSE statement is indentation sensitive. If the conditional expression is true, the ELSE statement
that is column-aligned with the IF plus all statements to the right of that column are skipped. Processing
continues with the next statement that begins in the same column as the ELSE or in a column to the left of
the ELSE.
An example of using the ELSE statement:
IF (&DOW = UP)
  &ACTION = SELL
ELSE
  IF (&DOW = DOWN)
    &ACTION = BUY
  ELSE
    &ACTION = HOLD
&DOW = &BEAR
In this example, if the value of &DOW is UP, variable &ACTION is set to SELL and processing continues
at the statement &DOW = &BEAR. The indented processing statements following the first ELSE statement
ELSE Statement
Chapter 6. Panel definition statement reference  207

## Page 236

execute if variable &DOW does not have a value of UP. The assignment statement, &ACTION = HOLD,
executes only if the value of &DOW is not UP or DOWN.
Figure 62 on page 208 shows a sample panel definition with an IF/ELSE statement pair. The current value
of variable PHA is tested for the local area code, 919. If the value of PHA is 919, variable RATE is set to
the value of variable &LOCAL. If the value of PHA is not 919, variable RATE is set to the value of variable
&LONGD.
 )BODY
 %----------------------------  EMPLOYEE RECORDS  ------------------------------
 %COMMAND===>_ZCMD                                                             %
 + 
 %EMPLOYEE SERIAL: &EMPSER
 + 
 +   TYPE OF CHANGE%===>_TYPECHG +  (NEW, UPDATE, OR DELETE)
 + 
 +   EMPLOYEE NAME:
 +     LAST   %===>_LNAME         + 
 +     FIRST  %===>_FNAME         + 
 +     INITIAL%===>_I+ 
 + 
 +   HOME ADDRESS:
 +     LINE 1 %===>_ADDR1                                   + 
 +     LINE 2 %===>_ADDR2                                   + 
 +     LINE 3 %===>_ADDR3                                   + 
 +     LINE 4 %===>_ADDR4                                   + 
 + 
 +   HOME PHONE:
 +     AREA CODE   %===>_PHA+ 
 +     LOCAL NUMBER%===>_PHNUM   + 
 + 
 )INIT
   &TYPECHG = TRANS (&TYPECHG  N,NEW  U,UPDATE  D,DELETE)
 )PROC
   &TYPECHG = TRUNC (&TYPECHG,1)
   IF (&PHA = ‘919’)
     &RATE = &LOCAL
   ELSE
     &RATE = &LONGD
 )END
Figure 62. Sample panel definition  with IF and ELSE statement
EXIT and GOTO statements
Nested IF/ELSE statements can easily become complex, especially since the IF statement is indentation
sensitive. The GOTO and EXIT statements allow you to avoid these complexities and achieve enhanced
performance during panel processing. You can transfer control back to the user as soon as processing
errors are detected.
The GOTO and the EXIT statements are both allowed in the )INIT, )REINIT, )PROC, )ABCINIT,
and )ABCPROC sections of the panel source definitions.
EXIT statement
EXIT
The EXIT statement has no parameters. When an EXIT statement is encountered during panel processing,
ISPF halts processing of the section in which the statement was found and bypasses all remaining
statements in that section. Further processing of the panel continues normally.
• Example 1: Simple GOTO/EXIT
EXIT and GOTO statements
208  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 237

)PROC
      IF (&CUSTNAME = ' ')
        GOTO NAMERR
        IF (&CUSTNUM = ' ')
          .msg=xxxxx     /* message indicating number is required   */
          EXIT           /* exit )PROC section                      */
      VER (&CUSTNAME,ALPHA,msg=xxxxx)   /* messages specific to     */
      VER (&CUSTNUM,NUM,msg=xxxxx)      /* data type - alpha or num */
      GOTO NXTSECT
      NAMERR:
        .msg=xxxxx       /* message indicating name must be entered */
        EXIT             /* exit )PROC section                      */
      NXTSECT:
        zero, one, or more statements
In this example, the VER statements are skipped if no values are entered for the CUSTNAME or
CUSTNUM variable fields. Processing for the )PROC is halted after the .msg variable is set.
• Example 2: Multiple GOTOs
    )INIT
      &var2 = ' '
      IF (&newcust = ' ')
        GOTO BYPASS
      IF (&newcust = 'renew')
        &var2 = 1
        GOTO NXTCHK1
      IF (&newcust = 'initial')
        &var2 = 2
        GOTO NXTCHK1
      ELSE
        GOTO BYPASS
      NXTCHK1:
      IF (&var2 = 1)
        &var3 = 1
        &var4 = 0
        GOTO NXTSECT
      ELSE
        &var4 = 1
        &var3 = 0
        GOTO NXTSECT
      BYPASS:
        &var3 = 0
        &var4 = 0
      NXTSECT:
        zero, one, or more statements
Assuming that the variable NEWCUST was entered and verified to contain one of the two values
on a previous panel display, this example illustrates that certain fields on the panel currently being
processed will or will not be set depending on the value of NEWCUST.
• Example 3: GOTO Label within IF/ELSE
    )INIT
      IF (&var1 = ' ')
        GOTO BYPASS
      IF (&var2 = 1)
        &var5 = 1
        &var6 = 0
        BYPASS:
        &var7 = 1
      ELSE
        zero, one, or more statements
If variable var1 is blank, control is transferred to the label BYPASS. Variables var5 and var6 are not set
and processing will continue as if the IF statement were TRUE. Variable var7 will be set to 1. The ELSE
branch is not executed.
GOTO statement
GOTO label
EXIT and GOTO statements
Chapter 6. Panel definition statement reference  209

## Page 238

where:
label
Literal value of the label to which you will branch. The label:
• Must be from 1 to 8 characters in length
• Must begin with an alphabetic character (A-Z, a-z)
• May contain any other alphameric character (A-Z, a-z, 0-9).
The literal value of the label used must be followed by a colon when it appears by itself as a label. For
example:
   label:
ISPF translates the value for the label to uppercase before it is processed.
There are no indentation restrictions on a GOTO and its corresponding label. They may be at different
indentation levels.
ISPF processes the GOTO statement as follows:
• ISPF assumes that transfer of control to the named label is downward.
• ISPF continues processing with the next sequential statement after the first occurrence of the named
label.
• ISPF ignores duplicate labels.
• ISPF may transfer control within the IF or ELSE branch of an IF/ELSE statement. If the label is within the
IF branch, processing continues with the next statement following the label as if the IF were true. If the
label is within the ELSE branch, processing continues with the next statement following the label as if
the IF were false.
ISPF issues a severe error message if it does not find a matching label below the GOTO statement and
within the same section in which the GOTO statement is coded. The label need not be on a line by itself.
The IF statement
The IF statement is a valuable tool used to verify a conditional expression. The conditional expression can
be as basic as testing the value of a variable or can be expanded to use VER statement constructs and
Boolean capabilities. This topic first defines the complete syntax of the IF statement. Other more detailed
topics describe:
• Basic IF value testing
• IF statement with VER constructs
• IF statement with Boolean operators
• IF statement with VSYM built-in function
IF statements are valid in the )INIT, )REINIT, )PROC, )ABCINIT, and )ABCPROC panel sections.
The syntax of the IF statement is shown here.
IF( conditional-expression
boolean-operator conditional-expression
)
conditional-expression
IF Statement
210  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 239

variable operator
,
value
VER ( variable
, NONBLANK
, keyword
,
value
)
IF statement…
⋮
(Optional ELSE statement…)
⋮
where:
Boolean-operator
The character symbol & or characters AND (AND Boolean operator) or the character symbol | or
characters OR (OR Boolean operator).
ELSE
The optional statement that specifies alternate processing if the IF condition is not satisfied.
Basic IF value testing
IF( variable operator
,
value )
IF statement…
⋮
(Optional ELSE statement…)
⋮
The parentheses in the syntax contain a conditional expression, in which the operator is expressed in
either uppercase character symbols, such as EQ, or in special symbols, such as =. These symbols can be
any of:
= or EQ
(equal to)
¬= or NE
(not equal)
> or GT
(greater than)
< or LT
(less than)
>= or GE
(greater than or equal)
<= or LE
(less than or equal)
¬> or NG
(not greater than)
¬< or NL
(not less than).
IF Statement
Chapter 6. Panel definition statement reference  211

## Page 240

You can specify comparison against up to 255 values for the EQ (=) and NE (¬=) operators. For the
remaining operators, you can specify comparison against only one value.
If you use a character symbol operator, it must be separated from the variable name and comparison
value by one or more blanks. For example:
  IF (&ABC EQ 365)
Separation of a special symbol operator from the variable name and comparison value is optional.
  IF (&ABC = 365)  is the same as  IF (&ABC=365)
A compound symbol operator, such as <= or NG, must not contain intervening blanks. For example:
  <=  cannot be  < =
In determining whether the criteria of a conditional expression are met, ISPF uses a numeric compare
if the value of the variable and the value being compared are whole numbers between -2147483648
and +2147483647. Thus, if &A is set to +1, the expression IF (&A=1) is evaluated as being true, using
the numeric compare. If the value of the variable and the value being compared are not whole numbers
between -2147483648 and +2147483647, ISPF uses a character compare, using the EBCDIC collating
sequence to evaluate the IF expression. For both numeric and character compares, trailing blanks are
ignored.
Examples of basic value testing:
• IF (&DSN = ‘’) — True if variable DSN is null or contains blanks.
• IF (&OPT EQ 1,2,5) — True if variable OPT contains any of the literal values 1, 2, or 5.
• IF (&A GE &B) — True if the value of variable A is greater than or equal to the value of variable B.
• IF (&A ¬= AAA,BBB) — True if variable A is not equal to AAA and not equal to BBB.
The IF statement is indentation sensitive. If the conditional expression is true, then processing continues
with the next statement. Otherwise, all following statements are skipped up to a column-aligned ELSE
statement, if one exists, or up to the next statement that begins in the same column as the IF or in a
column to the left of the IF. Example:
IF (&XYZ = ‘’)
  &A = &B
  &B = &PQR
  IF (&B = YES)
    &C = NO
&D = &ZZZ
In this example, processing skips to statement &D = &ZZZ from either IF statement if the stated condition
is false.
Note that the scope of the IF statement is not terminated by a blank line.
IF statement with VER constructs
The conditional expression on the IF statement now includes VER statement constructs with one
exception: the MSG= parameter is not allowed. The IF conditional-expression evaluates to TRUE (1) for
successful verifications and to FALSE (0) for failing verifications. See “The VER statement” on page 230
for complete explanation of the VER statement. An example of using VER statements with IF statements:
IF (VER (valid keyword parameters and values))
⋮
ELSE
  .MSG = nld122
  IF (VER (valid keyword parameters and values))
⋮
IF Statement
212  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 241

The VER statement can be split over more than one line, but the VER statement and the left parenthesis of
its keyword parameters must be on the same line. This example is invalid:
IF (VER
 (valid keyword parameters and values))
⋮
IF statement with VSYM built-in function
The syntax of the panel IF statement supports the VSYM built-in function within any of the conditional
expressions as either the variable on the left side of the operator or the value on the right side of the
operator. The VSYM built-in function can also be included in the variable on the VER statement specified
within an IF statement.
Examples of the VSYM built-in function in the IF statement
IF (VSYM(A) = &B)
IF (&A = VSYM(B))
IF (&A = VSYM(B), VSYM(C), &D)
IF (VSYM(A) = &B | VSYM(C) = &D)
IF (VER(VSYM(X),NAME)
IF statement and boolean operators
You can combine two or more conditional expressions on the IF statement. ISPF evaluates the conditional
expressions on the IF statement from left to right, starting with the first expression and proceeding to the
next and subsequent expressions on the IF statement until processing is complete.
The use of the AND Boolean operator takes precedence over the OR Boolean operator as shown in these
examples.
The number of conditional expressions you can specify on the IF statement is limited to 255.
The accepted symbols for the Boolean operators are:
• & or AND (AND Boolean operator)
AND processing returns a TRUE result for the IF statement only if all the conditional expressions
evaluate as TRUE.
• | or OR (OR Boolean operator)
OR processing returns a TRUE result for the IF statement if any of the conditional expressions evaluate
as TRUE. Also, for an IF statement to be evaluated as FALSE, all conditional expressions must be
evaluated as FALSE.
The Boolean operators must be separated by a preceding and following blank or blanks.
Examples of Boolean operators in the IF statement
• Example 1: Comparison of two expressions using different Boolean operators in two separate IF
statements.
IF (VER (&vara,NB,ALPHA) & VER (&varb,NB,ALPHA))
⋮
ELSE
  IF (&varc = 123 OR VER (&vard,NB,NUM))
⋮
The first IF statement will be successful only if both VER expressions are satisfied, while the IF
statement under the ELSE will be successful if either of the expressions on the IF statement are
satisfied.
• Example 2: Comparison of three expressions using the AND Boolean operator in the same IF statement,
with additional OR Boolean operators.
IF Statement
Chapter 6. Panel definition statement reference  213

## Page 242

IF (VER (&vara,NB,ALPHA) & VER (&varb,NB,ALPHA) &
    &varc = abc,xyz | &vard = 123 | &vard = 456)
⋮
ELSE
  .msg = nld123
The IF statement will be successful if the comparisons of the first three expressions evaluate to TRUE,
or if expressions four or five evaluate to TRUE.
• Example 3: Comparison of two pairs of expressions using the AND Boolean operator combined on the
same IF statement by the OR Boolean operator.
IF (VER (&vara,NB,ALPHA) AND &varb = abc OR
    VER (&vara,NB,ALPHA) AND &varb = xyz)
⋮
ELSE
  .msg = nld124
  .attr (vara) = 'color(yellow)'
  .attr (varb) = 'color(yellow)'
Either of the pairs of expressions must evaluate to TRUE to achieve a successful IF statement.
• Example 4: Comparison of three expressions showing that the AND operator has precedence.
IF (Expression-1 OR Expression-2 AND Expression-3)
⋮
ELSE
  .msg = nld125
Because the IF statement AND Boolean operator has precedence over the IF statement OR Boolean
operator, specifying an IF statement similar to the one shown might not give you the results you expected.
If you expected the previous statement to be evaluated like this:
  IF ( (expression1 OR expression2) AND expression3)
You would need to write either two separate IF statements:
IF (Expression-1 OR Expression-2)
  IF (Expression-3)
⋮
  Else
    .msg = nld126
Or two separate comparison pairs:
IF (Expression-1 AND Expression-3 OR
          Expression-2 AND Expression-3)
⋮
Else
  .msg = nld127
The PANEXIT statement
The ISPF panel user exit provides a way for you to extend the panel language processing of dialog
variables. This processing can include operations such as verification, transformation, translation, and
formatting of dialog variables passed to the panel user exit routine. Performing these operations in a
panel user exit routine reduces the logic required in the ISPF function programs.
Use the PANEXIT statement in a panel's )INIT, )REINIT, or )PROC section to invoke the panel user exit.
This statement causes ISPF to branch to the panel user exit routine. When the routine processing
completes, control returns to the next sequential panel language statement.
PANEXIT Statement
214  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 243

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
where:
value
Specifies the names of dialog variables being passed to the exit. The string of values, including the
parenthesis, cannot exceed 255 characters. The string of values can be represented by the name of a
dialog variable that contains a list of names of variables being passed to the exit routine.
PGM
Keyword that indicates that the exit routine being invoked was loaded when ISPF loaded the
application dialog or was loaded from the application. The application passes ISPF the address of
the exit routine in exit-add.
exit-add
This is the name of a 4-byte, FIXED format dialog variable that contains the address of the exit
routine, which can reside above or below the 16Mb line. The exit routine receives control in
AMODE=31 mode. This parameter is used in conjunction with the keyword PGM.
exit-data
This is the name of a 4-byte FIXED format dialog variable that contains a value, such as the address of
an information area, to be passed to the exit routine.
msgid
If no message identification is returned to ISPF from the exit routine, this parameter identifies the
message to be displayed if a variable fails the exit routine evaluation. If this parameter is not
specified, and no message identification is returned from the exit routine, ISPF issues a generic
message indicating that the exit routine evaluation failed.
LOAD
Keyword that indicates that the exit routine is to be loaded dynamically. The application passes ISPF
the module name of the exit routine that is to be dynamically loaded. The module name is passed in
the exit-mod parm.
exit-mod
This parameter identifies the name of the panel user exit routine module that is to be dynamically
loaded by ISPF. The panel user exit name can be passed as a literal or as a dialog variable that
contains the panel user exit name. This parameter is used in conjunction with the LOAD keyword.
REXX
Keyword that indicates the name of the Rexx panel exit that is to be loaded and run. The exit can
be an interpreted Rexx exec or an exec that was compiled into load module form. Standard search
sequences are used to load the Rexx program.
rexx-name
This parameter is the name of the Rexx program that is to be used as the panel exit. If the exit is
an interpreted Rexx exec and might conflict with an existing load module name, the name can be
preceded by a percent sign (%) to avoid using the load module. If the REXX program is in load module
format, ensure that it was linked with the MVS stub.
PANEXIT Statement
Chapter 6. Panel definition statement reference  215

## Page 244

TSOENV
Keyword that indicates that you want ISPF to use the current TSO environment. If your dialog does an
IRXINIT to create its own REXX environment, but this keyword is not specified, that environment is
not used to process the REXX code. The REXX code is instead invoked in ISPFs REXX environment.
On the PANEXIT statement you can specify that these are passed to the panel user exit routine:
• A list of dialog variables to be processed by the exit routine in one call. Variable values must be in
character format when passed and must remain in character format.
• A 4-byte area that you can use to pass the address of data to be used by the exit routine.
• The identification of a message to be issued if a variable fails the exit routine evaluation. ISPF uses this
value to set the .MSG control variable or, in the case of a panel user exit severe error (RC=20 or invalid
value), to set ZERRMSG.
Note:
1. A panel user exit routine cannot access any dialog variables except those passed on the call.
2. A panel user exit routine cannot issue requests for any ISPF services.
3. ISPF ignores any PANEXIT statement issued from dialog test option 7.2.
4. A PANEXIT statement cannot be issued from a selection panel that initiates a dialog before defining
the exit address.
5. Although panel exits can be written in Language Environment-conforming languages, the overhead of
initializing Language Environment each time the exit is called needs to be considered.
6. An LE-conforming PANEXIT must be written as a MAIN routine. Failure to do so may result in abends
or unpredictable results. As the data pointed to by register 1 has a particular format, the program must
be compiled and linked so that this data is not interpreted as runtime options. Consult the relevant
publications of the language being used for further information.
Following a successful validation exit, during which one or more dialog variable values are changed, ISPF
updates the values for all dialog variable names included on the PANEXIT statement. This allows the exit
routine to define dialog variables for cursor field or cursor position, and to return these values to ISPF
when an error has been detected.
How to LOAD the panel user exit routine
If the dialog function routine and the panel user exit routine are separate object modules, you can load
the panel user exit routine by either:
• Linking the exit routine object module to the dialog function object module containing the display
request for the panel from which the PANEXIT statement is issued. Thus, when ISPF loads the
application, it also loads the exit routine.
• Loading the exit routine from the application and passing to ISPF the address of the exit routine.
• Letting ISPF load the exit routine dynamically.
How to LOAD a REXX panel exit
REXX panel exits can interpreted Rexx programs or compiled Rexx programs (CREXX or load modules).
ISPF automatically loads the Rexx program by using standard system interfaces. For non-load module
programs, ISPF calls TSO to pre-process the program. The program remains loaded for as long as the
current screen is active. If you change your Rexx program and want to run the new copy, you must end any
split screens that used the previous copy.
REXX exits receive only one parameter — a hexadecimal representation of the address of the list of
addresses shown in Figure 63 on page 217. You can use the Rexx storage() function to view and modify
the parameters that are pointed to by that list, or you can use the ISPF function named ISPREXPX,
described in “Using ISPREXPX to read and modify parameters” on page 219.
Note that you can also code REXX statements directly within the source of a panel. See “The *REXX
statement” on page 221.
PANEXIT Statement
216  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 245

Invoking the panel user exit routine
A dialog invokes the panel user exit by issuing the PANEXIT statement from a panel's )PROC, )INIT,
or )REINIT section. If the LOAD keyword is specified, ISPF will issue an OS load to bring the load module
into virtual storage. ISPF then invokes the exit routine through a call (BALR 14,15). You must use standard
OS linkage conventions when invoking the panel user exit. The exit routine (called in AMODE 31) must
support 31-bit addressing.
Panel exits can be written in languages that use the Language Environment runtime environment.
However, a mixture of Language Environment-conforming main dialog code and service routine code
is not supported. Dialogs and service routines must either all be Language Environment-conforming or all
be Language Environment-nonconforming.
ISPF uses the standard parameter list format to pass parameters. Register one points to a list of
addresses; each address points to a different parameter as shown in Figure 63 on page 217. See
“Parameters passed from ISPF to the panel user exit routine” on page 217 for information on these
parameters.
          ┌─────────┐
reg 1 ───►│ addr 1  ├─► Exit Data
          ├─────────┤
          │ addr 2  ├─► Panel Name
          ├─────────┤
          │ addr 3  ├─► Panel Section
          ├─────────┤
          │ addr 4  ├─► Message ID
          ├─────────┤
          │ addr 5  ├─► Number of Variables
          ├─────────┤
          │ addr 6  ├─► Array of Variable Names
          ├─────────┤
          │ addr 7  ├─► Array of Variable Lengths
          ├─────────┤
          │ addr 8  ├─► String of Variable Values
          ⋘─────────┘
Figure 63. Standard parameter list format
The keyword, LOAD, on the PANEXIT panel statement, provides the option of dynamically loading a panel
user exit routine. PGM and LOAD are the only valid keywords. PGM indicates that a panel user exit using a
compiled source is being invoked. LOAD indicates that the panel user exit routine named by the exit-mod
parameter is to be dynamically loaded by ISPF.
ISPF checks the keyword to determine if the panel user exit routine is to be dynamically loaded. If it
is, ISPF issues an OS load to bring the load module into virtual storage. The search sequence for link
libraries is: job pack area, ISPLLIB, steplib, link pack area, linklib. See z/OS ISPF Services Guide for further
discussion of the search order using LIBDEF.
The panel user exit routine is loaded only once per SELECT level the first time the panel is displayed.
The loaded panel user exit routine is not deleted until the SELECT, which first displayed the panel, is
terminated.
Parameters passed from ISPF to the panel user exit routine
Parameters passed to the panel user exit routine are (in the order passed):
1. Exit Data
The value of the dialog variable identified on the PANEXIT statement to contain exit data. Its format is
a fullword fixed value. If no exit data area is provided, ISPF passes binary zeros.
2. Panel Name
The name of the panel from which the panel user exit is being invoked. Its format is CHAR(8),
left-justified in the field. ISPF ignores any changes made to this parameter by the exit routine.
3. Panel Section
A 1-character code that identifies the panel section from which the panel user exit is being invoked.
Its format is CHAR(1). Its value is:
PANEXIT Statement
Chapter 6. Panel definition statement reference  217

## Page 246

I
for the )INIT section
R
for the )REINIT section
P
for the )PROC section.
4. Message ID
The identification of the message used to set the .MSG value if the variable evaluation fails. In case
of a severe error in the exit routine processing, ISPF uses this value to set variable ZERRMSG. Its
format is CHAR(8). When the exit routine is invoked, it contains eight blanks (X'40'). On return to ISPF,
if the value in Message ID is not blank, ISPF assumes the value to be a message ID, which must be
left-justified in the field.
5. Number of Variables
The dimension of the array of variable names and the array of variable lengths passed to the
panel user exit routine. Its format is a fullword fixed value. ISPF ignores any changes made to this
parameter by the exit routine.
6. Array of Variable Names
An array of dialog variable names being passed to the panel user exit routine. Each array entry has a
format of CHAR(8), left-justified in the array. ISPF ignores any changes made to this parameter by the
exit routine.
7. Array of Variable Lengths
An array of dialog variable lengths being passed to the panel user exit routine. Each array entry format
is a fullword fixed value. If the exit routine is a REXX routine that uses the ISPREXPX to set and return
the variables, then the exit routine is permitted to increase or decrease the length of any variables
passed back from the exit, except ZRXRC and ZRXMSG. Otherwise, if the exit routine changes any of
the variable length values, a severe error results.
8. String of Variable Values
A character buffer of dialog variable values mapped by the array of variable lengths and the array of
variable names. The length of the buffer is the sum of the lengths in the array of variable lengths. The
exit routine returns updated dialog variable values to ISPF in this buffer.
Return codes and error processing
Return codes, set in the panel user exit routine, recognized by ISPF are:
0
Successful operation.
8
Exit-defined failure. ISPF sets the .MSG control variable and displays or redisplays the panel with the
message.
20 (or code unrecognized by ISPF)
Severe error in the exit routine.
For an exit routine return code of 8, ISPF sets the .MSG control variable by using this search order:
1. If the value in the Message ID parameter is not blank on return to ISPF, that value is used for setting
the .MSG control variable.
2. If the value in the Message ID parameter is blank on return, the value (if any) specified for the MSG=
keyword on the PANEXIT statement is used for setting the .MSG control variable.
3. If neither the Message ID parameter nor the MSG= keyword has been given a value, the default ISPF
exit error message is used for setting the .MSG control variable.
The panel section in which the .MSG control variable is set affects the message display as follows:
• )INIT or )REINIT section: the message is displayed on the panel.
• )PROC section: the panel, including the message to be displayed, is redisplayed.
PANEXIT Statement
218  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 247

If the return code from the exit routine is either 20 or not one of the recognized codes, the display service
terminates with a severe error condition. ISPF sets the ZERRMSG system variable by using this search
order:
1. If the value in the Message ID parameter is not blank on return to ISPF, it is used for setting the
ZERRMSG system variable. This allows the exit routine to define the message to be used in case of a
severe error.
2. If the value in the Message ID parameter is blank on return, the value (if any) specified for the MSG=
keyword on the PANEXIT statement is used for setting the ZERRMSG system variable.
3. If neither the Message ID parameter nor the MSG= keyword has been given a value, the default ISPF
exit error message is used for setting ZERRMSG.
If CONTROL ERRORS CANCEL is in effect, ISPF displays on the severe error panel the message indicated
by the value of ZERRMSG.
Using ISPREXPX to read and modify parameters
A Rexx panel exit receives only the storage address of the standard panel exit parameter list. Although
you can use the standard Rexx storage() function to read and modify the list, ISPF supplies a program
called ISPREXPX to set local Rexx variables that reflect the information passed to and from the panel exit.
ISPREXPX syntax
Call ISPREXPX('I')
to initialize Rexx variables
Call ISPREXPX('T')
to set ISPF variables from the Rexx variables of the same name
ISPREXPX establishes several variables within the Rexx program. The stem variable VARNAMES.n
contains the names of the variables passed to the program. ISPREXPX then creates variables of those
names, called "named variables".
The Rexx program must ensure that changes to the variables are done to the named variables and not to
the VARNAMES.n stem variable. For example, if the PANEXIT statement on the panel passes in a variable
named ZDATA, then ISPREXPX creates a named variable called ZDATA. The Rexx program must refer to
and update that variable. If you do not know the exact name that is specified on the PANEXIT statement
in the panel that calls the Rexx exit, you can get the name from the VARNAMES.n stem variable and use
the INTERPRET instruction to get and set the actual variable.
A REXX panel exit can only increase or decrease the length of any variables passed back from the exit to
the ISPF dialog by means of the command, ISPREXPX 'T'.
Table 20. Variables and their meanings
Variable Explanation
user variables The variables as named in the PANEXIT statement. For example, a PANEXIT
statement like PANEXIT((ZDATA,USER),REXX...) creates variables ZDATA and
USER. Changes to the variables are returned to ISPF. If the length changes, the
new value is truncated or padded with blanks as needed to keep the original
length.
VARNAMES.0
VARVALS.0
VARLENS.0
All of these variables contain the number of variable names passed to the panel
exit. Changes to these variables are ignored.
MSGID Message ID to set in case of error. It is blank on entry to the exit. Changes to
this variable are used.
PANELNAME The name of the panel being processed. Changes to this variable are ignored.
PANELSECTION Panel section 'I', 'R', or 'P'. Changes to this variable are ignored.
PANEXIT Statement
Chapter 6. Panel definition statement reference  219

## Page 248

Table 20. Variables and their meanings (continued)
Variable Explanation
EXDATA A hexadecimal representation of the address of the user data. Changes to this
variable are ignored, but the program might change the data to which this
address points.
Return codes
These return codes are possible:
0
Normal result. Variables were retrieved or set successfully.
16
Parameter error. Incorrect parameter passed to ISPREXPX.
20
Error. Another error occurred. Most likely there is a failing return code from a Rexx service called by
ISPREXPX.
Example
This sample exit changes the case of all data in the variable ZDATA. It also overlays the beginning of the
variable ZDATA with the string '**REXX**'. The name ZDATA is used on the PANEXIT statement in the
panel source and is assigned to the variable name VARNAMES.1.
/*  REXX panel exit:  panexit((zdata),REXX,sample)  */
call ISPREXPX 'i'
zdata=overlay('02'x'** REXX **''01'x,translate(zdata, ,
   'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', ,
   'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'))
call ISPREXPX 't'
Note: You can see how this panel works by saving this example in a REXX library using the name SAMPLE
and changing the Browse panel ISRBROBA to include this line in the )INIT and )REINIT sections:
panexit((zdata),REXX,sample)
The REFRESH statement
The REFRESH statement provides a means to force specified fields in the panel body to be retrieved
before a redisplay.
REFRESH (
,
value )
where:
value
Name of an input or output field in the panel body.
Typically, when a panel is redisplayed, the automatic fetching of variables that appear in the panel body
is bypassed. As a result, all variables are normally displayed as the user last saw them, even though the
variable contents can have been changed. REFRESH causes the contents of specified fields to be retrieved
and allows the user to see any changes that have occurred since the panel was last displayed.
The REFRESH statement can appear within the )PROC or )REINIT section of a panel definition. ISPF flags
it as an error if it appears in the )INIT section. When this statement is encountered, the specified input/
output fields within the panel body are retrieved from the corresponding dialog variables prior to redisplay
of the panel.
REFRESH statement
220  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 249

A value of * indicates that all input/output fields on the panel are retrieved. You can omit the parentheses
if only one field is refreshed.
• Example 1:
)PROC
⋮
 IF (.MSG ¬= ‘’)
   &STMT = ‘Correct invalid field and press Enter key’.
 IF (.MSG = ‘ ’)
   &STMT = ‘ ’
 REFRESH STMT
If the panel is displayed again and if the control variable .MSG is set to nonblank in the )PROC section,
the panel field STMT is reset to Correct the ... Enter key. Otherwise, the field is set to blank.
• Example 2:
)REINIT
  REFRESH(SEL, RENAME)
Both panel fields SEL and RENAME are reset with their current values before any redisplay.
• Example 3:
)REINIT
  REFRESH(*)
All of the panel fields are reset to their current values.
• Example 4:
)REINIT
  REFRESH(&RVARS)
The variable RVARS will contain a list of one or more panel fields to be refreshed.
A field that is refreshed on the screen remains unchanged for multiple redisplays unless it is again
refreshed.
The *REXX statement
The *REXX statement is used to invoke REXX code in a panel's )INIT, )REINIT, or )PROC section. The REXX
can be coded within the panel source immediately after the *REXX statement, or the name of a member
containing a REXX program can be supplied.
*REXX
(
*,
,
value
,(
membname
membname_var
) ,TSOENV
)
where:
*
Specifies that all the dialog variables defined in the panel )BODY section are to be passed to the REXX
code for processing.
value
Specifies the names of dialog variables passed to the REXX code for processing.
membname
The name of a member (as a literal) in the standard search sequences used to load REXX programs.
See Note 7.
*REXX statement
Chapter 6. Panel definition statement reference  221

## Page 250

membname_var
The name of a variable whose value is the name of a member in the standard search sequences used
to load REXX programs. See Note 7.
TSOENV
Keyword that indicates that you want ISPF to use the current TSO environment. If your dialog does an
IRXINIT to create its own REXX environment, but this keyword is not specified, that environment is
not used to process the REXX code. The REXX code is instead invoked in ISPFs REXX environment.
Note:
1. The string of values, including the parentheses, cannot exceed 255 characters. The string of values can
be represented by the name of a dialog variable containing a list of variables being passed to the REXX
code.
2. The REXX code within a panel procedure is stored in an internal table which contains the statements
for the )INIT, )REINIT, )AREA, and )PROC sections of the panel. The size of this table is limited to
64K, so a large number of REXX statements coded directly within a panel procedure could cause
this table to overflow, resulting in error message ISPP321. If this error occurs, consider using the
(member) option on the *REXX statement so the REXX is loaded from a member in the standard search
sequences used for REXX programs.
3. When the REXX program has been compiled into load module format, it needs to have been linked with
the MVS stub.
4. The REXX code cannot access any dialog variables except those specified on the *REXX statement.
5. The REXX code cannot issue requests for any ISPF or SCLM services.
6. REXX coded within the panel source must be terminated by a *ENDREXX statement.
7. The member can contain interpreted REXX or compiled REXX. Compiled REXX can be either the output
generated by the REXX compiler when using the CEXEC option or a load module generated when
link-editing the output generated by the REXX compiler when using the OBJECT option.
8. If your dialog does an IRXINIT to create its own REXX environment, that environment will not be used
to process this REXX code. The REXX code is invoked in ISPFs REXX environment.
Processing ISPF dialog variables with panel REXX
ISPF dialog variables can be processed by panel REXX code. Dialog variables are made available to the
REXX code via the parameters specified on the *REXX statement:
• Specifying * as the first parameter causes all the dialog variables associated with the input and output
fields on the panel to be passed to the panel REXX code.
• Specifying a dialog variable name causes that dialog variable to be passed to the REXX code.
• The dialog variable values must be in character format when passed to the REXX code and must remain
in character format.
ISPPRXVP: dialog variable processor for panel REXX
The ISPF module ISPPRXVP is used to make ISPF dialog variables available to panel REXX, and to update
the dialog variables after they have been processed by panel REXX.
When the panel REXX is interpreted REXX (that is, the REXX statements are coded directly in a panel
procedure or the member specified on *REXX statement contains interpreted REXX) ISPF creates calls to
ISPPRXVP to perform these tasks:
• Set up corresponding REXX variables for the ISPF dialog variables before the panel REXX is invoked
• Update the ISPF dialog variables with any changes made by the panel REXX after it has finished.
This is done by ISPF generating these REXX statements before and after the supplied panel REXX code:
*REXX statement
222  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 251

Call ISPPRXVP 'I'
 If rc!=0 then do
   say 'ISPPRXVP Init failed rc=' rc
   Return
 End
 Call p_01A2B3C0
 Call ISPPRXVP 'T'
 If rc!=0 then
   say 'ISPPRXVP Term failed rc=' rc
 Return
 P_01A2B3C0:
 ⋮
 panel REXX code
 ⋮
 Return
(Bold text indicates REXX generated by ISPF.)
Note: The 11 lines of REXX code generated by ISPF before the supplied panel REXX and the line of
REXX code generated by ISPF after the supplied panel REXX will affect the results obtained from the
SOURCELINE function. For example using SOURCELINE() in interpreted panel REXX returns a value that is
12 more than the number of source lines of panel REXX.
The EXIT statement and interpreted or compiled panel REXX
If the interpreted panel REXX code uses the EXIT statement to terminate REXX processing, the
termination call to ISPPRXVP generated by ISPF is not executed. Therefore, any changes made to
REXX variables are not applied to the corresponding ISPF dialog variables. If you need to use the EXIT
statement in your panel REXX and you want changes applied to the ISPF dialog variables, ensure a
termination call to ISPPRXVP (that is, Call ISPPRXVP 'T') is run before the EXIT statement.
When the panel REXX is compiled REXX, ISPF does not create these initialization and termination calls to
ISPPRXVP. Therefore, panel developers must include these calls in their panel REXX code.
Return codes and error processing
ISPF provides these system dialog variables for return code and error processing in panel REXX:
ZRXRC
Available for panel REXX to pass a return code back to ISPF. Length is 2 bytes. The corresponding
REXX variable is initialized with a value of 0.
ZRXMSG
Available for panel REXX to provide a message ID used to set the .MSG value. Length is 8 bytes. The
corresponding REXX variable is initialized with a value of 8 blanks.
ISPF recognizes these return codes passed back by panel REXX in the dialog variable ZRXRC:
0
Successful operation.
8
Panel REXX defined failure. ISPF sets the .MSG control variable and displays or redisplays the panel
with the message.
20
Severe error in the panel REXX.
Any other return code not recognized by ISPF is treated as a severe error in the panel REXX.
When control returns to ISPF after the panel REXX has executed, if ZRXRC contains a return code of 8,
ISPF sets the .MSG control variable using this search order:
1. If the value in ZRXMSG is not blank on return to ISPF, that value is used to set the .MSG control
variable.
2. If the value in ZRXMSG is blank on return, the default ISPF panel REXX error message ISPP335 is used
to set the .MSG control variable.
*REXX statement
Chapter 6. Panel definition statement reference  223

## Page 252

The panel section in which the .MSG control variable is set affects the message display as follows:
• )INIT or )REINIT section: The message is displayed on the panel.
• )PROC section: The panel, including the message to be displayed, is redisplayed.
If the return code in ZRXRC is either 20 or is not one of the recognized codes, the display service
terminates with a severe error condition. ISPF sets the ZERRMSG system variable using this search order:
1. If the value in ZRXMSG is not blank when control returns to ISPF, it is used to set the ZERRMSG system
variable. This allows the panel REXX to define the message to be used in case of a severe error.
2. If the value in ZRXMSG is blank when control returns to ISPF, ZERRMSG is set to ISPP336. This is the
default ISPF message for severe errors relating to panel REXX.
If CONTROL ERRORS CANCEL is in effect, ISPF displays on the severe error panel the message indicated
by the value of ZERRMSG.
An example of using panel REXX
The panel shown demonstrates the use of the *REXX statement to invoke REXX code from the )INIT
and )PROC sections. The application displays cost, tax, and sales commission values for an order quote.
)PANEL
)ATTR DEFAULT(%+_) FORMAT(MIX)
 ~ TYPE(PT)
 ˋ TYPE(PIN)
 ! TYPE(FP)
 @ TYPE(NT)
 % TYPE(NEF)
 # TYPE(NEF) JUST(RIGHT)
 * TYPE(VOI) JUST(RIGHT)
)BODY WINDOW(70,20) CMD(ZCMD)
@                        ~Widget Order Quotes@                     @
!Command ===>%Z                                          @
@
ˋEnter the number of widgets to be ordered and the quoted price.
@
!Number of Widgets. . .#Z      @
!Quoted Price . . . . .#Z      @
@
!Total Cost ex Tax. . .*Z        @
!Total Tax. . . . . . .*Z        @
!Total Cost . . . . . .*Z        @
@
!Sales Commission . . .*Z        @
@
)INIT
.ZVARS = '(ZCMD NWIDGETS QPRICE TCSTXTAX TOTTAX TOTCOST SCOMM)'
/* Call REXX routine VALUSER to validate the user is allowed to use  */
/* this application.                                                 */
*REXX(ZPANELID,ZUSER,(VALUSER))
/* If the user is not allowed, display a message and protect the     */
/* input fields.                                                     */
IF (.MSG ¬= &Z)
  .ATTRCHAR(#) = 'TYPE(LI)'
)PROC
/* Call REXX routine VALUSER to validate the user is allowed to use  */
/* this application.                                                 */
*REXX(ZPANELID,ZUSER,(VALUSER))
/* If the user is not allowed, display a message and protect the     */
/* input fields.                                                     */
IF (.MSG ¬= &Z)
  .ATTRCHAR(#) = 'TYPE(LI)'
  EXIT
/* Initialize the cursor position variable.                          */
&CPOS = '--------'
&HPRICE = '      '
&LPRICE = '      '
/* Invoke panel REXX to validate input and calculate quote values.   */
*REXX(*,CPOS,LPRICE,HPRICE)
Trace O
upper zcmd
cpos = "'ZCMD'"
/************************************************************/
/* If the CLR command is entered in the command field,      */
/* clear all input/output fields and return to redisplay    */
*REXX statement
224  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 253

/* the panel.                                               */
/************************************************************/
If zcmd = 'CLR' then do
  nwidgets = ''
  qprice   = ''
  call Clear_Output
  return
End
/************************************************************/
/* Ensure the output fields are cleared.                    */
/************************************************************/
Call Clear_Output
/************************************************************/
/* Verify the value entered for the number of widgets is    */
/* a positive whole number.                                 */
/************************************************************/
if datatype(nwidgets,'N') = 0 |,
   pos('.',nwidgets) ¬= 0     |,
   pos('-',nwidgets) ¬= 0 then do
  cpos   = 'NWIDGETS'
  zrxmsg = 'TPRX001'
  zrxrc  = 8
  return
end
/************************************************************/
/* Verify the quoted price is a monetary value.             */
/************************************************************/
qprice = strip(qprice)
if substr(qprice,1,1) = '$' then
  qprice = substr(qprice,2)
if datatype(qprice,'N') = 0 |,
   (pos('.',qprice) ¬= 0 & ((length(qprice) - pos('.',qprice)) > 2)) then do
  cpos   = 'QPRICE  '
  zrxmsg = 'TPRX002'
  zrxrc  = 8
  return
end
/************************************************************/
/* Verify the quoted price is above the lowest possible     */
/* value.                                                   */
/************************************************************/
lprice = 12.50
if qprice < lprice then do
  cpos   = 'QPRICE  '
  zrxmsg = 'TPRX003'
  lprice = '$'||lprice
  zrxrc  = 8
  return
end
/************************************************************/
/* Verify the quoted price is above the highest possible    */
/* value.                                                   */
/************************************************************/
hprice = 25.00
if qprice > hprice then do
  cpos   = 'QPRICE  '
  zrxmsg = 'TPRX004'
  hprice = '$'||hprice
  zrxrc  = 8
  return
end
/************************************************************/
/* Calculate the total pre-tax cost.                        */
/************************************************************/
tcstxtax = format(nwidgets*qprice,5,2)
/************************************************************/
/* Calculate the total sales tax at a rate of 6.25%.        */
/************************************************************/
tottax = format(tcstxtax*0.0625,5,2)
/************************************************************/
/* Calculate the total cost after tax.                      */
/************************************************************/
totcost = format(tcstxtax+tottax,5,2)
/************************************************************/
/* Calculate the sales commission at a rate of 12.5% of the */
/* profit.                                                  */
/************************************************************/
scomm = format((tcstxtax-(nwidgets*lprice))*0.125,5,2)
/************************************************************/
/* Format the output fields for display.                    */
/************************************************************/
qprice = '$'||strip(qprice)
*REXX statement
Chapter 6. Panel definition statement reference  225

## Page 254

tcstxtax = '$'||strip(tcstxtax)
totcost = '$'||strip(totcost)
tottax = '$'||strip(tottax)
scomm = '$'||strip(scomm)
return
/************************************************************/
/* This routine clears the output fields.                   */
/************************************************************/
clear_output:
 tcstxtax = ''
 tottax   = ''
 totcost  = ''
 zcmd     = ''
 scomm    = ''
return
*ENDREXX
IF (.MSG ¬= &Z)
  .CURSOR = &CPOS
  REFRESH(*)
ELSE
 .CURSOR = ZCMD
/************************************************************/ 
/* )REINIT and REFRESH(*) ensure the calculated values      */ 
/* and dialog variables are properly refreshed.             */ 
/* This avoids output fields remaining unchanged            */ 
/* after being updated by REXX.                             */ 
/************************************************************/ 
)REINIT 
**REFRESH(*)** 
)END
/* IF (.MSG ¬= &Z AND .MSG NE TPRX000 AND &ZVERB NE CANCEL) .RESP = ENTER */
)END
The user of this application enters the number of widgets to be ordered and the price quoted to the
customer. The panel REXX coded directly in the )PROC section receives all the panel input and output
fields for processing. It also receives the CPOS variable used to set the cursor position, and the LPRICE
and HPRICE variables used to check that the quoted price is in a valid range. This panel REXX performs
these functions:
• Validates the values entered by the user. If any values are invalid, variable ZRXRC is set to 8, the
appropriate error message ID is set in variable ZRXMSG, the appropriate field name is stored in the
variable CPOS, and control is returned to ISPF.
• Calculates and formats the values displayed for the cost (ex tax), tax, total cost, and sales commission.
• Checks if the user has entered 'CLR' in the command. If so, all the input/output fields on the panel are
set to blanks.
The panel REXX routine in member VALUSER is invoked in the )INIT and )PROC sections. This routine
receives the system variables ZPANELID and ZUSER and checks if the user is allowed to use the panel.
This is the REXX code for VALUSER:
*REXX statement
226  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 255

/********************************************************/
/* Call ISPPRXVP to get the ISPF dialog variables into  */
/* REXX.                                                */
/********************************************************/
Call ISPPRXVP 'I'
/********************************************************/
/* This common REXX routine checks whether the user is  */
/* allowed to use the panel being displayed.            */
/********************************************************/
say 'zpanelid = ' zpanelid
say 'zuser = ' zuser
found = 0
users = ''
/********************************************************/
/* Set up the user list based on the panel Id.          */
/********************************************************/
if zpanelid = 'SQUOTE' then
  users = 'ADAMS MITCHELL JACKSON JAMES JONES WEBSTER'
else
if zpanelid = 'PORDER' then
  users = 'BRADLEY CONNOR EVANS PRINCE WALLS'
else
if zpanelid = 'INVENTRY' then
  users = 'BAXTER HILL NELSON SWAN WILSON'
/********************************************************/
/* Check that the user Id is in the user list.          */
/********************************************************/
do i = 1 to words(users)
  if zuser = word(users,i) then do
    found = 1
    leave
  end
end
/********************************************************/
/* If not found, pass back error message TPRX009 in     */
/* dialog variable ZRXMSG and set a return code of 8    */
/* in dialog variable ZRXRC.                            */
/********************************************************/
if ¬found then do
  zrxmsg = 'TPRX009'
  zrxrc  = 8
end
/********************************************************/
/* Call ISPPRXVP to get update the ISPF dialog          */
/* variables with the changes made in this REXX.        */
/********************************************************/
Call ISPPRXVP 'T'
Return
Figure 64. Sample member VALUSER to invoke panel REXX
Member VALUSER contains compiled REXX, so processing commences with a call to ISPPRXVP to
initialize REXX variables for the ISPF dialog variables ZPANELID, ZUSER, ZRXRC and ZRXMSG. Before
returning to ISPF there is also a call to ISPPRXVP to update these dialog variables with the values in the
corresponding REXX variables.
These are the messages used by this application:
TPRX001  'Invalid number          ' .TYPE=N NOKANA
'The value entered is not a positive whole number.'
TPRX002  'Invalid price           ' .TYPE=N NOKANA
'The value entered is not in the form $xx.yy'
TPRX003  'Quoted price too low    ' .TYPE=N NOKANA
'The quoted price cannot be lower than &LPRICE'
TPRX004  'Quoted price too high   ' .TYPE=N NOKANA
'The quoted price cannot be greater than &HPRICE'
TPRX009  'Not available           ' .TYPE=A .W=NORESP NOKANA
'This application is not available to user &ZUSER'
Panel REXX example supplied with ISPF
The member ISRVCALP in the ISPF panel library contains a panel which makes use of panel REXX.
The )INIT procedure section of the panel contains a *REXX statement which invokes the REXX in member
ISRVCHIL in the ISPF REXX exec library. This panel REXX code is used to enable color highlighting of the
*REXX statement
Chapter 6. Panel definition statement reference  227

## Page 256

entries in the trace data set generated by the ISPVCALL utility. ISPVCALL is used by the ISPF product
support team to assist in debugging customer reported problems.
The TOG statement
Use the TOG statement to alternate the value of a variable between two values.
TOG( mode, fld , &variable
, value1, value2
)
where:
mode
Mode in which TOG is to function:
• S—single, used for pull-downs and single-choice selection fields.
• M—multiple, used for multiple choice selection fields.
fld
Panel field used to determine whether &variable alternates.
&variable
Variable whose value may alternate between value1 and value2.
value1
Value &variable receives if &variable is not equal to value1. The default is 0. Value1 can be a dialog
variable or literal.
value2
Value &variable receives if &variable is equal to value1. The default is 1. Value2 can be a dialog
variable or literal.
Examples:
Value1 = 0
Value2 = 1
IF &variable = Value2
   &variable = Value1
ELSE
   &variable = Value2
The statement accepts numeric or alphabetic values. A numeric compare is performed on numeric data.
When scan encounters a comma (even if it is followed immediately by an another comma or a right
parenthesis) it assumes a value is given. The TOG value will be assigned a blank in this case. For example:
    TOG(S,fld1,test,)   value1 = ' ' value2 = 1
    TOG(S,fld1,test,,)  value1 = ' ' value2 = ' '
    TOG(S,fld1,test)    value1 = 0   value2 = 1 (both will use defaults)
If the TOG is in single mode, a check is made to determine if the data has been modified. If it has been
modified, then the TOG is performed.
If the TOG is in multiple mode, and a check determines that the data has been modified, then:
• If the field contained a character at the last display and it has not been changed to a blank, the TOG is
not performed.
• If the field contained a blank and now contains a character, the TOG is performed.
This is to ensure the selection is not deselected by a different character. Only by blanking the field
should the variable be deselected.
The TOG statement example in Figure 65 on page 229 uses both single and multiple mode combinations.
The single mode TOG statements are prefaced with IF statements and are performed based on the IF
statement condition. The multiple mode TOG statements are not conditional. They are performed with
each pass through this processing section.
TOG statement
228  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 257

)PROC
IF ( &CLS = 1 )
   TOG (S,CLS,&CHSPORT,'0','1')
IF ( &CLS = 2 )
   TOG (S,CLS,&CHSEDAN,'0','1')
IF ( &CLS = 3 )
   TOG (S,CLS,&CHLUXRY,'0','1')
IF ( &PERFMOD ^= ' ' )
   &PERFMOD = '/'
   &PERFORM = 'MODERATE'
ELSE &PERFORM = '0'
TOG (M,PERFMOD,&CHPERFO,'0','1')
IF ( &PERFSUP ^= ' ' )
   &PERFSUP = '/'
   &PERFORM = 'SUPER'
ELSE &PERFORM = '0'
TOG (M,PERFSUP,&CHSUPER,'0','1')
IF ( &PERFULT ^= ' ' )
   &PERFULT = '/'
   &PERFORM = 'ULTRA'
ELSE &PERFORM = '0'
TOG (M,PERFULT,&CHULTRA,'0','1')
)END
Figure 65. TOG statement example
The VEDIT statement
The VEDIT statement identifies the variables on which ISPF must do mask validation. The VEDIT
statement should precede all other )PROC statements that involve variables, such as the VER statement
or the VPUT statement. It must precede any statements that refer to a VMASKed variable. A VEDIT
statement must be coded for all masked variables defined in the panel. An example is shown in Figure 66
on page 229.
VEDIT ( variable
,MSG= value
)
where:
variable
Specifies the name of a dialog variable, whose value is to be verified against the mask pattern
specified by the VMASK service.
MSG=value
Optional. Can be set to a message ID in the processing section to cause a message to be displayed.
)ATTR  DEFAULT(%+_)
   @ TYPE(INPUT)  INTENS(LOW)
)BODY
%-------------------------------TEST PANEL-----------------------------
%COMMAND ===>_ZCMD
%
+   PHONE  %===>@CVAR        + (999)999-999
+   TIME   %===>@FVAR +        HH:MM
+
+
+
+
+      Press%ENTER+to leave this panel
)INIT
)PROC
 VEDIT (CVAR)
 VEDIT (FVAR)
)END
Figure 66. VEDIT example
VEDIT statement
Chapter 6. Panel definition statement reference  229

## Page 258

The VER statement
Use the verify statement, VER, to check that the current value of a variable meets some criteria. Typically,
it is used in the processing section to verify the data stored in a dialog variable. Verification of an input
variable value is performed after the value has been stored in the variable pool. The current rules for
padding, justification, and VDEFINE apply to the value stored in the pool. The syntax shown here and the
associated text describe the types of verification provided by ISPF.
The syntax of the VER statement supports the VSYM built-in function in the variable parameter. In
addition, the verification processing for the types DSNAME, DSNAMEF, DSNAMEFM, DSNAMEPQ, and
DDSNAMEQ resolves system symbols within the variable name and updates the variable in the panel field.
Therefore, there is no need to include VSYM within the variable parameter on the VER statement when
you specify any one of these DSNAME types.
Example:
VER(VSYM(X),NAME,MSG=ABC123)
VER statement
230  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 259

VER ( variable
NONBLANK
ALPHA
ALPHAB
BIT
DBCS
DSNAME
DSNAMEF
DSNAMEFM
DSNAMEPQ
DSNAMEQ
EBCDIC
ENUM
FILEID
HEX
IDATE
INCLUDE
,IMBLK
, value1
, value2
IPADDR4
ITIME
JDATE
JSTD
LEN, relational-operator , expected-length
LIST, value
LISTV, varlist
LISTVX, varlist
LISTX,
,
value
MIX
NAME
NAMEF
NUM
PICT, string
PICTCN, mask-character, field-mask , string
RANGE, lower, upper
STDDATE
STDTIME
,MSG= value
)
where:
variable
Name of the variable to be checked.
VER statement
Chapter 6. Panel definition statement reference  231

## Page 260

NONBLANK
Optional keyword. Specifies that the variable must contain a value and not all blanks. NONBLANK,
or NB, can be specified with another type verification, such as ALPHA, NUM, or HEX. Do this by
specifying the NONBLANK keyword after the variable name but before the other keyword. Example:
VER (&A,NB,PICT,NNN-NNNN)
is equivalent to:
VER (&A,NONBLANK)
VER (&A,PICT,NNN-NNNN)
If the variable does not meet the verification criteria, ISPF displays a message. The message can be
specified in the MSG=value parameter, where value is a message ID. If no message is specified, an
ISPF-supplied message is displayed, based on the type of verification. Even if a VER fails, processing
of the panel's )PROC and )REINIT statements is performed.
keyword
Specifies the verification criteria. One of these keywords must be specified:
ALPHA
The variable must contain only lowercase or uppercase alphabetic characters (A-Z, a-z, #, $, or
@). Blanks are not allowed.
ALPHAB
The variable must contain only lowercase or uppercase alphabetic characters (A-Z or a-z). Blanks
are not allowed.
BIT
The variable must contain all zeros and ones.
DBCS
The variable must contain only valid DBCS characters.
DSNAME
The variable must contain a valid TSO data set name. A data set name qualifier must begin with
an alphabetic character (A-Z, $, @, or #). The remaining characters must be either uppercase
alphanumeric or a hyphen (-). A period is used to connect each qualifier in the data set name.
ISPF first determines if the TSO/E NOPREFIX PROFILE option is in use. If it is, ISPF does use a
prefix in the calculation of the data set length. A maximum of 44 characters can be entered for
a data set name, if that data set name is enclosed in quotes. If the TSO/E NOPREFIX PROFILE
option is in use, a maximum of 44 characters can be entered for a data set name when it is not
enclosed within quotes. If the TSO/E NOPREFIX PROFILE option is not in use, a maximum of 42
characters can be entered for a data set name, not enclosed in quotes. ISPF uses the minimum
data set prefix of two characters (one character and a period separator) during its calculation of
the data set name length.
Note: The verification processing for DSNAME resolves system symbols within the variable name
and updates the variable in the panel field. Therefore, when you specify the verification type
DSNAME, there is no need to include VSYM within the variable parameter on the VER statement.
DSNAMEF
This parameter provides the same function as DSNAME with the additional feature that asterisks
(*) and percent signs (%) can be used within the qualifiers. You can use DSNAMEF to filter a list of
data sets.
A single asterisk within a qualifier indicates that zero or more characters can occupy that position.
Consecutive asterisks are not valid within a qualifier.
A single percent sign indicates that any one alphanumeric or national character can occupy that
position. One to eight percent signs can be specified in each qualifier.
VER statement
232  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 261

Note: The verification processing for DSNAMEF resolves system symbols within the variable name
and updates the variable in the panel field. Therefore, when you specify the verification type
DSNAMEF, there is no need to include VSYM within the variable parameter on the VER statement.
DSNAMEFM
This parameter provides the same function as DSNAMEF, but asterisks (*) and percent signs (%)
can only be used within a member name, not within the qualifiers. You can use DSNAMEFM to
filter members in a data set.
A single asterisk within a member name indicates that zero or more characters can occupy that
position.
A single percent sign indicates that any one alphanumeric or national character can occupy that
position. One to eight percent signs can be specified in each member name.
Note: The verification processing for DSNAMEFM resolves system symbols within the variable
name and updates the variable in the panel field. Therefore, when you specify the verification
type DSNAMEFM, there is no need to include VSYM within the variable parameter on the VER
statement.
DSNAMEPQ
This parameter provides the same function as DSNAMEQ, except if the TSO data set name starts
with a parenthesis and no closing parenthesis is found, DSNAMEPQ adds the closing parenthesis
and the end quote.
Note: The verification processing for DSNAMEPQ resolves system symbols within the variable
name and updates the variable in the panel field. Therefore, when you specify the verification
type DSNAMEPQ, there is no need to include VSYM within the variable parameter on the VER
statement.
DSNAMEQ
This parameter provides the same function as DSNAME with the additional feature that if the TSO
data set name starts with a quotation mark and no ending quotation mark is found, DSNAMEQ
adds the ending quotation mark for you.
Note: The verification processing for DSNAMEQ resolves system symbols within the variable name
and updates the variable in the panel field. Therefore, when you specify the verification type
DSNAMEQ, there is no need to include VSYM within the variable parameter on the VER statement.
EBCDIC
The variable must contain only valid EBCDIC characters.
ENUM
The variable can contain, in addition to numeric characters:
Plus sign (+)
Negative number indicators
Delimiter symbols
Decimal symbol (.)
Certain national language decimal symbol (,).
ISPF ignores leading blanks. Blanks between characters (except the French language delimiter)
and trailing blanks are not allowed. This includes blanks between leading or trailing signs and
the adjacent character. Use of any characters other than those listed results in ISPF issuing an
appropriate error message.
The ENUM parameter allows verification of a numeric variable that has been expressed in a more
natural style. ISPF verifies variable values for correct decimal and comma notation plus correct
sign placement.
Negative number indicators include a leading or trailing minus sign and a number enclosed by
parentheses. The decimal and delimiter symbols can vary according to national language. The
negative number indicators are common to all national languages.
VER statement
Chapter 6. Panel definition statement reference  233

## Page 262

Use of delimiter symbols is optional. However, if they are used, ISPF validates the delimiter
symbols beginning at the left-most symbol that it finds in the variable being verified. In case of an
invalid placement or omission of a delimiter symbol, ISPF issues an appropriate error message.
Use of the decimal symbol is optional. A maximum of one decimal symbol is allowed. If used, the
decimal must be correctly placed in relation to any delimiter symbols used. Delimiter symbols are
not allowed to the right of a decimal symbol. In case of an invalid placement of a decimal symbol,
ISPF issues an appropriate error message. Table 21 on page 234 illustrates decimal and delimiter
symbol use for each of the national languages supported by ISPF. 
Table 21. Decimal and delimiter symbols
Language Whole Fractional
Danish 999,999.88 0.789
English 999,999.88 0.789
French 999.999,88 0,789
German 999.999,88 0,789
Italian 999.999,88 0,789
Japanese 999,999.88 0.789
Korean 999,999.88 0.789
Portuguese 999.999,88 0,789
Spanish 999.999,88 0,789
Traditional Chinese 999,999.88 0.789
Simplified Chinese 999,999.88 0.789
Swiss-German 999.999,88 0,789
The variable being verified can contain leading blanks. Any trailing blanks in the variable's value
in the variable pool cause a verify error condition. Trailing blanks result from defining the variable
by using the VDEFINE service with the NOBSCAN option specified. These trailing blanks are not
overlaid when the variable is updated by a panel operation if the corresponding panel field has a
justification attribute of LEFT or ASIS.
Note: ISPF treats fields containing the nonnumeric characters allowed when using VER ENUM as
character fields. To use these fields in numeric operations, an installation can need to provide a
routine to convert the fields from character to numeric data. The ISPF VDEFINE exit routine is one
option available for incorporating these conversion routines.
Table 22 on page 234 shows examples of results when verifying variable values (English) with the
ENUM keyword specified. 
Table 22. Verifying variable values with the ENUM keyword specified 
Value Results Reason
+2574 Valid Leading plus sign is allowed
-2574 Valid Leading minus sign allowed
25.74 Valid Decimal allowed
.2574 Valid Leading decimal allowed
2,574 Valid Delimiter character allowed (but not required)
(2,574) Valid Alternate method of showing a negative value allowed
VER statement
234  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 263

Table 22. Verifying variable values with the ENUM keyword specified  (continued)
Value Results Reason
2574- Valid Trailing minus sign allowed
2574+ Invalid Trailing plus sign not allowed
-2574- Invalid Double negative indication not allowed
( 2,574 ) Invalid Two errors; blanks not allowed between either sign indicator
and the adjacent character
35,543785 Invalid If used, the delimiter character must be inserted at every
appropriate point (35,543,785)
4,5932.673 Invalid Delimiter must be positioned in relation to decimal
(45,932.673)
33.452.78 Invalid Only one decimal allowed in numeric field
8.364,798 Invalid Delimiter not allowed to right of decimal
FILEID
The variable must contain a valid file ID in CMS syntax. The file name and file type, if given, must
be from 1-8 alphanumeric characters, including A-Z, 0-9, $, #, @, +, - (hyphen), : (colon), and _
(underscore). The filemode must be a single letter (A-Z), optionally followed by a single digit (0-9).
In addition, one or more fields of the fileid can be an asterisk (*) or a string of characters followed
by an asterisk. For example:
tr* status
All files having a file name beginning with the letters tr and having a file type of status.
* exec
All files having a file type of exec.
HEX
The variable must contain only hexadecimal characters (0-9, A-F, a-f).
IDATE
The international date (IDATE) format contains 8 characters, including the national language date
delimiter. The format represents a date expressed in a 2-digit year (YY), month (MM), and day
(DD). Valid values for YY are 00-99. Valid values for MM are 01-12. Valid values for DD are 01-31.
ISPF verifies for a valid date and national language date delimiter. For the United States, the
format is YY/MM/DD.
INCLUDE
Defines a list of value parameters, each specifying the character types a verify field is allowed to
contain.
IMBLK
Optional positional subparameter. Indicates that the variable is allowed to contain embedded
blanks. Any leading or trailing blank characters are ignored.
value1,value2
Specifies ALPHA, ALPHAB, or NUM; at least one value must be specified. The specification of
two different values are combined and indicate to ISPF that the field can contain data of either
type. ISPF issues an error message if more than two values are specified.
Example:
)PROC
   VER (&vara,NB,INCLUDE,IMBLK,ALPHAB,NUM,MSG=NSL001)
   VER (&varb,NB,INCLUDE,IMBLK,NUM,MSG=NSL002)
   VER (&varc,NB,INCLUDE,ALPHA,NUM,MSG=NSL003)
⋮
VER statement
Chapter 6. Panel definition statement reference  235

## Page 264

This example illustrates that the variable vara can contain any alphabetic (A-Z or a-z) or numeric
character as well as embedded blanks; varb can contain numeric characters only and embedded
blanks; and variable varc can only contain alphabetic characters (A-Z, a-z, #, $, or @) and numeric
characters (0-9), but no embedded blanks.
IPADDR4
The variable must contain a valid IP (Internet Protocol) address in dotted decimal notation (as the
decimal representation of four 8-bit values, concatenated with dots). For example, 128.2.7.9 is
a valid IP version 4 address. The first octet (8-bit value) can range from 0 to 223 in decimal
notation. The remaining three octets of the IP version 4 address can range from 0 to 255
in decimal notation. IPADDR4 verifies standard IP version 4 IP addresses. IPADDR4 does not
support Classless Inter-Domain Routing (CIDR) notation.
ITIME
The international date (ITIME) format contains 5 characters, including the national language time
delimiter. The format represents a date expressed in a 2-digit hour (HH), and a 2-digit minute
(MM). Valid values for HH are 00-23. Valid values for MM are 00-59. For the United States, the
format is HH:MM.
JDATE
The Julian date (JDATE) format contains 6 characters, including the period (.) delimiter. The
format represents a date expressed in a 2-digit year (YY), and a 3-digit day of the year (DDD). Valid
values for YY are 00-99. Valid values for DDD are 001-365 (or 001-366 for leap years). The format
is YY.DDD.
JSTD
The Julian standard date (JSTD) format contains 8 characters, including the period (.) delimiter.
The format represents a date expressed in a 4-digit year (YYYY), and a 3-digit day of the year
(DDD). Valid values for YYYY are 0000-9999. Valid values for DDD are 001-365 (or 001-366 for
leap years). The format is YYYY.DDD.
LEN,relational-operator,expected-length
The length of the variable (number of characters) must satisfy the condition expressed by the
relational operator and expected length.
You can use the LEN function in a panel's )INIT, )REINIT, or )PROC section to verify the number of
characters (bytes) in a variable that is currently residing in the variable pool.
For DBCS character strings the number of bytes in the string is twice the number of characters.
relational-operator
Valid relational operators are:
= or EQ
Equal to
< or LT
Less than
> or GT
Greater than
<= or LE
Less than or equal
>= or GE
Greater than or equal
¬= or NE
Not equal
¬> or NG
Not greater than
¬< or NL
Not less than.
VER statement
236  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 265

You can specify the relational operator either as a special symbol (=, <, and so forth) or as a
character symbol (EQ, LT, and so forth) expressed in uppercase. A relational operator can be
expressed either as a literal value (remember to enclose special symbol values in quotes) or as
a dialog variable containing the value.
expected-length
The expected-length operand is a positive number having a maximum of 5 characters, with
which ISPF compares the number of characters in the variable data. Like the relational
operator, the expected-length operand can be expressed as a literal value or as a dialog
variable containing the value.
Example:
VER (&NAME,LEN,‘<=’,8)
This statement verifies that the number of characters defining the value of variable &NAME is
less than or equal to 8.
Example:
VER (&NAME,LEN,NG,&SIZE)
This statement verifies that the number of characters defining the value of variable &NAME is
not greater than the value of dialog variable &SIZE
When input fields are stored in their corresponding dialog variables, any keyed leading or
trailing pad characters associated with right or left justification of the variable field are deleted
before being stored.
The length of a variable, used by ISPF for comparison, is the total number of characters in
the variable as it is currently stored in the variable pool. Thus, for a variable created using the
VDEFINE service with NOBSCAN specified, any trailing blanks are included in the length value
used for comparison.
If a variable has been defined using the VDEFINE service but currently has no value, ISPF uses
a length value of zero for comparison.
LIST,value1,value2, ...
The variable must contain one of the listed values. The maximum number of listed values allowed
is 100.
LISTV,varlist
Allows the use of a variable containing a list of values to be used for variable field verification.
varlist
When defined within the panel, this is the name of a variable, preceded by an &, that contains
a list of values that will be compared to the value contained in the verify variable. The varlist
variable can contain up to 100 values. Each value in the varlist variable must be delimited by
a comma or at least one blank. A value in the varlist variable containing any of these special
characters should be enclosed in single quotes (' '):
Blank < ( + | ) ; ¬ - , > : =
To specify the ampersand character in a value contained in the varlist variable, or a period in a
value contained in the varlist variable when it immediately follows a dialog variable name, you
must double these characters. To specify the single quote character in a value contained in the
varlist variable, use two single quote characters enclosed within single quotes ('').
If the varlist is set in the dialog, use the notation that is correct for the programming language
used to code the dialog.
Example:
)PROC
⋮
VER statement
Chapter 6. Panel definition statement reference  237

## Page 266

VER (&areacode,NONBLANK,LISTV,&varlist,MSG=NSL011)
⋮
The variable specified in the VER LISTV variable parameter must be set before being referenced in
the statement. (The variable used in the previous example could have been assigned these values
in the )INIT section of the panel definition.)
      &varlist ='919 914 212'
Note: To have quotes as part of an assignment, you must double the number of quotes used in
each previous layer. For example:
 &list1 = ‘one o‘‘ne‘ yields one o‘ne
 &list2 = ‘two t‘‘‘‘wo‘ yields two t‘‘wo
LISTVX,varlist
The LISTVX ("varlist exclude") keyword enables you to specify a variable containing a list of values
that the field variable must not contain. If LISTVX is used, the keyword NONBLANK is implied. The
varlist follows the same rules as the varlist for LISTV.
LISTX,value1,value2,...
The LISTX ("list exclude") keyword enables you to list values that the field variable must not
contain. If LISTX is used, the keyword NONBLANK is implied. The maximum number of listed
values allowed is 100.
MIX
The variable must contain all valid DBCS, EBCDIC, shift-in, and shift-out characters.
NAME
The variable must contain a valid name, following the rules of member names, up to eight
alphanumeric characters (A-Z, #, $, @, 0-9). It can also contain X'C0' (that is, a '{' for a 037
code page), but not as the first character. The first character must be alphabetic (A-Z, $, @, or #).
NAMEF
This parameter provides the same function as NAME with the additional feature that asterisks (*)
and percent signs (%) can be used within the qualifiers. You can use DSNAMEF to filter a list of
data sets.
A single asterisk within a qualifier indicates that zero or more characters can occupy that position.
Consecutive asterisks are not valid within a qualifier.
A single percent sign indicates that any one alphanumeric or national character can occupy that
position. One to eight percent signs can be specified in each qualifier.
NUM
The variable must contain all numeric characters (0-9). However, leading blanks are acceptable.
PICT,string
The variable must contain characters that match the corresponding type of character in the
picture string. The string parameter can be composed of these characters:
C
any character
A
any alphabetic character (A-Z, a-z, #, $, @)
N
any numeric character (0-9)
9
any numeric character (same as N)
X
any hexadecimal character (0-9, A-F, a-f)
VER statement
238  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 267

In addition, the string can contain any special characters that represent themselves. For example:
VER (xxx,PICT,‘A/NNN’)
In this example, the value must start with an alphabetic character, followed by a slash, followed
by 3 numeric characters. The length of the variable value and the picture string must be the same.
Trailing blanks are not included.
PICTCN,mask-character,field-mask,string
The VER statement keyword PICTCN, with its three parameters, enables you to check a variable
for specific constants within the variable.
VER (variable,PICTCN,mask-character,field-mask,string)
variable
Name of the variable to be checked.
mask-character
Any special character that represents itself. If you select one of these special characters as a
mask-character, the mask-character and the field-mask containing the mask-character must
be enclosed in quotes:
¬
'not' symbol
=
equal sign
.
period
>
greater than symbol
<
less than symbol
)
right parenthesis
(
left parenthesis
‘
single quote
Note: The mask-character cannot be one of the picture string characters (C, A, N, 9, X, c, a, n,
x).
field-mask
A combination of constants and the mask-character. The field-mask is used to audit the string.
For example, your mask-character is a slash mark (/) and the constants are V, R, and M in
the positions shown: 'V/ /R/ /M/ /'. A single quote can be used as a constant but avoid using a
mask-character that must be enclosed in single quotes when a single quote is a constant.
string
A combination of constants and picture string characters. The picture string characters can be:
C
any character
A
any alphabetic character (A-Z, a-z, #, $, @)
N
any numeric character (0-9)
VER statement
Chapter 6. Panel definition statement reference  239

## Page 268

9
any numeric character (same as N)
X
any hexadecimal character (0-9, A-F, a-f)
The picture string characters must be in the positions indicated by the mask-character in the
field-mask parameter. For example, 'VNNRNNMNN'.
The three parameters mask-character, field-mask, and string can be dialog variables.
Here are some examples:
In this VER PICTCN statement the mask-character is the not symbol (¬), the constants are V,R,
and M. The picture string characters are N (any numeric character 0-9). If fld1 = V10R20M00 it
passes the verification. If fld1 = V10R20M0Y it fails because Y is not a numeric character.
VER (&fld1,PICTCN,'¬','V¬¬R¬¬M¬¬',VNNRNNMNN)
In this VER PICTCN statement the mask-character is the asterisk (*), the constants are O and S.
The picture string characters are N (any numeric character 0-9) and A (any alphabetic character
A-Z, a-z,#,$,@). If fld1 = OS390R8 it passes verification. If fld1 = OS39018 it fails because 1 is
not an alphabetic character.
VER (&fld1,PICTCN,*,OS*****,OSNNNAN)
RANGE,lower,upper
The variable must contain all numeric characters (0-9). It can also contain a leading plus (+) or
minus ( -). Its value must fall within the specified lower and upper limits, which can be either
positive or negative. The length of the specified variable is limited to 16 digits, in addition to the
plus or minus sign. Further, the lower and upper parameters can consist of no more than 16 digits
each, in addition to the plus or minus sign, if used. Any characters in excess of the 16 allowed are
truncated.
STDDATE
The standard date (STDDATE) format contains 10 characters, including the national language date
delimiter. The format represents a date expressed in a 4-digit year (YYYY), 2-digit month (MM),
and a 2-digit day (DD). Valid values for YYYY are 0000-9999. Valid values for MM are 01-12. Valid
values for DD are 01-31. ISPF verifies for a valid date and national language date delimiter. For the
United States, the format is YYYY/MM/DD.
STDTIME
The standard time (STDTIME) format contains 8 characters, including the national language time
delimiter. The format represents a time expressed in a 2-digit hour (HH), 2-digit minute (MM), and
a 2-digit second (SS). Valid values for HH are 00-23. Valid values for MM are 00-59. Valid values
for SS are 00-59. For the United States, the format is HH:MM:SS.
MSG=value
value contains the message issued if the current value of the variable does not meet the criteria being
checked.
For all tests except NONBLANK, LISTX, and LISTVX, a blank value is acceptable. That is, if you enter a
value, or leave a nonblank initial value unchanged, it must conform to the specified condition. If a variable
value is stored as all blanks, the value passes any verification test except NONBLANK.
The cursor is automatically placed at the beginning of the field that was last referred to in any panel
definition statement when a message is displayed because of:
• A verification failure that sets .MSG
• A .MSG=value condition in a TRANS statement
• An explicit setting of .MSG
VER statement
240  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 269

If a verification failure occurs on a field not defined in the panel body then the cursor is placed based on
the above rules. Consider setting .MSG to a value with a message ID for a message with an appropriate
explanation.
Figure 67 on page 241 shows a sample panel with VER statements to verify that information entered
meets these criteria:
• The truncated value of TYPECHG is N, U, or D.
• The three name variables, LNAME, FNAME, and I, contain all alphabetic characters.
• The PHA (area code) field contains all numeric characters and a length of 3.
• The PHNUM (local number) field contains 3 numeric characters followed by a hyphen, followed by 4
numeric characters.
For the TYPECHG test, a message ID has been specified in the event that the test fails. In all the other
cases, an ISPF-provided message is displayed if the variable fails the verification test.
 )BODY
 %----------------------------  EMPLOYEE RECORDS  ------------------------
 %COMMAND===>_ZCMD                                                       %
 + 
 %EMPLOYEE SERIAL: &EMPSER
 + 
 +   TYPE OF CHANGE%===>_TYPECHG +  (NEW, UPDATE, OR DELETE)
 + 
 +   EMPLOYEE NAME:
 +     LAST   %===>_LNAME         + 
 +     FIRST  %===>_FNAME         + 
 +     INITIAL%===>_I+ 
 + 
 +   HOME ADDRESS:
 +     LINE 1 %===>_ADDR1                                   + 
 +     LINE 2 %===>_ADDR2                                   + 
 +     LINE 3 %===>_ADDR3                                   + 
 +     LINE 4 %===>_ADDR4                                   + 
 + 
 +   HOME PHONE:
 +     AREA CODE   %===>_PHA+ 
 +     LOCAL NUMBER%===>_PHNUM   + 
 + 
 )INIT
   IF (&PHA = ‘ ’)
     &PHA = 301
   &TYPECHG = TRANS (&TYPECHG N,NEW U,UPDATE D,DELETE)
  )PROC
   &TYPECHG = TRUNC (&TYPECHG,1)
   VER (&TYPECHG,LIST,N,U,D,MSG=EMPX210)
   VER (&LNAME,ALPHAB)
   VER (&FNAME,ALPHAB)
   VER (&I,ALPHAB)
   VER (&PHA,LEN,‘=’,3)
   VER (&PHA,NUM)
   VER (&PHNUM,PICT,‘NNN-NNNN’)
 )END
Figure 67. Sample panel definition  with v erific ation 
The VGET statement
The VGET statement copies variables from the shared or application profile variable pool or from system
symbols.
VGET name-list
ASIS
SHARED
PROFILE
SYMDEF
SYMNAMES(  symname-list )
VGET statement
Chapter 6. Panel definition statement reference  241

## Page 270

where:
name-list
Specifies one or more dialog variables, separated by commas or blanks, whose values are to be
copied from the shared or application profile pool or from system symbols. The names are passed in
standard name-list format. A name-list of more than one name must be enclosed in parentheses.
ASIS
Variable values are to be copied from the shared variable pool, if found there; otherwise, they are to
be copied from the application profile pool. ASIS is the default value.
SHARED
Variable values are to be copied from the shared variable pool.
PROFILE
Variable values are to be copied from the application profile variable pool. ISPF deletes any shared
pool variables having the same name, even if they do not exist in the application profile pool.
SYMDEF
The values for the variables defined by name-list are to be obtained from the system symbols.
SYMNAMES(symname-list)
symname-list lists the names of one or more system symbols that are to be obtained. It is specified
in the same format as the name-list parameter. Where symname-list is omitted, the system symbols
obtained are the same as those specified on the name-list parameter.
One reason why you might use the SYMNAMES parameter is that some system symbols may have
the same name as a reserved or read-only dialog variable. In this case you must specify a different
variable name in name-list and specify the actual symbol name in symname-list. For example, you
could specify this command to obtain the current value for the static symbol SYSCLONE and store it in
a variable named CLONE:
VGET (CLONE) SYMDEF SYMNAMES(SYSCLONE)
If there are fewer symbol names in symname-list than names in the name-list, then the symbol
names are used from the symname-list until there are no more corresponding symbol names, then
the remaining names in the name-list are used. In other words, if there are five names in name-list
and only three symbol names, the symbol names are used for the first three symbols and the last two
names in the name-list are used for the remaining symbols.
If the number of symbol names in symname-list exceeds the number of names in name-list, a severe
error occurs.
This is an optional parameter. It is only valid when the SYMDEF parameter is also specified.
Note:
1. The length of the constructed VGET statement can not exceed 255 characters.
2. Specifying a non-modifiable variable in a VGET statement in a selection panel results in a severe error.
DISPLAY service panel
When processing a DISPLAY or TBDISPL service request, ISPF normally searches for dialog variable
values in the order:
1. Function pool
2. Shared pool
3. Application profile pool
To give you control over the pool from which ISPF retrieves variable values, the VGET statement in a
panel's )INIT, )REINIT, or )PROC section allows you to specify that ISPF is to copy one or more variable
values from either the shared pool or application profile pool to the function pool. If one or more of these
variables already exist in the function pool, their values are updated with the values of the corresponding
VGET statement
242  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 271

variables accessed by the VGET statement. Any of these variables that do not exist in the function pool
are created and updated with the values of those accessed by the VGET statement.
Examples:
)PROC
   VGET (XYZ ABC) PROFILE
This VGET statement in a panel's )PROC section causes the current values for variables XYZ and ABC to be
copied from the profile pool and updated in the function pool and used as the variable values for display
of a panel field. If XYZ and ABC do not already exist in the function pool, they are created then updated.
)PROC
   VGET (LHHMMSS) SYMDEF
This VGET statement causes the current value for the dynamic system variable LHHMMSS to be obtained.
)PROC
   VGET (LTIME) SYMDEF SYMNAMES(LHHMMSS)
This VGET statement causes the current value for the dynamic system variable LHHMMSS to be placed in
the dialog variable LTIME.
SELECT service panel
At the time ISPF processes a SELECT service request, there is no function pool. Therefore, ISPF normally
searches for dialog variable values in the order:
1. Shared pool
2. Profile pool
When specified on a selection panel, the VGET statement functions as follows:
• If the variable value is taken from the profile pool, the shared pool value, if it exists, is deleted.
• Otherwise, the VGET statement has no effect.
Further processing of the variables on the selection panel, other than by the VGET statement, is described
in “SELECT service and variable access” on page 53.
Here is an example of a VGET statement on a selection panel, where the specified variable exists in both
the shared and profile pools:
VGET FNAME PROFILE
This statement causes ISPF to retrieve the current value of variable FNAME from the profile pool and
display it in the corresponding panel field. Any updates to the variable are made to the profile pool. ISPF
deletes the variable from the shared pool.
The VPUT statement
While variables entered from a panel are automatically stored in the function variable pool, variables
can also be stored in the shared and profile variable pools by VPUT statements used in the )INIT, )
REINIT, )ABCINIT, )ABCPROC, or )PROC sections of the panel definition.
VPUT name-list
ASIS
SHARED
PROFILE
where:
VPUT statement
Chapter 6. Panel definition statement reference  243

## Page 272

name-list
Specifies the names of one or more dialog variables whose values are to be copied from the function
pool to the shared or profile pool.
ASIS
Specifies that the variables are to be copied to the pool in which they already exist or that they are
to be copied to the shared pool, if they are new. If the variables exist in both the shared and profile
pools, they are copied only to the shared pool.
SHARED
Specifies that the variables are to be copied to the shared pool.
PROFILE
Specifies that the variables are to be copied to the application profile pool. Any shared pool variables
with the same names are deleted.
Note: The length of the constructed VPUT statement can not exceed 255 characters.
Example:
)PROC
  VPUT (XYZ ABC) PROFILE
This statement causes current values for variables XYZ and ABC to be stored in the profile pool by a VPUT
operation.
The syntax for the VPUT statement is the same as that for the VPUT service when it is invoked from a
command procedure except that the ISPEXEC command verb is omitted.
The VSYM statement
The VSYM statement updates the value of dialog variables found in the function pool by resolving the
values of any system symbols. This includes all system static symbols and dynamic symbols and any
user-defined static symbols. The z/OS MVS Initialization and Tuning Reference has details on system static
and dynamic symbols. Consult your system programmer for any locally defined user symbols as these are
system and installation dependent.
VSYM name-list
where:
name-list
Specifies the names of one or more dialog variables whose values in the function pool are to be
processed to resolve system symbols. The names are passed in the standard name-list format.
Note: The length of the constructed VSYM statement can not exceed 255 characters.
Example:
  VSYM (DSNL)
Using ISPF control variables
Control variables are used to control and test certain conditions pertaining to the display of a
panel or message. Only those that apply to displays are discussed here. They can be used only in
the )INIT, )REINIT, and )PROC sections of a panel definition.
These control variables are described:
• .ALARM: see “.ALARM” on page 246
• .ATTR: see “.ATTR and .ATTRCHAR” on page 247
VSYM statement
244  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 273

• .ATTRCHAR: see “.ATTR and .ATTRCHAR” on page 247
• .AUTOSEL: see “.AUTOSEL” on page 249
• .CSRPOS: see “.CSRPOS” on page 250
• .CSRROW: see “.CSRROW” on page 250
• .CURSOR: see “.CURSOR” on page 251
• .HELP: see “.HELP variable” on page 252
• .MSG: see “.MSG variable” on page 253
• .NRET: see “.NRET key” on page 253
• .PFKEY: see “.PFKEY” on page 254
• .RESP: see “.RESP variable” on page 254
• .TRAIL: see “.TRAIL” on page 255
• .ZVARS: see “.ZVARS” on page 255
Control variables are automatically reset to blank when the panel display service first receives control.
If .MSG, .CURSOR, and .CSRPOS are still blank after processing of the initialization section, they are set
to the values passed by the calling sequence, if any, for an initial message or cursor placement. Under
certain conditions, processing of the initialization section is bypassed.
Once .CURSOR, .CSRPOS, .MSG, and .RESP have been set to nonblank by panel processing, they retain
their initial values until the panel is displayed, or redisplayed, at which time they are reset.
The control variables
.ALARM
.AUTOSEL
.CURSOR
.HELP
.MSG
.PFKEY
.RESP
have a length of 8 bytes. When set in an assignment statement to a longer value, the value is truncated. If
these control variables are tested in a conditional expression, the compare value (literal or dialog variable)
must not be longer than 8 bytes.
Figure 68 on page 246 shows an example in which both .HELP and .CURSOR have been set in the )INIT
section of the panel definition.
VSYM statement
Chapter 6. Panel definition statement reference  245

## Page 274

)BODY
 %----------------------------  EMPLOYEE RECORDS  ------------------------------
 %COMMAND===>_ZCMD                                                             %
 +
 %EMPLOYEE SERIAL: &EMPSER
 +
 +   TYPE OF CHANGE%===>_TYPECHG +  (NEW, UPDATE, OR DELETE)
 +
 +   EMPLOYEE NAME:
 +     LAST   %===>_LNAME         +
 +     FIRST  %===>_FNAME         +
 +     INITIAL%===>_I+
 +
 +   HOME ADDRESS:
 +     LINE 1 %===>_ADDR1                                   +
 +     LINE 2 %===>_ADDR2                                   +
 +     LINE 3 %===>_ADDR3                                   +
 +     LINE 4 %===>_ADDR4                                   +
 +
 +   HOME PHONE:
 +     AREA CODE   %===>_PHA+
 +     LOCAL NUMBER%===>_PHNUM   +
 +
 )INIT
   .HELP = PERS032
   .CURSOR = TYPECHG
   IF (&PHA = ‘  ’)
     &PHA = 301
   &TYPECHG = TRANS (&TYPECHG N,NEW U,UPDATE D,DELETE)
 )PROC
   &TYPECHG = TRUNC (&TYPECHG,1)
   VER (&TYPECHG,LIST,N,U,D,MSG=EMPX210)
   VER (&LNAME,ALPHAB)
   VER (&FNAME,ALPHAB)
   VER (&I,ALPHAB)
   VER (&PHA,NUM)
   VER (&PHNUM,PICT,‘NNN-NNNN’)
 )END
Figure 68. Sample panel definition  with control variables
.ALARM
The .ALARM control variable can be set in an assignment statement within the )INIT, )REINIT, or )PROC
sections to control the terminal alarm.
.ALARM = value
where:
value
YES, NO, a blank, or null.
YES
Causes the terminal alarm to sound when the panel is displayed.
NO
Causes the terminal alarm to be silent when the panel is displayed.
blank
Causes the terminal alarm to be silent when the panel is displayed.
null
Causes the terminal alarm to be silent when the panel is displayed.
Note: value can also be a variable containing the value YES, NO, a blank or null.
Examples:
 .ALARM = YES
 .ALARM = &ALRM
.ALARM Control Variable
246  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 275

In the first example, the .ALARM setting is YES, which causes the terminal alarm to sound when the panel
is displayed. In the second example, the alarm setting can be turned on (YES) or off (NO) according to the
current value of the variable ALRM. If the panel is displayed with a message that has .ALARM = YES, the
alarm sounds regardless of the setting of .ALARM within the panel assignment statement.
Control variable .ALARM can also appear on the right side of an assignment statement. For example:
&ALRM = .ALARM
might be used to save the setting of .ALARM in variable ALRM.
.ATTR and .ATTRCHAR
See:
• “.ATTR variable” on page 247
• “.ATTRCHAR” on page 248
• “Using .ATTR and .ATTRCHAR with table display panels” on page 248
• “Things to remember when using attribute override control variables” on page 249
.ATTR variable
The .ATTR control variable can be set in the )INIT, )REINIT, or )PROC section to allow attributes to be
changed on a field basis.
.ATTR ( field ) = '
,
keyword ( value) '
where:
field
Can be:
• The name of any input or output field that occurs in the panel body or area section.
• The .CURSOR control variable, which indicates the field where the cursor is currently positioned.
• The name of a dialog variable, preceded by an ampersand. The variable must contain the name of an
input or output field that occurs in the panel body, .CURSOR, or a blank.
keyword (value)
A legitimate attribute keyword and value for that attribute.
Examples:
 .ATTR (.CURSOR) = ‘COLOR(YELLOW) HILITE(REVERSE)’
 .ATTR (&FLD)    = ‘HILITE(&HLTE)’
 .ATTR (&FLD)    = ‘PAS(ON)’
In the first example, the color and highlighting of the field containing the cursor is overridden. In the
second example, the name of the field whose highlighting attribute is to be overridden is found in dialog
variable FLD and the highlighting value is in variable HLTE.
Overriding the cursor field (.CURSOR) and the alternate long or short message field attributes can be
particularly useful if the panel is being redisplayed because of a translation or verification failure. When
such a failure occurs, the cursor is automatically placed on the field in error and the message ID to be
displayed is automatically placed in the message area.
For example, if SMFIELD is specified on the )BODY statement as the alternate short message field,
a )REINIT section could be specified as follows:
.ATTR and .ATTRCHAR Control Variables
Chapter 6. Panel definition statement reference  247

## Page 276

)REINIT
 .ATTR (.CURSOR) = ‘COLOR(RED) HILITE(USCORE)’
 .ATTR (SMFIELD) = ‘HILITE(BLINK)’
This will cause the field in error to be redisplayed in red and underscored, and the error message to blink.
Only the specified attributes are overridden. Any other attributes associated with the field remain in
effect.
When a field attribute is overridden in the )INIT section of a panel, the override remains in effect if
the panel is redisplayed (unless the attribute is again overridden by another statement in the )REINIT
section). However, an attribute override in the )PROC or )REINIT section of the panel remains in effect
only for a single redisplay of that panel, should a redisplay occur. This allows one field at a time to be
highlighted as errors are found. Once the user corrects the error, the field reverts to its normal attributes.
.ATTRCHAR
The .ATTRCHAR control variable can be set in the )INIT, )REINIT, or )PROC section to override attributes
for all fields related to an existing attribute character.
.ATTRCHAR(<  char)= '
,
 keyword ( value) '
where:
char
Can be:
• One of the special characters, one-digit character, or two-digit hexadecimal codes used to denote
attribute characters within the panel.
• The name of a dialog variable, the value of which must contain an attribute character, two-digit
hexadecimal code, or a blank.
char follows the rules for literals. That is, it must be enclosed in single quotes if it contains any of the
special characters listed in “Using variables and literal expressions in text fields and panel sections”
on page 96.
keyword (value)
A legitimate attribute keyword and value for that attribute.
When a field attribute is overridden in the )INIT section of a panel, the override remains in effect if the
panel is redisplayed unless the attribute is again overridden by another statement in the )REINIT section.
However, an attribute override in the )PROC or )REINIT section of the panel remains in effect only for a
single redisplay of that panel, should a redisplay occur.
See “Relationship to Control variables .ATTR and .ATTRCHAR” on page 168 for a description of
appropriate and inappropriate override conditions for CUA and basic panel-element attributes.
Using .ATTR and .ATTRCHAR with table display panels
The effect that an attribute override has on a table display panel depends on whether the override is
permanent (overridden in the )INIT section) or temporary (overridden in the )REINIT or )PROC section). If
the attribute override for a field or attribute character in the scrollable section of a panel is:
• Permanent, the override for the specified field or character is effective for every model set displayed
• Temporary, the override for the specified field or character is effective for only the last selected model
set processed
Any scrolling activity performed when temporary overrides are in effect causes the affected attributes to
be cleared, including any temporary overrides in the fixed portion of the panel, and the original attributes
to be put into effect. In addition, if a table is redisplayed after model sets have been selected and a scroll
has taken place, any .ATTR or .ATTRCHAR temporary overrides are not put into effect.
.ATTR and .ATTRCHAR Control Variables
248  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 277

Things to remember when using attribute override control variables
• The .ATTR or .ATTRCHAR control variable cannot appear on the right side of an assignment statement.
• Several characteristics (for example, INTENSITY, COLOR, and CAPS) can be changed with one attribute
override statement. However, only one field can be changed by a single .ATTR statement, and only one
attribute character or hexadecimal code can be changed by a single .ATTRCHAR statement.
• The TYPE keyword can be overridden by .ATTR or .ATTRCHAR. You can change the TYPE:
from INPUT/CUA input types    to OUTPUT/CUA output types
from OUTPUT/CUA output types  to INPUT/CUA input types
from TEXT/CUA text types      to TEXT/CUA text types
from DATAIN                   to DATAOUT
from DATAOUT                  to DATAIN
Exceptions:  CUA TEXT types AB, ABSL, PS, RP
However, if you attempt to change the TYPE of a field from TEXT to INPUT, a dialog error will result.
See “Relationship to Control variables .ATTR and .ATTRCHAR” on page 168 for a description of
appropriate and inappropriate override conditions for CUA and basic panel-element attributes.
• The command field or scroll amount field cannot be changed to TYPE(OUTPUT) by an attribute override
assignment.
• The first .ATTR assignment that is encountered within a panel section for a particular field is the one
that is honored. Subsequent .ATTR assignments for that field are ignored. In this example, FIELD1 will
be blue and FIELD2 will be yellow:
)INIT
   .ATTR(FIELD1) = COLOR(BLUE)
   .ATTR(FIELD2) = COLOR(YELLOW)
   .ATTR(FIELD1) = COLOR(RED)
• Similarly, the first .ATTRCHAR assignment that is encountered within a panel section for a particular
attribute character or hexadecimal code is the one that is honored.
• Be careful when overriding the pad character. Since the string of overridden attribute keywords is in
quotes, the new pad character must be specified either without quotes or in double quotes, as follows:
  .ATTR(FIELD1) = ‘PAD($)’
  .ATTR(FIELD2) = ‘PAD(‘’*‘’)’
• If both an .ATTRCHAR assignment and an .ATTR assignment apply to the same field, the .ATTR
assignment takes precedence.
Example:
)BODY
⋮
%===>_FIELD1             + 
)INIT
 .ATTRCHAR(_) = ‘COLOR(YELLOW)’
 .ATTR(FIELD1)  = ‘COLOR(WHITE)’
)REINIT
 IF (.MSG ¬= ‘ ’)
    .ATTR(FIELD1) = ‘COLOR(RED) HILITE(BLINK)’
    .ATTRCHAR(_) = ‘COLOR(BLUE)’
)PROC
 VER(&FIELD1,NB)
)END
When this panel is initially displayed, FIELD1 will be white and all other input fields will be yellow. If the
panel is redisplayed with a message, FIELD1 will be blinking red and all other input fields will be blue.
If the panel is redisplayed without a message, FIELD1 will revert to white, and all other input fields will
revert to yellow.
.AUTOSEL
The .AUTOSEL control variable is used in conjunction with table display panels to specify auto-selection.
.AUTOSEL Control Variable
Chapter 6. Panel definition statement reference  249

## Page 278

.AUTOSEL =
YES
NO
where:
YES
Indicates that if the CSRROW parameter or control variable is specified, the row is to be retrieved even
if the user did not explicitly select the row. This is called auto-selection.
NO
Indicates that if the CSRROW parameter or control variable is specified, the row is to be retrieved only
if the user explicitly selects the row by entering data in the corresponding model set on the screen.
If the CSRROW parameter or control variable is not specified, .AUTOSEL is ignored. .AUTOSEL can be set
in the )INIT or )REINIT section. Any assignment made to .AUTOSEL in the )PROC section is ignored.
.CSRPOS
The .CSRPOS control variable can be set in the )INIT or )REINIT section and controls where in a field the
cursor is to be set.
.CSRPOS = integer
variable = .CSRPOS
where:
integer
Specifies the position in the field to which the cursor is set. This position applies regardless
of whether the cursor placement was specified using the CURSOR calling sequence parameter,
the .CURSOR control variable in the )INIT or )REINIT section, or the default cursor placement. If
cursor-position is not specified or is not within the field, the default is one, the first position of the
field.
The .CSRPOS control variable can appear on the right side of an assignment statement, making it act like a
function. Thus, the cursor field name and its position within a field can be stored in variables.
Example:
&CPOS  = .CSRPOS
In the preceding statement, the position (an integer value) of the cursor within the input or output field or
area is returned in variable CPOS.
.CSRROW
The .CSRROW control variable is used in conjunction with table display panels.
.CSRROW = CRP-number
variable = .CSRROW
where:
CRP-number
Table current-row-pointer number corresponding to the model set on the display where the cursor
is to be placed. If the specified row does not have a corresponding model set displayed on the
screen, the cursor is placed at the command field. The row will be auto-selected under either of these
conditions:
• If the CSRROW parameter is specified on the TBDISPL service either without AUTOSEL(NO) being
specified on TBDISPL or .AUTOSEL(NO) specified as a panel definition statement.
.CSRPOS Control Variable
250  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 279

• If the .CSRROW control variable is specified as a panel definition statement either without
AUTOSEL(NO) being specified on TBDISPL or .AUTOSEL(NO) specified as a panel definition
statement.
The .CSRROW control variable can appear on the right side of an assignment statement, making it act like
a function. Thus, the table row number corresponding to the model set on the display where the cursor is
to be placed can be stored in a variable.
Example:
&CROW = .CSRROW
.CURSOR
The .CURSOR control variable can be set in the )INIT or )REINIT section to control the placement of the
cursor.
.CURSOR = string
variable = .CURSOR
where:
string
A character string that matches a field name or a DYNAMIC or GRAPHIC area name in the panel body.
Its value cannot be a character string that matches a scrollable area name, but it can be a character
string that matches a field name within the scrollable area.
Example:
 .CURSOR = DSN
This example causes the cursor to be placed at field DSN. The .CURSOR control variable overrides any
cursor position specified on the DISPLAY or TBDISPL service request.
Notes:
• When the .MSG control variable is set (explicitly or indirectly by TRANS or VER statements),
the .CURSOR control variable is set to the field that was last referenced.
• If the .CURSOR control variable is set (explicitly or as the result of the .MSG variable being set) multiple
times in a )INIT or )REINIT section, the first setting takes precedence and subsequent settings are
ignored.
The .CURSOR control variable can appear on the right side of an assignment statement, making it look like
a function.
Example:
&CNAME = .CURSOR
If the control variable .CURSOR is not explicitly initialized, or if it is set to blank, the initial field where the
cursor is positioned (default placement) is determined as follows:
1. The panel body is scanned from top to bottom, and the cursor is placed at the beginning of the first
input field that meets these conditions:
• It must be the first or only input field on a line.
• It must not have an initial value; that is, the corresponding dialog variable must be null or blank.
• It must not have a field name of ZCMD.
2. If the stated criteria are not met in the panel body, the scrollable areas are searched using the same
criteria.
.CURSOR Control Variable
Chapter 6. Panel definition statement reference  251

## Page 280

3. If the criteria are still not met, the cursor is placed on the first input field in the panel body or scrollable
area, usually the command field.
4. If the panel has no input fields, the cursor is placed at the upper-left corner of the screen.
The cursor is automatically placed at the beginning of the field that was last referred to in any panel
definition statement when a message is displayed because of:
• A verification failure that sets .MSG
• A .MSG=value condition in a TRANS
• An explicit setting of .MSG
Examples:
&XYZ = TRANS (&A ... MSG=xxxxx)
&A = TRANS (&XYZ ... MSG=xxxxx)
VER (&XYZ,NONBLANK)  VER (&B,ALPHA)
Assume that field XYZ exists in the panel body, but there are no fields corresponding to variables A or B.
In all the preceding examples, the cursor would be placed on field XYZ if a message is displayed.
.HELP variable
The .HELP control variable can be set in the initialization section to establish a tutorial (extended help)
panel to be displayed if the user enters the HELP command.
.HELP = panelname
variable = .HELP
where:
panelname
Name of the tutorial panel to be displayed.
Example:
 .HELP = ISPTE
This example causes tutorial panel ISPTE to be displayed when the user enters the HELP command.
The .HELP control variable can appear on the right side of an assignment statement, making it act like a
function.
.HHELP variable
The .HHELP control variable can be set in the initialization section to establish a tutorial (extended help)
panel to be displayed if the user enters the HELP command from within HELP.
.HHELP = panelname
where:
panelname
Name of the tutorial panel for help to be displayed.
Example:
 .HHELP = ISP00006
This example causes tutorial panel ISP00006 to be displayed when the user enters the HELP command
from HELP. This also happens to be the default setting. The Dialog Tag Language generates the
setting .HHELP = ISP00006 for any help panels it builds.
.HELP Control Variable
252  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 281

.MSG variable
The .MSG control variable can be set to a message ID, typically in the processing section, to cause a
message to be displayed.
.MSG = msgid
variable = .MSG
where:
msgid
The message ID of the message to be displayed.
Example:
 .MSG = ISPE016
This variable is automatically set by use of the MSG=value keyword on a TRANS statement if there is no
match with the listed values, or on a VER statement if the verification fails.
Notes:
• When the .MSG control variable is set (explicitly or indirectly by TRANS or VER statements),
the .CURSOR control variable is set to the field that was last referenced.
• If the .MSG control variable is set (explicitly or indirectly by TRANS or VER statements) multiple times in
a )INIT or )REINIT section, the first setting takes precedence and subsequent settings are ignored.
.NRET key
On enabled panels, the .NRET key retrieves the library names from the current library referral list or data
set name from the current data set referral list. Unlike some other dot variables, .NRET can be assigned
multiple times in panel logic.
.NRET = ON
OFF
DSN
LIB
where:
ON
Sets the NRETRIEV command table entry active.
OFF
Sets the NRETRIEV command table entry inactive.
DSN
Tells ISPF that the NRETRIEV command retrieved a name from the current data set referral list.
LIB
Tells ISPF that the NRETRIEV command retrieved a name from the current library referral list.
Other values are reserved by ISPF. No messages are given in case of an assignment that is not valid.
When .NRET is used as the source for an assignment statement it always returns a null.
The user is responsible for assigning NRETRIEV to a PF key. NRETRIEV is normally inactive but can be
made active by using the .NRET=ON assignment in the )INIT and )REINIT section of a panel. If it is turned
on, .NRET=OFF must be executed in the )PROC section of the panel. Failure to turn off .NRET in the )PROC
section of the panel can lead to errors when the NRETRIEV key is pressed on subsequent panels.
NRETRIEV sets these variables in the FUNCTION pool:
.MSG Control Variable
Chapter 6. Panel definition statement reference  253

## Page 282

Variable
Function
ZNRPROJ
Project name
ZNRGRP1
First group name
ZNRGRP2
Second group name
ZNRGRP3
Third group name
ZNRGRP4
Fourth group name
ZNRTYPE
Type name
ZNRMEM
Member name
ZNRODSN
Other data set name
ZNRVOL
Volume associated with the other data set name
ZNRLIB
Successful library retrieve (YES or NO)
ZNRDS
Successful data set retrieve (YES or NO)
(H = Host, W = Workstation)
.PFKEY
The .PFKEY control variable is set to a value that reflects the function key pressed by a user while the
panel is being displayed.
.PFKEY = value
variable = .PFKEY
where:
value
The function key (F01-F24) pressed by a user.
The value of .PFKEY can be examined in the )PROC section of the panel and copied into dialog variables
through use of assignment statements. If no function key is pressed by the user, .PFKEY contains
blanks. .PFKEY is blank during processing of the )INIT and )REINIT sections.
The .PFKEY control variable can appear on the right side of an assignment statement, making it act like a
function.
.RESP variable
The .RESP control variable indicates normal or exception response on the part of the user.
.RESP = ENTER
END
variable = .RESP
.PFKEY Control Variable
254  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 283

where:
ENTER
Normal response. ISPF always sets .RESP to ENTER unless the user enters an END or RETURN
command.
END
Exception response. ISPF sets .RESP to END if the user enters an END or RETURN command.
The value in .RESP can be tested in the processing section to determine the user's response.
Example:
IF (.RESP = END)
Setting .RESP in the )INIT or )REINIT section of the panel definition has no effect if a message is being
displayed.
The )INIT or )REINIT section can be coded with these statements to ensure that the panel is not
displayed, regardless of whether a message was specified on the DISPLAY service request.
Example:
)INIT   or   )REINIT
  IF (.MSG ¬= &Z)
    .MSG = &Z
  .RESP = END
This variable can be set in a panel processing section to force an END or ENTER response. This can be
useful if a verification has failed (or .MSG was set) and you want that panel to be redisplayed with the
message even if the user entered END or RETURN.
The .RESP control variable can appear on the right side of an assignment statement, making it act like a
function.
.TRAIL
The .TRAIL control variable contains the remainder following a truncate (TRUNC) operation.
variable = .TRAIL
where:
variable
Assigned the value in .TRAIL.
If the contents of a variable are truncated to a specified length, all remaining characters are stored
in .TRAIL. If the contents of a variable are truncated at the first occurrence of a special character, the
remaining characters following the special character are stored in .TRAIL.
.ZVARS
The .ZVARS control variable can be set in the initialization section to a list of variable names that
correspond to Z place-holders in the body and/or model lines.
.ZVARS = var
'( varlist )'
variable = .ZVARS
where:
var
Name that corresponds to a Z place-holder.
.TRAIL Control Variable
Chapter 6. Panel definition statement reference  255

## Page 284

varlist
One or more variable names that correspond to Z place-holders.
The .ZVARS control variable can appear on the right side of an assignment statement, making it act like a
function.
Using Z variables as field name place-holders
In the body and area sections of a panel definition and in the model lines for a table display panel,
the name of an input or output field can be represented by the single character Z. This serves as a
place-holder; the actual name of the field is defined in the initialization section of the panel definition.
Use of place-holders allows the definition of short fields for which the lengths of the variable names
exceed the lengths of the fields.
The actual names of these fields are assigned in the initialization section of the panel definition. The
names are in a name list, enclosed in parentheses if more than one name is specified, assigned to the
control variable .ZVARS. The first name in the list corresponds to the first Z place-holder that appears in
the body or model lines. The second name in the list corresponds to the second Z, and so forth.
In the example shown in Figure 69 on page 256, the input field labeled TYPE is 1 character long and the
next two input fields are each 2 characters long. The names of these three fields are TYPFLD, LNGFLD,
and OFFSET, respectively.
 )BODY
 ----------------------------  TITLE LINE  ------------------------------------
 %COMMAND===>_ZCMD                                                            %
 %  .
    .
    .
    .
 +   TYPE  %===>_Z+    (A for alpha, N for numeric)
 +   LENGTH%===>_Z +   (0 to 99)
 +   OFFSET%===>_Z +   (0 to 99)
    .
    .
    .
 )INIT
   .ZVARS = '(TYPFLD LNGFLD OFFSET)'
Figure 69. Example of Z variable place-holders
The name list assigned to .ZVARS must be enclosed in single quotes because the list contains special
characters (parentheses) and blanks. As with other name lists, either commas or blanks can be used to
separate the names in the list. .ZVARS can also be set to a dialog variable that has a valid name list as its
value. For example:
 .ZVARS = &NLIST
where the value of &NLIST is (TYPFLD LNGFLD OFFSET). See “Defining the area section” on page 138 for
the description of how to use Z place-holders in scrollable panel areas.
.ZVARS Control Variable
256  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
