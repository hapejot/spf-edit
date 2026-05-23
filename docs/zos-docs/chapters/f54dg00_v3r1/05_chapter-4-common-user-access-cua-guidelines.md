# Chapter 4. Common User Access (CUA) guidelines

Source file: f54dg00_v3r1.md
Start page: 107
Page span: 107-114

## Page 107

Chapter 4. Common User Access (CUA) guidelines
This topic briefly describes how ISPF supports the Common User Access (CUA) guidelines. The CUA
guidelines define a user interface in terms of common elements, such as the way information appears on
a screen, and interaction techniques, such as the way users respond to what appears on a screen. See the
SAA CUA Basic Interface Design Guide via BookManager® tools.
ISPF supports the CUA guidelines in several ways. You can:
• Define a list of function keys to be associated with each panel.
• Define an action bar and pull-downs on a panel.
• Define and display pop-up windows.
• Define and display help panels for field-level help, extended help, and keys help. See Chapter 7, “ISPF
help and tutorial panels,” on page 257 for more information about CUA help panels.
With ISPF, the panel ID is displayed according to CUA defaults and the PANELID command acts as a
toggle.
ISPF also lets you indicate, for an application session, if you want to use CUA defaults. If selected, the
Panel display CUA mode option on the ISPF Settings panel controls:
• The location of the function keys on the panel in relation to the command and message lines.
• The appearance and display format of the keys.
Using the dialog tag language to define dialog elements
The Dialog Tag Language (DTL) is a set of markup language tags that you can use to define dialog
elements. You can use DTL tags in addition to or instead of ISPF methods for defining panels, messages,
and command tables. In addition, when you define a panel using DTL tags, you can assign a specific
keylist to be associated with and displayed on that panel, if requested by the user.
The DTL defines the source information for the dialog elements, and the ISPF dialog tag language
conversion utility converts the source file to a format ISPF understands. The z/OS ISPF Dialog Tag
Language Guide and Reference explains in detail how to create the various elements using the DTL and
ISPF conversion utility.
Keylists
The key assignments active for an application panel are defined and stored within keylists. These key
assignments allow the user to request commands and other actions through the use of function keys.
Key assignments for your application are displayed in the function key area of application panels. Keylists
can be shared across all users by defining them using DTL. This creates an xxxxKEYS table that is placed
in the ISPTLIB concatenation. Users can modify keylists using the KEYS and KEYLIST commands. Both
commands invoke the Keylist utility. Modifications to keylists are stored in the user's application profile,
thus they are called private.
You can view or modify keylists either through the KEYLIST command or the Keylist settings choice
from the Function keys pull-down on the ISPF Settings panel. You can control whether your application
uses keylists or not with the KEYLIST command or the Keylist settings choice from the Function keys
pull-down on the ISPF Settings panel. You can also control whether you use keylists as provided with
the application or with user modifications. You assign the keylist to a particular panel by using the keylist
keyword on the )PANEL statement or by using the keylist attribute on the PANEL tag. For a description of
the panel section, see “Defining the panel section” on page 192.
© Copyright IBM Corp. 1980, 2025 79

## Page 108

Action bars and pull-downs
An action bar is the panel element located at the top of an application panel that contains action bar
choices for the panel. Each action bar choice represents a group of related choices that appear in
the pull-down associated with the action bar choice. When the user selects an action bar choice, the
associated pull-down appears directly below the action bar choice. Pull-downs contain choices that, when
selected by the user, perform actions that apply to the contents of the panel.
For complete details on coding action bars and pull-downs, refer to the z/OS ISPF Dialog Tag Language
Guide and Reference or the “Defining the action bar choice section” on page 133.
Pop-up windows
Pop-up windows display information that extends the user's interaction with the underlying panel. When
a pop-up is displayed, the user must finish interacting with that pop-up window before continuing with
the dialog in the underlying panel.
The ADDPOP service allows your application to use pop-up windows. After you issue the ADDPOP service,
subsequent DISPLAY, TBDISPL, or SELECT service calls display panels in that pop-up window until your
application issues a corresponding REMPOP service or issues another ADDPOP service.
You specify the location of the pop-up window using the ADDPOP service call.
You can specify the size of the window (width and depth) on the panel definition BODY statement or use
the WIDTH and DEPTH attributes on the DTL PANEL tag. If you do not specify the size, the Dialog Manager
displays the pop-up window in a 76 X 22 window with a border.
Each pop-up window created as a result of a successful ADDPOP service call can also have a window title.
The title is embedded in the top of the window frame border and can be only one line in length. If the title
is longer than the window frame, the dialog manager truncates it. To define the window title, set system
variable ZWINTTL to the desired window title text.
This example will display three pop-up windows, as shown in Figure 32 on page 81. The window that
panel B is displayed within will have the title POPUP WINDOW TITLE.
PROC 0
ISPEXEC ADDPOP
ISPEXEC DISPLAY PANEL(A)
ISPEXEC ADDPOP POPLOC(F1)
SET ZWINTTL = POPUP WINDOW TITLE
ISPEXEC DISPLAY PANEL(B)
SET ZWINTTL =
ISPEXEC ADDPOP
ISPEXEC DISPLAY PANEL(C)
80  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 109

Figure 32. Example panel displaying three pop-up windows
The REMPOP service removes the current pop-up window. After you call the REMPOP service, a
subsequent DISPLAY service will either display a panel in the full panel area of the screen or in a
lower-level pop-up window, if it is active.
See z/OS ISPF Services Guide for a complete description of the ADDPOP and REMPOP services.
Movable pop-ups
ISPF provides two ways for you to move the currently active pop-up window: the WINDOW command, and
manual movement using two terminal interactions and no specific ISPF command.
WINDOW command
The WINDOW command can be associated with a function key or can be typed on the command line.
The cursor placement specifies the new location for the upper-left corner of the pop-up window frame.
If the pop-up window does not fit on the physical screen at the specified location, it is repositioned to fit
following the current pop-up window positioning rules. The cursor is placed in the same relative position it
occupied before a dialog or help pop-up window was moved.
If the cursor location would be covered as a result of moving a modeless message window, the cursor
is repositioned to the first input field on the active panel. If an input field does not exist, the cursor is
positioned in the upper-left corner of the active panel. The cursor is returned to its intended location if
the modeless message window is moved to a location that no longer conflicts with cursor display. Cursor
positioning is not affected by an input field that becomes protected as a result of a modeless message
window position unless the cursor itself would be covered. In other words, the cursor can be positioned
on a protected input field.
The WINDOW command is an immediate action command. Panel processing is not performed when this
command is used.
If the WINDOW command is typed in the command line, the cursor should be moved to the desired
window position before pressing Enter.
Chapter 4. Common User Access (CUA) guidelines  81

## Page 110

If the WINDOW command is included in the keylist associated with the currently active application
panel, the user can move the cursor to any position on the screen, press the function key assigned to
the WINDOW command, and the pop-up is repositioned to the user's cursor position. The WINDOW
command can be included in the keylist by the application developer, or the user can use the KEYLIST
utility to add it to the keylist.
For panels that do not include the KEYLIST keyword in the )PANEL statement, the application can assign
the WINDOW command to a ZPFnn system variable. The user can also associate WINDOW with a function
key by using the ZKEYS command to access the function key assignment utility.
If the split screen is used, the pop-up cannot be moved to a different logical screen. The new pop-up
window location must be in the same logical screen in which the pop-up was originally located. A pop-up
is not displayed over the split line. The split line cuts off the pop-up at the split line location; the pop-up is
not automatically repositioned to fit above the split line.
Note: Pull Down Choice (PDC), Action Bar is also a pop-up window, so the split screen line cuts off the
Action Bar location, too. The pop-up is not automatically repositioned to fit above the split line.
If the WINDOW command is requested when pop-up windows are not active, a message is displayed
to the user. A pop-up window containing an Action Bar panel cannot be moved while a pull-down is
actively displayed. A message is displayed to the user if the WINDOW command is requested during this
condition.
Manual movement
The second method for moving pop-up windows involves two terminal interactions but does not require
a unique ISPF command. A user can request window movement by placing the cursor anywhere on the
active window frame and pressing Enter. ISPF acknowledges the window move request by displaying
WINDOW MOVE PENDING message. The alarm will sound if the terminal is so equipped. The message
text will be yellow/high intensity if the Panel display CUA mode option on the ISPF Settings panel has
been selected. Otherwise, the message text will be white/low intensity.
Place the cursor where you want the upper-left corner of the window frame to be, and press Enter a
second time. The window is moved to the new location as though the WINDOW command had been
issued. The rules for cursor placement inside the window, and window placement on the physical display,
are the same as those described for the WINDOW command.
Pop-up movement considerations
Modeless and modal message pop-up windows can be moved in the same manner as dialog pop-up
windows.
Only the active pop-up window can be moved. If a modal or modeless message pop-up is displayed over
a dialog pop-up window, only the message pop-up window can be moved. The underlying dialog pop-up
window cannot be moved while a message pop-up window is displayed over it.
Input fields that are partially or totally covered by a pop-up window become protected fields (data cannot
be entered into the field). If a field becomes totally uncovered as a result of moving the pop-up window,
the field is restored to an unprotected field (data can be entered into the field).
Field-level help
Field-level help provides help panels for fields defined on an application panel. When the cursor is on a
field and you request HELP, ISPF displays the help panel defined for that field. See “Defining the HELP
section” on page 182.
Extended help
Extended help provides general information about the contents of a panel. The information in extended
help can be an overall explanation of items on the panel, an explanation of the panel's purpose in the
82  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 111

application, or instructions for the user to interact with the panel. The user invokes extended help by
issuing the command EXHELP. EXHELP requests ISPF to display help text for the entire panel.
For more information about help, see “.HELP variable” on page 252 and Chapter 7, “ISPF help and tutorial
panels,” on page 257.
Keys help
Keys help provides the user with a brief description of each key defined for a panel. You define the
contents of this help panel. The user invokes keys help by issuing the command KEYSHELP.
KEYSHELP requests ISPF to display the help panel for the current keylist. The help panel name can be
provided as part of the keylist definition. If the keys help panel is not identified in the keylist definition, it
can be supplied in the ZKEYHELP system variable. Use separate ZKEYHELP variable values for each keys
help panel to be displayed.
Reference phrase help
Reference phrase (RP) help is available on all panels. Place the cursor on a highlighted reference phrase
within a panel, request help, and you receive the help panel defined for that reference phrase.
When a panel with reference phrases is displayed for the first time, the cursor is positioned in the
upper-left corner. After a reference phrase is selected and control is returned to the original panel, the
panel scrolls automatically to put the cursor on the reference phrase from which the reference phrase
help was invoked. The exact scroll position might not be the same as when the reference phrase help
was invoked. ISPF positions the reference phrase at the top of the display is scrolling is necessary to
display the reference phrase help field. The reference phrase is an input-capable field that allows tabbing.
Therefore, the reference phrase text is refreshed whenever the panel is redisplayed.
Reference phrase help panels themselves can also contain reference phrases. When a reference phrase
help panel is canceled, the panel from which reference phrase help was requested is redisplayed. All
other help facilities are available from a reference phrase help panel.
The TYPE(RP) attribute in the panel attribute section is used to identify a reference phrase in a panel. See
“Defining the attribute section” on page 143. An entry is then placed in the )HELP section of the panel
for each reference phrase attribute coded in the )BODY or optional )AREA panel sections. This example is
a )HELP section reference phrase definition:
)HELP
  FIELD(ZRPxxyyy)  PANEL(panel-name)
xx
00 for a reference phrase defined in )BODY section and 01 to 99 for the number of the scrollable area
in which the reference phrase is defined.
Each scrollable area is assigned a sequential number based on its relative position within the panel
body. The scrollable area closest to the upper-left corner of the panel body is assigned number 01
with each additional scrollable area, scanning left to right, top to bottom, assigned the next sequential
number. A maximum of 99 scrollable areas in any given panel may contain reference phrases.
yyy
001 to 999 for the relative number of the reference phrase within the panel body or within a particular
scrollable area.
panel-name
Name of the help panel to be displayed when HELP for this reference phrase is requested.
A reference phrase can wrap around multiple terminal lines in panels that are not displayed in a window.
A reference phrase that logically wraps in a pop-up window requires the beginning of each wrapped line
to contain a RP field attribute, and there must be an entry in the )HELP section for each wrapped line. This
is also true for panels containing the WINDOW() keyword that are not displayed in a pop-up window. The
additional )HELP section entries would normally be pointing to the same panel.
Chapter 4. Common User Access (CUA) guidelines  83

## Page 112

The example in Figure 33 on page 84 illustrates both single and multiple line reference phrases.
  )PANEL
  )ATTR
    #  TYPE(RP)
    $  AREA(SCRL) EXTEND(OFF)
  )BODY
  +This is sample text.  This is a #Reference Phrase+.
  +This is an example of a #Reference Phrase being
   physically continued to the next line.+
  +  *********************
  +  *$SAREA1           $*      ****************
  +  *$                 $*      *$SAREA2      $*
  +  *$                 $*      *$            $*
  +  *********************      *$            $*
  +  *********************      *$            $*
  +  *$SAREA3           $*      *$            $*
  +  *$                 $*      ****************
  +  *$                 $*
  +  *********************
  +This is an example of a #Reference Phrase being+
  #logically continued to the next line.+
  +
  )AREA SAREA1
  +                  +
  #Area 01 Ref Phrase+
  +                  +
  )AREA SAREA2
  +            +
  +  #Area 02+ +
  + #Reference++
  +  #Phrase+  +
  )AREA SAREA3
  +                  +
  #Area 03 Ref Phrase+
  +                  +
  )HELP
   FIELD(ZRP00001)  PANEL(BODY0001)
   FIELD(ZRP00002)  PANEL(BODY0002)
   FIELD(ZRP00003)  PANEL(BODY0003)
   FIELD(ZRP00004)  PANEL(BODY0003)
   FIELD(ZRP01001)  PANEL(AREA0101)
   FIELD(ZRP02001)  PANEL(AREA0201)
   FIELD(ZRP02002)  PANEL(AREA0201)
   FIELD(ZRP02003)  PANEL(AREA0201)
   FIELD(ZRP03001)  PANEL(AREA0301)
  )END
Figure 33. Reference phrase help example
START service
You can use the START service to start a dialog in a new logical screen. This function is similar to the
function nesting made available with action bars except that the "nesting" occurs in a new logical screen.
You can invoke the START service in any of these ways:
• From any command line, you can enter a command in this form:
START some_dialog
some_dialog can be:
– A command from the command table; for example, MYCMD1
– A command with parameters (must be in quotes); for example, 'MYCMD1 PARM1'
– A dialog invocation; for example, PANEL(MYPAN1), or 'PGM(MYPGM1) PARM(MYPARM1,MYPARM2)'
• You can code a pull-down choice,
ACTION RUN(START) PARM(some_dialog)
where some_dialog is the same as previously outlined.
• You can code a selection panel option,
84  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 113

'PGM(ISPSTRT) PARM(some_dialog)'
For example,
&ZSEL = TRANS(&XX
         0,'PGM(ISPSTRT) PARM(PGM(MYPGM0))'
         1,'PGM(ISPSTRT) PARM(PGM(MYPGM1) PARM(MYPARM1))'
         2,'PGM(ISPSTRT) PARM(CMD(MYCMD1 MYPARM2))'
         3,'PGM(ISPSTRT) PARM(PANEL(MYPANEL1))'
• From a dialog, you can invoke,
ISPEXEC SELECT PGM(ISPSTRT) PARM(some_dialog)
where some_dialog is the same as previously described.
Note:
1. The some_dialog must not exceed 249 characters. It will be truncated at 249 without warning.
2. For ISPF functions that have service interfaces, such as EDIT and BROWSE, you should use the service
invocations. Using ISPSTRT passing the selection strings from panel ISR@PRIM does not work in all
situations and is not supported.
If the maximum number of logical screens do not exist when the START command is invoked and:
• some_dialog is a command from the command table, the new screen is invoked with the default initial
command (in non-display mode) and the command is run. When the user ends the dialog this new
screen still exists.
• if some_dialog is specified as PGM(xxx), CMD(xxx), or PANEL(xxx), the new screen is invoked with
PGM(xxx), CMD(xxx), or PANEL(xxx) as the initial command, program, or panel. The result is that when
you end the xxx dialog, this new screen is terminated.
If the maximum number of logical screens has already been reached when the START command is
invoked, the specified some_dialog is executed on top of the currently displayed screen. The result is that
when you end the dialog, ISPF returns to the previously displayed screen.
On 3270 displays, if ISPF is not in split screen mode the START command and ISPSTRT program split the
screen at the top or bottom line of the display. If ISPF is already in split screen mode, ISPF starts the new
screen in the opposite screen, using the existing split line location.
Chapter 4. Common User Access (CUA) guidelines  85

## Page 114

86  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
