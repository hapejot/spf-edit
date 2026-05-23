# Chapter 3. Getting started: designing application panels

Source file: f54dt00_v3r1.md
Start page: 59
Page span: 59-84

## Page 59

Chapter 3. Getting started: designing application
panels
Each application panel you create serves a specific purpose, with unique fields, messages, and help
information defined for each one. This chapter explains how to define elements that are common among
application panels. This includes defining the application panels, and the interactive elements of panels,
including action bars, instruction text, and command areas. We also tell you how to arrange the contents
of your application panels using panel regions and dividers.
The PANDEF tag allows you to define in one place common attributes and values for the panels in your
application—see “Defining panel defaults” on page 50.
Defining application panels: the PANEL tag
You use the panel tag, its associated attributes, and the required panel end tag to define an application
panel and the specific characteristics of the panel.
The PANEL start and end tags define the beginning and ending of an application panel. The PANEL start
tag defines:
• Panel name
• Name of the help panel for the application panel
• Name of the panel default
• Dimensions of the panel
• Associated key mapping list
• KEYLTYPE value
• APPLID value
• Cursor placement
• CCSID number
• MENU keyword
• PRIME keyword
• TUTOR keyword
• WINDOW value
• WINTITLE value
• APPTITLE value
• PAD value
• PADC value
• OUTLINE value
• EXPAND value.
• MSGLINE value
• TITLINE value
• CMDLINE value
• ATTRUSE value
• ENDATTR value
• TYPE value
• SMSG value
Defining application panels: the PANEL tag
© Copyright IBM Corp. 1989, 2024 27

## Page 60

• LMSG value
• ASIS keyword
• ACTBAR keyword
• MERGESAREA value
• PANELSTMT value
• ENTKEYTEXT value
• IMAPNAME value
• IMAPROW value
• IMAPCOL value
• TMARGIN value
• BMARGIN value
• ERRORCHECK value
• ZUP value
• ZCONT value
• AUTONRET value
• AUTOTCMD value
• Panel title text
With the exception of the required NAME attribute used to identify the name of the application panel, all
of the attributes for the PANEL tag are optional. Many attributes have default values that the conversion
utility assumes if you do not specify the attribute. This topic describes these attributes, and how to use
them.
The PANEL start and end tags look like this, respectively:
<panel name=mainpan>
⋮
</panel>
In the preceding example, we included the required NAME attribute and its value mainpan on the PANEL
start tag. ISPF requires that each panel definition contain this attribute and an associated value to identify
the panel. The panel name is also used as the panel ID when the panel ID is displayed. The “NAME=*”
notation sets the panel name to be the same as the member name of the input DTL source file. If multiple
panel definitions have been combined within a single source file, then this notation should be used for
only one panel definition within the file.
The panel name must follow the naming convention described in “Rules for variable names” on page 179.
Note: During conversion when the PREP option is active, the conversion utility uses a temporary PDS to
store ISPF source format panels.
The file name for interactive use is: tsoprefix.TEMPDTLW.DTLPANnn, or, if the TSO NOPREFIX profile
option is in effect, the file name is: tsouserid.TEMPDTLW.DTLPANnn. nn is the screen number.
For batch, the file name is: tsoprefix.TEMPDTLW.DTLBATCH.Ttttttt.Rnnnnn, or,
if the TSO NOPREFIX profile option is in effect, the file name is:
tsouserid.TEMPDTLW.DTLBATCH.Ttttttt.Rnnnnn.
The batch file name is uniquely created for each ISPDTLC invocation by including the system time and a
random number as the last two qualifiers of the name.
The ISPPREP utility is called to convert all of the generated panels from ISPF source format to
preprocessed format at one time to improve performance.
Defining application panels: the PANEL tag
28  z/OS: z/OS ISPF DTL Guide

## Page 61

The panel title
The text that appears as the title of the panel is called the title text. You define the title text by coding it as
tag text for the PANEL start tag.
This example uses the text "Catalog Ordering System" as title text:
<panel name=mainpan>Catalog Ordering System
⋮
</panel>
Panel size (width and depth)
Use the DEPTH and WIDTH attributes of the PANEL tag to define the size of an application panel. The
PANEL tag has a default WIDTH value of 76 characters and a default DEPTH value of 22 lines. If you
specify WINDOW=NO, the default WIDTH is 80 and the default DEPTH is 24. These are the values the
conversion utility assumes if you do not specify dimensions for WIDTH and DEPTH.
Here is an example that defines the panel size as 60 characters wide and 15 lines deep:
<panel name=mainpan width=60 depth=15>Catalog Ordering System
⋮
</panel>
To make the width of the panel 76 characters (the default width), we only need to specify a value for
DEPTH, as in this markup:
<panel name=mainpan depth=15>Catalog Ordering System
⋮
</panel>
This results in a panel with a default width of 76 characters and a specified depth of 15 lines.
Because you can display application panels in pop-ups, you should allow for pop-up borders (added by
ISPF at run time) when you define the WIDTH and DEPTH values for application panels. When the panel is
displayed in a pop-up, ISPF adds two lines to the depth specified and 4 characters to the width specified
for pop-up borders. Remember that ISPF cannot display a panel whose size exceeds the device size and
issues an error message at run time in this situation.
Key mapping lists
To specify the function keys that are active for an application panel, use the KEYLIST attribute of the
PANEL tag. This attribute specifies the name of the key mapping list you define for use with the panel. A
key mapping list contains the keys that are active while the panel is displayed. The key mapping list also
specifies what command is run when each key is pressed.
This PANEL definition refers to a key mapping list named key01:
<panel name=mainpan keylist=key01>Catalog Ordering System
⋮
</panel>
For more information about defining key mapping lists, see Chapter 9, “Defining key mapping lists,” on
page 147.
Associated help panels
To provide help for an application panel (also called extended help), specify the name of the associated
help panel with the HELP attribute of the PANEL tag. The help panel you specify appears when the user
requests extended help while in the application panel or when contextual help is requested for an item
on the panel, but no contextual help is available for the item. The help panel you specify is also displayed
when the user requests extended help while in a contextual help panel associated with an item on the
panel.
Defining application panels: the PANEL tag
Chapter 3. Getting started: designing application panels  29

## Page 62

This panel definition refers to a help panel named ordhelp:
<panel name=mainpan help=ordhelp>Catalog Ordering System
⋮
</panel>
“Help panels” on page 130 tells you how to create help panels for your application.
Panel defaults
The PANEL tag attribute PANDEF provides the name of a panel default definition. Attribute values defined
on the named PANDEF tag are used for the current panel unless the attribute has also been specified on
the PANEL tag.
Cursor placement
The PANEL tag attributes, CURSOR, CSRINDEX, and CSRPOS, allow you to specify where the cursor is
placed when the panel is initially displayed. If you do not specify a specific cursor position, ISPF places
the cursor in the first field in the PANEL definition that can contain the cursor.
Use the CURSOR attribute to specify the field that is to contain the cursor. Use the CSRINDEX and
CSRPOS attributes to identify positions within the field you specify with the CURSOR attribute. CSRINDEX
and CSRPOS can only be used when the CURSOR attribute is used.
The CURSOR attribute
Use the CURSOR attribute to specify the value of the NAME attribute of a CHOICE or SELFLD tag, or the
value of the DATAVAR attribute of a CHOFLD, DTAFLD or LSTCOL tag. Here are the characteristics of cursor
placement:
CHOFLD
The cursor appears in the first character position of the choice field. Cursor positioning is valid only
when the USAGE attribute of the CHOFLD tag specifies INPUT or BOTH.
CHOICE
The cursor appears in the entry field of the specified choice in a multiple-choice selection field.
DTAFLD
The cursor appears in the first character position of the data field. Cursor positioning is valid only
when the USAGE attribute of the DTAFLD tag specifies INPUT or BOTH.
LSTCOL
The cursor appears in the first row in the list column. Cursor positioning is valid only when the USAGE
attribute of the LSTCOL tag specifies INPUT or BOTH.
SELFLD
The cursor appears in the entry field of the specified single-choice selection field.
Chapter 5, “Application panel fields,” on page 71 provides a complete description of the types of
interactive fields you can define for your application panels.
You can also place the cursor in the command area of the panel by specifying cmdarea as the CURSOR
value.
“Defining a command area” on page 48 provides a complete description of the CMDAREA tag.
Here is an example where the CURSOR attribute specifies the data field DATAVAR value place. When the
panel is initially displayed, the cursor appears in the first character position of that field. Figure 10 on
page 31 shows the formatted result.
<!doctype dm system>
<panel name=mainpan1 cursor=place>Travel Agency
    <selfld name=dest selwidth=50 pmtwidth=15>Destinations:
      <choice>London
      <choice>Madrid
      <choice>Paris
      <choice>Zurich
Defining application panels: the PANEL tag
30  z/OS: z/OS ISPF DTL Guide

## Page 63

</selfld>
    <divider>
    <dtafld datavar=place entwidth=9 pmtwidth=5>Other
<cmdarea>
</panel>
This example, and other examples in this chapter, include tag markup for elements such as fields and
variables that have not yet been discussed, to illustrate the formatting characteristics of some tags. The
syntax of these elements are not important for the purposes of these examples. Syntax conventions of
these elements in discussed in later chapters of this document.
                               Travel Agency
 
 Destinations:
 __  1.  London
     2.  Madrid
     3.  Paris
     4.  Zurich
 
 Other _________
 
 
 
 
 
 
 
 
 
 
 
 
 Command ===> ____________________________________________________________
Figure 10. Cursor placement
If no cursor placement was specified in the PANEL tag for the preceding example, the cursor would
appear in the entry field of the Destinations single-choice selection field when the panel is initially
displayed.
The CSRINDEX attribute
To place the cursor in a table row within a list field, use the CURSOR attribute to specify the data variable
name for a list column within the list field, and the CSRINDEX attribute to specify the table row number
where the cursor should be placed. The value you assign to CSRINDEX must be numeric.
The CSRPOS attribute
If you use the CURSOR attribute to place the cursor within an input-only, or input/output data field or list
column, or the command area, you can also define a specific character position for the cursor using the
CSRPOS attribute.
The value you assign to the CSRPOS attribute must be numeric. This numeric value indicates the number
of character positions from the left margin of the field where the cursor is placed, where a 1 specifies that
the cursor should be in the first character position.
Other panel attributes
See “PANEL (Panel)” on page 376 for more information.
KEYLTYPE
This attribute is used to add the SHARED keyword to the KEYLIST parameter of the )PANEL statement.
Defining application panels: the PANEL tag
Chapter 3. Getting started: designing application panels  31

## Page 64

APPLID
This attribute is used to add the application ID to the KEYLIST parameter of the )PANEL statement.
CCSID
This attribute specifies the coded-character-set identifier as defined by the Character Data
Representation Architecture.
MENU
This attribute specifies that the panel is an ISPF menu selection panel.
PRIME
This attribute is used with the MENU attribute to specify an ISPF primary option menu.
TUTOR
This attribute specifies that the panel is to be an ISPF tutorial panel.
WINDOW
This attribute is used to control the generation of the WINDOW keyword on the panel )BODY
statement.
WINTITLE
This attribute is used to add a title on a pop-up window border.
APPTITLE
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
PAD
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PADC
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
OUTLINE
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
EXPAND
This attribute causes the ISPF EXPAND keyword to be added to the panel )BODY statement.
MSGLINE
This attribute controls the provision for a long message line in the generated panel. When
MSGLINE=NO, the blank line for the long message is not added to the panel )BODY section.
TITLINE
This attribute controls the provision for a panel title line in the generated panel. When TITLINE=NO,
the title line is not added to the panel )BODY section. This attribute allows a panel formatted as a
dynamic area to provide the panel title as part of the dynamic area data.
CMDLINE
This attribute controls the automatic addition of the command area to a menu selection or table
display panel. When CMDLINE=NO, the command area is not automatically generated when the
CMDAREA tag is not present in the DTL source file.
ATTRUSE
This attribute controls the use of panel attribute characters in the range of x‘01’ through x‘2F’. When
ATTRUSE=YES, dynamic area attributes (specified with the ATTR tag) can be assigned low-order hex
values normally reserved for use by the conversion utility.
ENDATTR
This attribute specifies that when the last attribute on any panel body line is ‘normal text’ (CUA), it is
replaced by the default ‘text’ (ISPF) attribute.
TYPE
This attribute specifies that the panel is used for host display, for display by a client that is using the
JSON API, or both.
Defining application panels: the PANEL tag
32  z/OS: z/OS ISPF DTL Guide

## Page 65

SMSG
This attribute provides the name of the field where the short message is to be placed.
LMSG
This attribute provides the name of the field where the long message is to be placed.
ASIS
This attribute specifies that the command and long message fields are to appear on the display as
specified in the generated panel definition. When ASIS is specified, any user request specified on the
Settings panel, or by setting the system variable ZPLACE is ignored.
ACTBAR
This attribute causes the action bar information for the panel to be generated, overriding the
NOACTBAR invocation option.
MERGESAREA
This attribute specifies that a panel with a single scrollable area be reformatted to combine the
scrollable area into the panel body.
PANELSTMT
This attribute controls the creation of the )PANEL statement.
ENTKEYTEXT
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAME
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPROW
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPCOL
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
TMARGIN
This attribute provides the number of blank lines to format at the top of the panel as a top margin.
BMARGIN
This attribute provides the number of blank lines to format at the bottom of the panel as a bottom
margin.
ERRORCHECK
This attribute specifies that error checking code is added to the )PROC panel section.
ZUP
This attribute provides the name of the tutorial panel to be assigned to the ZUP variable.
ZCONT
This attribute provides the name of the tutorial panel to be assigned to the ZCONT variable.
Defining action bars and pull-downs
To create a consistent user interface, you should design your applications according to the object-action
process sequence defined by the SAA Common User Access. The action bar is a major user interface
component that helps you achieve consistency in your applications.
The action bar is the panel element located at the top of an application panel that contains action
bar choices for the panel. Each action bar choice represents a group of related choices that appear in
the pull-down associated with the action bar choice. When the user selects an action bar choice, the
associated pull-down appears directly below the action bar choice. Pull-downs contain choices that, when
selected by the user, perform actions that apply to the contents of the panel.
Defining action bars and pull-downs
Chapter 3. Getting started: designing application panels  33

## Page 66

Panel design note:
ISPF and DTL provide the tools to help you create the object-action process sequence in your
application, but it is your responsibility as an application designer to ensure that the contents of your
action bar are actions that can be applied to the objects contained within your panel.
Typically, application panels intended for display within primary windows contain action bars that
present the user with all of the available actions that apply to that panel. Application panels that
are displayed as pop-ups should not include the action bar. Instead, actions for a pop-up panel are
presented in the function key area.
Figure 11 on page 34 shows an action bar with the File pull-down menu displayed.
Figure 11. Action bar and pull-down
The tags you use to create the action bar and its pull-down menus are:
AB
To start an action bar definition. The required AB end tag ends an action bar definition.
ABC
To define each of the action bar choices.
PDC
To define the choices on the pull-down associated with an action bar choice.
ACTION
To specify an action to be taken when the pull-down choice is selected. The ACTION tag is coded
within the PDC tag.
M
To specify a mnemonic character for action bar choice or pull-down choice selection.
Coding an action bar definition
This list describes how to code an action bar definition:
• Code the AB start tag immediately after the PANEL start tag and before any other tags in the panel.
• Following the AB start tag, code an ABC tag for each action bar choice in the action bar. The text you
specify on the ABC tag is the text that appears in the action bar as the action bar choice.
• Code the associated PDC tags within the ABC tags. The text you specify on the PDC tag is the text that
appears as the pull-down choice.
Defining action bars and pull-downs
34  z/OS: z/OS ISPF DTL Guide

## Page 67

• Following each PDC tag, code one or more ACTION tags to specify what type of action occurs when that
pull-down choice is selected by the user.
The ACTION tag RUN attribute (and its internal-command-name) define a command action for the
pull-down choice. If you define multiple ACTION tags for a pull-down choice, one of which contains a
RUN value, code the RUN action last, because any actions specified after a RUN action are ignored.
• End the action bar definition with the required AB end tag.
Here is an example that shows the markup for the action bar shown in Figure 11 on page 34. The detailed
markup for the File pull-down is included.
<panel name=mainpan2 depth=15>Sample Application
  <ab>
    <abc>File
      <pdc>Add Entry
        <action run=add>
      <pdc>Delete Entry
        <action run=delete>
      <pdc>Update Entry
        <action run=update>
      <pdc>Exit
        <action run=Exit>
    <abc>View
⋮
    <abc>Options
⋮
    <abc>Help
⋮
  </ab>
⋮
</panel>
Pull-down choice actions
A pull-down choice provides an immediate action to the user. To ensure that a pull-down choice performs
an immediate action, you should code an ACTION tag that specifies the RUN attribute for each pull-down
choice. The value you assign to RUN tells ISPF which command to run when the user selects the choice.
In the preceding markup example, each ACTION definition uses the RUN attribute to specify a command.
Each of these commands must be defined within the command table for the application. Chapter 8, “The
application command table,” on page 143 tells you how to define commands for an application.
In addition to the RUN action, you can specify other types of actions to occur when a pull-down choice is
selected. The SETVAR and TOGVAR attributes on the ACTION tag can be used to set and toggle variables
which the application can use to determine the processing to perform.
Remember, any SETVAR or TOGVAR actions for a pull-down choice must be coded before any ACTION
definition specifying the RUN action, because actions coded after RUN are ignored.
A pull-down choice may be marked as unavailable. The UNAVAIL attribute is used to provide a variable
name that is used by ISPF to determine the availability of the pull-down choice. When the variable value is
1, the pull-down choice is unavailable.
Action bar help
You can provide help for each action bar choice and pull-down choice with the HELP attribute on the ABC
and PDC tags, respectively. By specifying the name of a help panel or message for the action bar choice
or pull-down choice, ISPF knows which help information to display when the user requests help on that
choice. If you do not specify help for a pull-down choice, the help for the action bar choice is displayed,
when the user requests help. If there is no help defined for the action bar choice, the extended help panel
is displayed.
Defining action bars and pull-downs
Chapter 3. Getting started: designing application panels  35

## Page 68

This example adds the HELP attribute to each of the action bar choices and pull-down choices in the
action bar defined in “Coding an action bar definition” on page 34. The values specified with each HELP
attribute are the NAME values of defined help panels.
<!doctype dm system>
<panel name=mainpan3 width=50 depth=15>Sample Application
  <ab>
    <abc help=hfile>File
      <pdc help=hnew>Add Entry
        <action run=add>
      <pdc help=hopen>Delete Entry
        <action run=delete>
      <pdc help=hsave>Update Entry
        <action run=update>
      <pdc help=hexit>Exit
        <action run=Exit>
    <abc help=hview>View
⋮
    <abc help=hoption>Options
⋮
    <abc help=hhelp>Help
⋮
  </ab>
⋮
</panel>
In the preceding example, we defined a help panel named hhelp for the Help action bar choice.
Common User Access requires that you put the Help action bar choice as the last action bar choice in an
action bar definition. You should code the Help action bar pull-down in this way:
<abc help=hhelp>Help
  <pdc>Extended help
    <action run=exhelp>
  <pdc>Keys help
    <action run=keyshelp>
</abc>
“Help panels” on page 130 tells you how to define help panels.
Preselected pull-down choices
You can define a pull-down choice as being preselected with the CHECKVAR and MATCH attributes of the
PDC tag. The CHECKVAR attribute specifies the name of a variable that you set at run time to indicate if
the pull-down choice should be preselected. The MATCH attribute defines a value that causes the choice
to be preselected. ISPF compares the value of the variable named for the CHECKVAR attribute to the
MATCH value, and if they are equal, the choice appears preselected when the pull-down is displayed.
Continuing with the library application, assume that the user can view the files in the library sorted by
name, owner, date, or size. Preselecting a pull-down choice provides a visual cue to the user of the current
sort order.
To preselect any of the pull-down choices, the same CHECKVAR value is specified for each choice, and a
unique MATCH value is specified for each choice. The application variable specified with CHECKVAR is set
to the MATCH value to indicate the sorting option being used. The variable specified with CHECKVAR is
changed each time the sorting option is changed. This provides a visual reminder to the user of how the
files are sorted.
Mnemonic choice selection
ISPF supports mnemonic selection of action bar choices.
Mnemonic selection of action bar choices is automatically determined by ISPDTLC when a non-DBCS
conversion is in process. When DBCS is specified, mnemonics are not automatically generated. The
default mnemonic character generation can be overridden by adding the MNEMGEN=NO attribute to
the AB tag for non-DBCS conversions. The mnemonic character that is selected is the first alphabetic
Defining action bars and pull-downs
36  z/OS: z/OS ISPF DTL Guide

## Page 69

or numeric character from the current action bar choice description text that is not already used as
a mnemonic character within the action bar. If a unique mnemonic character cannot be selected,
the conversion utility issues a message. DBCS characters cannot be specified as mnemonics. See “M
(Mnemonic)” on page 351 for a description of how to provide a mnemonic character that is not part of the
normal choice description.
Mnemonic selection of action bar choices may be specified by placing the M tag immediately in front of
the character to be used as a mnemonic within the ABC text.
The automatic mnemonic generation does not replace any valid mnemonic specified by the M tag. (If the
mnemonic character specified by the M tag is a duplicate of a mnemonic character previously selected
by the generation process, a message is issued and ISPDTLC attempts to replace the duplicate value that
was specified.) This processing allows the combination of specific character selection with the automatic
generation feature, as long as the characters automatically generated and the characters specified (by the
M tag) are unique.
<!doctype dm system (
  <!ENTITY actnfile system>
  <!ENTITY actnoptn system>
  <!ENTITY actnhelp system>
)>
<panel name=pdcxmp1>Sample Application
  <ab>
    &actnfile;
    <abc>View
      <pdc checkvar=sorttype match=N>Name
        <action run=name>
      <pdc checkvar=sorttype match=O>Owner
        <action run=owner>
      <pdc checkvar=sorttype match=D>Date
        <action run=date>
      <pdc checkvar=sorttype match=S>Size
        <action run=size>
    &actnoptn;
    &actnhelp;
  </ab>
  <topinst>
  <area>
  </area>
  <cmdarea>
</panel>
If the application sets the variable sorttype to "D" before the panel is displayed, then the Date choice is
preselected.
Defining the panel body
In this topic we tell you how to use DTL to define elements of the panel body such as instruction text,
areas, regions, and dividers.
Panel instructions
DTL provides you with the TOPINST, PNLINST, and BOTINST tags to define instructions for your
application panels. None of the tags have required end tags associated with them.
Use the instruction tags to provide text that tells the user how to interact with the panel or how to
continue with an application.
If the COMPACT attribute is not specified, a blank line is added to the panel after each TOPINST tag and
before each PNLINST or BOTINST tag.
You must code the top and bottom instruction tags outside of the portion of the panel defined with the
AREA tag and its matching end tag. (“The AREA tag” on page 38 explains how to use the AREA tag).
Code the TOPINST tag immediately after the action bar definition (or the PANEL start tag if the panel does
Defining the panel body
Chapter 3. Getting started: designing application panels  37

## Page 70

not contain an action bar). Code the BOTINST following the main body of the panel, before the PANEL end
tag. You may code PNLINST tags within the AREA tag.
This application panel markup contains both types of instructions. Figure 12 on page 38 shows the
results.
<!doctype dm system>
<panel name=mainpan5>Item Selection
  <topinst>Select one of the following items and press Enter.
    <selfld name=itemtyp selwidth=76>
      <choice>Automotive
      <choice>Hardware
      <choice>Health and beauty
      <choice>Lawn and garden
      <choice>Sporting goods
    </selfld>
  <botinst>To exit the application, press F3.
</panel>
                               Item Selection
 Select one of the following items and press Enter.
 __  1.  Automotive
     2.  Hardware
     3.  Health and beauty
     4.  Lawn and garden
     5.  Sporting goods
 To exit the application, press F3.
 
Figure 12. Top and bottom instructions
The AREA tag
The AREA tag (and its matching end tag) defines the main portions of the panel body. You code all of the
interactive fields for the panel within AREA definitions.
Add an AREA definition to the previous application panel markup.
<!doctype dm system>
<panel name=mainpan6 depth=18>Item Selection
  <topinst>Select one of the following items and press Enter.
  <area>
    <selfld name=itemtyp selwidth=76>
      <choice>Automotive
      <choice>Hardware
      <choice>Health and beauty
      <choice>Lawn and garden
      <choice>Sporting goods
    </selfld>
  </area>
  <botinst>To exit the application, press F3.
</panel>
As stated in “Panel instructions” on page 37, you must code the top and bottom instruction tags outside
of the AREA definition. In this example, we coded only a selection field within the AREA definition.
Defining the panel body
38  z/OS: z/OS ISPF DTL Guide

## Page 71

The AREA tag has an optional MARGINW attribute that you can use to specify the width of the panel body
margins. This is useful for arranging fields on a panel.
The MARGINW attribute has a default value of 1. You can specify a different value to increase the size
of the panel body margins. For example, we could specify a margin width for the AREA in the preceding
markup example.
<!doctype dm system>
<panel name=mainpan7>Item Selection
  <topinst>Select one of the following items and press Enter.
  <area marginw=10>
    <selfld name=itemtyp selwidth=58>
      <choice>Automotive
      <choice>Hardware
      <choice>Health and beauty
      <choice>Lawn and garden
      <choice>Sporting goods
    </selfld>
  </area>
  <botinst>To exit the application, press F3.
</panel>
We specified a margin width of 10. Here is how the panel looks now:
                               Item Selection
 Select one of the following items and press Enter.
          __  1.  Automotive
              2.  Hardware
              3.  Health and beauty
              4.  Lawn and garden
              5.  Sporting goods
 To exit the application, press F3.
 
Figure 13. AREA MARGINW=10
Scrollable areas
You specify a scrollable area with the Dialog Tag Language by coding the AREA tag and specifying the
DEPTH, EXTEND, and DIV attributes for the area. When the DEPTH attribute is present, the conversion
utility generates the )AREA section in the panel definition, along with the corresponding )ATTR and )BODY
entries for the scrollable area.
Help panels generated by the Conversion Utility that contain all of the help panel text within an AREA
tag (with DEPTH specified) are not split into separate panels. The conversion utility places the text in
an )AREA section, which allows you to define panels up to the display size limit of ISPF.
If you specified DEPTH to signal the creation of a panel with a scrollable area, you can also specify the
EXTEND and DIV attributes.
You can specify EXTEND=ON to allow the panel to expand to the logical window size. If you intend
to have the panel in a pop-up window, you should not code the EXTEND attribute. Panels that specify
EXTEND=ON cannot be preprocessed.
Defining the panel body
Chapter 3. Getting started: designing application panels  39

## Page 72

You use the DIV attribute to control the creation of a divider line before and after the scrollable area.
If you specify DIV=BLANK, a blank divider line is added before and after the area. If you specify
DIV=SOLID, a visible divider is created. The visible divider formats with an attribute byte on each end
of the line of dashes, which causes the line to appear with a 1-character "space" on both ends. Omitting
the DIV attribute or specifying DIV=NONE causes the area to be created without divider lines.
The conversion utility uses the DEPTH attribute value to reserve a fixed amount of space in the panel
body. This space, together with the divider lines, if specified, is considered as part of the body within
the depth limit specified (or defaulted) on the PANEL tag. When EXTEND=OFF, the minimum depth for a
scrollable area is two lines, one for the scrolling indicator line and at least one line of displayable text.
Here is markup that shows how to code a scrollable panel. Figure 14 on page 41, Figure 15 on page 41,
and Figure 16 on page 42 show the formatted result.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar2 system>
  <!entity sampabc system>)>
&sampvar2;
<PANEL NAME=scrarea3 KEYLIST=keylxmp>File-A-Case
<AB>
&sampabc;
</AB>
<TOPINST COMPACT>
         Type in client's name and case number (if applicable).
<TOPINST>Then select an action bar choice.
<AREA>
<DTAFLD DATAVAR=caseno PMTWIDTH=12 ENTWIDTH=7 DESWIDTH=25>Case No
   <DTAFLDD>(A 7-digit number)
<DTAFLD DATAVAR=name PMTWIDTH=12 ENTWIDTH=25 DESWIDTH=25>Name
   <DTAFLDD>(Last, First, M.I.)
<DTAFLD DATAVAR=address PMTWIDTH=12 ENTWIDTH=25>Address
<DIVIDER>
<SELFLD NAME=casesel PMTWIDTH=30 PMTLOC=before SELWIDTH=38>Choose
one of the following
   <CHOICE CHECKVAR=case MATCH=civ>Civil
   <CHOICE CHECKVAR=case MATCH=estate>Real Estate
   <CHOICE CHECKVAR=case MATCH=environ>Environmental
</SELFLD>
</AREA>
<AREA DEPTH=6>
<SELFLD TYPE=multi PMTWIDTH=35 SELWIDTH=50>Check type of offense committed
   <CHOICE NAME=patin HELP=patin CHECKVAR=val>Patent Infringement
   <CHOICE NAME=defa HELP=defame CHECKVAR=def>Defamation
   <CHOICE NAME=cont HELP=cont CHECKVAR=con>Breach of Valid Contract
   <CHOICE NAME=priv HELP=priv CHECKVAR=pri>Invasion of Privacy
   <CHOICE NAME=incr HELP=incr CHECKVAR=icr>Interference with Contractual
           Relations
   <CHOICE NAME=disp HELP=disp CHECKVAR=dis>Improper Disposal of Medical
           By-Products
   <CHOICE NAME=fraud HELP=fraud CHECKVAR=fra>Fraud
</SELFLD>
</AREA>
<CMDAREA>Enter a command
</PANEL>
Defining the panel body
40  z/OS: z/OS ISPF DTL Guide

## Page 73

File  Search  Help
 ------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
 Case No  . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following    __  1.  Civil
                                    2.  Real Estate
                                    3.  Environmental
                                                            More:     +
 Check type of offense committed
 _  Patent Infringement
 _  Defamation
 _  Breach of Valid Contract
 Enter a command ===> ___________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 14. Scrollable panel area
After scrolling, the panel looks like this:
   File  Search  Help
 ------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
 Case No  . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following    __  1.  Civil
                                    2.  Real Estate
                                    3.  Environmental
                                                            More:   - +
 _  Breach of Valid Contract
 _  Invasion of Privacy
 _  Interference with Contractual Relations
 _  Improper Disposal of Medical By-products
 Enter a command ===> ___________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 15. Application panel area
After scrolling, the last choice in the list is visible.
Defining the panel body
Chapter 3. Getting started: designing application panels  41

## Page 74

File  Search  Help
 ------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
 Case No  . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following    __  1.  Civil
                                    2.  Real Estate
                                    3.  Environmental
                                                            More:   -
 _  Invasion of Privacy
 _  Interference with Contractual Relations
 _  Improper Disposal of Medical By-products
 _  Fraud
 Enter a command ===> ___________________________________________________
  F1=Help        F3=Exit        F5=Display     F6=Keyshelp   F10=Actions
 F12=Cancel
Figure 16. Scrollable panel area
Multiple AREA tags
The default AREA tag formatting arranges areas vertically within the panel body.
The WIDTH and DIR attributes of the AREA tag allow areas to be formatted horizontally.
Here is markup that shows horizontal areas. Figure 17 on page 43 shows the formatted result.
<!DOCTYPE DM SYSTEM(
  <!entity sampvar2 system>
  <!entity sampabc system>)>
&sampvar2;
<PANEL NAME=scrarea4 KEYLIST=keylxmp>File-A-Case
<AB>
&sampabc;
</AB>
<CMDAREA>Enter a command
<TOPINST COMPACT>
         Type in client's name and case number (if applicable).
<TOPINST>Then select an action bar choice.
<AREA width=50 dir=horiz>
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
</AREA>
<AREA width=26 dir=horiz>
<SELFLD TYPE=multi PMTWIDTH=24 SELWIDTH=26 depth=10>
  Check type of offense
   <CHOICE NAME=patin HELP=patin CHECKVAR=val>Patent Infringement
   <CHOICE NAME=defa HELP=defame CHECKVAR=def>Defamation
   <CHOICE NAME=cont HELP=cont CHECKVAR=con>Breach of Valid Contract
   <CHOICE NAME=priv HELP=priv CHECKVAR=pri>Invasion of Privacy
   <CHOICE NAME=incr HELP=incr CHECKVAR=icr>Interference with Contractual
           Relations
   <CHOICE NAME=disp HELP=disp CHECKVAR=dis>Improper Disposal of Medical
           By-Products
   <CHOICE NAME=fraud HELP=fraud CHECKVAR=fra>Fraud
Defining the panel body
42  z/OS: z/OS ISPF DTL Guide

## Page 75

</SELFLD>
</AREA>
</PANEL>
   File  Search  Help
 -------------------------------------------------------------------------
                                File-A-Case
 Type in client's name and case number (if applicable).
 Then select an action bar choice.
                                                  Check type of offense
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
Figure 17. Multiple horizontal areas
The DYNAMIC AREA tag
You specify a dynamic area in the )BODY section by coding the DA and ATTR tags. The DA tag is used to
define the dynamic area in the panel )BODY section. The ATTR tag is used to specify the )ATTR section
entries for DATAIN, DATAOUT, and CHAR attribute types used within the dynamic area. A dynamic area
allows you to specify an area of the panel to format with your application. See the z/OS ISPF Dialog
Developer's Guide and Reference for more information.
The GRAPHIC AREA tag
You specify a graphic area in the panel )BODY section by coding the GA tag. A Graphic area allows you to
define a specific portion of the screen for a GDDM display. See the z/OS ISPF Dialog Developer's Guide and
Reference for more information.
Defining the panel body
Chapter 3. Getting started: designing application panels  43

## Page 76

The DIVIDER tag
You can separate the elements on a panel or the regions you define for a panel with the DIVIDER tag. A
DIVIDER definition produces either a blank or visible divider line, depending on the value you assign to
the TYPE attribute of the DIVIDER tag. The visible divider line can be a dashed line or a solid line, or it can
contain text.
The default value, NONE, produces a blank divider line. The values DASH, SOLID, and TEXT produce a
visible divider line.
For horizontally formatted dividers,
• When the GRAPHIC invocation option is specified, SOLID produces a solid line for host display and
DASH produces a dashed line.
• When NOGRAPHIC is specified, both SOLID and DASH produce a dashed line.
Both SOLID and DASH use the "|" character (obtained from the ISPF literals table) for vertically formatted
dividers. The value TEXT (used in combination with the FORMAT attribute) is valid only for dividers within
vertical regions and specifies that blank padding is used for the supplied text.
The GAP attribute specifies whether the divider line completely crosses the panel area or region that
contains the divider, or if a 1-character gap is to remain at either end of a horizontally formatted divider.
The valid values for the GAP attribute are YES (the default), and NO.
The value you assign to GUTTER specifies the size (in characters) of the total width of the divider.
For vertical formatting the default is 1, because ISPF allots 1 line of screen space for the divider. For
horizontal formatting the default GUTTER size is 3, because an attribute byte is placed both before and
after the divider character. Any value more than the default is split to either side of the divider. If the
GUTTER value is an even number, the conversion utility increases the number by 1 so that the divider is
centered within the defined width. The GUTTER attribute is useful for creating blank space on a panel.
The NOENDATTR attribute is valid only when formatting dividers within horizontal regions. When
NOENDATTR is specified, the ending attribute is not added to the divider. With NOENDATTR and a
GUTTER size of 1, a divider of one blank character can be created. With a GUTTER size of 2, TYPE=SOLID
can be used to produce a visible divider.
The FORMAT attribute is valid only when formatting dividers within vertical regions. The FORMAT attribute
must be specified to have ISPDTLC process the text provided with the DIVIDER tag. FORMAT specifies the
text placement within the divider line as START, CENTER, or END.
Here is an example where there are two DIVIDER tags defined. The first DIVIDER does not specify a TYPE
attribute, and produces a blank horizontal line. The second DIVIDER specifies TYPE=SOLID, and produces
a visible divider.
<!doctype dm system>
<panel name=fields1>Selections
  <area>
    <dtacol selwidth=24 pmtwidth=15>
    <selfld name=item>Pick an item:
      <choice>Widget
      <choice>Doohickey
      <choice>Gizmo
    </selfld>
    <divider>
    <selfld name=color>Pick a color:
      <choice>Red
      <choice>Green
    </selfld>
    <divider type=solid gap=no>
    <selfld name=size>Pick a size:
      <choice>Minuscule
      <choice>Behemoth
    </selfld>
    </dtacol>
  </area>
  <botinst>To exit the application, press F3.
</panel>
Figure 18 on page 45 shows the result:
Defining the panel body
44  z/OS: z/OS ISPF DTL Guide

## Page 77

Selections
 
 Pick an item:
 __  1.  Widget
     2.  Doohickey
     3.  Gizmo
 
 Pick a color:
 __  1.  Red
     2.  Green
 -------------------------------------------------------------------------
 Pick a size:
 __  1.  Minuscule
     2.  Behemoth
 
 To exit the application, press F3.
 
 
 
  
Figure 18. Area dividers
The dashed line in the second divider in the preceding example extends across the entire AREA definition
to both margins because we specified GAP=NO in the DIVIDER definition.
The REGION tag
You can further define the areas of your panel, and how you want the information in the areas arranged,
with the REGION tag. Using one or more regions within a PANEL definition provides an easy way of
arranging the elements on a panel. Like the PANEL and AREA tags, the REGION end tag is required.
The DIR (direction) attribute of the REGION tag specifies how the elements within a region are arranged,
either horizontally or vertically. The default value is VERT, which arranges the elements within the region
vertically. This means that if you do not specify a horizontal region (DIR=HORIZ), or if you do not define a
region at all, the panel elements are arranged vertically by default.
In this example, the selection fields are arranged vertically, because no DIR value is defined for the
REGION tag.
<!doctype dm system>
<panel name=fields2>Selections
  <area>
    <region>
      <dtacol selwidth=24 pmtwidth=15>
      <selfld name=item>Pick an item:
        <choice>Widget
        <choice>Doohickey
        <choice>Gizmo
      </selfld>
      <divider>
      <selfld name=color>Pick a color:
        <choice>Red
        <choice>Green
      </selfld>
      <divider type=solid gap=no>
      <selfld name=size>Pick a size:
        <choice>Minuscule
        <choice>Behemoth
      </selfld>
      </dtacol>
    </region>
  </area>
  <botinst>To exit the application, press F3.
</panel>
Defining the panel body
Chapter 3. Getting started: designing application panels  45

## Page 78

Selections
 
 Pick an item:
 __  1.  Widget
     2.  Doohickey
     3.  Gizmo
 
 Pick a color:
 __  1.  Red
     2.  Green
 -------------------------------------------------------------------------
 Pick a size:
 __  1.  Minuscule
     2.  Behemoth
 
 To exit the application, press F3.
 
 
 
  
Figure 19. Vertical region
We'll specify the HORIZ value for the region to change the layout of the selection fields to horizontal.
Figure 20 on page 47 shows the result.
<!doctype dm system>
<panel name=fields3>Selections
  <area>
    <region dir=horiz>
      <dtacol selwidth=20 pmtwidth=15>
      <selfld name=item>Pick an item:
        <choice>Widget
        <choice>Doohickey
        <choice>Gizmo
      </selfld>
      <divider type=solid gutter=5>
      <selfld name=color>Pick a color:
        <choice>Red
        <choice>Green
      </selfld>
      <divider type=solid gutter=5>
      <selfld name=size>Pick a size:
        <choice>Minuscule
        <choice>Behemoth
      </selfld>
      </dtacol>
    </region>
  </area>
  <botinst>To exit the application, press F3.
</panel>
Defining the panel body
46  z/OS: z/OS ISPF DTL Guide

## Page 79

Selections
 Pick an item:        |   Pick a color:        |   Pick a size:
 __  1.  Widget       |   __  1.  Red          |   __  1.  Minuscule
     2.  Doohickey    |       2.  Green        |       2.  Behemoth
     3.  Gizmo        |                        |
 To exit the application, press F3.
 
Figure 20. Horizontal region
In the markup for this example, we also changed the format of the DIVIDER tags to provide additional
space and a visible line between the selection fields. We did this by specifying TYPE=SOLID and
GUTTER=5 on each of the DIVIDER tags. Also the divider lines are now vertical. That's because of the
way DTL handles dividers within regions. DTL adheres to these formatting rules for DIVIDER tags within
regions:
• A DIVIDER tag coded within a vertical region formats horizontally.
• A DIVIDER tag coded within a horizontal region formats vertically.
Here is markup to show how REGION and DIVIDER tags format under different circumstances. This
example shows both horizontal and vertical regions, as well as solid and blank dividers.
<!doctype dm system>
<panel name=mainpan8>Application
 <topinst>Complete the information below and press Enter.
 <area>
   <dtafld datavar=name entwidth=25 pmtwidth=9>Name
   <dtafld datavar=addr entwidth=25 pmtwidth=9>Address
   <region dir=horiz>
     <dtafld datavar=city pmtwidth=9 entwidth=25>City
     <dtafld datavar=stat pmtwidth=5 entwidth=2>State
     <dtafld datavar=zip pmtwidth=8 entwidth=5>Zip code
   </region>
   <divider type=solid gutter=3>
   <region dir=horiz>
     <selfld name=grade pmtwidth=32 selwidth=33>Highest education level
       <choice>Some high school
       <choice>High school graduate
       <choice>Some college
       <choice>College graduate
       <choice>Some post-graduate work
       <choice>Post graduate degree
     </selfld>
     <divider gutter=5>
     <region>
       <info width=30>
         <p compact>Complete if applicable:
       </info>
       <dtafld datavar=grad pmtwidth=10 entwidth=11>Date of graduation
       <dtafld datavar=field pmtwidth=10 entwidth=11>Field of study
     </region>
   </region>
 </area>
</panel>
Defining the panel body
Chapter 3. Getting started: designing application panels  47

## Page 80

Figure 21 on page 48 shows how the preceding markup formats.
                                Application
 Complete the information below and press Enter.
 Name  . . _________________________
 Address   _________________________
 City  . . _________________________  State __  Zip code _____
  ------------------------------------------------------------------------
                                        Complete if applicable:
 Highest education level                Date of
 __  1.  Some high school               graduation ___________
     2.  High school graduate           Field of
     3.  Some college                   study  . . ___________
     4.  College graduate
     5.  Some post-graduate work
     6.  Post graduate degree
 
Figure 21. Horizontal region
This is an example of nesting regions. The data fields for entering the graduation date and field of study
are arranged in a vertical region that is nested within a horizontal region.
The ALIGN, DEPTH, EXTEND, INDENT, LOCATION, WIDTH and GRPBOX attributes allow additional
formatting control. The DEPTH and EXTEND attributes are used with scrollable regions. ALIGN, INDENT,
LOCATION, and WIDTH affect the placement of fields within the region and the placement of the region
within the panel. GRPBOX allows for the display of a title for the region. The title is specified following the
REGION tag ending delimiter.
Defining a command area
Many applications are dependent on a command area in their panels. You define a command area and
specify the prompt text of the command area with the CMDAREA tag. The conversion utility supplies the
prompt symbol (===>) and provides the entry field in the command area for user input.
The conversion utility always formats the command area at the top of the panel. An ISPF runtime option
determines the actual display location of the command line.
The command area contains an entry field and command prompt text, and is normally displayed at the
bottom of an application panel. Users can enter commands in the command entry field. All commands
entered into the command entry field are validated against the commands you define within the
application command table and the ISPF-provided commands. For more information about defining the
application command table, see Chapter 8, “The application command table,” on page 143.
<!doctype dm system (<!entity actnbar system>)>
<panel name=cmdxmp1>Application Name
  &actnbar;
  <topinst>Sample command area panel
  <area>
  </area>
  <CMDAREA>
</panel>
Here is how the command area displays on the panel:
Defining a command area
48  z/OS: z/OS ISPF DTL Guide

## Page 81

File  View  Options  Help
 -------------------------------------------------------------------------
                              Application Name
 
 Sample command area panel
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 Command ===> ____________________________________________________________
Figure 22. Command area
In Figure 22 on page 49 we did not specify the text of the command prompt, so the conversion utility
automatically added the text "Command" (or its translated equivalent), which is the default text. If we
wanted to specify something other than this text, we could have coded it as tag text, as in this example:
<!doctype dm system (<!entity actnbar system>)>
<panel name=cmdxmp2>Application Name
  &actnbar;
  <topinst>Sample command area panel
  <area>
  </area>
  <cmdarea>Enter a command
</panel>
You can code up to 59 bytes of prompt text on a standard 76-byte width panel when overriding the
default text. Here is how the command prompt looks now:
Defining a command area
Chapter 3. Getting started: designing application panels  49

## Page 82

File  View  Options  Help
 -------------------------------------------------------------------------
                              Application Name
 
 Sample command area panel
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 Enter a command ===> ____________________________________________________
Figure 23. Command area
Data entered on the command line can be forced to uppercase either by specifying CAPS = ON or
by including a VARCLASS tag to define the command area and an XLATL tag to specify translation to
uppercase. (The type attribute defines the space available on a standard 76 character width panel using
the default command prompt.)
  <varclass name=vccmd type='char 59'>
    <xlatl format=upper>
    </xlatl>
  </varclass>
  <varlist>
    <varclass name=zcmd varclass=vccmd>
  </varclass>
The AUTOTAB, CAPS, CMDLEN, CMDLOC, ENTWIDTH, NAME, NOINIT, NOJUMP, OUTLINE, PAD, PADC,
PLACE, PMTTEXT, PSBUTTON, PSVAL, PSVAR, SCRCAPS, SCROLLTAB, SCROLLVAR, and SCRVHELP are
attributes that control formatting, initialization, and presentation of the command area.
Defining panel defaults
DTL provides a tag that makes it easier to define attributes and values that are common for multiple
application panels: the PANDEF (panel default) tag. This tag must be coded in the source file before any
panels it is providing defaults for.
The default PANEL values you can define with the PANDEF tag are:
• The panel dimensions (DEPTH and WIDTH)
• The help panel
• The key mapping list
• The KEYLTYPE value
• The CCSID number
• The WINDOW value
• The WINTITLE value
• The APPTITLE value
• The PAD value
Defining panel defaults
50  z/OS: z/OS ISPF DTL Guide

## Page 83

• The PADC value
• The OUTLINE value
• The EXPAND value.
• The MERGESAREA value
• APPLID value
• ENTKEYTEXT value
• IMAPNAME value
• IMAPROW value
• IMAPCOL value
• TMARGIN value
• BMARGIN value
You can use a PANDEF tag to define all of these values, or some of them. You can also override a specific
panel default value for a referencing panel by specifying the attribute on the PANEL tag.
For instance, if you create a series of panels that all have the same dimensions and that all refer to the
same help panel and key mapping list, you can define these values in a PANDEF definition, and refer to
that definition in each of the application panels that use those values. The DTL compiler does the rest of
the work for you, as long as the default definition is available as part of the same source file as the panels
that refer to it.
For example, if you are creating a series of panels that all share the same values, you could create a
PANDEF definition like this:
<!doctype dm system>
<pandef id=printdef help=prnthlp depth=20 width=70 keylist=printkey>
And refer to the panel default like this on all of the panels in that series:
<panel name=panel01 pandef=printdef>A Panel
⋮
</panel>
<panel name=panel02 pandef=printdef>Another Panel
⋮
</panel>
When you compile this source file, the PANDEF definition provides those values for the panels that refer to
the panel default.
You can also use the PANDEF tag to define common values for individual PANEL attributes. For instance, if
the only commonality between application panels is the dimensions, you can use a panel default to define
the dimensions and refer only to those values in the application panel definitions:
<!doctype dm system>
<pandef id=size depth=20 width=70>
<panel name=panel01 help=help01
keylist=keylsta pandef=size>A Panel
⋮
</panel>
<panel name=panel02 help=help02
keylist=keylstb pandef=size>Another Panel
⋮
</panel>
To change the dimensions of the application panels that refer to a panel default, you only have to make
the change in one place: the PANDEF definition.
To override a PANDEF value, you must specify that value in the PANEL definition. Here is an example of a
panel default that defines both dimensions and a help panel. While all three PANEL definitions refer to the
Defining panel defaults
Chapter 3. Getting started: designing application panels  51

## Page 84

panel default, the panel with the NAME value panel03 specifies a different help panel, and thus overrides
the PANDEF HELP value.
<!doctype dm system>
<pandef id=pandef01 depth=20 width=70 help=help01>
<panel name=panel01 pandef=pandef01>
⋮
</panel>
<panel name=panel02 pandef=pandef01>
⋮
</panel>
<panel name=panel03 pandef=pandef01 help=help02>
⋮
</panel>
You can also define multiple panel defaults within a single source file, like this:
<!doctype dm system>
<pandef id=pandef01 depth=20 width=70 help=help01>
<pandef id=pandef02 depth=10 width=50 help=help02 keylist=klist01>
<panel name=panel01 pandef=pandef01>
⋮
</panel>
<panel name=panel02 pandef=pandef02>
⋮
</panel>
<panel name=panel03 pandef=pandef01 help=help02>
⋮
</panel>
Defining panel defaults
52  z/OS: z/OS ISPF DTL Guide
