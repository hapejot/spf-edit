# Chapter 5. Application panel fields

Source file: f54dt00_v3r1.md
Start page: 103
Page span: 103-132

## Page 103

Chapter 5. Application panel fields
Most of the direct interaction that takes place between the user and the application is through the use of
interactive fields. They provide a means for the user to communicate data to the application, as well as
receive data from the application.
The type of interaction the user has with the application depends on the task. The task, in turn,
determines the fields’ characteristics. The appearance of the fields, the application's response to user
input, and assistance such as messages and help information must all be considered when defining an
interactive field.
This topic explains how to use the Dialog Tag Language to define these types of fields and their operating
characteristics:
• Data fields
• Selection fields
• List fields.
This topic begins with a description of field prompts for data fields and selection fields.
Field prompts
A field prompt is static, descriptive text that explains the field it is associated with. Data fields and
selection fields support the use of field prompts. To define a field prompt for a data field or selection field,
specify the prompt text as the tag text on the DTAFLD and SELFLD tags.
The PMTLOC attribute defines the location of the prompt using one of these values:
PMTLOC = ABOVE
The prompt appears above and left-aligned with the field. This is the default for selection fields.
PMTLOC = BEFORE
The prompt appears directly in front of and on the same line as the field. This is the default for data
fields.
You should define the amount of space the prompt uses by specifying the PMTWIDTH attribute on the
DTAFLD and SELFLD tags. If the prompt text is longer than the width you specify on PMTWIDTH, the
prompt is word-wrapped on multiple lines. Using the PMTWIDTH attribute can ensure that multiple fields
with prompts are aligned evenly. If you do not specify PMTWIDTH, the field prompt length is determined
by the length of the prompt text.
When PMTLOC=BEFORE, the conversion utility inserts leader dots at the end of the prompt text to fill
the specified prompt width. For output-only data fields, a colon is used in place of the last leader dot.
For fields with this prompt location, it is a good idea to specify a PMTWIDTH with a value that allows for
leader dots after the prompt text. This lends consistency to the panel appearance. The conversion utility
issues a warning message when there is insufficient space for leader dots.
Figure 29 on page 72 shows how prompts appear.
Field prompts
© Copyright IBM Corp. 1989, 2024 71

## Page 104

Application Name
 Name . . . . ____________________
 Address  . . ____________________
 City . . . . ____________________
 State  . . . __
                                       Method of payment
 Age  . . . . __  1.  0  - 12          __  1.  Cash
                  2.  13 - 19              2.  Check
                  3.  20 - 29              3.  Credit card
                  4.  30 - 49
                  5.  50 - 64
                  6.  over 65
 Payment
 ___________
 
Figure 29. Prompt locations
The Name, Address, City, and State data fields show the prompts in front of the fields
(PMTLOC=BEFORE), as does the Age field, which shows a prompt for a selection field. The same prompt
width is used on the first five fields so that they align evenly. The Method of payment and Payment fields
demonstrate having the prompt above the field (PMTLOC=ABOVE).
Here is the markup used to demonstrate the field prompts in Figure 29 on page 72:
<!doctype dm system>
<varclass name=sampcls type ='char 20'>
<varclass name=statcls type='char 2'>
<varclass name=numcls  type='numeric 8 2'>
<varclass name=char1   type='char 1'>
<varlist>
  <vardcl name=name varclass=sampcls>
  <vardcl name=addr varclass=sampcls>
  <vardcl name=city varclass=sampcls>
  <vardcl name=stat varclass=statcls>
  <vardcl name=pay varclass=numcls>
  <vardcl name=age varclass=char1>
  <vardcl name=paymeth varclass=char1>
</varlist>
<panel name=pmt01>Application Name
  <area>
    <dtafld datavar=name entwidth=20 pmtwidth=12>Name
    <dtafld datavar=addr entwidth=20 pmtwidth=12>Address
    <dtafld datavar=city entwidth=20 pmtwidth=12>City
    <dtafld datavar=stat entwidth=2 pmtwidth=12>State
    <divider>
      <region dir=horiz>
        <selfld name=age selwidth=20 pmtloc=before pmtwidth=12>Age
          <choice>0  - 12
          <choice>13 - 19
          <choice>20 - 29
          <choice>30 - 49
          <choice>50 - 64
          <choice>over 65
        </selfld>
        <divider gutter=5>
        <selfld name=paymeth selwidth=24 pmtwidth=20>Method of payment
          <choice>Cash
          <choice>Check
          <choice>Credit card
        </selfld>
      </region>
Field prompts
72  z/OS: z/OS ISPF DTL Guide

## Page 105

<divider>
    <dtafld datavar=pay entwidth=11 pmtloc=above pmtwidth=7>Payment
  </area>
</panel>
Figure 30 on page 73 shows how the prompt width can affect the appearance of the prompt text.
                             Application Name
 This is a very, very long prompt __________
 This is a
 very, very
 long prompt  __________
 Here is another long prompt used to show word-wrapping of prompts
 __  1.  Choice 1
     2.  Choice 2
 Here is
 another long
 prompt used to
 show
 word-wrapping
 of prompts
 __  1.  Choice 1
     2.  Choice 2
 
Figure 30. Prompt widths
The prompts in the two data fields are formatted differently. The prompt text of the first data field is
not wrapped. It formats on one line, using as much space as necessary (up to the maximum available
formatting width). The second data field has the same prompt text, with a prompt width that is less than
the amount of space needed, so the prompt text is wrapped to as many lines as are needed. Similarly,
the two selection fields also demonstrate how the prompt text appears based on the prompt width.
The prompt text of data fields and selection fields can be displayed differently by omitting or specifying
different values for the PMTWIDTH attribute.
Here is the markup that demonstrates the field prompts in Figure 30 on page 73:
<!doctype dm system>
<varclass name=sampcls type='char 10'>
<varclass name=char1 type='char 1'>
<varlist>
  <vardcl name=samplea varclass=sampcls>
  <vardcl name=sampleb varclass=sampcls>
  <vardcl name=samplec varclass=char1>
  <vardcl name=sampled varclass=char1>
</varlist>
<panel name=pmt02>Application Name
  <area>
    <dtacol entwidth=10 selwidth=76>
    <dtafld datavar=samplea>This is a very, very long prompt
    <divider>
    <dtafld datavar=sampleb pmtwidth=12>This is a very, very long prompt
    <divider>
    <selfld name=samplec>Here is another long prompt used to show
            word-wrapping of prompts
      <choice>Choice 1
      <choice>Choice 2
    </selfld>
    <divider>
    <selfld name=sampled pmtwidth=14>Here is another long prompt used to show
                                     word-wrapping of prompts
      <choice>Choice 1
Field prompts
Chapter 5. Application panel fields  73

## Page 106

<choice>Choice 2
    </selfld>
    </dtacol>
  </area>
</panel>
Defining data fields
Data fields are used to display variable data and to allow the user to enter data. To define a data field,
use the DTAFLD tag. Every data field must have an associated variable, which is specified on the required
DATAVAR attribute. Like all variables used on the panel, the variable named on the DATAVAR attribute can
be declared using the VARDCL tag.
The purpose of the data field is defined using one of these values on the USAGE attribute of the DTAFLD
tag:
IN
Defines an entry (input-only) data field. An entry data field allows the user to enter data. When
an entry field is initially displayed, it is padded with underscore characters, unless the data is right-
justified.
OUT
Defines an output-only data field. An output-only data field is used to display the current value of the
variable associated with the data field. The user cannot tab to or interact with an output-only field.
BOTH
Defines an input/output data field. When an input/output field is initially displayed, the current value
of the associated variable is displayed, and the user can enter data into the field as well. If you do not
specify the USAGE attribute, BOTH is the default.
Data fields support field prompts, which can be placed in front of or above the data field.
This panel contains examples of all three types of data fields:
                             Library Inventory
 To add a book to the inventory, complete the fields below, and then press
 Enter.
 Title  . . . . __________________________________________________
 Author . . . . ____________________
 Publisher  . . SPOTH AND CRICK
 Number of
 pages  . . . . _____
 ---------------------------------------------------------------------------
 Today's date is  . : 08-10-89
 
Figure 31. Data fields 
Here is the markup for Figure 31 on page 74:
<!doctype dm system>
<varclass name=titlcls type='char 50'>
<varclass name=bookcls type='char 20'>
Defining data fields
74  z/OS: z/OS ISPF DTL Guide

## Page 107

<varclass name=pagecls type='numeric 5'>
<varclass name=datecls type='char 8'>
<varlist>
  <vardcl name=title   varclass=titlcls>
  <vardcl name=author  varclass=bookcls>
  <vardcl name=publish varclass=bookcls>
  <vardcl name=pages   varclass=pagecls>
  <vardcl name=curdate  varclass=datecls>
</varlist>
<panel name=dfdxmp1a>Library Inventory
  <topinst>To add a book to the inventory, complete the fields below,
           and then press Enter.
  <area>
    <dtafld datavar=title usage=in pmtwidth=14>Title
    <dtafld datavar=author usage=in entwidth=20 pmtwidth=14>Author
    <dtafld datavar=publish entwidth=20 pmtwidth=14>Publisher
    <dtafld datavar=pages usage=in entwidth=5 pmtwidth=14>Number of pages
    <divider type=solid gutter=3 gap=no>
    <dtafld datavar=curdate usage=out entwidth=8 pmtwidth=20>Today's date is
  </area>
</panel>
In the previous example, there are three input-only data fields, an input/output data field, and an output-
only data field. The value of the associated variable is not displayed in an input-only data field, so when
the panel is initially displayed, the Title, Author, and Number of pages fields are blank. The Publisher
data field assumes the default, BOTH, so the current value of the associated variable, publish, is displayed
in the data field when the panel is initially displayed. The output-only data field is used to display the
current date. The user cannot interact with this data field, since it is used only to display variable data.
The user can enter data into any of the data fields except the output-only field.
Data field width
The width of a data field is determined by the value you specify for the ENTWIDTH attribute of the DTAFLD
tag. You should specify ENTWIDTH for all data fields. In the previous example, ENTWIDTH is specified for
each DTAFLD tag except for the Title field, whose length is determined as discussed next.
If you do not specify a value for ENTWIDTH, the width of the data field is determined by the value
specified for the TYPE attribute of the VARCLASS tag associated with the variable named in the DTAFLD
DATAVAR attribute. For example, the Title field in Figure 31 on page 74 has an entry width of 50 as
determined by the variable class titlcls, which has the TYPE value “char 50”. This variable class is
associated with the data field through the variable declaration title, which is specified as the data field's
DATAVAR attribute value. For more information about variables and variable classes, see Chapter 4,
“Variables and variable classes,” on page 53.
The formatted width of the field is 2 positions more than the ENTWIDTH value to provide for an attribute
byte both before and after the field. The maximum width for an entry field is the remaining available
formatting width in the panel.
Note: The conversion utility tracks the remaining width available for use. For data fields, the width of the
entry field has first priority, followed by the prompt width, and then the description width.
Data field descriptions
In addition to a field prompt, you can provide additional descriptive text for a data field using the
DTAFLDD (data field description) tag. You code the DTAFLDD tag following the definition of the data field
being described. The DTAFLDD tag has no attributes or required end tag. Multiple data field descriptions
can be coded if necessary, and each description begins a new line.
The data field description appears to the right of the entry field, taking up as much room as is available,
unless you have used the DESWIDTH attribute of the DTAFLD tag to specify a width for the description.
If the DESWIDTH attribute is defined, the data field description is displayed within the description width
specified (or defaulted), and word-wrapped on multiple lines, if necessary.
This panel contains data field descriptions.
Defining data fields
Chapter 5. Application panel fields  75

## Page 108

Library Inventory
 To add a book to the inventory, complete the fields below, then press
 Enter.
 Title . . . . . __________________________________________________
 Author  . . . . ____________________  Last name, First name, M.I.
 Publisher . . . SPOTH AND CRICK
 Total number of
 pages . . . . . ______  (1 - 99999)
 
Figure 32. Data field  description
Here is the markup used to generate the panel in Figure 32 on page 76:
<!doctype dm system>
<varclass name=titlcls type='char 50'>
<varclass name=bookcls type='char 20'>
<varclass name=pagecls type='numeric 5'>
<varlist>
  <vardcl name=title   varclass=titlcls>
  <vardcl name=author  varclass=bookcls>
  <vardcl name=publish varclass=bookcls>
  <vardcl name=pages   varclass=pagecls>
</varlist>
<panel name=dfdxmp4>Library Inventory
  <topinst>To add a book to the inventory, complete the fields below,
  then press Enter.
  <area>
    <dtacol pmtwidth=15>
    <dtafld datavar=title usage=in entwidth=50>Title
    <dtafld datavar=author usage=in entwidth=20 deswidth=30>Author
      <dtafldd>Last name, First name, M.I.
    <dtafld datavar=publish entwidth=20>Publisher
    <dtafld datavar=pages usage=in entwidth=5 deswidth=15>
            Total number of pages
      <dtafldd>(1 - 99999)
    </dtacol>
  </area>
</panel>
Data field help
ISPF allows you to provide help on a data field using the HELP attribute on the DTAFLD tag. If you specify
the name of a help panel or message for a data field, ISPF knows which help information to display when
the user selects help on the data field. If you do not specify help for a data field, the extended help panel
(specified with the HELP attribute of the enclosing PANEL tag) is displayed.
Here is an example that shows how to provide help for data fields:
<!doctype dm system>
<varclass name=titlcls type='char 50'>
Defining data fields
76  z/OS: z/OS ISPF DTL Guide

## Page 109

<varclass name=bookcls type='char 20'>
<varclass name=pagecls type='numeric 5'>
<varlist>
  <vardcl name=title   varclass=titlcls>
  <vardcl name=author  varclass=bookcls>
  <vardcl name=publish varclass=bookcls>
  <vardcl name=pages   varclass=pagecls>
</varlist>
<panel name=dfdxmp5>Library Inventory
  <topinst>To add a book to the inventory, complete the fields below,
  then press Enter.
  <area>
    <dtacol pmtwidth=15>
    <dtafld datavar=title help=hlptitl entwidth=50>Title
    <dtafld datavar=author help=hlpauth entwidth=20 deswidth=30>Author
      <dtafldd>Last name, First name, M.I.
    <dtafld datavar=publish help=hlppubl entwidth=20>Publisher
    <dtafld datavar=pages help=hlppage entwidth=5 deswidth=15>
            Total number of pages
      <dtafldd>(1 - 99999)
    </dtacol>
   </area>
</panel>
Other data field attributes
There are several other attributes you can specify to tailor a data field to meet the requirements of your
application. See “DTAFLD (Data Field)” on page 275 for more information. Here is a list that describes
each of the remaining DTAFLD attributes and what you can do with them:
REQUIRED
This attribute allows you to indicate if the data field requires input. When you assign a value of YES
to this attribute, the user must enter data into the field before ISPF accepts the panel as valid. The
default REQUIRED value is NO. This attribute is only valid for data fields defined as input-only or as
input/output.
MSG
This attribute identifies the message that should be displayed when the user does not enter any data
into an input-required data field. If you do not specify this attribute, ISPF displays a default message.
This attribute is valid only if REQUIRED=YES.
Chapter 7, “Messages,” on page 137 tells you how to define application messages.
ALIGN
This attribute allows you to align the variable data within the data field. The default value for ALIGN is
start, which aligns the data from the left side of the data field. You can also center the data within the
field with the center value, or justify the data from the right side of the field with the end value.
AUTOTAB
This attribute provides automatic cursor movement between data fields. If you specify AUTOTAB=YES
for a data field, the cursor automatically moves to the next field that is capable of input. If no other
field capable of input exists on the panel, the cursor returns to the beginning of the data field.
DISPLAY
The value you assign to this attribute, either yes (the default) or no, determines if the data appears on
the screen when the user enters it. One way to use DISPLAY=NO is for defining a password.
VARCLASS
This attribute allows you to override the variable class that is specified on the variable declaration
(VARDCL) for the data field's data variable (DATAVAR). See Chapter 4, “Variables and variable
classes,” on page 53 for a description of variables and variable classes.
FLDSPACE
This attribute specifies the space reserved for the data-entry field. When the FLDSPACE value is larger
than the entry width plus any attributes, blanks are added following the data-entry field. This provides
spacing before DTAFLDD tag descriptions.
Defining data fields
Chapter 5. Application panel fields  77

## Page 110

NOENDATTR
This attribute specifies that no ending attribute character is placed after the data field. NOENDATTR is
valid only when WINDOW=NO is specified or when data fields are being formatted within a horizontal
region.
PAD
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PADC
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”.
OUTLINE
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
PMTFMT
This attribute controls the generation of prompt leader characters. The default is to create CUA leader
dots.
PSVAR
This attribute provides the name of a variable that is to be set when a DTAFLD is clicked on for
point-and-shoot selection.
PSVAL
This attribute provides the value to be placed in the field specified by the PSVAR attribute.
PAS
This attribute provides a variable name that contains the value ON to enable point-and-shoot for this
data field, or OFF to disable point-and-shoot. When PSVAR and PSVAL have been specified without
the PAS attribute, the point-and-shoot field is automatically enabled.
CSRGRP
The CSRGRP attribute, in combination with the PAS attribute, enables the use of a cursor group by a
client that is using the JSON API.
EXPAND
The EXPAND attribute, used in combination with EXPAND=xy on the PANEL definition, causes the
expand characters to be added to the DTAFLD definition, effectively allowing ENTWIDTH to expand.
FLDWIDTH
The FLDWIDTH attribute, used in combination with WINDOW=NO on the PANEL definition, provides
the width of a data field that spans multiple lines.
ATTRCHANGE
The ATTRCHANGE attribute specifies that, if required, an additional )ATTR section entry (which can
apply to multiple fields) be created instead of a unique ".ATTR" override entry for the current field.
INIT
The INIT attribute provides an initial value for the data field.
DBALIGN
The DBALIGN attribute is used only for DBCS language conversion when PMTLOC=ABOVE to align the
prompt text with the data field.
DEPTH
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAME
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
IMAPNAMEP
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
Defining data fields
78  z/OS: z/OS ISPF DTL Guide

## Page 111

PLACE
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
PMTSKIP
This attribute, used during horizontal field formatting of input fields, specifies that the cursor should
move past the prompt text to the input field.
DESSKIP
This attribute, used during horizontal field formatting of input fields, specifies that the cursor should
move past the description text to the next input field.
FLDTYPE
This attribute specifies whether CUA or traditional ISPF attribute definitions are used.
COLOR
When FLDTYPE=ISPF, this attribute specifies the color of the field.
INTENS
When FLDTYPE=ISPF, this attribute specifies the intensity of the field.
HILITE
When FLDTYPE=ISPF, this attribute specifies the highlighting for the field.
ATTRCHAR
This attribute provides a user selected panel attribute for the data field.
CAPS
This attribute specifies whether the field is displayed in uppercase characters.
NOJUMP
This attribute specifies that the JUMP function is disabled for the data field.
AUTOTYPE
This attribute specifies whether ISPF panel logic is added to support the AUTOTYPE function.
AUTOVOL
This attribute specifies an associated volume name when AUTOTYPE = DSN.
AUTODMEM
This attribute specifies whether a member name is part of the data set name when AUTOTYPE = DSN.
VARDCL
This attribute specifies whether the field name is validated to the panel variables specified with the
VARDCL tag.
Defining selection fields
Selection fields allow the user to select from a group of choices on an application panel. You can specify if
only one choice can be selected from a selection field, or if multiple choices are allowed.
In either case, you use the same DTL tags to define a selection field. The SELFLD (selection field) tag and
its required end tag define a selection field. The CHOICE (selection choice) tag defines a choice within a
selection field. You code the CHOICE tags between the SELFLD start and end tags, like this:
<selfld>
  <choice>
  <choice>
  <choice>
</selfld>
Each CHOICE tag defines a choice within the selection field.
Like data fields, selection fields support field prompts, which can be placed in front of or above the
selection field. Field prompts are described in “Field prompts” on page 71.
To define the selection field type use the TYPE attribute of the SELFLD tag. The values you can assign to
TYPE are:
Defining selection fields
Chapter 5. Application panel fields  79

## Page 112

SINGLE
Specifies the selection field as being a single-choice field. Choices in a single-choice selection field
appear in a list with an entry field preceding the first choice in the list. The conversion utility prefixes
the text of each choice with a number, so the selection field choices are numbered sequentially. Users
indicate choice selection by typing the number of the choice they want in the entry field.
MULTI
Specifies the selection field as being a multiple-choice field. Choices in a multiple-choice selection
field appear in a list with a single-character entry field preceding each choice. Users indicate choice
selection by typing any nonblank character in the entry fields.
MENU
Specifies the selection field as being a menu-choice field. Choices in a menu-choice selection field are
similar to those in a single-choice selection field. TYPE=MENU is valid only when the MENU keyword
has been specified on the PANEL tag.
MODEL
Specifies the selection field as being a model-choice field. Choices in a model-choice selection field
are similar to those in a menu-choice selection field. TYPE=MODEL is valid only when the MENU
keyword has been specified on the PANEL tag.
TUTOR
Specifies the selection field as being a tutor-choice field. Choices in a tutor-choice selection field are
similar to those in a menu-choice selection field. TYPE=TUTOR is valid only when the MENU keyword
has been specified on the PANEL tag.
The CHOICE tag has two attributes associated with it that are important when defining a selection field:
CHECKVAR and MATCH. The CHECKVAR and MATCH attributes are used to preselect choices in the
selection field. The CHECKVAR attribute can also communicate to the application which selections were
made by the user.
The value specified on the CHECKVAR attribute is the name of a dialog variable that is defined by the
application. Both the application and ISPF can set the check variable. Here are topics that describe how
the CHECKVAR and MATCH attributes are used for each type of selection field.
Single-choice fields
Use a single-choice selection field when you have a fixed set of choices that are mutually exclusive.
That is, the user can select only one of the choices by typing the choice number in the entry field. You
can specify the preselected choice in a single-choice selection field so that one item is already selected
when the panel is displayed. The user can either leave the preselected choice or enter a different choice
number.
To preselect choices in a single-choice selection field, and to find out which choice was selected by the
user, you should specify the CHECKVAR and MATCH attributes for each CHOICE tag. For a single-choice
field, all of the enclosed choices should refer to the same check variable, but they should have unique
MATCH values. The example markup shows how this is coded:
<!doctype dm system>
<varclass name=daycls type ='char 1'>
<varlist>
  <vardcl name=day varclass=daycls>
  <vardcl name=choice varclass=daycls>
</varlist>
<panel name=singsel>Schedule Appointments
  <topinst>Choose the most convenient day for your appointment,
           then press Enter.
  <area>
    <selfld name=choice selwidth=30 pmtwidth=9>Weekdays:
      <choice checkvar=day  match=M>Monday
      <choice checkvar=day  match=T>Tuesday
      <choice checkvar=day  match=W>Wednesday
      <choice checkvar=day  match=H>Thursday
Defining selection fields
80  z/OS: z/OS ISPF DTL Guide

## Page 113

<choice checkvar=day  match=F>Friday
    </selfld>
  </area>
</panel>
To preselect a certain choice, set the check variable, day, to the match value for that choice. Assume that
the check variable, day, is set to M before the panel is displayed. When the panel is displayed, the choice,
Monday, is selected as shown in Figure 33 on page 81.
                           Schedule Appointments
 Choose the most convenient day for your appointment, then press Enter.
 Weekdays:
 1   1.  Monday
     2.  Tuesday
     3.  Wednesday
     4.  Thursday
     5.  Friday
 
Figure 33. Single-choice selection field 
If the user decides that another day is more convenient, another choice might be selected. This causes
the check variable to be updated with the match value of the newly selected choice. For example, if the
user selects Friday (by typing "5" in the entry field), the check variable, day, contains "F" when control is
returned to the application.
Note: The TYPE attribute does not have to be specified on a single-choice selection field because
TYPE=SINGLE is the default. However, you must specify the NAME attribute for single-choice selection
fields.
Multiple-choice fields
Use a multiple-choice selection field when you have several choices for the user, but they are not mutually
exclusive. Each choice acts independently as a toggle, and selecting one of the choices does not affect
any of the other choices in the selection field.
To preselect choices in a multiple-choice selection field, and to find out which choices were selected by
the user, specify the CHECKVAR, MATCH, and NOMATCH attributes for each CHOICE tag.
On a multiple-choice selection field, define a unique check variable for each enclosed CHOICE. You can
let the MATCH value default to 1, or specify the MATCH attribute with a value of your choice. Also, you can
let the NOMATCH value default to 0, or specify the NOMATCH attribute with a value of your choice. Here is
how a multiple-choice selection field is coded:
<!doctype dm system>
<varclass name=sampcls type ='char 1'>
<varlist>
  <vardcl name=dry varclass=sampcls>
  <vardcl name=cut varclass=sampcls>
  <vardcl name=per varclass=sampcls>
Defining selection fields
Chapter 5. Application panel fields  81

## Page 114

<vardcl name=fac varclass=sampcls>
  <vardcl name=man varclass=sampcls>
  <vardcl name=ped varclass=sampcls>
  <vardcl name=ch1 varclass=sampcls>
  <vardcl name=ch2 varclass=sampcls>
  <vardcl name=ch3 varclass=sampcls>
  <vardcl name=ch4 varclass=sampcls>
  <vardcl name=ch5 varclass=sampcls>
  <vardcl name=ch6 varclass=sampcls>
</varlist>
<panel name=multsel>Schedule Appointments
  <area>
    <dtacol pmtwidth=45 selwidth=76>
    <selfld type=multi>Choose the services needed, then press Enter.
      <choice name=ch1 checkvar=dry>Dry haircut
      <choice name=ch2 checkvar=cut>Shampoo, haircut, and style
      <choice name=ch3 checkvar=per>Permanent or body wave
      <choice name=ch4 checkvar=fac>Facial
      <choice name=ch5 checkvar=man>Manicure
      <choice name=ch6 checkvar=ped>Pedicure
    </selfld>
    </dtacol>
  </area>
</panel>
You specify preselected choices for a multiple-choice selection field just as you would for a single-choice
selection field. Set the check variable for the preselected choices to the match values (or the default value
of 1) for those choices. When a choice is preselected, a slash (/) is displayed in the entry field preceding
the choice.
When the user types a value in an entry field in a multiple-choice selection field, ISPF toggles the choice
in this way:
• If the choice is already selected and the user enters a blank in the entry field, ISPF deselects the choice
and sets the check variable to the NOMATCH value for the choice, or to 0 if the NOMATCH attribute is
not specified.
• If the choice is not selected and the user types a nonblank character in the entry field, ISPF selects the
choice and sets the check variable to the MATCH value for the choice, or to 1 if the MATCH attribute is
not specified. If the choice is not selected, ISPF sets the check variable to the NOMATCH value for the
choice, or to 0 if the NOMATCH attribute is not specified.
In the preceding markup, the MATCH attribute was not specified, so the check variables toggle between 0
and 1 (the default MATCH and NOMATCH values) as the user selects and deselects items.
Because ISPF is setting the check variable, you should not use the SETVAR or the TOGVAR attributes of
the ACTION tag to refer to the check variable.
Figure 34 on page 83 shows how the multiple-choice selection field in the preceding markup appears
with the choices Facial and Pedicure preselected.
Defining selection fields
82  z/OS: z/OS ISPF DTL Guide

## Page 115

Schedule Appointments
 Choose the services needed, then press Enter.
 _  Dry haircut
 _  Shampoo, haircut, and style
 _  Permanent or body wave
 /  Facial
 _  Manicure
 /  Pedicure
 
Figure 34. Multiple-choice selection field 
Menu-choice fields
Use a menu-choice selection field to create an ISPF option menu. Menu-choice fields are similar to
single-choice fields. That is, the user can select only one of the choices presented. The entry field for this
type of selection field is the command line, which is formatted with the word Option instead of Command.
As with single-choice selections, you can specify a preselected choice so that one item is already selected
when the panel is displayed.
The CHOICE tag is followed by an ACTION tag which specifies the type of selection (PANEL, PGM, CMD, or
EXIT), and other attributes required by the ISPF SELECT service.
When creating an option menu, the MENU keyword is required on the PANEL tag. The optional PRIME
keyword causes the creation of a primary option menu. The SELFLD tag must specify TYPE=MENU.
Depending on the panel being created, the SELFLD tag attributes ENTWIDTH, FCHOICE, and TRAIL, and
the CHOICE tag attribute SELCHAR might be required. See Chapter 12, “Tag reference,” on page 179 for
more information on the PANEL, SELFLD, CHOICE, and ACTION tags.
The example markup creates a sample option menu:
<!doctype dm system ()>
<!--  MENU selection panel example -->
<panel name=menusel1 menu>Sample Option Menu
  <topinst>Enter a selection choice
  <region indent=4>
    <selfld type=menu entwidth=1 selwidth=40>
      <choice checkvar=xtest1 match=a>Select Command
        <action run=tstch1 type=cmd parm='1234'
                newappl=aaaa passlib newpool suspend
                lang=crex nocheck mode=fscr>
      <choice checkvar=xtest1 match=b>Select Panel
        <action run=tstch2 type=panel
                addpop newappl=aaaa passlib newpool suspend>
      <choice checkvar=xtest1 match=c>Select Program
        <action run=tstch3 type=pgm parm=abcd
                newappl=aaaa passlib newpool suspend
                nocheck mode=fscr>
      <choice checkvar=xtest1 match=x>Exit
        <action run=exit type=exit>
    </selfld>
  </region>
Defining selection fields
Chapter 5. Application panel fields  83

## Page 116

<cmdarea>
</panel>
The resulting panel is:
                             Sample Option Menu
 Enter a selection choice
     1 Select Command
     2 Select Panel
     3 Select Program
     4 Exit
Option ===> _____________________________________________________________
Figure 35. Sample option menu
Model-choice fields
Use a model-choice selection field to create an ISPF edit model selection menu. Model-choice fields
are similar to single-choice or menu-choice fields. That is, the user can select only one of the choices
presented. The entry field for this type of selection field is the command line, which is formatted with
the word Option instead of Command. As with single-choice or menu-choice selections, you can specify a
preselected choice so that one item is already selected when the panel is displayed.
The CHOICE tag is followed by an ACTION tag which specifies the type of selection (PANEL, PGM, CMD, or
EXIT), and other attributes required by the ISPF SELECT service.
When creating an edit model menu, the MENU keyword is required on the PANEL tag. The SELFLD tag
must specify TYPE=MODEL. Depending on the panel being created, the SELFLD tag attributes ENTWIDTH,
FCHOICE, and TRAIL, and the CHOICE tag attributes SELCHAR, HIDEX, and TRUNC might be required.
See Chapter 12, “Tag reference,” on page 179 for more information about the PANEL, SELFLD, CHOICE,
and ACTION tags.
Tutor-choice fields
Use a tutor-choice selection field to create an ISPF tutorial selection menu. Tutor-choice fields are similar
to menu-choice fields. That is, the user can select only one of the choices presented. The entry field
for this type of selection field is the command line, which is formatted with the word Option instead of
Command. As with menu-choice selections, you can specify a preselected choice so that one item is
already selected when the panel is displayed.
The CHOICE tag is followed by an ACTION tag that must specify the type of selection as PANEL, and other
attributes required by the ISPF SELECT service.
When creating a tutorial menu, the MENU keyword is required on the PANEL tag. The SELFLD tag must
specify TYPE=TUTOR. Depending on the panel being created, the SELFLD tag attributes ENTWIDTH and
FCHOICE, and the CHOICE tag attribute SELCHAR might be required. See Chapter 12, “Tag reference,” on
page 179 for more information on the PANEL, SELFLD, CHOICE, and ACTION tags.
Defining selection fields
84  z/OS: z/OS ISPF DTL Guide

## Page 117

Selection field help
ISPF enables you to provide help on selection fields. For single-choice selection fields, you specify the
name of a help panel or message for the selection field with the HELP attribute of the SELFLD tag. For
multiple-choice selection fields, you specify the name of a help panel or message for each of the choices
in the selection field with the HELP attribute of the CHOICE tags. For menu-choice, model-choice, or
tutor-choice fields, the selection field is the command line. The name of the help panel or message must
be provided on the CMDAREA tag. If you specify help for a single-choice selection field, a menu-choice
selection field, or for choices in a multiple-choice selection field, ISPF displays that help information
when the user requests help and the cursor is on that panel element. If there is no help defined, the
extended help panel is displayed.
Here is an example that shows how to code a help panel for a single-choice selection field:
  <selfld name=choice
help=dayhelp>Weekdays:
    <choice checkvar=day match=M>Monday
    <choice checkvar=day match=T>Tuesday
    <choice checkvar=day match=W>Wednesday
    <choice checkvar=day match=H>Thursday
    <choice checkvar=day match=F>Friday
  </selfld>
This example shows how to code help panels for choices in a multiple-choice selection field.
  <selfld type=multi>Choose the services needed:
    <choice name=ch1 help=dryhlp>Dry haircut
    <choice name=ch2 help=cuthlp>Shampoo, haircut, and style
    <choice name=ch3 help=permhlp>Permanent or body wave
    <choice name=ch4 help=facehlp>Facial
    <choice name=ch5 help=manihlp>Manicure
    <choice name=ch6 help=pedihlp>Pedicure
  </selfld>
Selection width
The SELWIDTH attribute of the SELFLD tag should be used to define the amount of space taken up by
the choice-description-text of each CHOICE tag. This attribute is used to control the formatting of panels
defined with horizontal regions. If you do not specify a SELWIDTH value, the conversion utility reserves
the remaining available formatting width for the text.
When specifying an explicit SELWIDTH value, you must take into consideration the components of
the selection field, as well as the choice-description-text. The conversion utility reserves a number of
positions on the lines that selection field choices appear on for the entry fields, 3270 attributes, and,
in the case of single-choice, menu-choice, model-choice, and tutor-choice selection fields, the choice
prefixes. See the SELWIDTH attribute in “SELFLD (Selection Field)” on page 421 for a discussion of the
amount of space reserved for each choice type.
These reserved positions must be added to the length of the choice-description-text in the SELWIDTH
value you specify. Here is an example of markup that contains two selection fields, one single-choice and
one multiple-choice, within a horizontal region. To format the selection fields properly, ensure that the
SELWIDTH values you specify are adequate for the reserved positions and the choice-description-text. The
largest choice-description-text in the first selection field is 9 characters, which, when combined with the
10 reserved positions in the field, means you must specify a SELWIDTH value of at least 19. The largest
choice-description-text in the second selection field is 27 characters, which, when combined with the 5
reserved positions in the field, means you must specify a SELWIDTH value of at least 32.
<!doctype dm system>
<varclass name=char1 type='char 1'>
<varclass name=char2 type='char 2'>
<varlist>
  <vardcl name=person varclass=char2>
  <vardcl name=ch1 varclass=char1>
  <vardcl name=ch2 varclass=char1>
  <vardcl name=ch3 varclass=char1>
  <vardcl name=ch4 varclass=char1>
Defining selection fields
Chapter 5. Application panel fields  85

## Page 118

<vardcl name=ch5 varclass=char1>
  <vardcl name=ch6 varclass=char1>
</varlist>
<panel name=servsel>Service Selections
  <topinst>Select the stylist and services you want, then press Enter.
  <area>
    <region dir=horiz>
    <selfld name=person selwidth=19 pmtwidth=15>Stylist
      <choice checkvar=stylst match=1>Cecilia
      <choice checkvar=stylst match=2>Dana
      <choice checkvar=stylst match=3>Laurel
      <choice checkvar=stylst match=4>Pierce
      <choice checkvar=stylst match=5>Stephenie
    </selfld>
    <divider>
    <selfld type=multi selwidth=32 pmtwidth=15>Services
      <choice name=ch1 checkvar=dry>Dry haircut
      <choice name=ch2 checkvar=cut>Shampoo, haircut, and style
      <choice name=ch3 checkvar=per>Permanent or body wave
      <choice name=ch4 checkvar=fac>Facial
      <choice name=ch5 checkvar=man>Manicure
      <choice name=ch6 checkvar=ped>Pedicure
    </selfld>
    </region>
  </area>
</panel>
Here is the formatted result:
                             Service Selections
 Select the stylist and services you want, then press Enter.
 Stylist               Services
 __  1.  Cecilia       _  Dry haircut
     2.  Dana          _  Shampoo, haircut, and style
     3.  Laurel        _  Permanent or body wave
     4.  Pierce        _  Facial
     5.  Stephenie     _  Manicure
                       _  Pedicure
 
Figure 36. Selection field  SELWIDTH attribute
Other selection field attributes
There are several other attributes you can specify to tailor a selection field to meet the requirements of
your application. See “SELFLD (Selection Field)” on page 421 for more information. Here is a list that
describes each of the remaining SELFLD attributes and what you can do with them:
ENTWIDTH
This attribute controls the entry width for single-choice, menu-choice, model-choice, and tutor-choice
selections.
REQUIRED
This attribute allows you to indicate if the single-choice selection field requires input. When you
assign a value of YES to this attribute, the user must enter data into the field before ISPF accepts the
panel as valid. The default REQUIRED value is NO.
Defining selection fields
86  z/OS: z/OS ISPF DTL Guide

## Page 119

MSG
This attribute identifies the message that should be displayed when the user does not enter any data
into the selection field. If you do not specify this attribute, ISPF displays a default message. This
attribute is valid only if REQUIRED=YES.
Chapter 7, “Messages,” on page 137 tells you how to define application messages.
FCHOICE
This attribute controls the first choice number for single-choice, menu-choice, model-choice, and
tutor-choice selections. The value can be either 0 or 1.
AUTOTAB
This attribute provides automatic cursor movement between fields. If you specify AUTOTAB=YES for
a selection field, the cursor automatically moves to the next field that is capable of input. If no other
field capable of input exists on the panel, the cursor returns to the selection field.
DEPTH
This attribute specifies that the selection list is to be formatted as a scrollable area. A list formatted
into multiple columns (using CHOICECOLS) is formatted as multiple scrollable areas.
EXTEND
This attribute is valid only when DEPTH has been specified and specifies that the scrollable area is to
be expanded at run time to the size of the logical screen.
TRAIL
This attribute is used with menu-choice selections to specify the name of one or more variables that
applications use to obtain TRAIL information created by option menu selection processing.
CHOICECOLS
This attribute is used to specify the number of columns to create for the selection list. When
multiple columns are requested, the number of choices placed in each column is obtained from the
CHOICEDEPTH attribute.
CHOICEDEPTH
This attribute specifies the number of choices to be formatted into each column of choices. If more
choice entries are specified than can be formatted in the available number of columns specified by
the CHOICECOLS attribute, the remaining choice entries are placed in the rightmost (or only) available
column for the current SELFLD tag.
CWIDTHS
This attribute specifies the number of bytes to be allocated for each column of CHOICE entries. The
‘w1 w2...wn’ notation provides the number of bytes for each column. You may use an asterisk or a
number combined with an asterisk to specify a proportional allocation of column space.
PAD
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”.
PADC
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”
OUTLINE
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”.
SELMSG
This attribute specifies the message that is displayed when an invalid single-choice entry is selected.
SELMSGU
This attribute specifies the message that is displayed when an unavailable single-choice entry is
selected.
INIT
This attribute controls the single-choice and multiple-choice selection field variables initialization in
the panel )INIT section.
Defining selection fields
Chapter 5. Application panel fields  87

## Page 120

VERIFY
This attribute controls the single-choice verification and menu-choice, model-choice, or tutor-choice
selection logic generation in the panel )PROC section.
REFRESH
This attribute controls the creation of the REFRESH statement in the )REINIT section for multi-choice
selection variables.
SELFMT
This attribute controls the placement of the choice selection character(s) within the width specified by
ENTWIDTH.
CHKBOX
This attribute enables the display of multiple-choice fields as check boxes by a client that is using the
JSON API.
ZGUI
This attribute controls the creation of the VGET (ZGUI) statement created as part of the )INIT section
for multiple-choice selection definitions using the "&multipmt" built-in ENTITY.
CSRGRP
This attribute, in combination with CHKBOX=YES, provides a cursor group identification for multi-
choice selections.
TSIZE
This attribute provides the number of bytes to indent multiple lines of CHOICE text.
LISTTYPE
This attribute controls the display of single-choice selection lists.
LISTREF
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
LISTDEPTH
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
DBALIGN
This attribute, used for DBCS fields when PMTLOC=ABOVE, specifies alignment of the prompt text
with the selection input field.
NOSEL
This attribute provides a value to be placed in the CHECKVAR variable (specified by the CHOICE tag),
when no selection is made from a single-choice selection list.
SELDEFAULT
This attribute specifies a default choice selection for a single-choice selection list.
PMTSKIP
This attribute, used during horizontal field formatting, specifies that the cursor should move past the
prompt text to the input field.
FLDTYPE
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
COLOR
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
INTENS
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
HILITE
This attribute is accepted in order to support existing DTL source files that use it. However, it no longer
affects the displayed panel.
Defining selection fields
88  z/OS: z/OS ISPF DTL Guide

## Page 121

SELCHECK
This attribute is used with menu-choice selection to specify that panel logic be included in selection
processing to check for selection choices that are not valid.
Data columns
The DTACOL (data column) tag can be used to define values for data fields and selection fields that
are coded within the data column. If you have a group of data fields and selection fields on the same
application panel, the DTACOL tag is a convenient short-cut for ensuring alignment of the fields.
The DTACOL tag has these attributes:
PMTWIDTH
Applies to data fields and selection fields
ENTWIDTH
Applies to data fields only
DESWIDTH
Applies to data fields only
SELWIDTH
Applies to selection fields only
FLDSPACE
Applies to data fields only
PAD
Applies to data fields only
PADC
Applies to data fields only
OUTLINE
Applies to data fields only
PMTFMT
Applies to data fields only
AUTOTAB
Applies to data fields only
ATTRCHANGE
Applies to data fields only
PMTLOC
Applies to data fields only
DBALIGN
Applies to data fields only
VARCLASS
Applies to data fields only
REQUIRED
Applies to data fields only
CAPS
Applies to data fields only
These attributes serve the same purposes in DTACOL definitions as they do in CHOFLD, DTAFLD, and
SELFLD definitions. The only difference is that when you use them with a DTACOL tag, they define those
values for all of the data fields and selection fields coded between the DTACOL start and end tags.
Here is an example of markup that uses a data column to define a prompt width, entry width, and
description width for the data fields and the selection field coded within the data column. Because we
Data columns
Chapter 5. Application panel fields  89

## Page 122

want to limit the entry width of the State and Zip code fields, we defined ENTWIDTH values in the
DTAFLD definitions for these fields that override the DTACOL ENTWIDTH value.
<!doctype dm system>
<varclass name=sampcls type ='char 30'>
<varclass name=statcls type ='char 2'>
<varclass name=zipcls  type ='char 5'>
<varclass name=char1cls  type ='char 1'>
<varlist>
  <vardcl name=name varclass=sampcls>
  <vardcl name=addr varclass=sampcls>
  <vardcl name=city varclass=sampcls>
  <vardcl name=stat varclass=statcls>
  <vardcl name=day  varclass=char1cls>
  <vardcl name=zipc varclass=zipcls>
 </varlist>
<panel name=dcolxmp>Schedule Appointments
  <topinst>Enter your name and address and
  choose the most convenient day for your appointment.
  <area>
    <dtacol pmtwidth=12 entwidth=30 deswidth=29 selwidth=30>
      <dtafld datavar=name>Name
        <dtafldd>Last, First, M.I.
      <dtafld datavar=addr>Address
        <dtafldd>If it applies, include apartment number
      <dtafld datavar=city>City
      <dtafld datavar=stat entwidth=2>State
        <dtafldd>Use 2-character abbreviation
      <dtafld datavar=zipc entwidth=5>Zip code
      <divider type=solid gutter=3>
      <selfld name=day pmtloc=before>Weekdays
        <choice>Monday
        <choice>Tuesday
        <choice>Wednesday
        <choice>Thursday
        <choice>Friday
      </selfld>
    </dtacol>
  </area>
</panel>
Here is how the panel formats:
                           Schedule Appointments
 Enter your name and address and choose the most convenient day for your
 appointment.
 Name . . . . ______________________________  Last, First, M.I.
 Address  . . ______________________________  If it applies, include
                                              apartment number
 City . . . . ______________________________
 State  . . . __  Use 2-character abbreviation
 Zip code . . _____
 ---------------------------------------------------------------------------
 Weekdays . . __  1.  Monday
                  2.  Tuesday
                  3.  Wednesday
                  4.  Thursday
                  5.  Friday
 
Figure 37. Data column
Data columns
90  z/OS: z/OS ISPF DTL Guide

## Page 123

Defining list fields
A list field is used to display ISPF table data in column format, and to allow the user to enter data in the
column rows. The list field supports vertical scrolling if all of the data in the list field is not visible.
If you define a list field in a panel, the ISPF application program must use the TBDISPL service to display
the panel.
The tags you use to define a list field are:
LSTFLD
To define the list field. A matching end tag is required.
LSTGRP
To define column group headings. A matching end tag is required.
LSTCOL
To define a column within a list field. You code a LSTCOL tag for each column of data in the list field.
LSTVAR
To define a variable model line.
A list field can contain one or more columns of data, where each column can be input-only, output-only, or
input/output, as defined by the USAGE attribute on the LSTCOL tag. These are the values you can specify
on the USAGE attribute:
IN
Defines an input-only list column. An input-only column is underscore-filled when it is initially
displayed, unless the data is right-justified, and the user can enter data into any of the rows in the
input column.
OUT
Defines an output-only list column. When the panel is initially displayed, output-only columns display
the value of the ISPF table variable associated with the list column. The user cannot interact with an
output-only list column.
BOTH
Defines an input/output list column. Input/output list columns display the value of the ISPF table
variable associated with the list column when the panel is initially displayed, as well as allowing
the user to enter data into any of the rows in the column. BOTH is the default value for the USAGE
attribute.
The data that is associated with each list column is specified on the DATAVAR attribute of the LSTCOL tag.
Like all variables used on the panel, the data variable should be declared using the VARDCL tag.
The conversion utility builds a model section into the converted application panel. The model section
begins with a )MODEL header statement, which includes the variables named by the DATAVAR attributes
of each of the LSTCOL tags defined within the LSTFLD.
Application panels defined using the LSTFLD tag must be displayed using the ISPF TBDISPL service.
You can specify the optional ROWS=SCAN attribute on the LSTFLD tag to indicate that only those rows
meeting the criteria established by a previous TBSARG service are to be displayed.
You can define a column heading for any of the list columns in the list field by specifying the column
heading text as the tag text on the LSTCOL tag. You can specify the optional DIV attribute on the LSTFLD
tag to create a divider line between the display of table rows. The column headings do not scroll when the
list field is scrolled.
A scroll amount field can be placed at the right end of the command line by specifying the SCROLLVAR
attribute on the LSTFLD tag. Field level help for the SCROLLVAR field is specified using the SCRVHELP
attribute. The scroll amount field is displayed in uppercase characters when the SCRCAPS=ON attribute is
specified.
This panel shows a list field with six columns. The first column is output-only, and the remaining columns
are input/output.
Defining list fields
Chapter 5. Application panel fields  91

## Page 124

Scheduling Account Visits          ROW 1 to 9 of 9
 Enter the account name in the appropriate time slot.
                Monday      Tuesday     Wednesday   Thursday    Friday
 08:00 - 08:59  _________   _________   _________   _________   _________
 09:00 - 09:59  _________   _________   _________   _________   _________
 10:00 - 10:59  _________   Simmons     _________   _________   _________
 11:00 - 11:59  _________   _________   _________   _________   _________
 12:00 - 12:59  _________   _________   Douglass    Campbell    _________
 01:00 - 01:59  _________   _________   _________   _________   _________
 02:00 - 02:59  _________   _________   _________   _________   _________
 03:00 - 03:59  _________   _________   _________   _________   _________
 04:00 - 04:59  _________   _________   _________   _________   _________
 ***************************** Bottom of data *****************************
 Command ===>                                             Scroll ===> CSR 
  F1=Help     F2=Split    F3=Exit     F9=Swap    F12=Cancel
Figure 38. List field 
Here is the markup we used to create the panel:
<!doctype dm system>
<varclass name=timecls type='char 13'>
<varclass name=vc1     type ='char 9'>
<varlist>
  <vardcl name=timecol varclass=timecls>
  <vardcl name=moncol  varclass=vc1>
  <vardcl name=tuecol  varclass=vc1>
  <vardcl name=wedcol  varclass=vc1>
  <vardcl name=thrcol  varclass=vc1>
  <vardcl name=fricol  varclass=vc1>
</varlist>
<panel name=lstfld2>Scheduling Account Visits
 <topinst>Enter the account name in the appropriate time slot.
 <area>
  <lstfld scrollvar=scrlamt scrvhelp=scrhelp>
    <lstcol datavar=timecol usage=out colwidth=13>
    <lstcol datavar=moncol colwidth=9>Monday
    <lstcol datavar=tuecol colwidth=9>Tuesday
    <lstcol datavar=wedcol colwidth=9>Wednesday
    <lstcol datavar=thrcol colwidth=9>Thursday
    <lstcol datavar=fricol colwidth=9>Friday
  </LSTFLD>
</area>
<cmdarea>
</panel>
List group headings
You can define additional headings for the columns in a list field using the LSTGRP (list group) tag and its
matching end tag. You can define a list group for a single list column or for multiple list columns. You nest
the list columns you want to provide additional heading text for within the LSTGRP definition.
At least one field from the first line of the model set must be included within a LSTGRP definition.
The HEADLINE attribute of the LSTGRP tag allows you to place dashes in the list group heading. This is
handy for list groups that span across several list columns. Specify HEADLINE=YES to produce a dashed
list group heading.
Defining list fields
92  z/OS: z/OS ISPF DTL Guide

## Page 125

The ALIGN attribute of the LSTGRP tag allows you to control the format position of the list group heading.
The default value is CENTER. The heading can be left- or right-justified by specifying the values START or
END, respectively.
Here is an example where a LSTGRP definition is added to the list field shown in Figure 38 on page 92.
                         Scheduling Account Visits          ROW 1 to 9 of 9
 Enter the account name in the appropriate time slot.
                --------------------- Appointments ----------------------
                Monday      Tuesday     Wednesday   Thursday    Friday
 08:00 - 08:59  _________   _________   _________   _________   _________
 09:00 - 09:59  _________   _________   _________   _________   _________
 10:00 - 10:59  _________   Simmons     _________   _________   _________
 11:00 - 11:59  _________   _________   _________   _________   _________
 12:00 - 12:59  _________   _________   Douglass    Campbell    _________
 01:00 - 01:59  _________   _________   _________   _________   _________
 02:00 - 02:59  _________   _________   _________   _________   _________
 03:00 - 03:59  _________   _________   _________   _________   _________
 04:00 - 04:59  _________   _________   _________   _________   _________
 ***************************** Bottom of data *****************************
 Command ===>                                             Scroll ===> CSR 
  F1=Help     F2=Split    F3=Exit     F9=Swap    F12=Cancel
Figure 39. List group
The text of the list group, Appointments is centered within the dashes. Here is how we coded the list
group:
<!doctype dm system>
<varclass name=timecls type='char 13'>
<varclass name=vc1     type ='char 9'>
<varlist>
  <vardcl name=timecol varclass=timecls>
  <vardcl name=moncol  varclass=vc1>
  <vardcl name=tuecol  varclass=vc1>
  <vardcl name=wedcol  varclass=vc1>
  <vardcl name=thrcol  varclass=vc1>
  <vardcl name=fricol  varclass=vc1>
</varlist>
<panel name=lstgrp2>Scheduling Account Visits
 <topinst>Enter the account name in the appropriate time slot.
 <area>
  <lstfld scrollvar=scrlamt scrvhelp=scrhelp>
    <lstcol datavar=timecol usage=out colwidth=13>
    <lstgrp headline=yes>Appointments
      <lstcol datavar=moncol colwidth=9>Monday
      <lstcol datavar=tuecol colwidth=9>Tuesday
      <lstcol datavar=wedcol colwidth=9>Wednesday
      <lstcol datavar=thrcol colwidth=9>Thursday
      <lstcol datavar=fricol colwidth=9>Friday
    </lstgrp>
  </lstfld>
</area>
<cmdarea>
</panel>
Defining list fields
Chapter 5. Application panel fields  93

## Page 126

List column width
You can use the COLWIDTH attribute of the LSTCOL tag to determine the data width to be used by the
column. If you do not specify this attribute, the data width and column formatting width are determined
by the actual length of the column-heading. If the width of the column-heading text is greater than the
COLWIDTH, it is used as the column formatting width.
The minimum width value is 1 and the maximum is the remaining available panel (or region) width. If
the column-heading and the COLWIDTH attribute are omitted, the data width and column formatting
width are determined by the TYPE value of the associated VARCLASS. If a VARCLASS TYPE value is not
available, the size of the column variable name (specified by the DATAVAR attribute) determines the
width.
You should code the COLWIDTH attribute with a value equal to the length of the table data variable.
Other list column attributes
There are several other attributes that can be used in the LSTCOL tag. Many of these attributes are the
same as attributes on the DTAFLD tag. This list describes these LSTCOL attributes and how they are used:
ALIGN
This attribute aligns the variable data within the list column. The default value for ALIGN is start,
which aligns the data from the left side of the column. You can also center the data within the
column with the center value, or align the data to the right side of the column with the end value.
The attribute value end is useful for right-aligning numbers within an output-only column, because
numbers are typically right-aligned.
ATTRCHANGE
This attribute specifies that, if required, an additional )ATTR section entry (which can apply to multiple
fields) be created instead of a unique ".ATTR" override entry for the current field.
AUTOTAB
This attribute specifies automatic tabbing. If you assign a value of YES to this attribute, the cursor
automatically moves to the next field that is capable of user input when the user enters the last
character in the current list column. The default value for AUTOTAB is NO. This attribute is only valid
for list columns defined as input-only or as input/output.
CAPS
This attribute specifies whether the data column is displayed in uppercase characters.
CLEAR
This attribute specifies that the column is a table extension variable, which should be cleared before
the row is displayed. Column names with the CLEAR attribute are identified by the CLEAR keyword on
the )MODEL statement.
COLOR
When COLTYPE=ISPF, this attribute specifies the color for the column.
COLSPACE
The COLSPACE attribute specifies the total number of bytes for the column width, including the
leading and trailing attributes, and the trailing blank for input fields. The use of the COLSPACE
attribute causes column heading text longer than the COLSPACE value to be flowed into multiple lines.
COLTYPE
The COLTYPE attribute specifies the attribute type to be used for the column.
CSRGRP
This attribute, in combination with the PAS attribute, enables the use of a cursor group by a client that
is using the JSON API.
DISPLAY
This attribute specifies whether the data column is visible when the panel is displayed.
Defining list fields
94  z/OS: z/OS ISPF DTL Guide

## Page 127

FORMAT
This attribute specifies how the data column and its column heading are formatted. If you do not
specify this attribute, or if you specify the attribute value START, then the column formats as in ISPF
Version 3.1 and ISPF Version 3.2.
HELP
This attribute specifies the help panel name to display when the user requests help on the list column.
HILITE
When COLTYPE=ISPF, this attribute specifies the highlighting for the column.
INTENS
When COLTYPE=ISPF, this attribute specifies the intensity for the column.
LINE
This attribute specifies the model line that contains the variable. You can specify lines 1-8.
MSG
This attribute identifies the message that should be displayed when the user does not enter any
data into an input-required list column. If you do not specify this attribute, ISPF displays a default
message. This attribute is valid only if REQUIRED=YES. Chapter 7, “Messages,” on page 137 tells you
how to define application messages.
NOENDATTR
This attribute specifies that no ending attribute character is placed after the data column.
NOENDATTR is ignored for the last data column on each model line. See “LSTCOL (List Column)”
on page 332 for more information about the NOENDATTR attribute.
OUTLINE
This attribute provides for displaying lines around the field on a DBCS terminal. You can define this
attribute as a variable name preceded by a “%”. See “LSTCOL (List Column)” on page 332 for more
information about the OUTLINE attribute.
PAD
This attribute specifies the pad character for initializing the field. You can define this attribute as a
variable name preceded by a “%”. See “LSTCOL (List Column)” on page 332 for more information
about the PAD attribute.
PADC
This attribute specifies the conditional padding character to be used for initializing the field. You can
define this attribute as a variable name preceded by a “%”. See “LSTCOL (List Column)” on page 332
for more information about the PADC attribute.
PAS
This attribute is used to control the generation of the point-and-shoot indicator for table display
panels. You can define this attribute as a variable name preceded by a “%”.
POSITION
This attribute allows you to specify the starting position of the data column. The POSITION value must
be greater than the end of the last formatted data column for that model line and less than the right
panel margin. Column formatting for adding the data column and text takes place after the starting
position has been established. See “LSTCOL (List Column)” on page 332 for more information.
REQUIRED
This attribute indicates if this column is required to have input for any modified row. For input-
required columns (REQUIRED=YES), ISPF does not validate the panel unless the user has entered
data into that column. If you do not specify this attribute, input is not required on the list column. This
attribute is only valid for list columns defined as input-only or as input/output.
TEXT
This attribute specifies a short description of the data column. Text can be placed before or after the
data column. See “LSTCOL (List Column)” on page 332 for more information.
TEXTLOC
This attribute specifies the location of the TEXT relative to the data column. Text can be placed on
either side of the data column. See “LSTCOL (List Column)” on page 332 for more information.
Defining list fields
Chapter 5. Application panel fields  95

## Page 128

TEXTFMT
This attribute specifies the format of the text within the length of the text area. The text can be left-
justified, centered, or right-justified. See “LSTCOL (List Column)” on page 332 for more information.
TEXTLEN
This attribute specifies the amount of space to reserve for formatting the descriptive text. This helps
you line up text on different model lines, and if the space reserved is longer than the descriptive text,
TEXTLEN permits formatting within the reserved space with the TEXTFMT attribute. See “LSTCOL (List
Column)” on page 332 for more information.
TEXTSKIP
This attribute specifies the cursor should move past the text to the next input field.
VARCLASS
This attribute allows you to override the variable class that is specified on the variable declaration
(VARDCL) for the list column's data variable (DATAVAR). See Chapter 4, “Variables and variable
classes,” on page 53 for a description of variables and variable classes.
Defining group headings
The Group Header (GRPHDR) tag defines a group heading in the panel )BODY section.
The FORMAT attribute is used to control the type of text formatting. You can choose formatting similar to
the LINES tag or the P tag. For example, if FORMAT=NONE, the text formats as if you used a LINES tag.
However, if FORMAT=START, CENTER, or END, the text flows to multiple lines and is formatted at the right,
center or left part of the space reserved for the group heading.
Here is a short description of the other available attributes:
WIDTH
This attribute specifies the number of columns reserved for the group heading. The default value is
the remaining panel width.
FMTWIDTH
This attribute specifies the number of columns (of the WIDTH value) to use for formatting the group
heading. The default is the WIDTH value. By specifying a FMTWIDTH that is less than the WIDTH
value, the group heading text can be formatted on multiple lines.
INDENT
This attribute specifies the number of bytes that the group heading is to be indented.
HEADLINE
This attribute specifies whether dashes are added to span the width of the group heading not
occupied by text.
DIV
This attribute specifies the type of divider line to be placed before and after the group heading text.
DIVLOC
This attribute specifies whether the divider is to be added before the group heading, after the group
heading, or both before and after the group heading.
COMPACT
This attribute causes the group heading to format without a blank line before the group heading.
STRIP
This attribute causes leading and trailing blanks to be removed from the group heading text.
Defining point-and-shoot fields
The Point-and-Shoot (PS) tag is used to identify a portion of panel )BODY section text to be used for
point-and-shoot selection. When a point-and-shoot selection is made, a variable is set to a specified value
before normal )PROC section processing. The PS tag attributes identify the variable name and the value
associated with each point-and-shoot selection.
The PS tag requires a matching end tag to indicate the end of the point-and-shoot text.
Defining point-and-shoot fields
96  z/OS: z/OS ISPF DTL Guide

## Page 129

See the z/OS ISPF Dialog Developer's Guide and Reference for more information about point-and-shoot
selection.
Defining scrollable fields
A scrollable field can be used when the size of the field defined on the panel is smaller than the amount
of data to be displayed. With the cursor placed in the field, the LEFT and RIGHT commands can be used
to scroll the data displayed. In addition, the EXPAND command can be used to display the data in a popup
window.
With DTL, fields that can be made scrollable are defined using the DTAFLD or LSTCOL tags. A field is made
scrollable by nesting a SCRFLD tag in the DTAFLD or LSTCOL tag. Here are the attributes of the SCRFLD
tag that allow you to specify dialog variables to contain scroll indicators. The conversion utility generates
output fields on the panel to allow the scroll indicators to be displayed along with the scrollable field:
INDVAR
A 2-byte left and right scroll indicator that shows whether left and right scrolling can be performed.
LINDVAR
A 1-byte left scroll indicator that shows whether left scrolling can be performed.
RINDVAR
A 1-byte right scroll indicator that shows whether right scrolling can be performed.
SINDVAR
A separator scroll indicator that shows the length of the scrollable field and whether left and right
scrolling can be performed.
LCOLIND
A left column position indicator that shows the position of the character currently displayed in the
leftmost byte of the scrollable field.
RCOLIND
A right column position indicator that shows the position of the character currently displayed in the
rightmost byte of the scrollable field.
SCALE
A scale indicator showing the positions of the columns currently displayed in the scrollable field.
Here is the markup used for the Data Columns example (see Figure 37 on page 90), modified to display
the Name and Address fields as scrollable fields. The Name field is displayed with a separator scroll
indicator and the Address field is displayed with a scale indicator. The conversion utility automatically
generates the separator scroll indicator below the Name field and the scale indicator below the Address
field.
<!doctype dm system>
<varclass name=sampcls type ='char 30'>
<varclass name=statcls type ='char 2'>
<varclass name=zipcls  type ='char 5'>
<varclass name=char1cls  type ='char 1'>
<varlist>
  <vardcl name=name varclass=sampcls>
  <vardcl name=addr varclass=sampcls>
  <vardcl name=city varclass=sampcls>
  <vardcl name=stat varclass=statcls>
  <vardcl name=day  varclass=char1cls>
  <vardcl name=zipc varclass=zipcls>
 </varlist>
<panel name=scr1xmp depth=24>Schedule Appointments
  <topinst>Enter your name and address and
  choose the most convenient day for your appointment.
  <area>
    <dtacol pmtwidth=12 entwidth=30 deswidth=29 selwidth=30>
      <dtafld datavar=name>Name
        <dtafldd>Last, First, M.I.
        <scrfld displen=50 sindvar=namesi>
      <dtafld datavar=addr>Address
        <scrfld displen=80 scale=addrsi>
      <dtafld datavar=city>City
Defining scrollable fields
Chapter 5. Application panel fields  97

## Page 130

<dtafld datavar=stat entwidth=2>State
        <dtafldd>Use 2-character abbreviation
      <dtafld datavar=zipc entwidth=5>Zip code
      <divider type=solid gutter=3>
      <selfld name=day pmtloc=before>Weekdays
        <choice>Monday
        <choice>Tuesday
        <choice>Wednesday
        <choice>Thursday
        <choice>Friday
      </selfld>
    </dtacol>
  </area>
</panel>
 
This is how the panel displays:
                           Schedule Appointments
 Enter your name and address and choose the most convenient day for your
 appointment.
 Name . . . . Veryveryverylongsurname, Alexa  Last, First, M.I.
              ----------------------------->
 Address  . . Apartment 52b, 446 Verylongstr
              ----+----1----+----2----+----3
 City . . . .                               
 State  . . .     Use 2-character abbreviation
 Zip code . .      
 ---------------------------------------------------------------------------
 Weekdays . .     1.  Monday
                  2.  Tuesday
                  3.  Wednesday
                  4.  Thursday
                  5.  Friday
 Command ===>                                             Scroll ===> CSR 
  F1=Help     F2=Split    F3=Exit     F9=Swap    F12=Cancel
Figure 40. Scrollable field 
When the scrollable field is defined using the LSTCOL tag the conversion utility automatically generates,
along with the column heading, output fields for any scroll indicators you specify. Here is the markup used
for the List Group Headings example (see Figure 39 on page 93), modified to display the Appointment
data in scrollable fields. This would allow more information than just the account name to be stored and
displayed in the Appointment data. A scale indicator is displayed with the heading for each day's column.
<!doctype dm system>
<varclass name=timecls type='char 13'>
<varclass name=vc1     type ='char 9'>
<varlist>
  <vardcl name=timecol varclass=timecls>
  <vardcl name=moncol  varclass=vc1>
  <vardcl name=tuecol  varclass=vc1>
  <vardcl name=wedcol  varclass=vc1>
  <vardcl name=thrcol  varclass=vc1>
  <vardcl name=fricol  varclass=vc1>
</varlist>
<panel name=scrxmp2>Scheduling Account Visits
 <topinst>Enter the appointment details in the appropriate time slot.
 <area>
  <lstfld scrollvar=scrlamt scrvhelp=scrhelp>
    <lstcol datavar=timecol usage=out colwidth=13>
Defining scrollable fields
98  z/OS: z/OS ISPF DTL Guide

## Page 131

<lstgrp headline=yes>Appointments
      <lstcol datavar=moncol colwidth=9>Monday
        <scrfld displen=30 scale=monscl>
      <lstcol datavar=tuecol colwidth=9>Tuesday
        <scrfld displen=30 scale=tuescl>
      <lstcol datavar=wedcol colwidth=9>Wednesday
        <scrfld displen=30 scale=wedscl>
      <lstcol datavar=thrcol colwidth=9>Thursday
        <scrfld displen=30 scale=thrscl>
      <lstcol datavar=fricol colwidth=9>Friday
        <scrfld displen=30 scale=friscl>
    </lstgrp>
  </lstfld>
</area>
<cmdarea>
</panel>
 
This is how the panel displays:
                         Scheduling Account Visits          ROW 1 to 9 of 9
 Enter the account name in the appropriate time slot.
                --------------------- Appointments ----------------------
                Monday      Tuesday     Wednesday   Thursday    Friday
                ----+----   ----+----   ----+----   ----+----   ----+----
 08:00 - 08:59                                                           
 09:00 - 09:59                                                           
 10:00 - 10:59              Hart - Pl                                    
 11:00 - 11:59                                                           
 12:00 - 12:59                          Wife - lu                        
 01:00 - 01:59              XYZ - rev               ABC - upd            
 02:00 - 02:59                                                           
 03:00 - 03:59                                                           
 04:00 - 04:59                                                  Rod - ten
 ***************************** Bottom of data *****************************
 Command ===>                                             Scroll ===> CSR 
  F1=Help     F2=Split    F3=Exit     F9=Swap    F12=Cancel
Figure 41. Scrollable field  within a list column
Defining scrollable fields
Chapter 5. Application panel fields  99

## Page 132

Defining scrollable fields
100  z/OS: z/OS ISPF DTL Guide
