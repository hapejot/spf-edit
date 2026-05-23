# Chapter 7. ISPF help and tutorial panels

Source file: f54dg00_v3r1.md
Start page: 285
Page span: 285-292

## Page 285

Chapter 7. ISPF help and tutorial panels
Online help and tutorial panels are a set of panels that a developer can include to provide online
information for an application user. Help and tutorial panels can contain information that is helpful to
a first-time user. They also can instruct a user to take specific actions based on a particular condition that
has occurred during the application processing.
All ISPF help panels that are created using the Dialog Tag Language display in a pop-up window. ISPF
help panels created using the ISPF panel source statements and containing the WINDOW keyword on the
panel's )BODY statement also display in a pop-up window. If field-level help is being displayed, the ISPF
help facility attempts to position the pop-up window relative to the object field.
The width and depth values specified on the HELP tag or on the WINDOW keyword must be valid for
the device on which these help panels are displayed. See the z/OS ISPF Dialog Tag Language Guide and
Reference for details on the HELP tag. For details on the WINDOW keyword, see WINDOW(width,depth).
You can provide several types of help or tutorial panels. The ISPF tutorial is included with the product.
Extended help (panel help)
Provides general information about the contents of a panel. The information in extended help can be
an overall explanation of items on the panel, an explanation of the panel's purpose in the application,
or instructions for the user to interact with the panel.
See the description of the .HELP variable in “.HELP variable” on page 252 for more information.
Field-level help
Provides help panels for fields defined on an application panel.
When the user enters the HELP command, ISPF displays the help panel defined for the field on which
the cursor is located.
You may define field-level help for action bar choices and pull-down choices, as well as for fields
within the panel body. If you are creating panels with field level help using Dialog Tag Language, refer
to the z/OS ISPF Dialog Tag Language Guide and Reference for a description of the tag attributes you
should use. Otherwise, for more information about defining the )HELP section of the panel, refer to
“Defining the HELP section” on page 182.
HELP FOR HELP
Provides help for using the help or tutorial facility.
Keys help
Provides a brief description of each key defined for a panel. See “Keys help” on page 83 for more
information about keys help.
Message help
Provides help for ISPF messages. See “How to define a message” on page 266 for more information.
Reference phrase help
Provides help for reference phrases. See “Reference phrase help” on page 83 for more information.
Tutorial
Describes the ISPF product. The tutorial is included with ISPF. See “The ISPF tutorial panels” on page
260 for more information.
TUTOR command
Provides a direct path to specific tutorial panels, in effect indexing Help hierarchies by panel
identifiers.
Processing help
You can request help from an application panel or a help panel. You can also specify a keylist to be
associated with a help panel.
© Copyright IBM Corp. 1980, 2025 257

## Page 286

Help requests from an application panel
When the user enters the HELP command, ISPF displays a help or tutorial panel according to this
sequence:
1. When a short message appears on an application panel and the user requests HELP, ISPF displays the
long message.
2. If a long message is on the screen and the user requests HELP, ISPF checks to see if message help is
defined.
• If message help is defined, ISPF displays that panel. If the user requests help from the message help
panel, the Help Tutorial panel is displayed.
• If message help is not defined, ISPF checks to see if field-level help is defined for the field on which
the cursor is located.
– If field-level help is defined, ISPF displays that panel. If the user requests HELP from the field-
level help panel, the Help Tutorial panel is displayed.
– If field-level help is not defined, ISPF checks for panel help.
- If panel help is defined, ISPF displays that panel. If the user requests HELP from the panel help
panel, the Help Tutorial panel is displayed.
- If panel help is not defined, ISPF displays the first panel within the application's tutorial.
3. When an application panel has been displayed and the user requests HELP, ISPF checks to see if
field-level help is defined for the field on which the cursor is located.
• If field-level help is defined, ISPF displays that panel. If the user requests HELP from the field-level
help panel, the Help Tutorial panel is displayed.
• If field-level help is not defined, ISPF checks for panel help.
– If panel help is defined, ISPF displays that panel. If the user requests HELP from the panel help
panel, the Help Tutorial panel is displayed.
– If panel help is not defined, ISPF displays the first panel within the application's tutorial.
Figure 70 on page 259 illustrates the panel flow for help according to the ISPF search sequences.
258  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 287

Figure 70. Help panel flow
Keys help request from an application panel
When an application panel is displayed and the user requests KEYSHELP, ISPF displays the keys help
panel (provided that keys help is defined).
If the panel contains a short message or long message and the user requests KEYSHELP, ISPF displays
the keys help panel without following the search sequence as illustrated in Figure 70 on page 259.
Extended help request from an application panel
When an application panel is displayed and the user requests EXHELP, ISPF displays the extended help
panel (provided that extended help is defined).
If the panel contains a short message or long message and the user requests EXHELP, ISPF displays the
extended help panel without following the search sequence as illustrated in Figure 70 on page 259.
Chapter 7. ISPF help and tutorial panels  259

## Page 288

Help available from a help panel
This list describes the ISPF help facilities available when a help panel or tutorial panel is displayed:
• If the user requests HELP from any help or tutorial panel, ISPF displays the help for help panel defined
by the .HHELP control variable. If the variable is not defined, then ISPF displays the Help Tutorial panel.
• If the user requests EXHELP from any help or tutorial panel (except from the extended help panel), ISPF
displays extended help.
• If the user requests KEYSHELP from any help or tutorial panel (except the keys help panel), ISPF
displays keys help.
• If the help panel contains a reference phrase, and the user requests HELP while the cursor is positioned
on a reference phrase, ISPF displays the reference phrase help panel defined. When a reference phrase
help panel is canceled, the help panel from which reference phrase help was requested is redisplayed.
All other help facilities are available from a reference phrase help panel.
Ending help
When the user requests END or EXIT from any help panel (except the Help Tutorial panel), ISPF returns to
the original application panel. If the user requests END or EXIT from the Help Tutorial panel, ISPF returns
to the previous panel.
If the user requests CANCEL from any help or tutorial panel, ISPF returns to the previous panel.
ISPF default keylist for help panels
You can specify a keylist to be associated with a help panel by using the keylist attribute on the HELP tag
(DTL) or by using the )PANEL statement in your panel definition. If you do not specify a keylist, ISPF uses
the keys defined for ISPHELP to display in the function area of the help panel when it is displayed.
The key settings and forms for ISPHELP are shown in Table 23 on page 260. For more information about
keylists, refer to the "Settings (option 0)" topic in the z/OS ISPF User's Guide Vol II.
Table 23. ISPHELP key settings
Key Command Form
F1 HELP Short
F2 SPLIT Long
F3 EXIT Short
F5 EXHELP Short
F6 KEYSHELP Short
F7 UP Long
F8 DOWN Long
F9 SWAP Long
F10 LEFT Long
F11 RIGHT Long
F12 CANCEL Short
The ISPF tutorial panels
A tutorial panel is a special type of panel that is processed by the ISPF tutorial program. This program
invokes the panel display service to display the panel.
A user invokes the ISPF program that displays tutorial panels in four ways:
260  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 289

• As an option from a menu
• Directly or indirectly from any non-tutorial panel by entering the HELP command or by pressing the
function key assigned to the HELP command.
• By selecting a choice from a Help pull-down
• Through the use of the TUTOR command
Transfer into and out of the tutorial using the HELP command is transparent (no action required) to ISPF
functions.
ISPF tutorial panels are arranged in a hierarchy. Generally, this hierarchy is a table of contents, each
succeeding level of which contains a more detailed list of topics. When the tutorial is entered from a
menu, the first panel to be displayed is usually the top of the hierarchy. The name of the first panel is
passed as a parameter to the ISPTUTOR program.
When the tutorial is entered by use of the HELP command, the first panel to be displayed is a panel within
the hierarchy, appropriate to what you were doing when help was requested.
When viewing the tutorial, you can select topics by entering a selection code or by simply pressing Enter
to view the next topic. On any panel, you can also enter these commands:
BACK or B
To return to the previously viewed panel
SKIP or S
To advance to the next topic
UP or U
To display a higher-level list of topics
TOC or T
To display the table of contents
INDEX or I
To display the tutorial index
Note: If you enter the UP command after viewing a portion of a tutorial sequentially and if you do not
select a new topic from the displayed list, you can resume the tutorial at the next sequential topic on the
list by entering the NEXT command or by pressing Enter.
You can use these keys whenever you are in the tutorial:
ENTER
To display the next sequential page or scroll a scrollable help panel
HELP
To redisplay this page for help information
END
To terminate the tutorial
UP
To display a higher level list of topics (rather than typing UP)
DOWN
To skip to the next topic (rather than typing SKIP)
RIGHT
To display the next page (rather than pressing Enter) or to scroll a scrollable help panel
LEFT
To display the previous page (rather than typing BACK) or to scroll a scrollable help panel
When running under tutorial and trying to scroll past the end of the scrollable area, a message will be
displayed indicating that no more information is available in the scrollable area. If RIGHT or ENTER is
pressed again, ISPF will follow the normal tutorial flow and display the next help panel if one has been
defined. The same is true when scrolling to the TOP of the scrollable AREA; a message indicating that no
more information is available will be displayed, and if LEFT is pressed, the previous tutorial panel will be
displayed if one has been defined.
Chapter 7. ISPF help and tutorial panels  261

## Page 290

Cursor positioning usually defines which scrollable area will be scrolled. However, when in tutorial, if the
cursor is not within a scrollable area, the first area defined in the )BODY section will be scrolled. The LEFT
and RIGHT commands should be included in any keylist specified for a scrollable help panel.
If you issue the HELP command while viewing a tutorial, ISPF displays a tutorial panel that contains a
summary of commands that are available to the tutorial user.
When you end the tutorial, using the END or RETURN command, the panel from which you entered the
tutorial is displayed again.
The name of the top panel must be specified by dialog variable ZHTOP. The name of the first index panel
must be specified by ZHINDEX. It is recommended that these two dialog variables be initialized at the
beginning of the application to ensure that the user can always display the tutorial top or index, regardless
of how the tutorial was entered. One way to initialize these variables is to set them from the primary
option menu, as shown in “Example of a primary option menu” on page 107.
The index is optional. It is a collection of panels in which topics are arranged in alphabetical order. You
can jump to the index from any point by using the INDEX command. The index need not be connected to
the main tutorial hierarchy. It can be a topic that you can select from the main table of contents or other
panels. A list of the last 20 tutorial panels displayed, including the current panel, is maintained by ISPF.
You should issue the TOP or INDEX command instead of the BACK command if you want to view panels
displayed before the last 20 panels.
Each tutorial panel must have a next selection input field. Generally, you should use the name ZCMD for
this field. A tutorial panel should also have a processing section in which these variables are set:
ZSEL or SEL
Specifies the name of the next panel to be displayed based on the topic selected by the user, by
translating ZCMD to a panel name. The panel name can be preceded by an asterisk (*) to indicate a
topic that can be explicitly selected by the user, but which is bypassed if the user presses Enter to
view the next topic.
The maximum number of entries allowed is 100.
If a panel does not have any selectable topics, omit ZSEL.
ZUP or UP
Specifies the name of the parent panel from which this panel was selected. Generally, ZUP can be
omitted since the tutorial program remembers the sequence of selections that lead to the display
of this panel. ZUP is used only if this panel is the first to be displayed by a user entering the HELP
command, or if it is selected from the tutorial index and the user then enters the UP command.
ZUP is ignored when it is defined in the top panel (defined by ZHTOP).
ZCONT or CONT
Specifies the name of the next continuation panel. If there is no continuation panel, ZCONT should be
omitted.
ZIND
When set to a value of YES, specifies that a page in the tutorial is an index page. For example:
)PROC
  &ZIND = YES
The ZIND variable is used only on index pages; it should not be set on other tutorial panels.
Use variable names ZSEL, ZUP, and ZCONT. Variables SEL, UP, and CONT are provided only for
compatibility with the previous SPF product.
A panel cannot have both a continuation panel and selectable topics. However, the last panel in a
sequence of continuation panels can have selectable topics.
Help/tutorial panels can contain variables so that dialog information, including information entered by a
user, can be displayed on the help panel. Function variables, as well as shared and profile variables, can
be displayed.
262  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 291

Figure 71 on page 263 shows a sample hierarchy of tutorial panels. Panels A and B have three selectable
topics each. Panels C and D2 have two selectable topics each. The other panels have no selectable topics.
Panel D1 has a continuation page (D2), and panel F1 has two continuation pages (F2 and F3).
In Figure 71 on page 263, assuming that panel A is the highest-level table of contents, the viewer can get
to A from any point by issuing the TOC command. A viewer currently on panel F1, F2, or F3 can return to
panel B by issuing the BACK command. Then, from B, the SKIP command would take the viewer to panel
C. If the user enters the TUTOR command along with a panel identifier parameter, a specific tutorial panel
within the Help hierarchy is displayed. From that point on, any movement within the hierarchy is the same
as if the user had reached the panel by any other means.
Figure 71. Sample tutorial hierarchy
Two sample tutorial panels are shown in Figure 72 on page 263 and Figure 73 on page 264. These are
assumed to be panels B and F2, respectively, in the hierarchy in Figure 71 on page 263.
 %TUTORIAL ------------------ 3270 DISPLAY TERMINAL --------------------TUTORIAL
 %NEXT SELECTION ===>_ZCMD                                                     +
+
 %                     -----------------------------------
                       |       General Information       |
                       |         3270 Key Usage          |
                       -----------------------------------
 +
    The IBM 3270 display terminal has several keys which will assist you
    in entering information.  These are hardware defined keys; they do not
    cause a program interruption.
+
    The following topics are presented in sequence,
    or can be selected by number:
+
      %1+ Insert and Delete Keys
      %2+ Erase EOF (to End-of-Field) Key
+
    The following topic will be presented only if
    explicitly selected by number:
+
      %3+ New Line and TAB Keys
+
 )PROC
   &ZSEL = TRANS(&ZCMD  1,E  2,F1  3,*G  *,'?')
   &ZUP  = A
 )END
Figure 72. Sample tutorial panel definition  (panel B)
Panel B has three selectable topics. In the processing section, ZCMD is translated to a panel name (E, F1,
or G) corresponding to the selected option, and the result is stored in ZSEL. If none of the valid options
is selected, a question mark (?) is returned as the translated string, which causes the tutorial program to
display an invalid option message.
Chapter 7. ISPF help and tutorial panels  263

## Page 292

Note that option 3 is translated to *G. This indicates that panel G is displayed if the user selects option
3, but is bypassed if the user repeatedly presses Enter to view each topic. The order in which topics are
presented when Enter is pressed is the same as the order in which they appear in the TRANS function. If
option 3 is selected, pressing the Enter key does not display the other topics.
In panel B, the name of the parent panel (A) is stored in variable ZUP.
 %TUTORIAL -------------------- ERASE EOF KEY ------------------- TUTORIAL
 %NEXT SELECTION ===>_ZCMD                                                     +
 +
    When the erase EOF (erase to end-of-field) key is used, it will appear
    to blank out the field.  Actually, null characters are used in erasing
    to the next attribute byte, thus making it easy to use the insert
    mode, which requires null characters.
 +
    If the erase EOF key is pressed when the cursor is not within an input
    field, the keyboard will lock.  Press the RESET key to unlock the
    keyboard.
 +
    You can try out the erase EOF key by entering data on line 2, then
    moving the cursor back over part or all of the data and pressing the
    key.
 +
                         (Continued on next page)
 +
 )PROC
   &ZCONT = F3
 )END
Figure 73. Sample tutorial panel definition  (panel F2)
Panel F2 (Figure 73 on page 264) has no selectable topics, but does have a continuation page. The name
of the continuation panel (F3) is stored in variable ZCONT. The name of the parent panel (B) could have
been stored in ZUP, but this was omitted assuming that F2 cannot be directly entered by use of the HELP
command or from the tutorial index.
If you call ISPTUTOR from an edit macro, be sure to save and restore the environment at that point. For
example:
ISREDIT MACRO
ISPEXEC CONTROL DISPLAY SAVE
ISPEXEC SELECT PGM(ISPTUTOR) PARM(panel-id)
ISPEXEC CONTROL DISPLAY RESTORE
EXIT
264  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
