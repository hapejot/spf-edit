# Chapter 1. Introduction to the Dialog Tag Language (DTL)

Source file: f54dt00_v3r1.md
Start page: 35
Page span: 35-42

## Page 35

Chapter 1. Introduction to the Dialog Tag Language
(DTL)
The Dialog Tag Language (DTL) is a tag-based language used to define many of the elements that make up
the type of application known as a dialog, the communication between a person and a computer. You can
define these elements using DTL, and use them in your ISPF applications.
The elements you produce with DTL are used by ISPF as the user interface for your ISPF applications. The
programs you write using ISPF services and a programming language use the dialog elements you create
for an application.
The overview of DTL, and the dialog elements you create with DTL, are provided in these topics:
• Why the Dialog Tag Language?
This topic explains why you would want to use DTL to create elements for ISPF applications.
• What is the Dialog Tag Language?
This topic explains what the Dialog Tag Language is and how it works.
• Dialog elements
This topic explains and illustrates the dialog elements. These elements are:
– Application panels
– Help panels
– Messages
– An application command table
– Key mapping lists.
• Variables and variable classes
This topic discusses the definition of variables you include in dialog element definitions.
• What is the ISPF Conversion Utility?
This topic describes the conversion utility, the compiler you use to convert your DTL source files for use
by ISPF.
Why the Dialog Tag Language (DTL)?
If you are already familiar with a tag-based markup language, such as HTML (Hypertext Markup
Language), you will find that DTL is very similar. IBM created DTL for many of the same reasons that
HTML was created:
• Markup tags are easy to use. Because tag names are short and relate directly to the structure of the
dialog elements, they are also easy to remember.
• DTL lends flexibility to application development. Panels can be quickly changed without your having
to tediously line up text and fields. This gives you greater control over application development and
updates.
• DTL provides consistency when many programmers are working on the same application, or when
programmers who are new to your company must update existing applications. Since each programmer
is using the same tags, only minor adjustments may be needed to achieve complete uniformity.
• DTL techniques improve the way in which interactive programs, like ISPF applications, are developed.
The language concentrates on the role of the various elements and their interrelationships, and ISPF
takes care of their form and appearance at run time.
Why the Dialog Tag Language (DTL)?
© Copyright IBM Corp. 1989, 2024 3

## Page 36

• DTL also enforces some formatting rules defined by the Systems Application Architecture® Common
User Access (CUA), so you do not have to be familiar with all of the CUA formatting rules. Therefore, the
CUA skills required by programmers who are developing CUA-conforming applications are significantly
reduced.
• DTL enables multicultural support and the conversion utility provides NLS translations for certain key
words. 
In other words, DTL is an application development and maintenance system that is sophisticated, flexible,
and easy to use.
Examples of DTL usage by ISPF are provided in data set ISP.SISPGxxx, where xxx is a standard ISPF
language identifier. Consult your ISPF system administrator for the actual location of these examples.
What is the Dialog Tag Language?
In “Why the Dialog Tag Language (DTL)?” on page 3 we referred to DTL as a tag-based markup language
that is similar to HTML. The two have much in common. For example, markup is a term that is usually
associated with documentation. It is an old typesetting term that formerly meant the instructions with
which a document was "marked up" to show how the document should be set in type.
Today, this definition has been expanded to include information that is added to a document to enable
a person or system to process it. Just as markup information can describe a document's characteristics
or the processing to be performed, it can also describe the characteristics or processing related to dialog
elements. This is where the tags come in.
We call DTL a tag-based markup language simply because the markup consists of tags that determine not
only what each element is, but also how it is processed. To convert the dialog elements into a format that
is usable by ISPF, you must convert them to ISPF elements with ISPDTLC, the ISPF conversion utility. (See
“What is the ISPF conversion utility?” on page 9 for more information.)
Another thing that DTL and HTML have in common, of course, is the tags themselves, which have these
similarities:
• They are very short and easy to remember.
• They are often accompanied by text.
• Many DTL tags are almost identical to corresponding HTML tags.
These are all reasons that familiarity with HTML makes it easy to learn DTL. The preceding bulleted, or
unordered, list can be created in both HTML and DTL using this tagging:
<ul>
  <li>They are very short and easy to remember.</li>
  <li>They are often accompanied by text.</li>
  <li>Many DTL tags are almost identical to corresponding HTML tags.</li>
</ul>
Here, the <ul> and </ul> tags, respectively, begin and end the unordered list. This type of list is called
an unordered list because the list items are not numbered. The individual list items are defined by the
<li> tags and consist of the accompanying text.
As you can see from the preceding example, DTL tags act as control words that specify how the text of
source files is interpreted by the conversion utility. This concept is based on the Standard Generalized
Markup Language (SGML), which is a standard of the International Standards Organization (ISO). The
conventions of the Dialog Tag Language are based on the SGML standard.
After you are finished marking up a source file, use the conversion utility to convert the file into a format
usable by your ISPF application. In addition to processing the file, the conversion utility also checks and
verifies the syntax of your markup, and notifies you of any errors. After conversion, the elements you
defined in your source file are stored within ISPF libraries.
You can use ISPF dialog test facilities to display application panels and messages after they have been
converted. Displaying your panels is a good idea to make sure they format properly.
What is the Dialog Tag Language?
4  z/OS: z/OS ISPF DTL Guide

## Page 37

You should now have a basic understanding of DTL and how it works. The next section builds on this
understanding by describing the types of elements that you can define with DTL.
Dialog elements
This topic provides a descriptive overview of the dialog elements you can create for an ISPF application.
These elements include:
• “Application panels” on page 5
• “Help panels” on page 6
• “Messages” on page 7
• “Application command table” on page 8
• “Key mapping lists” on page 8
Application panels
Application panels are the primary element of the user interface for an application. They allow users to
interact with your application through the use of data fields, selection fields, and other interactive fields.
Application panels appear in primary and pop-up windows.
Figure 2 on page 5 shows a full-screen application panel. Following that is a list of the elements that
make up an application panel.
   File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number if applicable.
 Then, select an action bar choice.
 Date . . . : 12/29/90
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
Figure 2. Application panel
Application panel elements
Action bar
The action bar appears in the top portion of the panel. It contains keyword choices that provide
users access to available actions for the current panel. When the user selects an action bar choice, a
pull-down containing choices appears directly below the action bar choice.
Panel title
The panel title appears below the action bar.
Dialog elements
Chapter 1. Introduction to the Dialog Tag Language (DTL)  5

## Page 38

Panel body
The panel body serves as the main work area of the panel. The panel body contains the input and
output fields, selection fields, and other text.
Additionally, the panel body can contain optional top and bottom instructions, which provide
instructional text to the user. Top instructions appear below the panel title and above the interactive
fields on the panel. Top instructions tell the user how to interact with the panel. Bottom instructions
appear below the interactive fields on the panel. Bottom instructions tell the user how to interact with
the panel, or how to continue with the application.
Message area
ISPF uses the message area (or message pop-ups) to display messages to users while they are
working in the panel.
Command area
The optional command area (or command line) consists of two components: the command field
prompt and the command entry field. Application users can use the command entry field to enter
commands or requests to the ISPF application.
Function key area
The optional function key area, which appears at the bottom of the panel immediately below the
command area (if one is defined), contains the key assignments for dialog actions valid for the
application panel. The user can request that function keys not be displayed.
Note: The message area and the command area for panels defined with DTL appear at the bottom of
the panel if the user has selected the "Command line at bottom" option on the ISPF Settings panel, or
the application has set ZPLACE to BOTTOM. For more information on placement options, refer to the
discussion of the ISPF Settings panel in the z/OS ISPF Dialog Developer's Guide and Reference.
Chapter 3, “Getting started: designing application panels,” on page 27 tells you how to define
application panels and panel elements.
Help panels
Help panels appear in pop-up windows in response to user requests for assistance during ISPF
application sessions. ISPF processes these help requests and displays the help panels.
Using DTL, you can create help panels that provide help for:
• An entire application panel (extended help or panel help)
• A specific field on an application panel (contextual help or field help)
• Messages (message help)
• The function key area (keys help).
Figure 3 on page 7 illustrates a help panel. Following that is a list that defines each of the elements that
make up a help panel.
Dialog elements
6  z/OS: z/OS ISPF DTL Guide

## Page 39

Figure 3. Help panel
Help panel elements
Help panel title
The help panel title appears at the topmost portion of the panel, followed by a blank line separating
the panel title from the panel body. If the help panel text exceeds the defined depth of the help panel,
a scrolling indicator appears in the right margin of the blank line following the panel title.
Help panel body
The help panel body contains the text of the help panel.
Text within the help panel is protected, which means that the user cannot interact with the text. You
define this static text within an information region in the panel definition.
Function key area
If you are creating a help panel that does not end with a scrollable area, note that ISPF reserves
4 lines at the bottom of the panel for function keys. The display of keys in the function key area is
controlled by the user through the ISPF FKA command.
Chapter 6, “Information regions and help panels,” on page 101 tells you how to define help panels and
information regions.
Messages
You can use DTL to define messages that display in response to a user request or action, or that provide
additional information. Messages can confirm a user-requested action, report an error in user input, or
notify the user of an error or exception condition. Figure 4 on page 8 illustrates a message displayed in
the message area of an application panel (highlighting added).
Dialog elements
Chapter 1. Introduction to the Dialog Tag Language (DTL)  7

## Page 40

File  Search  Help
 --------------------------------------------------------------------------
                         Library Card Registration
 Type in patron's name and card number if applicable.
 Then, select an action bar choice.
 Date . . . : 12/29/90
 Card No. . . _______  (A 7-digit number)
 Name . . . . _________________________  (Last, First, M.I.)
 Address  . . _________________________
 Choose one of the following           Check valid branches
 __  1.  New                           _  North Branch
     2.  Renewal                       _  South Branch
     3.  Replacement                   _  East Branch
                                       _  West Branch
 You must type your name in the Name field.
 Enter a command ===> ______________________________________________________
  F1=Help        F2=Split       F3=Exit        F6=KEYSHELP    F9=Swap
 F12=Cancel
Figure 4. Message displayed in message area
The messages you define for an application are stored within message members. You use DTL to define
the messages and message members.
Chapter 7, “Messages,” on page 137 provides a complete description of defining messages and message
members.
Application command table
You can use DTL to define commands that perform actions requested by the user. The valid commands for
an application are defined and stored within an internal application command table. You can define only
one command table for an application.
Valid commands include those assigned to pull-down choices, function keys, and commands entered in
command entry fields.
Chapter 8, “The application command table,” on page 143 tells you how to define commands and
application command tables.
Key mapping lists
The key assignments that are active for an application are defined and stored within key mapping lists.
These key assignments allow the user to request commands and other actions through the use of function
keys. Key assignments for your application are displayed in the function key area of application panels.
Chapter 9, “Defining key mapping lists,” on page 147 tells you how to define key assignments and key
mapping lists.
Variables and variable classes
Variables are used to communicate information between an application and the user. Each variable you
define for a DTL-defined dialog element can be declared, or identified, within a variable list. In addition,
each variable can be associated with a variable class that defines its type and length characteristics. The
variable class can also be used to define translations and validity checks that are used when a value is
displayed on a panel or entered by a user.
Chapter 4, “Variables and variable classes,” on page 53 tells you how to declare variables and define
variable classes.
Variables and variable classes
8  z/OS: z/OS ISPF DTL Guide

## Page 41

What is the ISPF conversion utility?
ISPDTLC is the ISPF conversion tool that converts Dialog Tag Language (DTL) source files to ISPF panel
language source format or executable preprocessed ISPF format. ISPF provides you with an invocation
panel that allows you to specify a number of options for the conversion, or you can use conversion utility
command syntax from the command line of your terminal. Chapter 10, “Using the conversion utility,” on
page 151 provides a complete description of both methods.
What is the ISPF conversion utility?
Chapter 1. Introduction to the Dialog Tag Language (DTL)  9

## Page 42

What is the ISPF conversion utility?
10  z/OS: z/OS ISPF DTL Guide
