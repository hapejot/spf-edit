# Chapter 11. Markup declarations and DTL macro

Source file: f54dt00_v3r1.md
Start page: 203
Page span: 203-210

## Page 203

Chapter 11. Markup declarations and DTL macro
reference
This chapter provides a detailed look at these items:
• “Document-type declaration” on page 171
• “Entity declarations” on page 172
• “Sample entity definitions” on page 175
• “DTL macros” on page 177
Document-type declaration
The document-type declaration (DOCTYPE) identifies the source file document type and the rules the
source file must follow.
<!DOCTYPE DM SYSTEM
[ entity-declarations ]
( entity-declarations )
>
DOCTYPE
Indicates that this is a document-type declaration.
DM
Indicates that this is a DTL source file defining dialog elements.
SYSTEM
Indicates that the rules for the DOCTYPE are contained in an external file.
[ | (
Indicates the beginning of the declaration subset. Either the left bracket or the open parenthesis can
be used to begin the declaration subset. The declaration subset can contain entity declarations and
parameter entity references. If the left bracket is coded, it must be the value X'AD'.
entity-declarations
The entity declarations you define for the source file must be coded within the declaration subset.
“Entity declarations” on page 172 contains a complete description of entity declarations.
] | )
Indicates the end of the declaration subset. Either the right bracket or the close parenthesis can be
used to end the declaration subset. If the right bracket is coded, it must be the value X'BD'.
Description
A document-type declaration identifies the source file document type and rules the source file must
follow.
The DOCTYPE declaration must appear in a DTL source file before any tag markup, although it can be
preceded by comments. Files that are embedded in a source file intended for compilation cannot contain
a DOCTYPE declaration.
Example
The DOCTYPE statement declares the source file as a DM type file.
<!doctype dm system>
<varclass name=varc type='char 10'>
Doctype
© Copyright IBM Corp. 1989, 2024 171

## Page 204

<varlist>
  <vardcl name=vard varclass=varc>
</varlist>
<panel name=panel>
⋮
Entity declarations
Entities are symbolic names that are used to insert text into a file.
<!ENTITY
%
entity-name SYSTEM
"filespec"
Entity text options
>
Entity text options
CDATA SPACE REPLACE COPIES(n) X2C
ATTR
"entity-text"
ENTITY
Indicates this is an entity declaration.
%
Indicates a parameter entity declaration, which must be followed by at least one space.
entity-name
The name of the entity. It must follow these rules:
• Length
– 1-8 for file embed entity names
– 1-8 for parameter entity names
– 1-17 for other entity names
• The first character must be A-Z, a-z, @, #, or $.
• Remaining characters, if any, can be A-Z, a-z, @, #, $, or 0-9.
When an entity name is more than 8 bytes in length, one or more of the remaining characters must
be an underscore.
• Entity names are case-sensitive.
• The entity name for a parameter entity can be specified as a variable name (that is, %&varname;).
The resolved name must follow the parameter entity naming rules.
CDATA
Indicates that any delimiter characters in entity-text are not interpreted as delimiters. This allows you
to define entities with tags in entity-text that are not interpreted as tags.
For example the entity-text "<panel>" is not interpreted as the PANEL tag if the CDATA keyword is
used.
The effect of CDATA is to delay substitution of the variable until all other text manipulation is
completed. For example, you should use CDATA to specify an entity-text string of blanks as normal text
processing removes leading and trailing blanks from text strings.
Note: CDATA cannot be used with parameter entities.
Entities
172  z/OS: z/OS ISPF DTL Guide

## Page 205

SPACE
Indicates that entity-text which spans multiple DTL source file records is formatted like the <p>
tag. (Leading and trailing blanks on entity-text lines are compressed to a single blank character.) In
addition, multiple blanks between words of entity-text are compressed to a single blank character.
REPLACE
Indicates that the current entity-text is to replace any previous definition of the same entity name.
COPIES(n)
Indicates that the entity-text is to be expanded by repeating the provided text n times. For example,
including COPIES(5) as a keyword with the text specified as "*" causes the entity text to be processed
as "*****".
X2C
Indicates that the specified hex format entity-text is to be converted to character format. Non-valid
hex values are processed as a regular entity character string.
ATTR
Indicates that the specified entity-text is a CUA text attribute. Valid values are: CH, CT, DT, ET, FP, NT,
PIN, PT, SAC, SI, WASL, and WT. These values are converted to their corresponding attribute byte.
Non-valid attribute codes are processed as a regular entity character string.
"entity-text"
The text associated with the entity reference. The text must be enclosed in single or double quotes.
The length of the entity-text must be less than or equal to 253 bytes.
SYSTEM
Indicates this entity refers to an external file.
"filespec"
The name of the file the entity refers to. The name must be enclosed in single or double quotes. If this
is not specified, it defaults to the name of the entity.
The SYSTEM parameter can optionally be followed by the file name for the included file. The file name
for MVS is a member name for a file provided on the invocation panel or specified as "DTLGML" entries
in the ISPDTLC profile.
Description
Entities are symbolic names that are used to insert text into a file. The text that an entity refers to can be a
simple string of characters or it can be the text from an entire file.
An entity reference is used to insert the text associated with the entity. Entities must be declared in the
declaration subset of the DOCTYPE declaration before they can be referred to. To refer to the entity in the
source file, the entity name is preceded by an ampersand (&) to indicate it is an entity or percent (%) to
indicate it is a parameter entity. Both types of entities are ended with a semicolon (;). A blank or the end of
the line can be used to end the entity reference instead of the semicolon.
Because entity declarations can only be made within the declaration subset, the parameter entity is the
only way to embed a file of entity declarations. Parameter entities are used when an entity reference is
needed in the declaration subset. References to parameter entities can only be made in the declaration
subset.
References to entities can be made anywhere in the source file after the end of the DOCTYPE declaration.
Note: To refer to an entity within a <SOURCE> tag in the source file, the entity name is preceded by a
percent (%) instead of an ampersand (&).
Because entity names are case-sensitive, ensure that references to entities are specified correctly.
Conditions
Entities that are declared do not have to be referred to.
Entities
Chapter 11. Markup declarations and DTL macro reference  173

## Page 206

Example
This example uses both entities and parameter entities. It embeds the file GLBENT with global entity
declarations, and a file with tags and text. It also uses entities and parameter entities that refer to text
strings.
The first entity declaration declares the "glbent" parameter entity as an external file.
The file name is defaulted to GLBENT. A parameter entity is used because this file contains entity
declarations. Because entity declarations can only be made in the declaration subset, the GLBENT file
is embedded with an entity reference within the declaration subset. The entity declarations in GLBENT
are for text that is used at the top and bottom of the panel. The "header" entity declaration refers to an
external file, and the "footer" is a text string. Both of these entities are referred to in the source file.
The second entity declaration, for “list”, is also a parameter entity. This declaration refers to a string, not
an external file. The text is the SL tag name, which is referred to in the next two entity declarations. These
two declarations, "slist" and “elist”, are used as the SL start and end tags. They are defined as entities so
the type of list can be changed in one place. To change the list type from a simple list (SL) to an unordered
list (UL), change the parameter entity "list" from SL to UL.
This is the content of the source file:
<!DOCTYPE DM SYSTEM [
<!ENTITY % glbent SYSTEM -- declaration of global entity file -->
%glbent;<!-- Embeds the global entity file -->
<!ENTITY % list "sl" -- type of list -->
<!ENTITY slist "<%list;>" -- type of list start tag. -- >
<!ENTITY elist "</%list;>" -- type of list end tag. -- >
]>
<panel name=showlist depth=22 width=45>Show Departments
  <area>
    <info width=40>
      &header;
      <p>The floors and departments are shown below:
      &slist;
        <li>First floor
          &slist;
            <li>Toys
            <li>Electronics
          &elist;
        <li>Second floor
          &slist;
            <li>Boys clothes
            <li>Girls clothes
          &elist;
      &elist;
      &footer;
    </info>
  </area>
</panel>
This is the content of the embedded file GLBENT:
<!ENTITY header SYSTEM "coname">
<!ENTITY footer "<p> We're always glad to help!">
This is the content of the embedded file CONAME:
<lines>
Frank's Children's World
Barnett, NC
</lines>
Figure 85 on page 175 shows the formatted result:
Entities
174  z/OS: z/OS ISPF DTL Guide

## Page 207

Show Departments
 Frank's Children's World
 Barnett, NC
 The floors and departments are shown below:
     First floor
         Toys
         Electronics
     Second floor
         Boys clothes
         Girls clothes
 We're always glad to help!
 
Figure 85. Entities and parameter entities
Sample entity definitions
The tag examples in Chapter 12, “Tag reference,” on page 179 use entity definitions to create the sample
panels. The entities used are called SAMPABC (to define the action bar); SAMPVAR1, SAMPVAR2, and
SAMPVAR3 (to provide VARCLASS and VARLIST definitions); and SAMPBODY (to provide a panel body
section).
The DTL definitions follow:
SAMPABC:
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
SAMPVAR1:
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
  <VARDCL NAME=card  VARCLASS=char7cls>
  <VARDCL NAME=north VARCLASS=char1cls>
Sample entity definitions
Chapter 11. Markup declarations and DTL macro reference  175

## Page 208

<VARDCL NAME=south VARCLASS=char1cls>
  <VARDCL NAME=east  VARCLASS=char1cls>
  <VARDCL NAME=west  VARCLASS=char1cls>
  <VARDCL NAME=nth VARCLASS=char1cls>
  <VARDCL NAME=sth VARCLASS=char1cls>
  <VARDCL NAME=est VARCLASS=char1cls>
  <VARDCL NAME=wst VARCLASS=char1cls>
</VARLIST>
SAMPVAR2:
<VARCLASS NAME=casecls TYPE='char 7'>
<VARCLASS NAME=namecls TYPE='char 25'>
<VARCLASS NAME=addrcls TYPE='char 25'>
<VARCLASS NAME=char1cls TYPE='char 1'>
<VARCLASS NAME=char2cls TYPE='char 2'>
<VARLIST>
  <VARDCL NAME=caseno  VARCLASS=casecls>
  <VARDCL NAME=name    VARCLASS=namecls>
  <VARDCL NAME=address VARCLASS=addrcls>
  <VARDCL NAME=casesel VARCLASS=char2cls>
  <VARDCL NAME=patin   VARCLASS=char1cls>
  <VARDCL NAME=defa    VARCLASS=char1cls>
  <VARDCL NAME=cont    VARCLASS=char1cls>
  <VARDCL NAME=priv    VARCLASS=char1cls>
  <VARDCL NAME=incr    VARCLASS=char1cls>
  <VARDCL NAME=disp    VARCLASS=char1cls>
  <VARDCL NAME=fraud   VARCLASS=char1cls>
</VARLIST>
SAMPVAR3:
<VARCLASS NAME=namecls  TYPE='char 7'>
<VARCLASS NAME=char1cls TYPE='char 1'>
<VARCLASS NAME=char2cls TYPE='char 2'>
<VARLIST>
  <VARDCL NAME=file  VARCLASS=namecls>
  <VARDCL NAME=type  VARCLASS=char2cls>
  <VARDCL NAME=marg  VARCLASS=char2cls>
  <VARDCL NAME=copy  VARCLASS=char2cls>
  <VARDCL NAME=duplx VARCLASS=char1cls>
</VARLIST>
SAMPBODY:
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
    <CHOICE CHECKVAR=CARD MATCH=NEW>New
    <CHOICE CHECKVAR=CARD MATCH=RENEW>Renewal
    <CHOICE CHECKVAR=CARD MATCH=REPLACE>Replacement
  </SELFLD>
  <SELFLD TYPE=multi PMTWIDTH=30 SELWIDTH=25>Check valid branches
    <CHOICE NAME=NORTH HELP=NTHHLP CHECKVAR=NTH>North Branch
    <CHOICE NAME=SOUTH HELP=STHHLP CHECKVAR=STH>South Branch
    <CHOICE NAME=EAST HELP=ESTHLP CHECKVAR=EST>East Branch
    <CHOICE NAME=WEST HELP=WSTHLP CHECKVAR=WST>West Branch
  </SELFLD>
  </REGION>
</AREA>
<CMDAREA>Enter a command
Sample entity definitions
176  z/OS: z/OS ISPF DTL Guide

## Page 209

DTL macros
A DTL macro is a DTL source member found in the concatenated DTL source libraries allocated as input to
ISPDTLC. The macro member can be empty or it can contain any DTL tag coding, including DTL comments.
The macro member is embedded into the current DTL source file when the macro name is encountered
during conversion. The file embed process is similar to Entity file embed.
DTL macro tag syntax is similar to regular DTL tag syntax. You invoke a macro by specifying the macro
member name using a special DTL tag open delimiter, like this:
<?macmemb>
The macro member name must conform to the DTL standard member name rules.
When ISPDTLC finds the <? open delimiter, a file embed is performed on the specified member name.
The reserved member name dummy can be specified to create a no operation (NOP) situation during the
embed cycle. If the specified member has no records, conversion continues with the next DTL source
record.
The content of the macro member can be any valid DTL source input. The member can contain multiple
tags and comment records just like any other DTL source file. DTL source file variables, sometimes
referred to as entities, are substituted using standard entity processing.
An advantage to using the macro syntax instead of an entity file embed is that you need not code the
entity declaration for the file to be embedded. ISPDTLC resolves the required information for you.
Another advantage is that you can specify entity values as part of the macro coding syntax, and
bypass the coding of other entity declarations. For example, if the entity variables subst_var_1,
subst_var_2, and subst_var_3 were coded within the macro using standard DTL syntax (that is,
&subst_var_1;,&subst_var_2;, and &subst_var_3;), you could invoke the macro and specify the
substitution values like this:
<?macmemb subst_var_1=subvalue1 subst_var_2=subvalue2
          subst_var_3=subvalue3>
ISPDTLC automatically defines the entities with the specified values. The values are stored using entity
REPLACE processing, so that if a previous definition exists, it is overwritten. The new definition remains
in effect until replaced, and can be referenced by any other part of the DTL source file. Entities defined in
this way must not be referenced by the first line of a macro.
Macro tags placed within the document declaration function use the same rules as macro tags found after
the document declaration. For example, you can use the macro syntax in place of parameter entities. The
parameter entity (really a file of other entity definitions) member pentmem can be embedded easily by
coding
<?pentmem>
within the document declaration. This syntax replaces the more complicated parameter entity coding of
<:ENTITY % pentmem; system>
         %pentmem;
In another example, the macro syntax can be used in place of entity tags.
<?dummy   panel_title='ISPF macro example'
          panel_width=60
          panel_depth=18>
This syntax replaces multiple entity definitions:
<:ENTITY panel_title 'ISPF macro example'>
<:ENTITY panel_width '60'>
<:ENTITY panel_depth '18'>
DTL macros
Chapter 11. Markup declarations and DTL macro reference  177

## Page 210

In the previous example the macro name dummy is used to bypass the file embed and enable the
attribute resolution process to establish the entity values.
The macro name dummy can also be used within a macro definition to provide default values for macro
entity variables. Example:
<:-- macro/include ISPZ@EX1 to format a 2 column example -->
<?dummy ?col1_indent=0 ?col1_width=30>
<region dir=horiz>
  region dir=vert width=&col1_width; indent=&col1_indent;>
    <pnlinst>&col1_text;
  </region>
  <region dir=vert>
    <pnlinst>&col2_text;
  </region>
</region>
In this example the entity variables col1_width and col1_indent have default values specified by the
dummy tag. The special syntax '?variable=value' provides the default values.
The default values for col1_width and col1_indent are used if you invoke the ISPZ@EX1 macro like this:
<?ispz@ex1 col1_text='text left' col2_text='text right'>
The default values for col1_width and col1_indent are overridden by those specified if you invoke
ISPZ@EX1 macro like this:
<?ispz@ex1 col1_width=40 col1_indent=4
    col1_text='text left' col2_text='text right'
DTL macros
178  z/OS: z/OS ISPF DTL Guide
