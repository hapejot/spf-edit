# Chapter 6. Information regions and help panels

Source file: f54dt00_v3r1.md
Start page: 133
Page span: 133-168

## Page 133

Chapter 6. Information regions and help panels
Some of the information displayed on panels is static, or fixed text that the user does not interact with
directly. This includes text such as top instructions and bottom instructions, prompt text, and data-field
description text. DTL provides you with another method of defining static text for application panels using
information regions.
Defining an information region on a panel allows you more flexibility for defining static text on a panel. The
tags you use to define the text of information regions are much more versatile than the tags you use to
define other types of static text, which means you can be more creative in the text you define.
In addition to using information regions on application panels, you must use them to define the text on
help panels you define for your application. This chapter explains how to define information regions on
application panels, and how to define help panels for your applications.
Defining an information region
Use the INFO tag and its required end tag to define an information region on a panel. You can code an
information region within an AREA, HELP, PANEL, or REGION definition.
Here is an example of an INFO definition:
<panel name=infopan width=42 depth=16>Information
  <area>
    <info>
    </info>
  </area>
</panel>
The INFO tag has an optional WIDTH attribute that defines the width of the information region. If
the value you assign the INFO WIDTH attribute is greater than the WIDTH available in the panel, the
conversion utility resets the value to the available width.
Note: You should code the WIDTH attribute if the information region is part of an application panel
definition that uses horizontal region capability.
The INFO tag only defines an information region. It does not define the text of the information region. DTL
provides you with a set of tags that define the text in information regions. These tags are:
• ATTENTION
• CAUTION
• DL (definition list)
• FIG (figure)
• Hn (heading)
• HP (highlighted phrase)
• LINES
• NOTE
• NOTEL (note list)
• NT (note)
• OL (ordered list)
• P (paragraph)
• PARML (parameter list)
• RP (reference phrase)
• PS (point-and-shoot)
Defining an information region
© Copyright IBM Corp. 1989, 2024 101

## Page 134

• SL (simple list)
• UL (unordered list)
• WARNING
• XMP (example).
With the exception of HP, PS, and RP, these tags can be coded only within an INFO definition. The next
section explains how to use each of these tags and some other tags that complement these tags within
information regions.
Defining basic text
You can define a lot of text using four basic units:
• Paragraphs
• Headings
• Lines
• Examples
Paragraphs
The tag you use most often in information regions is the P (paragraph) tag. Use the P tag to arrange text
as you would arrange a paragraph in your usual writing (to join one or more sentences related by their
subject matter into a single block of text).
When the paragraph text formats for display, the text starts at the current margin and the words
automatically wrap to fit within the margin. In addition, the conversion utility normally inserts a blank
line before each paragraph.
The P tag has an optional attribute, COMPACT, which causes the blank line before the paragraph to be
omitted. The P tag does not require a matching end tag.
We'll illustrate the use of the P tag with this example:
<!doctype dm system>
<panel name=infopan1 width=42>Information
  <area>
    <info width=40>
      <p>This is a paragraph.
      This sentence is also part of the paragraph.
    </info>
  </area>
</panel>
Notice that we coded the second sentence of the paragraph on a different line. It doesn't matter, because
the conversion utility treats it as part of the same paragraph and formats it accordingly. That is, two blanks
are automatically inserted between the sentences. Here is how the paragraph looks:
Defining basic text
102  z/OS: z/OS ISPF DTL Guide

## Page 135

Information
 This is a paragraph. This sentence is
 also part of the paragraph.
 
Figure 42. Paragraph
As you can see, the text of the paragraph is left-justified on the panel and the words automatically wrap to
fit within the defined dimensions of the information region.
We'll add another paragraph to the panel to illustrate how two paragraphs format:
<!doctype dm system>
<panel name=infopan2 width=42>Information
  <area>
    <info width=40>
      <p>This is a paragraph.
      This sentence is also part of the paragraph.
      <p>Here is another paragraph.
      Paragraphs are useful for providing
      information on panels.
    </info>
  </area>
</panel>
Figure 43 on page 103 shows the result:
               Information
 This is a paragraph. This sentence is
 also part of the paragraph.
 Here is another paragraph. Paragraphs
 are useful for providing information on
 panels.
 
Figure 43. Multiple paragraphs
In addition to the placement and wrapping of the text, the compiler separated the paragraphs with a
blank line.
Defining basic text
Chapter 6. Information regions and help panels  103

## Page 136

Headings
The Hn (heading) tag allows you to place headings in an information region. You use these headings to
define topics and subtopics of information. You can define four levels of headings:
H1
Centers text in the information region. Use this heading level to identify a main topic of information.
H2, H3, H4
Formats text against the left margin of the information region. Use one of these heading levels to
identify subtopics of information.
You must code headings sequentially. The conversion utility adds a blank line to the information region
before and after the formatted heading text. The heading tags have no attributes associated with them,
and they don’t require an end tag.
Here is markup that contains an information region using two heading levels and paragraphs following
each one.
<!doctype dm system>
<panel name=infopan3 width=42>Information
  <area>
    <info width=40>
      <h1>A Main Topic
      <p>Notice how the heading is in the
      center of the information region?
        <h2>A Subtopic
        <p>This heading is left-justified.
        <h2>Another Subtopic
        <p>Here's another level-two heading.
    </info>
  </area>
</panel>
Here is the formatted result:
               Information
               A Main Topic
 Notice how the heading is in the center
 of the information region?
 A Subtopic
 This heading is left-justified.
 Another Subtopic
 Here's another level-two heading.
 
Figure 44. Headings (H1-H2)
Lines
Occasionally, you'll want to present text that you don't want formatted by the compiler, or that you want
to show “as is”. You can use the LINES (lines) tag and its required end tag to do this. All text coded within
a LINES definition is treated as unformatted text, and you can position the text however you like on each
line. If the text line is too long to fit in the available width, the conversion utility truncates the text and
issues a warning message.
The LINES tag requires an end tag.
Defining basic text
104  z/OS: z/OS ISPF DTL Guide

## Page 137

There are many ways to use a LINES definition. Here we use it for a quotation:
<!doctype dm system>
<panel name=specact width=48>Special Activities
  <area>
    <info width=46>
      <lines>
      Between the dark and daylight,
      When the night is beginning to lower,
      Comes a pause in the days' occupations,
      That's known as the children's hour.
                              -Longfellow
      </lines>
      <p>Every Tuesday evening at seven
      o'clock, we present the Children's Hour,
      a one-hour recital of selected children's
      stories in our children's section.
    </info>
  </area>
</panel>
Our quotation appears just the way we marked it up:
                Special Activities
        Between the dark and daylight,
        When the night is beginning to lower,
        Comes a pause in the days' occupations,
        That's known as the children's hour.
                                -Longfellow
  Every Tuesday evening at seven o'clock, we
  present the Children's Hour, a one-hour
  recital of selected children's stories in our
  children's section.
 
Figure 45. LINES
Examples
The XMP (example) tag is similar to the LINES tag, in that it allows you to code unformatted text. However,
the text of an XMP definition is indented two spaces from the current margin, as opposed to the text of a
LINES definition, which is not indented from the current margin.
Like a LINES definition, you should avoid coding lines of text in an XMP definition that exceed the available
formatting width of the information region. If the text exceeds the defined width, it is truncated.
The XMP tag requires a matching end tag.
Here’s the formatted result of an example using the XMP tag:
Defining basic text
Chapter 6. Information regions and help panels  105

## Page 138

Order a Toy
 Type the catalog number of the toy you want to order
 and press Enter. The number must be a 6-digit number.
 For example:
   Catalog Number. . . 581678
 
Figure 46. XMP
The markup for the previous panel looks like this:
<!doctype dm system>
<panel name=toy1 width=57>Order a Toy
<area>
<info width=55>
<p>Type the catalog number of the toy you want to order
and press Enter.
The number must be a 6-digit number.
<p>For example:
<xmp>
Catalog Number. . . 581678
</xmp>
</info>
</area>
</panel>
Figures
The FIG (figure) tag is yet another way you can code text that isn't formatted. It works just like the LINES
tag, except you can add a ruled border above and below the figure to separate it from the rest of the
panel. You can also provide a caption for the figure using the FIGCAP tag.
Like the LINES and XMP tags, the FIG tag requires an end tag.
To define the ruled borders for the figure, use the FRAME attribute of the FIG start tag. The FRAME
attribute has two values, RULE, which is the default, and NONE. Because RULE is the default value, you
don't need to specify this attribute if you want ruled lines above and below the figure. To create a figure
without rules, specify NONE as the FRAME value.
The figure in this panel formats with a ruled border:
<!doctype dm system>
<panel name=toy2 width=57>Order a Toy
<area>
<info width=55>
<p>Type the catalog number of
the toy you want to order and
press Enter.
The number must be a 6-digit number.
<p>For example:
<xmp>
Catalog Number. . . 581678
</xmp>
<p>A description of the toy will appear.
<fig>
          ZOOM-A-GO DAREDEVIL SET
  Your kids will have hours of excitement
Defining basic text
106  z/OS: z/OS ISPF DTL Guide

## Page 139

playing with this full set of action toys.
  Requires 80 "AA" batteries.  Not included.
</fig>
</info>
</area>
</panel>
Here is the formatted panel:
                       Order a Toy
 Type the catalog number of the toy you want to order
 and press Enter. The number must be a 6-digit number.
 For example:
   Catalog Number. . . 581678
 A description of the toy will appear.
 ------------------------------------------------
           ZOOM-A-GO DAREDEVIL SET
   Your kids will have hours of excitement
   playing with this full set of action toys.
   Requires 80 "AA" batteries.  Not included.
 ------------------------------------------------
 
Figure 47. Figure with rules
If we wanted the figure to appear without a ruled border, we would have specified FRAME=NONE for the
FIG tag.
The FIG tag also has an optional WIDTH attribute that allows you to specify how the figure is aligned in
the information region. The valid values for WIDTH are PAGE and COL. PAGE, which is the default value,
aligns the figure along the left margin of the information region. COL indicates that the figure is aligned
along the current left margin; that is, the current margin defined by the tag the figure is nested in. This is
useful, for example, for aligning figures within list items.
Figure captions (FIGCAP) tag
To add a caption to the figure in Figure 47 on page 107, use a FIGCAP tag and caption text within the
figure definition, like this:
<!doctype dm system>
<panel name=toy3 width=57>Order a Toy
<area>
<info width=55>
<p>Type the catalog number of
the toy you want to order and
press Enter.
The number must be a 6-digit number.
<p>For example:
<xmp>
Catalog Number. . . 581678
</xmp>
<p>A description of the toy will appear.
<fig>
          ZOOM-A-GO DAREDEVIL SET
  Your kids will have hours of excitement
  playing with this full set of action toys.
  Requires 80 "AA" batteries.  Not included.
<figcap>Zoom-A-Go Daredevil Set
</fig>
</info>
</area>
</panel>
Defining basic text
Chapter 6. Information regions and help panels  107

## Page 140

The figure caption appears just below the bottom figure rule:
                       Order a Toy
 Type the catalog number of the toy you want to order
 and press Enter. The number must be a 6-digit number.
 For example:
   Catalog Number. . . 581678
 A description of the toy will appear.
 ------------------------------------------------
           ZOOM-A-GO DAREDEVIL SET
   Your kids will have hours of excitement
   playing with this full set of action toys.
   Requires 80 "AA" batteries.  Not included.
 ------------------------------------------------
 Zoom-A-Go Daredevil Set
 
Figure 48. Figure caption
Defining lists
Sometimes you want to present information to the user that is not appropriate in paragraph form, such as
list items, a sequence of items or actions, or definitions. For these situations (and many others), you can
use the DTL list tags to format your text appropriately.
You can create these types of lists:
Note lists
Format as numbered lists of notes under a header called Notes.
Simple lists
Format as indented lists of items without any preceding identifiers.
Unordered lists
Format as indented lists of items with each item preceded by a bullet (o), a hyphen (-), or dashes (--),
depending on the level of nesting.
Ordered lists
Format as indented lists of items with each item preceded by a number or letter indicating its
sequence in the list.
Definition lists
Format in two columns, with terms in one column and their matching descriptions in the other. You
can also specify headings for each column in the list. (This list is a definition list.)
Parameter lists
Format in two columns. This list is specifically designed to identify and define parameter terms.
The list items in note lists, simple lists, unordered lists, and ordered lists are created with the list item (LI)
tag. The LI tag does not require an end tag. It is implicitly ended by another LI tag, an LP tag, or the end
tag of the list it is coded within.
Note lists
See “Alerting users: notes, warnings, cautions, and attention” on page 121 for an example showing the
use of note lists.
Defining lists
108  z/OS: z/OS ISPF DTL Guide

## Page 141

Simple lists
A simple list is the least complex type of list. Use a simple list when the information you are presenting
does not follow a sequential pattern or when bullets are not required to discriminate one list item from
another.
Figure 49 on page 109 illustrates a simple list.
                  Virtues
 Around the child bend all the three sweet
 graces:
     Faith
     Hope
     Charity
 
Figure 49. Simple list
This is the markup for the panel:
<!doctype dm system>
<panel name=slistx1 width=44>Virtues
  <area>
    <info width=42>
      <p>Around the child bend all the
      three sweet graces:
      <sl>
        <li>Faith
        <li>Hope
        <li>Charity
      </sl>
    </info>
  </area>
</panel>
We used the SL tag and its matching end tag to define the simple list. We defined each of the list items by
nesting the LI tags definition. within the simple list
As you can see, our simple list formatted with a blank line between each of the list items. For cases where
you need to conserve space, you can use the COMPACT attribute to format the list without blank lines
between the list items.
Code the COMPACT attribute within the SL start tag (before the tag close delimiter), like this:
<!doctype dm system>
<panel name=slistx2 width=44>Virtues
  <area>
    <info width=42>
      <p>Around the child bend all the
      three sweet graces:
      <sl compact>
        <li>Faith
        <li>Hope
        <li>Charity
      </sl>
    </info>
  </area>
</panel>
Defining lists
Chapter 6. Information regions and help panels  109

## Page 142

Now the simple list is compacted:
                  Virtues
 Around the child bend all the three sweet
 graces:
     Faith
     Hope
     Charity
 
Figure 50. Compact simple list
You can also nest simple lists within other lists. The list items format at different indentation levels, based
on the level of nesting.
The indentation for the list item is based on the SPACE attribute of the LI tag and the enclosing list
tag. When SPACE=NO (or the SPACE attribute is not present) the list item indentation is 4 spaces. When
SPACE=YES, the indentation is 3 spaces. See Chapter 12, “Tag reference,” on page 179 for additional
information about the LI, SL, OL, and UL tags.
Unordered lists
Unordered lists are similar to simple lists, except each list item is preceded by symbol that is dependent
on the nesting level of the list. You don't have to supply the symbols–the conversion utility does that for
you.
Use an unordered list if the list items are long and you don't want to imply any particular sequence in the
list.
Here is an unordered list:
                 Window Shopper
 With Window Shopper, you can order many
 wonderful things, such as:
 o   Raindrops on roses
 o   Whiskers on kittens
 o   Bright copper kettles
 o   Warm woolen mittens
 o   Brown paper packages tied up with string
 And many more of your favorite things!
 
Figure 51. Unordered list
Here is the markup for this unordered list:
Defining lists
110  z/OS: z/OS ISPF DTL Guide

## Page 143

<!doctype dm system>
<panel name=winshop width=48>Window Shopper
  <area>
    <info width=46>
      <p>With Window Shopper, you can order many wonderful things,
      such as:
      <ul>
        <li>Raindrops on roses
        <li>Whiskers on kittens
        <li>Bright copper kettles
        <li>Warm woolen mittens
        <li>Brown paper packages tied up with string
      </ul>
      <p>And many more of your favorite things!
    </info>
  </area>
</panel>
For our unordered list, we used the UL tag and its matching end tag. As you can see, even though we
didn't code the bullet symbols (o) in the markup, they appear in front of each of the list items in the
unordered list.
We could make this list compact like our simple list example because the COMPACT attribute is also valid
for the UL tag. Likewise, we could use the SPACE attribute to control indentation of the list items for the
UL tag.
You can also define levels of unordered lists; that is, you can nest unordered lists within other unordered
lists. When you do this, the symbols preceding the list items in each level of the list vary, depending on
the level of nesting. Specifically, the list items in the first (or only) level of unordered list are preceded by
bullets (o), as shown in Figure 51 on page 110. If you nest another unordered list within an unordered
list,the list items in that list are preceded by hyphen symbols (-). A third-level unordered list has dashes
(--) preceding the list items. The nested tag text is aligned according to the level of nesting.
To show how this works, we'll create a panel with three levels of unordered lists.
<!doctype dm system>
<panel name=ulists width=42>Nested Unordered Lists
  <area>
    <info width=40>
      <ul>
        <li>First level, first item
        <li>First level, second item
         <ul>
           <li>Second level, first item
           <li>Second level, second item
             <ul>
               <li>Third level, only item
             </ul>
         </ul>
        <li>Back to the first level
      </ul>
    </info>
  </area>
</panel>
Here is how this panel looks:
Defining lists
Chapter 6. Information regions and help panels  111

## Page 144

Nested Unordered Lists
 o   First level, first item
 o   First level, second item
     -   Second level, first item
     -   Second level, second item
         --  Third level, only item
 o   Back to the first level
 
Figure 52. Nested unordered lists
If you nest more than three levels of unordered lists, the sequence of bullets, hyphens, and dashes
repeats. For example, a fourth level would be preceded by bullets, a fifth level by hyphens, and so on.
Remember, all lists must be explicitly ended with the appropriate list end tag.
Ordered lists
Ordered lists imply an outline sequence to the list items by preceding each of the list items with a number
or character depending on the level of nesting.
Here is an ordered list:
                   Window Shopper
 After you have placed your order with Window
 Shopper, you should...
 1.  Press the Enter key to leave the Order Panel.
 2.  Go to the receiving desk located at the front
     of the store.
 3.  Give the cashier the pink copy of your
     receipt.
 4.  Take your purchases home, and enjoy!
 
Figure 53. Ordered list
You don't supply the numbers for the list items in your markup; they are generated automatically. This
saves you time when you revise ordered lists, because you can insert, delete, or rearrange list items
without renumbering them yourself.
Here is the markup we used for this list:
<!doctype dm system>
<panel name=winshop2 width=52>Window Shopper
  <area>
    <info width=50>
      <p>After you have placed your order with Window Shopper, you should...
Defining lists
112  z/OS: z/OS ISPF DTL Guide

## Page 145

<ol>
        <li>Press the Enter key to leave the Order Panel.
        <li>Go to the receiving desk located at the front of the store.
        <li>Give the cashier the pink copy of your receipt.
        <li>Take your purchases home, and enjoy!
      </ol>
    </info>
  </area>
</panel>
Like other types of lists, you can nest ordered lists within other lists. And, like unordered lists, the levels of
the lists you nest determine the characters that precede the list items.
Specifically, the conversion utility uses this sequence when processing list items in nested ordered lists:
• First-level list items are preceded by sequential numbers followed by a period and 2 spaces 1.
• Second-level list items are preceded by sequential lowercase alphabetic characters followed by a
period and 2 spaces 1.
• Third-level list items are preceded by sequential numbers followed by a close parentheses symbol and
2 spaces 1.
• Fourth-level list items are preceded by sequential lowercase alphabetic characters followed by a close
parentheses symbol and 2 spaces 1.
Note: Each level beyond the first level indents 41 spaces.
The sequence of nesting is repeated for levels of nesting beyond the fourth level. For example, the list
items in a fifth level of nesting are preceded by sequential numbers followed by a period.
To show you what this looks like, we'll nest three levels of ordered lists in this markup. We'll use the
COMPACT attribute in the third level to conserve space.
<!doctype dm system>
<panel name=olists width=42>Nested Ordered Lists
  <area>
    <info width=40>
      <ol>
        <li>Step one (first level)
        <li>Step two (first level)
          <ol>
            <li>Step one (second level)
            <li>Step two (second level)
              <ol compact>
                <li>Step one (third level)
                <li>Step two (third level)
              </ol>
            <li>Step three (second level)
          </ol>
        <li>Step three (first level)
      </ol>
    </info>
  </area>
</panel>
Here is how the DTL compiler formats this panel:
1 The default indentation for a list item is 4 spaces. When the SPACE=YES attribute is coded, the indentation
is 3 spaces. See the LI and OL tag descriptions in Chapter 12, “Tag reference,” on page 179 for more
information.
Defining lists
Chapter 6. Information regions and help panels  113

## Page 146

Nested Ordered Lists
 1.  Step one (first level)
 2.  Step two (first level)
     a.  Step one (second level)
     b.  Step two (second level)
         1)  Step one (third level)
         2)  Step two (third level)
     c.  Step three (second level)
 3.  Step three (first level)
 
Figure 54. Nested ordered lists
Definition lists
Definition lists allow you to identify a list of words or phrases and their corresponding definitions. A
simple definition list formats as a two-column list: the terms you define appear in the left column, and
the definitions for the terms appear in the right column. Definition lists are slightly more complex than the
previous lists we've discussed, because of the additional tags required to construct them.
The tags used to create a definition list are:
DL
Begins a definition list. The required end tag ends the list.
DT
Identifies the term being defined. The definition term is formatted in the left column of the list. It does
not require an end tag.
DD
Identifies the term description. Each definition description is formatted in the right column of the list,
immediately opposite or below its associated term. It does not require an end tag.
You can also create headings for definition list columns. There are two additional tags that you can use to
do this. They are:
DTHD
Defines a header for the definition term column.
DDHD
Defines a header for the definition description column.
Both of these tags are optional for creating definition lists. We'll show you how you can use them to
enhance definition lists later on in this topic.
Here is an example of a definition list:
Defining lists
114  z/OS: z/OS ISPF DTL Guide

## Page 147

Department Codes
 Use the following codes for each of the
 matching departments:
 AP        Appliances
 AU        Automotive
 GA        Garden shop
 HB        Health and beauty
 HO        Home decor
 SP        Sporting goods
 
Figure 55. Definition  list
Here is the markup:
<!doctype dm system>
<panel name=deptcode width=42>Department Codes
  <area>
    <info width=40>
      <p>Use the following codes for each of the
      matching departments:
      <dl>
        <dt>AP
        <dd>Appliances
        <dt>AU
        <dd>Automotive
        <dt>GA
        <dd>Garden shop
        <dt>HB
        <dd>Health and beauty
        <dt>HO
        <dd>Home decor
        <dt>SP
        <dd>Sporting goods
      </dl>
    </info>
  </area>
</panel>
A definition list can contain multiple definition terms. The TSIZE attribute of the enclosing DL tag specifies
the number of DT tags in a group and their respective widths. For example, TSIZE='10 5' specifies 2
definition term columns with sizes of 10 and 5 characters, respectively.
The DL tag has optional attributes:
TSIZE
specifies the space allocated for the term column or columns
BREAK
indicates if the definition formats on the same line as its associated term
COMPACT
determines if there is a space between each set of terms and descriptions
NOSKIP
suppresses the blank line normally placed before the list
INDENT
controls the indentation from the current left margin
FORMAT
controls the location of the definition term within the TSIZE space
Defining lists
Chapter 6. Information regions and help panels  115

## Page 148

DIVEND
determines whether a divider character is inserted following the DDHD and DD tag text
SPLIT
controls the format of the last DT tag in a multiple DT tag group
Use the TSIZE attribute to specify how much space you want for the definition term column (or columns).
The default value is 10 bytes, which also sets the default number of DT tags to one. If you want to specify
more (or less) space than the default, or multiple DT tags, use the TSIZE attribute to assign the value you
want.
Use the BREAK attribute to specify where the definition descriptions are supposed to start (on the same
line as the definition terms or on the next line). The BREAK attribute can be specified as NONE, ALL, or
FIT.
NONE
The definition descriptions start on the same lines as the definition terms.
ALL
All of the definition descriptions start on the line after the definition terms.
FIT
The definition descriptions are to start on the next line only when the definition term does not fit in the
allocated space and spills over into the description area.
The definition list in Figure 55 on page 115 used the default BREAK=NONE. We'll define another list that
uses BREAK=ALL.
<!doctype dm system>
<panel name=reverb1 width=52>Reverberations
  <area>
    <info width=50>
      <p>Reverberations is one of the most popular brands of electronic
      components available today.
      We stock the following Reverberations components:
      <dl break=all>
        <dt>CD Player Unit
        <dd>With auto-search, auto-off, power door, and
        a two-year warranty.
        <dt>Receiver
        <dd>Digital, 6-speaker hookup, and built-in
equalizer.
        <dt>Tape deck
        <dd>Supports metal and chrome cassettes, and comes with
        a two-year warranty.
      </dl>
    </info>
  </area>
</panel>
Figure 56 on page 117 shows how this definition list formats.
Defining lists
116  z/OS: z/OS ISPF DTL Guide

## Page 149

Reverberations
 Reverberations is one of the most popular brands
 of electronic components available today. We
 stock the following Reverberations components:
 CD Player Unit
           With auto-search, auto-off, power door,
           and a two-year warranty.
 Receiver
           Digital, 6-speaker hookup, and built-in
           equalizer.
 Tape deck
           Supports metal and chrome cassettes, and
           comes with a two-year warranty.
 
Figure 56. Definition  list (BREAK=ALL)
Because the TSIZE and BREAK attributes lend versatility to definition lists, we can rearrange this list
practically any way we want. We'll change the BREAK value to FIT, and increase the TSIZE to 13 to show
you what we mean. We'll also add headings to the list to show you how they format.
<!doctype dm system>
<panel name=reverb2 width=52>Reverberations
  <area>
    <info width=50>
      <p>Reverberations is one of the most popular brands of electronic
      components available today.
      We stock the following Reverberations components:
      <dl tsize=13 break=fit>
        <dthd>Component
        <ddhd>Features
        <dt>CD Player Unit
        <dd>With auto-search, auto-off, power door, and
        a two-year warranty.
        <dt>Receiver
        <dd>Digital, 6-speaker hookup, and built-in equalizer.
        <dt>Tape deck
        <dd>Supports metal and chrome cassettes, and comes with
        a two-year warranty.
      </dl>
    </info>
  </area>
</panel>
Here is how the panel looks now:
Defining lists
Chapter 6. Information regions and help panels  117

## Page 150

Reverberations
 Reverberations is one of the most popular brands
 of electronic components available today. We
 stock the following Reverberations components:
 Component    Feature
 CD Player Unit
              With auto-search, auto-off, power
              door, and a two-year warranty.
 Receiver     Digital, 6-speaker hookup, and
              built-in equalizer.
 Tape deck    Supports metal and chrome cassettes,
              and comes with a two-year warranty.
 
Figure 57. Definition  list (BREAK=FIT)
Parameter lists
Parameter lists are another way of defining terms in a list form. You use a parameter list when the terms
you are defining are related to the application in some way (for example, showing codes or parameters).
The tags you use to create parameter lists are the PARML tag and its required end tag, the PT (parameter
term) tag, and the PD (parameter description) tag. The parameter list tags work a lot like the definition list
tags in defining terms and descriptions, except there are no tags for defining list headings.
The PARML tag also contains the TSIZE, BREAK, COMPACT, INDENT, and SKIP attributes. The TSIZE
default value is 10 bytes, as it is for definition lists. However, the BREAK default value for parameter
lists is ALL, instead of NONE, as in definition lists. Thus, the parameter descriptions format on the lines
following the parameter terms unless you specify otherwise.
A parameter list can contain multiple parameter terms. The TSIZE attribute of the enclosing PARML
tag specifies the number of PT tags in a group and their respective widths. For example, TSIZE='10 5'
specifies 2 parameter term columns with sizes of 10 and 5 characters, respectively.
Here is the markup for a typical parameter list:
<!doctype dm system>
<panel name=ordnum width=52>Order Numbers
  <area>
    <info width=50>
      <p>The order number assigned to each inventory item
      represents specific information about the item.
      <p>Specifically,
      <parml>
        <pt>123
        <pd>The first 3 digits represent the
        department the item is stocked in.
        <pt>456
        <pd>The fourth, fifth, and sixth digits
        represent the item.
        <pt>78
        <pd>The seventh and eighth digits represent
        the lot number of the item.
      </parml>
    </info>
  </area>
</panel>
Here is the formatted parameter list:
Defining lists
118  z/OS: z/OS ISPF DTL Guide

## Page 151

Order Numbers
 The order number assigned to each inventory item
 represents specific information about the item.
 Specifically,
 123
           The first 3 digits represent the
           department the item is stocked in.
 456
           The fourth, fifth, and sixth digits
           represent the item.
 78
           The seventh and eighth digits represent
           the lot number of the item.
 
Figure 58. Parameter list
Nesting tags within lists
The format of your lists isn't confined to only list items. You can also nest other tags within the list items.
For example, if a list item requires an additional paragraph, you can nest a P tag following the list item.
This markup contains an ordered list with a paragraph nested within the second list item.
<!doctype dm system>
<panel name=winshop3 width=52>Window Shopper
  <area>
    <info width=50>
      <p>After you have placed your order with Window Shopper, you should...
      <ol>
        <li>Press the Enter key to leave the Order Panel.
        <li>Go to the receiving desk located at the front of the store.
          <p>Don't forget to bring your receipt!
        <li>Give the cashier the pink copy of your receipt.
        <li>Take your purchases home, and enjoy!
      </ol>
    </info>
  </area>
</panel>
The paragraph text follows the indentation of the preceding list item, like this:
                   Window Shopper
 After you have placed your order with Window
 Shopper, you should...
 1.  Press the Enter key to leave the Order Panel.
 2.  Go to the receiving desk located at the front
     of the store.
     Don't forget to bring your receipt!
 3.  Give the cashier the pink copy of your
     receipt.
 4.  Take your purchases home, and enjoy!
 
Figure 59. Nested paragraph within a list
Defining lists
Chapter 6. Information regions and help panels  119

## Page 152

The List Part (LP) tag
If you want to insert unindented text in a list, use the list part (LP) tag. The LP tag is useful for providing
information about the list items that follow it.
We added a list part to the panel shown in Figure 59 on page 119:
<!doctype dm system>
<panel name=winshop4 width=52>Window Shopper
  <area>
    <info width=50>
      <p>After you have placed your order with Window Shopper, you should...
      <ol>
        <li>Press the Enter key to leave the Order Panel.
        <li>Go to the receiving desk located at the front of the store.
          <p>Don't forget to bring your receipt!
        <li>Give the cashier the pink copy of your receipt.
          <lp>Occasionally, the item you ordered won't be in stock.
          If this occurs, the cashier will be happy to delete
          the item from your order.
        <li>Take your purchases home, and enjoy!
      </ol>
    </info>
  </area>
</panel>
Here is the formatted result:
                   Window Shopper
 After you have placed your order with Window
 Shopper, you should...
 1.  Press the Enter key to leave the Order Panel.
 2.  Go to the receiving desk located at the front
     of the store.
     Don't forget to bring your receipt!
 3.  Give the cashier the pink copy of your
     receipt.
 Occasionally, the item you ordered won't be in
 stock. If this occurs, the cashier will be happy
 to delete the item from your order.
 4.  Take your purchases home, and enjoy!
 
Figure 60. List part
Nesting lists within lists
In “Unordered lists” on page 110 and “Ordered lists” on page 112 we showed you how to define levels of
nested unordered and ordered lists. You can also nest different types of lists within other lists.
Here is an example of an unordered list nested within a definition list:
Defining lists
120  z/OS: z/OS ISPF DTL Guide

## Page 153

Payment Procedures
                 Methods of Payment
 Cash
          Of course, we always accept cash!
 Charge
          Your charge card is welcome here! We
          accept the following charge cards:
          o   BigCharge
          o   MoneyCard
          o   Plastic Express
 Personal Check
          We gladly welcome your personal check,
          with the proper identification.
 
Figure 61. Nested unordered list in a definition  list
Here is the markup we used to create the nested lists in Figure 61 on page 121:
<!doctype dm system>
<panel name=payment width=52>Payment Procedures
  <area>
    <info width=50>
      <h1>Methods of Payment
      <dl tsize=9 break=all>
        <dt>Cash
        <dd>Of course, we always accept cash!
        <dt>Charge
        <dd>Your charge card is welcome here!
        We accept the following charge cards:
          <ul compact>
            <li>BigCharge
            <li>MoneyCard
            <li>Plastic Express
          </ul>
        <dt>Personal check
        <dd>We gladly welcome your personal check,
        with the proper identification.
      </dl>
    </info>
  </area>
</panel>
You can nest any type of list within another list. Remember, whenever you nest lists, be sure that you end
each level with its proper end tag.
Alerting users: notes, warnings, cautions, and attention
DTL provides you with tags that you can use to alert the user to certain text that warrants special
attention. Whether you are noting a minor aspect of the application or alerting the user to the risk of
possible damage to programs or data, you can alert the user appropriately.
This topic discusses these tags:
• “Notes (NOTE, NT and NOTEL tags)” on page 121
• “Attention and warning (ATTENTION and WARNING tags)” on page 124
• “Caution (CAUTION tag)” on page 125
Notes (NOTE, NT and NOTEL tags)
The NOTE, NOTEL, and NT tags format as noted text. Use notes to emphasize minor points.
Alerting users: notes, warnings, cautions, and attention
Chapter 6. Information regions and help panels  121

## Page 154

When you use either the NOTE or NT tag, you get the text "Note:" followed by a space before the text you
specify. However, the text is formatted differently depending on which tag you use.
The NOTEL tag is formatted with the first line containing the text "Notes:" followed by a numbered list of
note information provided by the <LI> tag.
The NOTE tag
If the text is a single paragraph, you use the NOTE tag. The text is formatted as an unindented block,
like a paragraph. The NOTE tag does not require a matching end tag.
You use the NOTE tag like this:
<!doctype dm system>
<panel name=widget61 width=50>Widgets
  <area>
    <info width=48>
      <p>Choose the type of Widget you want to order by placing
      the cursor on the field and pressing Enter.
      <note>If the Widget you wish to order is not in stock, please
      refer to the "Back Order" panel to place an order.
    </info>
  </area>
</panel>
Figure 62 on page 122 shows how it formats.
                     Widgets
 Choose the type of Widget you want to order by
 placing the cursor on the field and pressing
 Enter.
 Note: If the Widget you wish to order is not in
 stock, please refer to the "Back Order" panel to
 place an order.
 
Figure 62. Note (NOTE tag)
The NOTEL tag
If more than one note is used for special attention information, you use the NOTEL tag. Each note is
provided by a separate LI tag. The notes are numbered similar to the format described in “Ordered
lists” on page 112. You use either the P or LP tag to add any additional paragraphs in the NOTEL
definition. Use the required end tag to end the NOTEL definition.
In this example, 2 notes are used, 1 with more than one paragraph. We use the NOTEL tag and its
required end tag along with LI tags to define the notes, and a P tag for the additional paragraph.
<!doctype dm system>
<panel name=widget63 width=50>Widgets
  <area>
    <info width=48>
      <p>Choose the type of Widget you want to order by placing
      the cursor on the field and pressing Enter.
      <notel>
        <li>If the Widget you wish to order is not in stock, please
            refer to the "Back Order" panel to place an order.
        <li>Back-ordered Widgets usually arrive within three days.
          <p>Please check again in three days.
      </notel>
      <p>If you want to order more than one Widget, specify the quantity
Alerting users: notes, warnings, cautions, and attention
122  z/OS: z/OS ISPF DTL Guide

## Page 155

and press Enter.
    </info>
  </area>
</panel>
Notice that the P tag in the note is coded before the NOTEL end tag, indicating that the second
paragraph belongs in the note.
This is how the panel looks now: 
                     Widgets
 Choose the type of Widget you want to order by
 placing the cursor on the field and pressing
 Enter.
 Notes:
 1.  If the Widget you wish to order is not in
     stock, please refer to the "Back Order"
     panel to place an order.
 2.  Back-ordered Widgets usually arrive within
     three days.
     Please check again in three days.
If you want to order more than one Widget,
specify the quantity and press Enter.
  
Figure 63. Notel (NOTEL tag)
As you can see, the text of the NOTEL tag is formatted as a list under the "Notes:" heading. The text of
the P tag is indented to match the list items.
The NT tag
If the note requires more than one paragraph, you use the NT tag. You use the P tag to add any
additional paragraphs in the NT definition. Use the required end tag to end the NT definition.
Another difference between the NOTE and NT tag is that the NT tag indents the note text from the left
panel margin.
In this example, the note is longer than one paragraph. We use the NT tag and its required end tag to
define the note, and a P tag for each additional paragraph.
<!doctype dm system>
<panel name=widget62 width=50>Widgets
  <area>
    <info width=48>
      <p>Choose the type of Widget you want to order by placing
      the cursor on the field and pressing Enter.
      <nt>If the Widget you wish to order is not in stock, please
      refer to the "Back Order" panel to place an order.
        <p>Back-ordered Widgets usually arrive within three days.
      </nt>
      <p>If you want to order more than one Widget, specify the quantity
      and press Enter.
    </info>
  </area>
</panel>
Notice that the P tag in the note is coded before the NT end tag, indicating that the second paragraph
belongs in the note.
This is how the panel looks now:
Alerting users: notes, warnings, cautions, and attention
Chapter 6. Information regions and help panels  123

## Page 156

Widgets
 Choose the type of Widget you want to order by
 placing the cursor on the field and pressing
 Enter.
 Note: If the Widget you wish to order is not in
       stock, please refer to the "Back Order"
       panel to place an order.
       Back-ordered Widgets usually arrive
       within three days.
 If you want to order more than one Widget,
 specify the quantity and press Enter.
 
Figure 64. Note (NT tag)
As you can see, the text of the NT tag is indented, as is the text of the P tag coded within the NT tag.
Attention and warning (ATTENTION and WARNING tags)
Attention statements and warning statements alert the user of a possible risk involved with a user action,
or of existing error conditions.
You must immediately precede the ATTENTION or WARNING tag with a P (paragraph) tag, LI (list item)
tag, or LP (list part) tag. The warning statement formats with the term "Warning:" before the text. The
attention statement formats with the term "Attention:" before the text.
The ATTENTION and WARNING tags have no associated attributes and require a matching end tag.
Here is the markup for a warning statement that formats as a paragraph.
<!doctype dm system>
<panel name=addfile width=50>Changing a File
  <area>
    <info width=48>
      <p>After you have made the desired changes
      to the file, press Enter to save the changes.
      <p><warning>Pressing Enter saves
      ALL changes made to the file.
      You can cancel this operation by pressing
      the F12=Cancel key.
      </warning>
    </info>
  </area>
</panel>
Here is the formatted result:
Alerting users: notes, warnings, cautions, and attention
124  z/OS: z/OS ISPF DTL Guide

## Page 157

Changing a File
 After you have made the desired changes to the
 file, press Enter to save the changes.
 Warning: Pressing Enter saves ALL changes made
 to the file. You can cancel this operation by
 pressing the F12=Cancel key.
 
Figure 65. Warning
Caution (CAUTION tag)
Caution statements indicate the greatest degree of severity. Like the WARNING tag, the CAUTION tag has
a required end tag, and must be preceded by a P (paragraph) tag, LI (list item) tag, or LP (list part) tag. The
caution statement formats with the term "CAUTION:" followed by the caution text on the next line.
<!doctype dm system>
<panel name=delfile width=50>Deleting a File
  <area>
    <info width=48>
      <p>To delete a file, type the file name in the
      "Delete this file" field and press Enter.
      <p>A message appears asking for verification.
      To continue the delete operation, press Enter.
      <p><caution>Verifying the delete operation
      permanently deletes the file from your records.
      There is no chance of recovery.
      </caution>
    </info>
  </area>
</panel>
Here is the formatted result:
                 Deleting a File
 To delete a file, type the file name in the
 "Delete this file" field and press Enter.
 A message appears asking for verification. To
 continue the delete operation, press Enter.
 CAUTION:
 Verifying the delete operation permanently
 deletes the file from your records. There is no
 chance of recovery.
 
Figure 66. Caution
Alerting users: notes, warnings, cautions, and attention
Chapter 6. Information regions and help panels  125

## Page 158

Emphasizing panel text
You can emphasize text on application panels or on help panels with highlighting by using the HP
(highlighted phrase) tag. You can also highlight words or phrases to indicate that additional information is
available by using the RP (reference phrase) tags. On a color terminal, the emphasized text displays in a
CUA defined color, or whatever color you set with the Color Change Utility.
Highlighting requires the use of 3270 attribute bytes to control the display of highlighted text. The
attribute positions before and after the highlighted text display as blank spaces. These attributes might
limit the formatting of your highlighted phrase or reference phrase.
Here is an example of highlighting:
<HP>To cancel this option</HP>, press the F12 key.
Here is the result:
To cancel this option, press the F12 key.
You can prevent this situation by writing statements that do not require punctuation following an HP or an
RP end tag.
Highlighted phrases
The HP (highlighted phrase) tag provides emphasis through highlighting. You can focus the user's
attention to particular sections of the panel text by highlighting words, phrases, or entire paragraphs.
The HP tag requires a matching end tag to indicate the end of a highlighted phrase.
This markup example shows the use of the HP tag:
<!DOCTYPE DM SYSTEM>
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
   <VARDCL NAME=south VARCLASS=char1cls>
   <VARDCL NAME=east  VARCLASS=char1cls>
   <VARDCL NAME=west  VARCLASS=char1cls>
 </VARLIST>
<PANEL NAME=hlitep>Library Card Registration
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
Emphasizing panel text
126  z/OS: z/OS ISPF DTL Guide

## Page 159

<PDC>Extended Help...
      <ACTION RUN=exhelp>
    <PDC>Keys Help...
      <ACTION RUN=keyshelp>
 </AB>
<TOPINST>Type in <HP>patron's name</HP> and <HP>card number</HP>
         (if applicable).
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
Here is the formatted result:
   File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number (if applicable).
 Then select an action bar choice.
 Date . . . : 08/29/90
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
Figure 67. Highlighted phrase example
Reference phrases
The RP (reference phrase) tag allows you to highlight words or phrases on panels to indicate that
additional help information is available. When a help panel with reference phrases is displayed, the cursor
is positioned in the first reference phrase. When an application panel containing reference phrases is
Emphasizing panel text
Chapter 6. Information regions and help panels  127

## Page 160

displayed, the cursor is positioned to the first reference phrase or panel input field, unless the cursor
setting has been specified by the application. The reference phrase is an input-capable field so that
the user can tab to successive reference phrases on the panel. The reference phrase text is refreshed
whenever the panel is displayed.
When the user places the cursor on a reference phrase and requests help, the reference phrase panel
or message is displayed. Reference phrase help panels themselves can also contain reference phrases.
When a user cancels a reference phrase help, the panel from which the user requested the reference
phrase help is displayed again. All other help facilities, such as KEYSHELP and EXHELP, are available from
a reference phrase help panel.
The RP tag requires a matching end tag to indicate the end of the reference phrase text.
This markup example shows the use of the RP tag.
<!DOCTYPE DM SYSTEM>
<HELP NAME=frenchl depth=12>Help for Masters Degree in French Literature
<area>
<info>
<p>
The Masters in French Literature (MFL) Program is also available
to students interested in
<rp help=liteve>evening studies.</rp>
<p>
Please consult your program advisers for details before registering for
a class.
</info>
</area>
</help>
<help name=liteve depth=13>Help for Evening Studies
<area>
<info>
<p>
Evening Studies offered by the French Literature
graduate program are available to students
interested in part-time and full-time studies.
All core courses and many electives are offered
in the evening on a rotating basis.  Please
consult your program advisers for details before
registering for a class.
</info>
</area>
</help>
Here is the formatted result:
   Help for Masters Degree in French Literature
 The Masters in French Literature (MFL) Program
 is also available to students interested in
 evening studies.
 Please consult your program advisers for details
 before registering for a class.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 68. Reference phrase example
Accordingly, when the user selects the reference phrase evening studies, the help panel specified by the
HELP attribute (help=liteve) is displayed.
Emphasizing panel text
128  z/OS: z/OS ISPF DTL Guide

## Page 161

Help for Evening Studies
 Evening Studies offered by the French Literature
 graduate program are available to students
 interested in part-time and full-time studies.
 All core courses and many electives are offered
 in the evening on a rotating basis.  Please
 consult your program advisers for details before
 registering for a class.
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 69. Reference phrase example of help attribute
The help-panel-name attribute specifies the name of the help panel to be displayed if the reference
phrase is selected.
Using information regions with other panel elements
You can use information regions to complement the other elements of an application panel in many
different ways. For example, you can use an information region to provide additional information for fields
on an application panel.
Here is a markup example where the information region uses a paragraph and a compact ordered list to
tell the user how to interact with the panel fields. Figure 70 on page 130 shows the formatted result.
<!doctype dm system>
<VARCLASS NAME=selcls TYPE='char 1'>
<VARLIST>
  <VARDCL NAME=day VARCLASS=selcls>
  <VARDCL NAME=time VARCLASS=selcls>
</VARLIST>
<panel name=appmnt>Make an Appointment
  <area>
    <info width=74>
      <p>To schedule an appointment, you must choose one
      selection from each field.
      <ol compact>
        <li>Choose a day from the first field.
        <li>Choose a time slot from the second field.
        <li>After you have completed both fields, press
        Enter to log your appointment and leave the panel.
      </ol>
    </info>
    <divider type=solid gutter=3>
    <region dir=horiz>
      <region>
      <selfld name=day selwidth=20 pmtwidth=9>Weekdays:
        <choice>Monday
        <choice>Tuesday
        <choice>Wednesday
        <choice>Thursday
        <choice>Friday
      </selfld>
      </region>
      <divider gutter=8>
      <region>
        <selfld name=time selwidth=20 pmtwidth=5>Time:
          <choice>9:00
          <choice>10:00
          <choice>11:00
          <choice>12:00
          <choice>1:00
          <choice>2:00
          <choice>3:00
          <choice>4:00
        </selfld>
       </region>
    </region>
Using information regions with other panel elements
Chapter 6. Information regions and help panels  129

## Page 162

</area>
</panel>
                            Make an Appointment
 To schedule an appointment, you must choose one selection from each field.
 1.  Choose a day from the first field.
 2.  Choose a time slot from the second field.
 3.  After you have completed both fields, press Enter to log your
     appointment and leave the panel.
  -------------------------------------------------------------------------
 Weekdays:                    Time:
 __  1.  Monday               __  1.  9:00
     2.  Tuesday                  2.  10:00
     3.  Wednesday                3.  11:00
     4.  Thursday                 4.  12:00
     5.  Friday                   5.  1:00
                                  6.  2:00
                                  7.  3:00
                                  8.  4:00
 
Figure 70. Information region
Help panels
This topic shows you how to use the DTL to define help panels that provide help to users while they are
using an ISPF application. We also show you how to link help panels with application panels.
Defining help panels
The HELP tag and its required end tag define a help panel. The HELP start tag indicates the beginning of a
help panel definition, and the HELP end tag closes the definition. All of the other tags that compose a help
panel are coded between these two tags. You also use the HELP tag to define the help panel title in the
same way you code panel title text with the PANEL tag, as tag content.
Here is an example of the HELP tag and its matching end tag:
<help name=help01>Help Panel Title
</help>
In this example we added the required NAME attribute and value to the HELP start tag. The NAME value
you assign must follow the standard naming convention described in “Rules for variable names” on page
179.
The value you assign to NAME is the value that elements such as application panels, fields, and messages
use to provide help to the user.
For example, if we define the help we want to provide for an application panel in a help panel with the
NAME value help01, we would specify that help panel like this in the PANEL definition:
<panel name=panel01 width=60 depth=18 help=help01>Application
The help panel help01 would appear when the user requests help for that application panel.
Like the PANEL tag, the HELP tag has WIDTH and DEPTH attributes that define the dimensions of the
panel. However, help panels differ from application panels. If the DEPTH attribute is specified on the
AREA tag, a single panel is created with a scrollable section to allow the display of longer sections of help
Help panels
130  z/OS: z/OS ISPF DTL Guide

## Page 163

text. Otherwise, the conversion utility generates as many help panels as needed (up to 37) for the help
text content you define. This means that you can define text for a help panel that exceeds the defined
depth, and, even though the text may not appear in the initial display of the panel, the user can view
the text through page scrolling. Examples of both types of help panel scrolling are shown in “A scrollable
panel” on page 132 and “Multiple panels in sequence” on page 134.
Because ISPF displays all DTL-defined help panels in pop-ups, the WIDTH and DEPTH values you specify
must allow for the addition of two lines (depth) and 4 characters (width) for pop-up borders. Therefore,
WIDTH=76 and DEPTH=22 are the maximum values that can be used with 80-by-24 display devices. The
HELP panel default values are WIDTH=50 and DEPTH=10.
Typically, you would define help panel values of WIDTH=60 and DEPTH=22 or less. The specified depth
must include allowance for the panel title line and its separator. A help panel that does not end with a
scrollable area also reserves four lines for the function key area.
Defining help panel text
The text you define for help panels cannot be modified by the user; it is for information purposes only. To
define this text, use an information region and the tags associated with information regions. The INFO tag
and its matching end tag are required in help panel definitions.
You can also use AREA definitions within help panels. Remember to code the entire INFO definition (start
and end tag) within the AREA definition, just as you would on an application panel. Here is an example:
<help name=help01>Help Panel Title
  <area>
    <info>
⋮
    </info>
  </area>
</help>
You can use any of the information region tags discussed in this chapter in a help panel. For example, you
use the P (paragraph) tag to define a paragraph of text on a help panel the same way you use it to define a
paragraph on an application panel.
Here is a help panel markup that has two paragraphs, an unordered list, a figure and figure caption to
define the help text. The specification of DEPTH=28 is valid only if the display terminal has 30 or more
display lines. Figure 71 on page 132 shows the formatted result.
<!doctype dm system>
<help name=olcthlp depth=28 width=50>Help for Online Catalog
<area>
<info>
<p>The Online Catalog provides
you with:
<ul compact>
<li>The book title
<li>Catalog number
<li>Page count
<li>The author
<li>A brief description
</ul>
<p>Here is an example:
<fig>
  The Yellow Subroutine
  365 Pages              1234.56
  John-Paul Georgenringo
  A young band of British programmers embarks on
  a voyage across a perilous "sea" language in
  search of FORTRAN and fame.
<figcap>Online Catalog Example
</fig>
</info>
</area>
</help>
Help panels
Chapter 6. Information regions and help panels  131

## Page 164

Help for Online Catalog
 The Online Catalog provides you with:
 o   The book title
 o   Catalog number
 o   Page count
 o   The author
 o   A brief description
 Here is an example:
 ------------------------------------------------
   The Yellow Subroutine
   365 Pages              1234.56
   John-Paul Georgenringo
   A young band of British programmers embarks on
   a voyage across a perilous "sea" language
   in search of FORTRAN and fame.
 ------------------------------------------------
 Online Catalog Example
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 71. Help panel
In Figure 71 on page 132, all of the text was displayed because the depth we defined for the help panel
was large enough to accommodate the text. However, the amount of help you want to provide for your
users can vary, and it's not always possible to display all of the help text you define in the initial panel
display, especially when you don't, or can't, specify a large DEPTH value for the help panel.
Depending on the use of the AREA tag, the conversion utility generates multiple panels or a single
scrollable help panel.
This help panel markup includes an information region that contains a paragraph, a definition list, and two
unordered lists nested within the definition list.
A scrollable panel
The addition of the DEPTH attribute on the AREA tag illustrates a scrollable panel.
<!DOCTYPE DM SYSTEM>
<help name=helpscr width=46 depth=16>ShelfBrowse for Kids
<area depth=10>
  <info>
    <p>ShelfBrowse can help you
    find any kind of book you are looking for.
    The two main categories for books are:
    <dl tsize=12>
      <dthd>Book
      <ddhd>Description
      <dt>Fiction
      <dd>Fiction books are stories
      that never really happened.
      The writer made them up.
      For example:
        <ul>
          <li>Fairy Tales
          <li>Mysteries
          <li>Science fiction stories
        </ul>
      <dt>Nonfiction
      <dd>Nonfiction books are about
      things that really exist.
      For example:
        <ul>
          <li>History books
          <li>Reference books
          <li>How-to books
Help panels
132  z/OS: z/OS ISPF DTL Guide

## Page 165

</ul>
    </dl>
  </info>
</area>
</help>
When initially displayed, the first part of the scrollable text is visible. For this example, to scroll down,
place the cursor on the first or last displayed line of text, and press Enter or the RIGHT (F11) key. Use the
LEFT (F10) key to scroll up.
 HELPSCR     ShelfBrowse for Kids
                                  More:     +
 ShelfBrowse can help you find any kind of
 book you are looking for. The two main
 categories for books are:
 Book         Description
 Fiction      Fiction books are stories that
              never really happened.  The
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 72. Help panel (example 1 of 4)
After you scroll down, this panel appears:
 HELPSCR      ShelfBrowse for Kids
                                   More:   - +
              never really happened. The
              writer made them up. For
              example:
              o   Fairy Tales
              o   Mysteries
              o   Science fiction stories
   F1=Help         F3=Exit         F5=Exhelp
   F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
  F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 73. Help panel (example 2 of 4)
After you scroll down, this panel appears:
 HELPSCR      ShelfBrowse for Kids
                                   More:    - +
              o   Science Fiction stories
 Nonfiction   Nonfiction books are about
              things that really exist. For
              example:
              o   History books
              o   Reference books
   F1=Help         F3=Exit         F5=Exhelp
   F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
  F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 74. Help panel (example 3 of 4)
After you scroll down, this panel appears:
Help panels
Chapter 6. Information regions and help panels  133

## Page 166

HELPSCR     ShelfBrowse for Kids
                                  More:   -
 Nonfiction  Nonfiction books are about
             things that really exist. For
             example:
             o   History books
             o   Reference books
             o   How-to books
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 75. Help panel (example 4 of 4)
There is only one additional line to display, so the scroll has moved the scrollable text up only one line.
Multiple panels in sequence
If no AREA tag is present or the AREA tag does not contain the DEPTH attribute, multiple help panels are
generated. ISPF simulates scrolling by displaying the set of multiple help panels in sequence.
If the help panel contains additional text, the conversion utility provides an indicator at the top of the
panel to notify the user. If additional text exists, the text More: is displayed followed by a + sign. Following
scrolling, if additional text stills exists, the indicator displays as “More:   - +”, indicating scrolling is
possible in either direction. If, following scrolling, no more text is available through scrolling forward, but
text is available by scrolling backward, the indicator displays as “More:   -”. Scrolling function keys are
defined by tutorial processing.
Here is markup that uses the previous example without a DEPTH attribute on the AREA tag to generate
multiple help panels. Because all of the data does not fit in one help panel, the conversion utility created
three panels: HELPSB, HELPSBX0, and HELPSBX1. The panels are displayed individually by tutorial
processing. Figures Figure 76 on page 135, Figure 77 on page 135, and Figure 78 on page 135 show
the formatted results with the function key area displayed in its short form.
<!DOCTYPE DM SYSTEM>
<help name=helpsb width=46 depth=16>ShelfBrowse for Kids
<area>
  <info>
    <p>ShelfBrowse can help you
    find any kind of book you are looking for.
    The two main categories for books are:
    <dl tsize=12>
      <dthd>Book
      <ddhd>Description
      <dt>Fiction
      <dd>Fiction books are stories
      that never really happened.
      The writer made them up.
      For example:
        <ul>
          <li>Fairy Tales
          <li>Mysteries
          <li>Science fiction stories
        </ul>
      <dt>Nonfiction
      <dd>Nonfiction books are about
      things that really exist.
      For example:
        <ul>
          <li>History books
          <li>Reference books
          <li>How-to books
        </ul>
    </dl>
  </info>
</area>
</help>
Help panels
134  z/OS: z/OS ISPF DTL Guide

## Page 167

HELPSB      ShelfBrowse for Kids
                                  More:     +
 ShelfBrowse can help you find any kind of
 book you are looking for. The two main
 categories for books are:
 Book        Description
 Fiction     Fiction books are stories that
             never really happened. The
             writer made them up. For
             example:
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 76. Help panel (example 1 of 3)
 HELPSBX0    ShelfBrowse for Kids
                                  More:   - +
             o   Fairy Tales
             o   Mysteries
             o   Science fiction stories
 Nonfiction  Nonfiction books are about
             things that really exist. For
             example:
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 77. Help panel (example 2 of 3)
 HELPSBX1    ShelfBrowse for Kids
                                  More:   -
             o   History books
             o   Reference books
             o   How-to books
  F1=Help         F3=Exit         F5=Exhelp
  F6=Keyshelp     F7=PrvTopic     F8=NxtTopic
 F10=PrvPage     F11=NxtPage     F12=Cancel
Figure 78. Help panel (example 3 of 3)
You can use any of the tags provided for information regions to define the text of the information regions
in your help panels.
Help panels
Chapter 6. Information regions and help panels  135

## Page 168

Help panels
136  z/OS: z/OS ISPF DTL Guide
