# Chapter 4. Variables and variable classes

Source file: f54dt00_v3r1.md
Start page: 85
Page span: 85-102

## Page 85

Chapter 4. Variables and variable classes
Much of the information displayed within dialog elements is derived directly from the tags used to define
it. Other information is obtained dynamically when the application is running, such as:
• Data that the user supplies
• Data that the application supplies
• Data that ISPF supplies.
In all of these cases, the data is derived from values specified in variables.
DTL provides you with tags to declare variables and to define the characteristics of these variables using
variable classes. Variables and variable classes are considered global because they can be referred to by
more than one element within the same source file. All variables referred to by dialog elements should
be declared. Variable names and variable classes should be used consistently throughout dialog elements
that are used in the same application.
Variables declared using DTL are accessible to your application through the dialog variable pools and
variable services provided by ISPF. Within ISPF display processing, all variable values are in character
format. ISPF transforms display variables between their dialog program format and internal display
processing character format when retrieving and storing variable values.
Note: Although the conversion utility processes all of the variable information provided in your DTL source
file and issues suppressible warning messages for missing VARDCL tags during the processing of several
other tags, such as DTAFLD and LSTCOL, ISPF does not require any of the tags described in the chapter to
generate a valid ISPF panel.
The conversion utility supports the SOURCE tag as an alternative means of placing variable processing
and validation statements directly into the ISPF panel.
Declaring variables
You declare variables for dialog elements by coding variable declarations in a variable list and specifying
the variable class associated with each declared variable.
The variable list (VARLIST) tag and its required end tag define the variable list. You code the variable list
after any variable classes and before any other tags.
To declare variables, use VARDCL (variable declaration) tags within the VARLIST definition. The VARDCL
tag has two required attributes, NAME and VARCLASS.
NAME
NAME specifies the variable used within the DTL source file.
For example, a data field definition includes a variable name in the DATAVAR attribute to specify the
variable that receives data when the user enters data in the field.
VARCLASS
VARCLASS specifies the variable class associated with the variable declaration. Variable classes
define the format and length of variable data plus translations and checks to perform on the data.
Here is an example where the variable list contains two variable declarations, referred to by the data
fields in the application panel:
<!doctype dm system>
<varclass name=authorc type='char 40'>
<varclass name=catnumc type='char 10'>
<varlist>
  <vardcl name=author varclass=authorc>
  <vardcl name=catnum varclass=catnumc>
</varlist>
Declaring variables
© Copyright IBM Corp. 1989, 2024 53

## Page 86

<panel name=books1>Book Title Search
  <area>
    <dtacol pmtwidth=20>
      <dtafld entwidth=40 datavar=author>Author
      <dtafld entwidth=10 datavar=catnum>Catalog number
    </dtacol>
  </area>
</panel>
Note: The ISPF Dialog Tag Language conversion utility does not require that you code the VARCLASS,
VARDCL, or VARLIST tags for a successful generation of a panel, command table, or message member
that includes variables. If the conversion utility finds a variable that does not have an associated VARDCL
definition, it issues a suppressible warning message.
The use of the VARCLASS, VARDCL, and VARLIST tags is required if you want to use the facilities provided
by the CHECKL and XLATL tags.
Defining variable classes
To complete the preceding example, we must code the variable classes that are referred to with the
VARDCL VARCLASS attributes. The variable class information must be defined if the conversion utility
is to generate )INIT and )PROC section statements for variable translations and validations. (“Variable
validation” on page 56 tells you how to define translations and validity checks.)
The VARCLASS (variable class) tag defines a variable class. You include variable classes in the same
source file as the dialog elements and variable list that refer to them. Additionally, you must code variable
classes in the source file before the variable list and dialog element definitions. You do this by coding
variable classes following the DOCTYPE statement or by coding this information in an external file and
embedding the file following the DOCTYPE statement.
There are two required attributes associated with the VARCLASS tag: NAME and TYPE.
NAME
NAME is used to identify and refer to the variable class.
TYPE
TYPE defines the format and entry-field width for variable data.
In addition to these required attributes, the VARCLASS tag has an optional MSG attribute. This attribute
specifies the message to be displayed if the variable fails any defined validity checks and no message
is defined for the XLATL or CHECKL tags. Chapter 7, “Messages,” on page 137 tells you how to define
messages.
Variable class types
DTL supports character variables and numeric variables. In addition, the conversion utility uses the
length specified in the TYPE attribute value of the VARCLASS tag to determine the entry width of fields
associated with the VARCLASS tag if this width is not defined with the tag used to create the field. For
more information about defining field entry widths, see Chapter 5, “Application panel fields,” on page
71.
Character variables
You can specify whether single-byte characters, double-byte characters, or mixed double-byte and
single-byte characters are permitted, as well as the maximum number of bytes the variable can accept.
Here is a description of each type:
Type
Description
'CHAR maximum-length'
Specifies a single-byte character string.
Defining variable classes
54  z/OS: z/OS ISPF DTL Guide

## Page 87

'DBCS maximum-length'
Specifies a double-byte character string. The maximum length must be an even number.
'MIXED maximum-length'
Specifies a character string containing single-byte characters, double-byte characters, or both.
Note: This type is treated as CHAR if the system does not support double-byte characters.
'ANY maximum-length'
Specifies a character string containing single-byte characters, double-byte characters, or both. It is
processed by the conversion utility as MIXED.
'EBCDIC maximum-length'
Specifies a single-byte character string.
'%varname maximum-length'
Specifies that a variable name is used for TYPE in the Panel definition. The application must ensure a
valid type is set before the panel is displayed.
'VMASK maximum-length'
A VEDIT statement is added to the generated panel. The ‘maximum-length’ is the default length for
associated variables. The application must use the VMASK service with a user-specified mask value.
'ITIME'
A VEDIT statement is added to the generated panel for associated variables. The default length for the
variables is 5. The application must use the VMASK service with a mask of ITIME.
'STDTIME'
A VEDIT statement is added to the generated panel for associated variables. The default length for the
variables is 8. The application must use the VMASK service with a mask of STDTIME.
'IDATE'
A VEDIT statement is added to the generated panel for associated variables. The default length for the
variables is 8. The application must use the VMASK service with a mask of IDATE.
'STDDATE'
A VEDIT statement is added to the generated panel for associated variables. The default length for the
variables is 10. The application must use the VMASK service with a mask of STDDATE.
'JDATE'
A VEDIT statement is added to the generated panel for associated variables. The default length for the
variables is 6. The application must use the VMASK service with a mask of JDATE.
'JSTD'
A VEDIT statement is added to the generated panel for associated variables. The default length for the
variables is 8. The application must use the VMASK service with a mask of JSTD.
We add the variable classes for authorc and catnumc to the markup example found in “Declaring
variables” on page 53. We assume an author's last name has a maximum of 40 characters, and a catalog
number is 10 characters.
<!doctype dm system>
<varclass name=authorc type='char 40'>
<varclass name=catnumc type='char 10'>
<varlist>
  <vardcl name=author varclass=authorc>
  <vardcl name=catnum varclass=catnumc>
</varlist>
<panel name=books2>Book Title Search
  <area>
    <dtacol pmtwidth=20>
      <dtafld entwidth=40 datavar=author>Author
      <dtafld entwidth=10 datavar=catnum>Catalog number
    </dtacol>
  </area>
</panel>
Variable class types
Chapter 4. Variables and variable classes  55

## Page 88

Numeric variables
You can use the NUMERIC type to ensure that a valid number is entered in the associated field. You can
specify the total number of digits (up to 16) allowed in the number and the number of fractional digits
allowed. The conversion utility generates a VER(variable ENUM) statement for input validation.
Here is an example of a variable class (pricevar) where the data entered in the associated field has a
maximum number of five digits, two of which are fractional:
<varclass name=pricevar
type='numeric 5 2'>
If you do not specify an entry width with the tag that defines the associated field, the conversion utility
calculates an entry width for the field based on the NUMERIC value and allow for a sign, thousands
separators, and a decimal point.
Variable validation
DTL allows you to define translate lists and validity checks as part of the variable class definition by
using tags nested within the VARCLASS tag. These built-in translations and checks are especially useful
because ISPF automatically performs them on variable values, so the dialog application does not need to.
Note: Translations and checks are performed only on variable values that are intended for display. For
instance, before displaying the data from a variable specified on the DATAVAR attribute of a DTAFLD
tag, ISPF performs any specified translations on the variable retrieved from the application to construct
the correct display value. However, ISPF does not perform translations on a variable specified as the
CHECKVAR attribute of a CHOICE tag.
Translate lists
Translate lists provide a means of translating a displayed variable value into a different dialog variable
pool value, and vice versa. Translation can occur on input (when the user enters a value), on output (the
value stored in the variable pool is translated before the user sees it), or both. This is based on the USAGE
value of the tag that refers to a variable using a variable class with translate lists.
To associate a translate list with a variable class, code the XLATL (translate list) tag and its required end
tag within the VARCLASS definition.
The type of translation is determined by the value assigned to the FORMAT attribute of the XLATL tag. The
two types of translations supported are:
• Uppercase translation
• Item translation
There is an optional MSG attribute on the XLATL tag that allows you to specify your own message to
display when input translation specified by the XLATL does not result in a match. For information about
defining your own messages, see Chapter 7, “Messages,” on page 137.
Upper
Allows you to translate a value to uppercase. To specify this translation, code FORMAT=UPPER on the
XLATL tag. This translation is always successful.
We'll add a translate list to the authorc variable class in the example under “Numeric variables” on page
56. The translate list converts the author's name to uppercase.
<varclass name=authorc type='CHAR 40' msg=liba001>
  <xlatl format=upper>
  </xlatl>
This figure shows the results on input and output translations for the previous example:
Variable validation
56  z/OS: z/OS ISPF DTL Guide

## Page 89

Figure 24. Variable translation results
Item translation
Allows you to translate an internal variable value to a displayed value (or vice-versa) on an item-for-item
basis. To specify this translation, either code FORMAT=NONE on the XLATL tag or omit the FORMAT
attribute because this is the default. You define the list of possible internal values and the corresponding
display values they should be translated to, or from, using the XLATI (translate item) tags nested within
the XLATL tag.
To specify an internal value (the value in the variable pool) for a translate item, use the VALUE attribute on
the XLATI tag. The XLATI tag text specifies what the user sees (for output) and enters (for input).
The display value is the XLATI tag text. If a display value of all blanks or a display value in which leading,
trailing, or embedded blanks are preserved is desired, use the literal (LIT) tag and its required end tag to
indicate that blanks are significant.
An explicit match is achieved during translation processing as follows:
• On input, an explicit match occurs when the value the user enters matches one of the specified display
values in the translate list. An explicit match also occurs when a display value is omitted (indicating any
value is acceptable) and the corresponding internal value is specified.
• On output, an explicit match occurs when the value from the variable pool matches one of the specified
internal values in the translate list. An explicit match also occurs when an internal value is omitted
(indicating any value is acceptable) and the corresponding display value is specified.
Omitting both the internal value and the display value does not produce an explicit match. This case is
discussed further on in this topic.
Translate list processing is case-sensitive. To ensure that a match results when the user enters the
correct display value but in a different or mixed case, code an uppercase conversion translate list before
the value translate list.
Here is an example where the variable class dayc uses an internal value for days of the week that is
different from the display value. The comparisons are on uppercase values, because FORMAT=UPPER is
provided before the item translation list.
<!doctype dm system>
<varclass name=dayc type='CHAR 9'>
  <xlatl format=upper>
  </xlatl>
  <xlatl msg=liba004>
    <xlati value=1>SUNDAY
    <xlati value=2>MONDAY
    <xlati value=3>TUESDAY
    <xlati value=4>WEDNESDAY
    <xlati value=5>THURSDAY
    <xlati value=6>FRIDAY
    <xlati value=7>SATURDAY
  </xlatl>
This figure shows how variable values of variable class dayc are translated on input and output.
Variable validation
Chapter 4. Variables and variable classes  57

## Page 90

Figure 25. Variable translation
The previous example shows one translate list with a finite number of translation items. This example
assumes that the only possible internal values are 1-7 and the only possible display values are the days of
the week. For input fields, a match must be found in this list, or the translation fails and message liba004
is displayed to the user.
Here is an example which allows allow a nonmatching value to be passed on for further processing (either
to another translate list or to the validity checks that follow) by coding an XLATI tag without an internal
value or a display value, to indicate that any value is acceptable:
<!doctype dm system>
<varclass name=dayc type='CHAR 9'>
  <xlatl format=upper>
  </xlatl>
  <xlatl>
    <xlati value=1>SUNDAY
    <xlati value=2>MONDAY
    <xlati value=3>TUESDAY
    <xlati value=4>WEDNESDAY
    <xlati value=5>THURSDAY
    <xlati value=6>FRIDAY
    <xlati value=7>SATURDAY
    <xlati>
  </xlatl>
Because multiple translate lists are permitted, we can expand this example to accept either the days of
the week spelled out or their accepted abbreviations. Because the last XLATI tag in the first translate
list has no internal or displayed value, the input value are passed on for further translate list or validity
checking.
<!doctype dm system>
<varclass name=dayc type='CHAR 9'>
  <xlatl format=upper>
  </xlatl>
  <xlatl>
    <xlati value=1>SUNDAY
    <xlati value=2>MONDAY
    <xlati value=3>TUESDAY
    <xlati value=4>WEDNESDAY
    <xlati value=5>THURSDAY
    <xlati value=6>FRIDAY
    <xlati value=7>SATURDAY
    <xlati>
  </xlatl>
  <xlatl>
    <xlati value=1>SUN
    <xlati value=2>MON
    <xlati value=3>TUES
    <xlati value=4>WED
    <xlati value=5>THUR
    <xlati value=6>FRI
    <xlati value=7>SAT
  </xlatl>
It is possible to omit only the internal value to indicate that any internal value is acceptable. This affects
input and output translate processing differently. When translating on input, the value is not translated
Variable validation
58  z/OS: z/OS ISPF DTL Guide

## Page 91

before being stored in the variable pool. When translating on output, any value not already matched is
translated to the displayed value.
In the following example, the branchc variable class illustrates translate processing when only the
internal value is omitted.
<!doctype dm system>
<varclass name=branchc type='CHAR 3'>
  <xlatl format=upper>
  </xlatl>
  <xlatl>
    <xlati value=1>RAL
    <xlati>CRY
  </xlatl>
Figure 26. Variable translation
It is also possible to omit only the display value to indicate that any display value is acceptable.
This affects input and output translate processing differently. When translating on input, any value not
already matched is translated to the internal value. When translating on output, the internal value is not
translated before it is displayed.
Here is a similar example, but with the branchc variable class changed, to show translate processing
when only the display value is omitted:
<!doctype dm system>
<varclass name=branchc type='CHAR 3'>
  <xlatl format=upper>
  </xlatl>
  <xlatl>
    <xlati value=1>RAL
    <xlati value=2>
  </xlatl>
Figure 27. Variable translation
It is possible to specify that less than the full input value be entered by the use of the TRUNC attribute.
Output translation is not affected.
We'll change the branchc variable class again to illustrate:
Variable validation
Chapter 4. Variables and variable classes  59

## Page 92

<!doctype dm system>
     <varclass name=branchc type='CHAR 3'>
       <xlatl format=upper>
       </xlatl>
       <xlatl format=none trunc=1>
         <xlati value=1>RAL
         <XLATI VALUE=2>
       </xlatl>
Figure 28. Variable translation
Validity checks
You use validity checks to automatically verify data input by the user. Code validity checks after any
translate lists.
To associate a validity check with a variable class, code the CHECKL (check list) tag and its required end
tag either following the last translate list, or if no translate list exists, following the VARCLASS start tag.
The individual check item that defines the test to perform is coded using the CHECKI (check item) tag
nested within the check list. You can code one CHECKI tag in a CHECKL definition. However, you can code
multiple CHECKL tags within a variable class definition.
There is an optional MSG attribute on the CHECKL tag that allows you to specify your own message to
display when the entered value fails the test. If you do not specify a message, ISPF Dialog Manager
supplies a default message for you. For more information about defining your own messages, see Chapter
7, “Messages,” on page 137.
A value entered by the user must pass the check item test for the check list to be considered successful.
Furthermore, because there can be multiple check lists defined, all check lists must be successful for the
validation to be successful.
The TYPE attribute of the CHECKI tag allows you to specify the various validity tests of the input. You can
define these types of validity check:
• “RANGE” on page 61
• “ALPHA” on page 61
• “VALUES” on page 62
• “VALUESX” on page 62
• “CHARS” on page 62 (limited to characters for HEX, BIT, and NUM tests)
• “FILEID” on page 63
• “DSNAME” on page 63
• “DSNAMEF” on page 63
• “DSNAMEFM” on page 63
• “DSNAMEPQ” on page 63
• “DSNAMEQ” on page 64
• “NAME” on page 64
Variable validation
60  z/OS: z/OS ISPF DTL Guide

## Page 93

• “NAMEF” on page 64
• “DBCS” on page 64
• “EBCDIC” on page 65
• “MIX” on page 65
• “ALPHAB” on page 65
• “PICT” on page 65
• “PICTCN” on page 66
• “LISTV” on page 66
• “LISTVX” on page 66
• “LEN” on page 67
• “ENUM” on page 67
• “BIT” on page 67
• “NUM” on page 67
• “HEX” on page 68
• “INCLUDE” on page 68
• “IDATE” on page 68
• “STDDATE” on page 68
• “JDATE” on page 69
• “JSTD” on page 69
• “ITIME” on page 69
• “STDTIME” on page 69
• “IPADDR4” on page 69
RANGE
To perform a range test, specify the check item TYPE attribute as RANGE. A range check allows you to
check a value within a numeric range including the end points. The PARM1 attribute specifies the lower
bound; PARM2 specifies the upper bound. The range delimiters can include 16 digits, and may contain a
preceding sign (- or +).
Here is an example where a range check for a NUMERIC variable class ensures that catalog numbers are
in the range 50 to 90000000:
<!doctype dm system>
<varclass name=catnumc type='NUMERIC 8'>
  <checkl msg=liba005>
    <checki type=range parm1=50 parm2=90000000>
  </checkl>
The conversion utility generates an ISPF range verification statement in the )PROC section.
ALPHA
To perform an alphabetic test, specify the check item TYPE attribute as ALPHA. An alpha check limits the
characters allowed to A-Z, a-z, #, $, and @.
Here is an example where an alpha check in the authorc variable class ensures that authors' names are
alphabetic:
<!doctype dm system>
<varclass name=authorc type='CHAR 40'>
  <checkl msg=liba006>
Variable validation
Chapter 4. Variables and variable classes  61

## Page 94

<checki type=alpha>
  </checkl>
The conversion utility generates an ISPF alpha verification statement in the )PROC section.
VALUES
To perform a values test, specify the check item TYPE attribute as VALUES. A values test allows you to
specify a list of values. The value the user enters must match one of the values in the list. The PARM1
attribute must have the value EQ. The PARM2 attribute specifies the list of values. Because case is
respected in a VALUES check, if you want case to be ignored, you must code an UPPER translation and
code the values all in uppercase.
Here is an example where a check in a variable class named subject ensures that the value entered is
MATH, SCIENCE, ENGLISH, or HISTORY:
<!doctype dm system>
<varclass name=subject type='char 10'>
  <xlatl format=upper>
  </xlatl>
  <checkl msg=liba008>
     <checki type=values parm1=eq
     parm2='MATH SCIENCE ENGLISH HISTORY'>
  </checkl>
The conversion utility generates a LIST verification statement in the )PROC section.
VALUESX
The check item type VALUESX is the opposite of VALUES. This test allows you to specify a list of values
that are not valid. The PARM1 attribute must have the value NE. The PARM2 attribute specifies the list of
values that are not valid. The value the user enters cannot match any of the values specified in the list.
Because case is respected in a VALUESX check, if you want case to be ignored, you must code an UPPER
translation and code the values all in uppercase.
Here is an example where a check in a variable class named subject ensures that the value entered is not
MATH, SCIENCE, ENGLISH, or HISTORY:
  <!doctype dm system>
  <varclass name=subject type='char 10'>
    <xlatl format=upper>
    </xlatl>
    <checkl msg=liba008>
       <checki type=valuesx parm1=ne
       parm2='MATH SCIENCE ENGLISH HISTORY'>
    </checkl>
The conversion utility generates a LISTX verification statement in the )PROC section.
CHARS
The conversion utility supports BIT, HEX and NUM validation with TYPE=CHARS. The PARM1
attribute must have the value EQ. The PARM2 attribute value can be either "01" for BIT validation,
"0123456789ABCDEFabcdef" for HEX validation, or "0123456789" for NUM validation.
Here is an example where a check list in the hexc variable class validates hexadecimal values:
<!doctype dm system>
<varclass name=hexc type='char 2'>
  <checkl msg=liba008>
     <checki type=chars parm1=eq parm2='0123456789ABCDEFabcdef'>
  </checkl>
The conversion utility generates an ISPF hex verification statement in the )PROC section.
Variable validation
62  z/OS: z/OS ISPF DTL Guide

## Page 95

FILEID
To perform a FILEID test, specify the check item TYPE attribute as FILEID.
Here is an example where a FILEID check in the infile  variable class ensures that specified variables
contain a valid file ID in CMS syntax:
<!doctype dm system>
<varclass name=infile type='CHAR 20'>
  <checkl msg=liba010>
     <checki type=fileid>
  </checkl>
The conversion utility generates an ISPF FILEID verification statement in the )PROC section.
DSNAME
To perform a DSNAME test, specify the check item TYPE attribute as DSNAME.
Here is an example where a DSNAME check in the namefile  variable class ensures that the specified
variables contain a valid TSO file name:
<!doctype dm system>
<varclass name=namefile type='CHAR 44'>
  <checkl msg=liba011>
     <checki type=dsname>
  </checkl>
The conversion utility generates a DSNAME verification statement in the )PROC section.
DSNAMEF
To perform a DSNAMEF test, specify the check item TYPE attribute as DSNAMEF.
Here is an example where a DSNAMEF check in the namefile  variable class ensures that the specified
variables contain a valid TSO file name:
<!doctype dm system>
<varclass name=namefile type='CHAR 44'>
  <checkl msg=liba011>
     <checki type=dsnamef>
  </checkl>
The conversion utility generates a DSNAMEF verification statement in the )PROC section.
DSNAMEFM
To perform a DSNAMEFM test, specify the check item TYPE attribute as DSNAMEFM.
Here is an example where a DSNAMEFM check in the namefile  variable class ensures that the specified
variables contain a valid TSO file name:
<!doctype dm system>
<varclass name=namefile type='CHAR 44'>
  <checkl msg=liba011>
     <checki type=dsnamefm>
  </checkl>
The conversion utility generates a DSNAMEFM verification statement in the )PROC section.
DSNAMEPQ
To perform a DSNAMEPQ test, specify the check item TYPE attribute as DSNAMEPQ.
Variable validation
Chapter 4. Variables and variable classes  63

## Page 96

Here is an example where a DSNAMEPQ check in the namefile  variable class ensures that the specified
variables contain a valid TSO file name:
<!doctype dm system>
<varclass name=namefile type='CHAR 44'>
  <checkl msg=liba011>
     <checki type=dsnamepq>
  </checkl>
The conversion utility generates a DSNAMEPQ verification statement in the )PROC section.
DSNAMEQ
To perform a DSNAMEQ test, specify the check item TYPE attribute as DSNAMEQ.
Here is an example where a DSNAMEQ check in the namefile  variable class ensures that the specified
variables contain a valid TSO file name:
<!doctype dm system>
<varclass name=namefile type='CHAR 44'>
  <checkl msg=liba011>
     <checki type=dsnameq>
  </checkl>
The conversion utility generates a DSNAMEQ verification statement in the )PROC section.
NAME
To perform a NAME test, specify the check item TYPE attribute as NAME.
Here is an example where a NAME check in the chapters variable class ensures that the variable contains
a valid name, obeying the rules of member names:
<!doctype dm system>
<varclass name=chapters type='CHAR 8'>
  <checkl msg=liba012>
     <checki type=name>
  </checkl>
The conversion utility generates a NAME verification statement in the )PROC section.
NAMEF
To perform a NAMEF test, specify the check item TYPE attribute as NAMEF.
Here is an example where a NAMEF check in the chapters variable class ensures that the variable contains
a valid name, obeying the rules of member names:
<!doctype dm system>
<varclass name=chapters type='CHAR 8'>
  <checkl msg=liba012>
     <checki type=namef>
  </checkl>
The conversion utility generates a NAMEF verification statement in the )PROC section.
DBCS
To perform a DBCS test, specify the check item TYPE attribute as DBCS.
Here is an example of a DBCS check in the dbdesc variable class. This ensures that specified variables
contain valid DBCS characters.
Variable validation
64  z/OS: z/OS ISPF DTL Guide

## Page 97

<!doctype dm system>
<varclass name=dbdesc type='DBCS 12'>
  <checkl msg=liba013>
     <checki type=dbcs>
  </checkl>
The conversion utility generates a DBCS verification statement in the )PROC section.
EBCDIC
To perform an EBCDIC test, specify the check item TYPE attribute as EBCDIC.
Here is an example where an EBCDIC check in the title variable class ensures that specified variables
contain valid EBCDIC characters:
<!doctype dm system>
<varclass name=title1 type='CHAR 40'>
  <checkl msg=liba014>
     <checki type=ebcdic>
  </checkl>
The conversion utility generates an EBCDIC verification statement in the )PROC section.
MIX
To perform a MIX test, specify the check item TYPE attribute as MIX.
Here is an example where a MIX check in the title2 variable class ensures that specified variables contain
valid DBCS and EBCDIC characters:
<!doctype dm system>
<varclass name=title2 type='CHAR 40'>
  <checkl msg=liba015>
     <checki type=mix>
  </checkl>
The conversion utility generates a MIX verification statement in the )PROC section.
ALPHAB
To perform an ALPHAB test, specify the check item TYPE attribute as ALPHAB. An ALPHAB check limits
the characters allowed to A-Z or a-z. Blanks are not allowed.
Here is an example where an ALPHAB check in the chapters variable class ensures that chapter names
are alphabetic:
<!doctype dm system>
<varclass name=chapters type='CHAR 8'>
  <checkl msg=liba016>
     <checki type=alphab>
  </checkl>
The conversion utility generates an ALPHAB verification statement in the )PROC section.
PICT
To perform a PICT check, specify the check item TYPE attribute as PICT. A PICT check allows you to
specify a pattern used to validate the variable. The PARM1 attribute must have the value EQ. The PARM2
attribute contains the validation character string.
Variable validation
Chapter 4. Variables and variable classes  65

## Page 98

Here is an example where a PICT check in the socsec variable class validates the format of a social
security number:
<!doctype dm system>
<varclass name=socsec type='CHAR 11'>
  <checkl msg=liba017>
     <checki type=pict parm1=eq parm2='nnn-nn-nnnn'>
  </checkl>
The conversion utility generates a PICT verification statement in the )PROC section.
PICTCN
To perform a PICTCN check, specify the check item TYPE attribute as PICTCN. A PICTCN check allows
you to specify a pattern containing required characters to validate the variable. The PARM1 attribute
contains a mask character. The PARM2 attribute contains the field-mask. The PARM3 attribute contains
the validation string.
Here is an example where a PICTCN check in the socsec variable class validates the format of a social
security number, including the hyphen (-) character in positions 4 and 7:
<!doctype dm system>
<varclass name=socsec type='CHAR 11'>
  <checkl msg=liba017>
     <checki type=pictcn parm1='*' parm2='***-**-****'
                  parm3='nnn-nn-nnnn'>
  </checkl>
The conversion utility generates a PICTCN verification statement in the )PROC section.
LISTV
To perform a LISTV check, specify the check item TYPE attribute as LISTV. A LISTV test allows you to
provide a variable name that has been defined by your application to contain a list of valid variable values.
The PARM1 attribute must have the value EQ. The PARM2 attribute must be a variable name entered with
"%" as the first character.
Here is an example where a LISTV check in the majors variable class validates major subjects, providing
the application has previously defined the listitem variable to contain the value “MATH SCIENCE ENGLISH
HISTORY”:
<!doctype dm system>
<varclass name=majors type='CHAR 8'>
  <checkl msg=liba018>
     <checki type=listv parm1=eq parm2=%listitem>
  </checkl>
The conversion utility generates a LISTV verification statement in the )PROC section.
LISTVX
The check item type LISTVX is the opposite of LISTV. A LISTVX test allows you to provide a variable name
that has been defined by your application to contain a list of variable values that are not valid. PARM1
attribute must have the value NE. The PARM2 attribute must be a variable name entered with "%" as the
first character.
Here is an example where a LISTVX check in the majors variable class validates major subjects, providing
the application has previously defined the listitem variable to contain the value “MATH SCIENCE ENGLISH
HISTORY”:
<!doctype dm system>
<varclass name=majors type='CHAR 8'>
  <checkl msg=liba018>
Variable validation
66  z/OS: z/OS ISPF DTL Guide

## Page 99

<checki type=listvx parm1=ne parm2=%listitem>
  </checkl>
The conversion utility generates a LISTVX verification statement in the )PROC section.
LEN
To perform a LEN check, specify the check item TYPE attribute as LEN. A LEN test allows you to validate
the length of the variable. The PARM1 attribute can be a relational operator or a variable name that
contains a relational operator. Valid relational operators are EQ, LT, GT, LE, GE, NE, NG, or NL. If a variable
name is used, it must be preceded by a “%”. The PARM2 value can be either a number or a variable name
that contains the number. If you enter a number, it must be in the range of 1-99999. If you use a variable
name, it must be preceded by a “%”.
Here is an example where a LEN check in the chapters variable class validates the length of chapter
names:
<!doctype dm system>
<varclass name=chapters type='CHAR 8'>
  <checkl msg=liba019>
     <checki type=len parm1=le parm2=8>
  </checkl>
The conversion utility generates a LEN verification statement in the )PROC section.
ENUM
To perform an ENUM check, specify the check item TYPE attribute as ENUM. An ENUM check allows you
to verify a variable as extended numeric. ISPF verifies variable values for correct decimal and comma
notation plus correct sign placement.
Here is an example where an ENUM check in the quantity variable class ensures that specified variables
are in correct extended numeric format:
<!doctype dm system>
<varclass name=quantity type='CHAR 10'>
  <checkl msg=liba020>
     <checki type=enum>
  </checkl>
The conversion utility generates an ENUM verification statement in the )PROC section.
BIT
To perform a BIT check, specify the check item TYPE as BIT. A BIT check allows you to verify that a
variable contains only 0’s and 1’s.
Here is an example where a BIT check in the choices variable class ensures that specified variables are in
BIT format:
<!doctype dm system>
<varclass name=choices type='CHAR 1'>
  <checkl msg=liba021>
     <checki type=bit>
  </checkl>
NUM
To perform a NUM check, specify the check item TYPE attribute as NUM. A NUM check allows you to verify
a variable as a numeric character 0-9.
Variable validation
Chapter 4. Variables and variable classes  67

## Page 100

Here is an example where a NUM check in the numbers variable class ensures that specified variables are
numeric:
<!doctype dm system>
<varclass name=numbers type='CHAR 5'>
  <checkl msg=liba022>
     <checki type=num>
  </checkl>
HEX
To perform a HEX check, specify the check item TYPE attribute as HEX. A HEX check allows you to specify
a variable that contains only hexadecimal characters (0-9, A-F).
Here is an example where a HEX check in the hexc variable class validates hexadecimal values:
<!doctype dm system>
<varclass name=hexc type='CHAR 2'>
  <checkl msg=liba008>
     <checki type=hex>
  </checkl>
INCLUDE
To perform an INCLUDE check, specify the TYPE attribute as INCLUDE, and, at a minimum, the PARM2
attribute as ALPHA, ALPHAB, or NUM. The PARM1 and PARM3 attributes are optional.
Here is an example where an INCLUDE check in the incl variable class allows an embedded blank and
validates the values for both the ALPHA and NUM characters:
<!doctype dm system>
<varclass name=incl type='CHAR 10'>
  <checkl msg=liba023>
     <checki type=include parm1=IMBLK parm2=ALPHA parm3=NUM>
  </checkl>
Note: See the z/OS ISPF Dialog Developer's Guide and Reference for more information about panel
variable verification.
IDATE
To perform an IDATE check, specify the TYPE attribute as IDATE. An IDATE check allows you to validate
an 8 character international date, including the national language date delimiter. The format for the United
States is YY/MM/DD.
This example validates an IDATE:
<!doctype dm system>
  <varclass name=idate type='CHAR 8'>
    <checkl msg=liba024>
     <checki type=idate>
    </checkl>
STDDATE
To perform an STDDATE check, specify the TYPE attribute as STDDATE. An STDDATE check allows you to
validate a 10 character standard date, including the national language date delimiter. The format for the
United States is YYYY/MM/DD.
This example validates an STDDATE:
<!doctype dm system>
Variable validation
68  z/OS: z/OS ISPF DTL Guide

## Page 101

<varclass name=stddate type='CHAR 10'>
    <checkl msg=liba025>
     <checki type=stddate>
    </checkl>
JDATE
To perform a JDATE check, specify the TYPE attribute as JDATE. A JDATE check allows you to validate a 6
character Julian date. The format is YY.DDD.
This example validates a JDATE:
<!doctype dm system>
  <varclass name=jdate type='CHAR 6'>
    <checkl msg=liba026>
     <checki type=jdate>
    </checkl>
JSTD
To perform a JSTD check, specify the TYPE attribute as JSTD. A JSTD check allows you to validate an 8
character Julian date. The format is YYYY.DDD.
This example validates a JSTD:
<!doctype dm system>
  <varclass name=jstd type='CHAR 8'>
    <checkl msg=liba026>
     <checki type=jstd>
    </checkl>
ITIME
To perform an ITIME check, specify the TYPE attribute as ITIME. An ITIME check allows you to validate
a 5 character international time, including the national language time delimiter. The format for the United
States is HH:MM.
This example validates an ITIME:
<!doctype dm system>
  <varclass name=itime type='CHAR 5'>
    <checkl msg=liba027>
     <checki type=itime>
    </checkl>
STDTIME
To perform a STDTIME check, specify the TYPE attribute as STDTIME. A STDTIME check allows you to
validate an 8 character standard time, including the national language time delimiter. The format for the
United States is HH:MM:SS.
This example validates a STDTIME:
<!doctype dm system>
  <varclass name=stdtime type='CHAR 8'>
    <checkl msg=liba028>
     <checki type=stdtime>
    </checkl>
IPADDR4
To perform a IPADDR4 check, specify the TYPE attribute as IPADDR4. A IPADDR4 check allows you to
verify a 15 character IP address of the format xxx.xxx.xxx.xxx.
Variable validation
Chapter 4. Variables and variable classes  69

## Page 102

This example validates an IPADDR4:
<!doctype dm system>
  <varclass name=ipaddr4 type='CHAR 15'>
    <checkl msg=liba029>
     <checki type=ipaddr4>
    </checkl>
Overriding variable classes
Some tags, such as DTAFLD, allow you to specify a different variable class for a variable other than the
default one that was specified when the variable was declared using the VARDCL tag. This is called
an overriding variable class and is used to perform different translates and validity checks from those
provided by the default variable class.
Variable validation
70  z/OS: z/OS ISPF DTL Guide
