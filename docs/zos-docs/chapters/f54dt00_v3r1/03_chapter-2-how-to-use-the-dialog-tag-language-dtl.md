# Chapter 2. How to use the Dialog Tag Language (DTL)

Source file: f54dt00_v3r1.md
Start page: 43
Page span: 43-58

## Page 43

Chapter 2. How to use the Dialog Tag Language (DTL)
This chapter describes the tag syntax conventions and mark-up declarations of the Dialog Tag Language
(DTL). It also explains how to use the DTL to create dialog element source files for your ISPF applications.
The markup style of DTL is based on the International Standards Organization (ISO) Standard Generalized
Markup Language (SGML). Markup languages allow you to specify, through the use of tags, how the text
of a file is to be formatted for use by an application. Because DTL is a markup language, you must follow
special rules and conventions when using it to define dialog elements.
Syntax conventions
The DTL tags act as control words that determine how the text in the source files is used. Each tag is
enclosed within a set of delimiter symbols that distinguish the tag as a control word (as opposed to
general text). Start tags, which initiate text interpretation, are preceded by the start tag open delimiter (<)
and followed by the close delimiter (>). End tags, which explicitly end text interpretation, are preceded by
the end tag open delimiter (</) and followed by the close delimiter (>).
For example, the DTL tags used to define the beginning and end of an application panel are the PANEL tag
and its matching end tag, which look like this:
<panel>
</panel>
DTL tags are free-form. Indentation of nested tags can be helpful for DTL source file readability.
All of the text that you define between a start and end tag is the tag definition . The DTL tag data extends
to the right boundary of the source file. Therefore, DTL source files cannot contain line sequence numbers.
The characteristics of the tag determine how the text or other tags coded within the tag definition is
formatted.
End tags are not required for all DTL tags. In many cases, the tag is implicitly ended by other start tags.
For this reason, optional end tags are not used in the markup examples in this information. Chapter 12,
“Tag reference,” on page 179 contains a detailed description of each DTL tag, and indicates when a tag
needs a corresponding end tag.
Attributes and values
Many DTL start tags contain attributes and values that define various physical and operating
characteristics of the dialog elements. While most attributes and values are optional, or contain default
settings, some are required.
For example, the PANEL tag has a required NAME attribute that must be specified to identify the panel.
The value you assign to the NAME attribute must be unique for each panel in a source file. This PANEL tag
has the NAME value "panel1":
<panel name=panel1>
</panel>
The PANEL tag also has two optional attributes, DEPTH and WIDTH, whose values specify the dimensions
of the panel. For these types of attributes, you specify a numeric value.
<panel name=panel1 depth=20 width=40>
</panel>
Values for some of the tag attributes are predetermined; that is, you can choose from one of a number of
keyword values for the tag. For example, the FIG (figure) tag has a FRAME attribute that specifies the top
and bottom borders of the figure. The value you assign to the FRAME attribute can be either RULE, which
Syntax conventions
© Copyright IBM Corp. 1989, 2024 11

## Page 44

produces a visible border above and below the figure, or NONE, which results in a figure without a border.
No other value is acceptable for the FRAME attribute.
RULE is the default value, which means that the figure formats with visible borders if you do not specify
the FRAME attribute.
The markup for a figure without ruled borders looks like this:
<fig frame=none>
</fig>
When coding attribute values you must use single or double quotes to enclose values that contain
characters other than A-Z, a-z, 0-9, a hyphen (-), or a period (.).
For example, the value assigned to the TYPE attribute of this VARCLASS tag contains a blank, so the value
must be enclosed in quotes:
<varclass name=boolean type='char 1'>
Some attributes can be assigned either a specific value, such as a number or a character string, or a
variable name. To distinguish a variable name from a specific value, precede the variable name with a
percent (%) sign. This convention is called % notation. The percent sign distinguishes the variable name
from a specific value. To specify a string that begins with a %, you must code an additional % before the
string to distinguish it from a variable name. (For example, to specify the string "%abc", code "%%abc").
Here is an example where the ACTION attribute uses % notation to specify a variable named "varname":
<cmdact action='%varname'>
The length of any attribute value is limited to 253 characters, unless stated otherwise. This includes the
lengths of any entity references that are a part of the value.
Generally, you can code tags, attributes, and values in uppercase, lowercase, or mixed case; the results
are always the same regardless of case. The conventions you must follow for case-sensitive processing
for each tag are described in Chapter 12, “Tag reference,” on page 179.
Tag text
The content or text of a tag is coded immediately following the start tag. This is the actual text that is
subject to formatting and translation. The text is processed according to the type of tag it follows.
For example, the text following this P (paragraph) tag is the actual text that appears in the panel after
formatting:
<p>The copy command allows you to copy single or multiple forms.
Because the tag text is processed according to the tag characteristics, not the way it is written in the
source file, the paragraph could also be marked up using more than one line, like this:
<p>
The copy command allows you to
copy single or multiple forms.
The formatted result is the same in either case.
In most cases, there is no limit to the amount of text you can code. However, keep in mind that the text
of some tags, such as the title of a PANEL tag, should be limited because of size constraints of the panel
they are coded within. Chapter 12, “Tag reference,” on page 179 describes text length restrictions (if they
exist) for each of the tags.
In most cases, multiple lines of text are concatenated. Concatenation, leading blanks, and trailing blanks
are processed in this way:
Syntax conventions
12  z/OS: z/OS ISPF DTL Guide

## Page 45

• Leading and trailing blanks between lines of text are not preserved. Instead, they are compressed to a
single blank when the lines are concatenated.
• The first line of tag text may start on the same line as the start tag, or on the next line. The formatted
result is the same.
The text of some tags, such as the FIG, LINES, and XMP tag, allow you to control where lines break. That
is, within the range of the tag, each output line is ended at the same point that you ended the input line.
With these tags, multiple lines are not concatenated, and all blanks are preserved.
Text formatting
ISPF determines if the text is to be formatted according to English rules or Asian rules, based on
the language specified on the conversion utility invocation. If the language is JAPANESE, CHINESET,
CHINESES, or KOREAN, ISPF uses the Japanese, Traditional Chinese, Simplified Chinese, or Korean text
formatting rules, respectively. If JAPANESE language is specified and the KANA option is also specified,
ISPF uses the Japanese Katakana formatting rules. Otherwise, the English formatting rules are used.
English rules for text formatting
Text exceeding the width of the available panel space is wrapped to the next line. The text is split at
blanks. However, if any word exceeds the panel space, then the word splits and continues on the next line.
Asian rules for text formatting
Some characters should not be placed at the beginning of a line, and some should not be placed at
the end of a line. These beginning-inhibited and ending-inhibited characters are different among the
languages but the required process is the same. Thus, ISPF uses the same text formatting process for
these Asian languages, but uses a different beginning-and-ending-inhibitor character table for each of the
languages.
The text is first split into words. An SBCS word is delimited by blanks, or SO/SI characters. Then any
beginning inhibitors are stripped from the beginning of the word and treated as separate words, and any
ending inhibitors are stripped from the end of the word and treated as separate words.
Adjoining DBCS alphanumeric characters (that is, Ward 42 characters) are treated as one DBCS word.
Then any beginning inhibitors are stripped from the beginning of the word and treated as separate words,
and any ending inhibitors are stripped from the end of the word and treated as separate words. All other
non-Ward 42 double-byte characters are treated as separate DBCS words.
If a word exceeds the available panel space, then the word splits and continues on the next line. If
the text consists of mixed data and does not fit in one line within the specified width, the first position
is always reserved for a SO character (if first word is double-byte) or for a blank (if the first word is
single-byte). This allows the text to be aligned properly.
Words that exceed the width of the available panel space are wrapped to the next line according to
following rules:
Figure 5. Text formatting rules
Syntax conventions
Chapter 2. How to use the Dialog Tag Language (DTL)  13

## Page 46

Where:
CE-1 and CE
Last two words that fit on line
CB and CB+1
First two words on next line
E
Ending inhibitor
B
Beginning inhibitor
X
Neither
Forward
Move CE to next line
Backward
Move CB to previous line
No process
Split as is
Note: If words CE or CB are single-byte words and are more than 1 character, or if CE or CB are
double-byte words and are more than 1 double-byte character, then no special processing is used; the
line is split as is.
When your panel contains several successive lines of mixed data from different tags, the alignment of
a short text string can appear to be shifted 1 byte further left than the surrounding text. This occurs
because a text string that fits on one line does not have the leading position reserved for the SO character
to use as many positions on the screen as possible.
You can control the alignment of successive lines of mixed data by adding a string of DBCS blanks to the
end of a short text string. This forces the SO character position to be reserved during formatting.
SBCS and DBCS blanks that end or begin a line are deleted.
Nesting tags
It is often necessary to code certain tags (and their text) within the definition of other tags (between the
start and end tags). This is called nesting.
A good example of nesting is the relationship between the DL (definition list) tag, the DT (definition term)
tag, and the DD (definition description) tag. The DL tag specifies a definition list and the DT and DD tags
specify the terms and descriptions of the items within the definition list. Consequently, the DT and DD
tags must be nested within a DL tag and its matching end tag if the list is to format properly.
Here is an example:
<dl>
  <dt>This is a definition term.
  <dd>This is a definition description.
  <dt>Another term.
  <dd>Another description.
</dl>
Note: Although it isn't required, we indented the nested tags in this example to illustrate nesting levels.
You can also do this in your own source files.
There are several tags that must be nested within the actual text of another start tag. These tags serve
to identify a condition for the text. In this example, the nested CMD tag follows the CMDTBL start tag
and precedes the CMDTBL end tag. The T (truncation) tag nested within the text of the CMD tag provides
truncation of the command text.
<CMDTBL APPLID=conv>
  <CMD NAME=delete>Del<T>ete
Syntax conventions
14  z/OS: z/OS ISPF DTL Guide

## Page 47

<CMDACT ACTION=setverb>
</CMDTBL>
Nesting tags can take on many different forms and can be complex. For example, some tags allow
multiple tags or multiple occurrences of the same tag to be nested, while other tags do not allow nesting
of any tags. You can also nest levels of certain tags, that is, nested tags within other nested tags.
Additionally, in many instances, you must nest certain tags within other tags. The tag descriptions in
Chapter 12, “Tag reference,” on page 179 describe the allowed and required conditions for nesting each
of the DTL tags.
Including comments in the generated panel or message member
You can use the COMMENT tag to add comments to the generated panel or message member file.
The TYPE attribute specifies the panel section for the comment. TYPE = END is automatically used for
message member processing. You provide the comment text in a manner similar to the paragraph tag.
ISPDTLC flows the text to a width of 66 bytes and adds "/* " before and " */" after each resulting comment
line.
Including Copyright Statements in the Generated Panel or Message Member
You can use the COPYR tag to add a copyright statement to the generated panel or message member.
The copyright statement is placed in the panel immediately following the )END panel section line, or
immediately following the last message in the message member. The text of the COPYR tag is limited to
66 bytes. ISPDTLC adds "/* " before and " */" after the copyright text. Each COPYR tag adds one line to the
generated panel.
Markup declarations
In addition to tag markup, you can also include markup declarations in your source files to define other,
related information. Markup declarations are control statements that specify how other markup (such as
tags) within a source file is to be interpreted.
For example, in order for the compiler to recognize your source files as being intended for DTL conversion
to ISPF elements, you must include a document type declaration at the beginning of each source file.
Like tags, markup declarations must be enclosed within a set of delimiter symbols so the compiler can
distinguish the declaration as a control statement. All markup declarations are preceded by the <! symbol
and followed by the > symbol.
Note: For multicultural support users of DTL the <! symbol can be replaced with the <: symbol.
DTL supports three types of markup declarations:
• Document type declarations
• Comments
• Entity declarations.
Declaring the document type
You must declare the document type before you can convert a source file that contains the tag markup
for dialog elements, Do this by coding the DOCTYPE declaration at the beginning of the source file. The
DOCTYPE declaration looks like this:
<!doctype dm system>
Where:
<!
Begins the markup declaration
DOCTYPE
Identifies the declaration as a document type declaration
Markup declarations
Chapter 2. How to use the Dialog Tag Language (DTL)  15

## Page 48

DM
Specifies that the source file contains tags used to define dialog elements for a Dialog Manager
application
SYSTEM
Indicates that the syntax rules for defining elements are contained in an external file
>
Closes the markup declaration.
External files that are embedded (through the use of entity declarations) within the source file intended
for conversion cannot contain a DOCTYPE declaration. They are converted using the DOCTYPE declaration
of the source file they are embedded within. For more information about entity declarations and
embedding external files within source files, see “Defining entities and parameter entities” on page 17.
Including comments in your markup
If you want to include notes, reminders, or other text that you don't want processed in your source files,
you can insert them as comments, and the conversion utility ignores them.
Note: You cannot place comments within any of the DTL tags. A comment placed within a start or end tag
causes the tag to end, and the text following the comment is treated as part of the tag content.
Like document type declarations, comments must be enclosed within markup declaration delimiters (<!
>). However, you must also delimit comments within markup declarations by preceding and following a
comment with two dashes (--), like this:
<!-- This is the text of the comment -->
Because the dashes act as comment delimiters, you can use them in any markup declaration. For
example, you can include a comment within a DOCTYPE declaration:
<!doctype dm system -- DECLARE DOCUMENT TYPE -->
Here is a comment that generates a warning message because the second set of dashes is interpreted
as the end of a comment and the text "Provides help for ordering" is treated as an additional markup
declaration:
<!-- Panel DMH022 -- Provides help for ordering -->
If you delete one of the dashes in the second set of dashes, or use another symbol, no error occurs.
<!-- Panel DMH022 - Provides help for ordering -->
This block comment produces a warning message because of the odd number of dashes in the first and
last lines of the block:
<!----------------------------------------->
<!--This source file contains all of the -->
<!-- help panels for the application     -->
<!----------------------------------------->
We could avoid this problem by using a different symbol between the comment dashes, like this:
<!--*************************************-->
<!--This source file contains all of the -->
<!-- help panels for the application     -->
<!--*************************************-->
ISPDTLC accepts comments which start with the 4 characters “<!--” and end with the 3 characters “-->”.
The minimum valid comment is 7 characters (“<!---->”).
You cannot nest comments within other comments. You can, however, code multiple comments within a
markup declaration, like this:
Markup declarations
16  z/OS: z/OS ISPF DTL Guide

## Page 49

<!-- Here a comment --
-- THERE A COMMENT --
-- Everywhere a comment, COMMENT-->
As you can see, each of the comments begin and end correctly with the comment delimiters.
You can use comment delimiters to temporarily ignore multiple lines (or a block) of DTL source text.
The block of text might include one or more DTL tags. To comment out a block of text, place an "open
comment" delimiter before the first line of the text, and a "close comment" delimiter after the last line of
text. For example:
<!--
<p> This is a multiple line of text block
<p> It is commented out for compile purposes
-->
When commenting out multiple lines of DTL source, use the MCOMMENT compiler option when coding the
ISPDTLC invocation syntax, or select the Process multiple line comment blocks option on the ISPDTLC
invocation panel.
Defining entities and parameter entities
You can define, or declare frequently used words, phrases, and longer character strings in your source
file as entities or parameter entities that represent text in the source file. You declare them within the
DOCTYPE statement of your source file. After you have declared them, you refer to the names of the
entities in place of the word or phrase in the text. This saves you time when marking up your text, and
allows you to globally change the defined words or phrases in one place in the source file.
You can use entities and parameter entities for these purposes:
• To replace single characters in text that are considered special characters. This can include characters
not available on a particular keyboard, or characters that have special meaning to the compiler, such as
the tag start delimiter (<), that you want to treat as normal text.
DTL provides you with a set of predefined single-character entities. See “Predefined entities” on page
23 for a list of these entities.
• To replace strings of text, such as words, phrases, and longer text strings used frequently in the source
file text.
• To embed entire files in a source file. This is useful for breaking up a source file into smaller, more
manageable files, and for declaring entities that are shared by different source files.
When you refer to an entity in the text of a source file, you must precede the entity reference with an
ampersand (&) and follow it with a semicolon (;) or a blank space. The text defined by the entity replaces
the entity reference in the formatted text.
Entities
Entities are symbolic statements that represent text strings in a source file. Like other markup
declarations, entity declarations must be enclosed within markup declaration delimiters (<! >). In
addition, you must place entity declarations within the declaration subset of the DOCTYPE statement.
The declaration subset is delimited by left and right brackets ([ ]) or parentheses () and is coded within the
DOCTYPE statement. If left and right brackets are coded, they must have the hex values of ‘AD’ and ‘BD’
respectively.
Within the markup declaration delimiters, you declare the entity with the term "entity", the name you
are assigning to the entity, and the text string the name represents. The text string of the entity must be
enclosed in single or double quotes.
<!doctype dm system (
<!entity name "text string">
)>
Markup declarations
Chapter 2. How to use the Dialog Tag Language (DTL)  17

## Page 50

Entity names must have these characteristics:
• 1-17 characters
• The first character must be alphabetic (A-Z, a-z, @, #, or $)
• Remaining characters, if any, can be A-Z, a-z, @, #, $, 0-9, or _
• Entity names are case-sensitive.
• Entity names of more than 8 bytes must contain at least 1 underscore character.
This example declares an entity named "guar" for the phrase "full, unconditional, money-back
guarantee":
<!doctype dm system [
<!entity guar "full, unconditional, money-back guarantee">
]>
Now that we've declared the entity, we can use the entity name in our source file text instead of the entire
text string. To specify an entity name in text, you must precede the name with an ampersand (&) and
follow it with a semicolon (;) or a blank, as we did in this panel text:
<!doctype dm system [
<!entity guar "full, unconditional, money-back guarantee">
]>
<panel name=widget21 width=40>Widgets
  <area>
    <info width=38>
      <p>You'll love the wide selection of merchandise
      in our Widgets department.
      <p>And, like all of our merchandise, Widgets come
      with our &guar;.
    </info>
  </area>
</panel>
As long as we declared the entity properly, the compiler recognizes the entity reference in the source file
and replaces it with the text of the entity declaration. Figure 6 on page 18 shows the result.
                Widgets
 You'll love the wide selection of
 merchandise in our Widgets department.
 And, like all of our merchandise,
 Widgets come with our full,
 unconditional, money-back guarantee.
 
Figure 6. Entity reference for text substitution
We can refer to the same entity in the text of the source file as many times as we like. If we should ever
want to change the text of the entity, we only have to do it in one place: the declaration subset.
A change to the previous example will show you what we mean.
<!doctype dm system [
<!entity guar “partial, conditional, non-refundable guarantee”>
]>
<panel name=widget22 width=40>Widgets
Markup declarations
18  z/OS: z/OS ISPF DTL Guide

## Page 51

<area>
    <info width=38>
      <p>You'll love the wide selection of merchandise
      in our Widgets department.
      <p>And, like all of our merchandise, Widgets come
      with our &guar;.
    </info>
  </area>
</panel>
The only change we made was to the text of the entity declaration, not the entity name. Following
reformatting, the text of the entity reference now looks like this:
                Widgets
 You'll love the wide selection of
 merchandise in our Widgets department.
 And, like all of our merchandise,
 Widgets come with our partial,
 conditional, non-refundable guarantee.
 
Figure 7. Entity reference for text substitution
If, for any reason you need to change the name of an entity, be sure to update all of the references to the
entity name in your text.
You can also define the text of an entity in an external file and refer to that file in an entity declaration.
If you do this, you must include the SYSTEM parameter in the entity declaration, to indicate to the
conversion utility that the file is external.
Note: You must include the external file in the concatenation of DTL source files defined to the conversion
utility.
For example, we'll define a text string we want to use as an entity in our source file in a file called
WIDGETS. Here are the contents of the WIDGETS file:
doohickeys, whatnots, and gizmos
To declare this file in the entity declaration in our source file, we code it like this, with the SYSTEM
parameter:
<!doctype dm system [
<!entity guar "full, unconditional, money-back guarantee">
<!entity widgets system>
]>
If we want to use the text string in our source file, we refer to the entity "widgets" (in this case, the file
name also serves as the entity name).
<!doctype dm system [
<!entity guar "full, unconditional, money-back guarantee">
<!entity widgets system>
]>
<panel name=widget23 width=42>More Widgets
  <area>
    <info width=40>
      <p>The fine selection of items in our Widgets department
Markup declarations
Chapter 2. How to use the Dialog Tag Language (DTL)  19

## Page 52

includes &widgets;.
      <p>And, like all of our merchandise, Widgets come with
      our &guar;.
    </info>
  </area>
</panel>
Figure 8 on page 20 shows the formatted result.
               More Widgets
 The fine selection of items in our
 Widgets department includes doohickeys,
 whatnots, and gizmos.
 And, like all of our merchandise,
 Widgets come with our full,
 unconditional, money-back guarantee.
 
Figure 8. Entity reference for text substitution and file  embedding
Anytime we want to update or change the text of the entity, we only need to change the text in the
WIDGETS file.
In the previous example, the name "widgets" serves as the external file name and as the entity name.
The SYSTEM parameter may optionally be followed by the file name for the included file. When the
SYSTEM parameter is used but no file name is provided, the entity name is used as the file name.
For instance, if you want to declare a different entity name for the WIDGETS file, "things" for example,
code it like this in the entity declaration:
<!doctype dm system [
<!entity guar “full, unconditional, money-back guarantee”
><!entity things system "widgets">
]>
Refer to the entity name, things, like this:
<!doctype dm system [
<!entity guar &“full, unconditional, money-back guarantee&”>
<!entity things system “widgets”>
]>
<panel name=widget24 width=42>More Widgets
  <area>
    <info width=40>
      <p>The fine selection of items in our Widgets department
      includes &things;.
      <p>And, like all of our merchandise, Widgets come with
      our &guar;.
    </info>
  </area>
</panel>
The formatted result of this markup is the same as that shown in Figure 8 on page 20, assuming no
changes were made to the text of the WIDGETS file.
Markup declarations
20  z/OS: z/OS ISPF DTL Guide

## Page 53

Parameter entities
Parameter entities allow you to place multiple entity declarations within an external file and refer to them
within a source file. To embed the entities into the source file, you must declare the external file as a
parameter entity. A parameter entity is identified by a percent symbol (%) following the term "entity"
and followed by a space and the entity name. See “Entity declarations” on page 172 for the syntax
description. You refer to a parameter entity within the DOCTYPE statement by preceding the entity name
with a percent symbol (%) and following it with a semicolon (;). This embeds the parameter entity file and
allows its entities to be referred to in the source file.
For example, we've declared all of our entities within an external file called SYMBOLS. Here are the
contents of the SYMBOLS file:
<!ENTITY sb "ShelfBrowse">
<!ENTITY cotime "ten days">
<!ENTITY xcotime "five days">
<!ENTITY nttime "three days">
<!ENTITY nitem "red checkout card">
<!ENTITY lfine "ten cents">
<!ENTITY cophone "555-1234">
The conversion utility locates the parameter entity using these rules for entity external files.
We can embed the SYMBOLS file into the declaration subset of the source file with a parameter entity
declaration within the DOCTYPE statement. As long as we declare the parameter entity and refer to it
properly, we can use any of the declared entities in the external file in the text of the source file.
<!doctype dm system
 [<!entity % SYMBOLS system> %SYMBOLS;]>
<panel name=chkout width=40 depth=22>Library Checkout Periods
  <area>
    <info width=38>
      <p>&sb; allows you to check out an inventory
      item for a maximum of &cotime;.
      However, you can renew the item for an additional
      &xcotime; by calling in your card number to our
      checkout phone line (&cophone;) any time of day.
      <p>If an inventory item is a new shelf item
      (indicated by the &nitem;), you may only reserve it for
      a maximum of &nttime;.
      You may not renew a new shelf item.
      <p>There is a fine of &lfine; per day for all
      items returned late.
    </info>
  </area>
</panel>
Figure 9 on page 21 shows the formatted result.
        Library Checkout Periods
 ShelfBrowse allows you to check out an
 inventory item for a maximum of ten
 days. However, you can renew the item
 for an additional five days by calling
 in your card number to our checkout
 phone line (555-1234) any time of day.
 If an inventory item is a new shelf
 item (indicated by the red checkout
 card), you may only reserve it for a
 maximum of three days. You may not
 renew a new shelf item.
 There is a fine of ten cents per day
 for all items returned late.
 
Figure 9. Parameter entities
Markup declarations
Chapter 2. How to use the Dialog Tag Language (DTL)  21

## Page 54

Parameter entity names must have these characteristics:
• 1-8 characters
• The first character must be alphabetic (A-Z, a-z, @, #, or $)
• Remaining characters, if any, must be A-Z, a-z, @, #, $, or 0-9
• Parameter entity names are case-sensitive.
Embedding source files
You can also use entities to embed entire files within your source file. For example, you could define
common variables for several panels in your source file in a separate file. These separate files are stored
as members of any input library specified to ISPDTLC. Here is markup that shows the contents of a file
called VARDEFS.
<varclass name=titlcls type='char 50'>
<varclass name=bookcls type='char 20'>
<varclass name=pagecls type='char 5'>
<varclass name=datecls type='char 8'>
<varlist>
  <vardcl name=title    varclass=titlcls>
  <vardcl name=author   varclass=bookcls>
  <vardcl name=publish  varclass=bookcls>
  <vardcl name=pages    varclass=pagecls>
  <vardcl name=curdate  varclass=datecls>
</varlist>
Another common markup file could be defined for an action bar. Here is markup that shows a portion of
the contents of a file called ACTNBAR.
<ab>
  <abc>File
    <pdc>Add Entry
      <action run=add>
    <pdc>Delete Entry
      <action run=delete>
    <pdc>Update Entry
      <action run=update>
    <pdc>Exit
      <action run=exit>
  <abc>View
⋮
  <abc>Options
⋮
  <abc>Help
⋮
</ab>
We can embed these files in a source file by coding entity references to the files in the source file
DOCTYPE statement.
<!doctype dm system [
<!entity actnbar system>
<!entity vardefs system>]>
&vardefs;
<panel name=dfdxmp21>Library Inventory
&actnbar;
  <topinst>To add a book to the inventory, complete the fields below,
           and then press Enter.
  <area>
    <dtafld datavar=title usage=in pmtwidth=14>Title
    <dtafld datavar=author usage=in pmtwidth=14>Author
    <dtafld datavar=publish pmtwidth=14>Publisher
    <dtafld datavar=pages usage=in pmtwidth=14>Number of pages
    <divider type=solid gutter=3>
    <dtafld datavar=curdate usage=out pmtwidth=20>Today's date is
  </area>
</panel>
Markup declarations
22  z/OS: z/OS ISPF DTL Guide

## Page 55

The variable definitions in VARDEFS are referred to by the data fields in the panel because the file was
embedded into the source file through the entity declaration. In the previous example, the entry width
information for each field is obtained from the variable definitions.
File embed entity names must have these characteristics:
• 1-8 characters
• The first character must be alphabetic (A-Z, a-z, @, #, or $)
• Remaining characters, if any, must be A-Z, a-z, @, #, $, or 0-9
• Entity names are case-sensitive.
Runtime substitution variables
If you need to include a dialog variable within your panel source that will be substituted at run time, the
output panel must be created to contain an "&variable" string. An example would be a reference to an
ISPF variable such as &ZDATE.
The conversion utility always tries to substitute each "&variable" found at conversion time with the
available entity definitions. If the conversion utility can't find an entity definition, it issues a warning
message, and then passes the original "&variable" into the output panel.
To avoid the warning message, you can use the predefined entity "&amp". You can code the variable in the
tag source as "&amp;variable" to make "&variable" appear in the panel.
You should use caution when designing panels that contain runtime substitution variables. The regular
panel formatting process might not leave sufficient space in the panel text line for the variable value to
be inserted. For example, a variable name of "&date" that requires 10 positions (YYYY/MM/DD) should be
coded as "&date(10);".
Predefined entities
The Dialog Tag Language provides you with a set of predefined entities that you can use in your source
files. You can use them when the symbol you want is not present on your keyboard, or conflicts with a
conversion utility delimiter symbol.
You do not need to declare a predefined entity to use it. If you use the entity in your source file as
you would an entity that you declare within your document subset, the conversion utility performs the
substitution for you. You should always use the pre-defined entities for all symbols that are used as part
of the tag language syntax.
The Dialog Tag Language predefined entities include:
&gtsym;
greater than (>)
&ltsym;
less than (<)
&colon;
colon (:)
&amp;
ampersand (&)
&semi;
semicolon (;)
&period;
period (.)
&quote;
single quote (')
&dquote;
double quote (")
Markup declarations
Chapter 2. How to use the Dialog Tag Language (DTL)  23

## Page 56

&ndash;
short dash (–)
&not;
not symbol (¬)
&us;
underscore (_)
&or;
logical or (|)
&sll;
back slash (\)
&lbrk;
left bracket ([)
&rbrk;
right bracket (])
&lbrc;
left brace ({)
&rbrc;
right brace (})
&minus;
minus sign (-)
&plus;
plus sign (+)
&rbl;
required blank ( )
&tpl;
text placeholder ( )
&eqsym;
equal sign (=)
&rdb;
required SBCS blank in DBCS mode ( )
&percent;
percent sign (%)
&dot;
dot (.)
&cmdpmt
command prompt (= = = >)
&rptr
right pointer (-->)
Any of these predefined entities can be coded with a replication factor. For example, &gtsym(5); creates
the string '>>>>>' in the substituted text.
Multicultural support text strings are also accessible as entities:
&more
More
&caution
CAUTION
&note
Note
&warning
Warning
Markup declarations
24  z/OS: z/OS ISPF DTL Guide

## Page 57

&command
Command
&alpha
abcdefghijklmnopqrstuvwxyz
&scroll
Scroll
&option
Option
&horizdiv
|
&multihst
Enter "/" to select option
&multigui
Check box to select option
&release
Release
&maintlvl
Level:
&created
Created -
&datetext
Date:
&timetext
Time:
&notes
Notes
&attentn
Attention
&tutorial
Tutorial
Points to remember:
1. Some of the symbols defined in the preceding list do not display on some non-programmable
terminals.
2. The &rbl; predefined entity creates one blank in the resulting panel text. To place three required blanks
in a text string, for example, you should code &rbl;&rbl;&rbl; in your tag source file.
3. The &tpl; predefined entity uses a hex FF code to reserve a space in DTL formatted text. After
formatting is completed, the hex FF character is replaced by a blank. As with any predefined entity,
you can change this default to another value. The current value of &tpl; is used for post-formatting text
replacement. Thus, if you prefer to use an @ as the reserved space character, define the entity in this
way:
<! ENTITY TPL '@'>
If multiple reserved spaces are required, you could use these entity definitions to reserve 10
characters:
<! ENTITY TPL '@'>
<! ENTITY MYTPL '@@@@@@@@@@'>
To use your own entity name, first define TPL to override the system default character for text
replacement. Second, add your entity definition, using the specified override character. When the &tpl;
is changed, be careful to select a character that is not otherwise used in your panel.
Markup declarations
Chapter 2. How to use the Dialog Tag Language (DTL)  25

## Page 58

4. The &rdb; predefined entity generates an SBCS blank when ISPDTLC is processing in DBCS mode, or a
null character when processing in SBCS mode.
5. The &dot; predefined entity generates a dot (or period) character in the text. The number of spaces
following the &dot; in the DTL source is maintained in the formatted panel.
Markup declarations
26  z/OS: z/OS ISPF DTL Guide
