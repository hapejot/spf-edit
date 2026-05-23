# Chapter 8. Command (option 6)

Source file: f54u200_v3r1.md
Start page: 389
Page span: 389-392

## Page 389

Chapter 8. Command (option 6)
When you select this option, the ISPF Command Shell panel shown in Figure 214 on page 351 is
displayed. You can enter TSO commands, CLISTs, and REXX EXECs on the Command line of any panel and
in the Line Command field on data set list displays (option 3.4). However, the ISPF Command Shell panel
provides additional capabilities:
• You can enter TSO commands, ISPF commands, CLISTs, and REXX execs in a separate, but optional,
ISPF Command field. This field is displayed only if your installation chooses to do so. The default
panel shown in Figure 214 on page 351 does not display this field. When you use this field, commands
that are typed in the TSO Command Entry field (==>) are not blanked out when you enter the SPLIT
command to split the screen.
Note: If you use this field, you will not have access to the saved command area (see “The saved
command area” on page 352).
• You can enter Session Manager mode, but only if this licensed program is installed. See “Using the
session manager” on page 354 for more information.
• You can enter a long command that continues on these two lines.
Figure 214. ISPF Command Shell panel (ISRTSO)
ISPF command shell panel action bar
The ISPF Command Shell panel action bar choices function as follows:
Note: The ISPF Command Shell panel action bar contains three pull-down choices that let you control the
saved command area.
• List
Command (option 6)
© Copyright IBM Corp. 1980, 2024 351

## Page 390

• Mode
• Functions
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
List
The List pull-down offers you these choices:
Update On
Makes the list of commands in the saved command area live; that is, new commands are
appended automatically.
Update Off
Makes the list of commands in the saved command area static; that is, new commands are not
appended automatically.
The current setting is shown as an unavailable choice.
Mode
The Mode pull-down offers you these choices:
Retrieve
Allows commands to be retrieved from the saved command area and placed on the TSO Command
Entry field (==>) so that you can edit them before they are executed. This mode is the default.
Execute
Allows commands to be retrieved from the saved command area and executed in one step.
Delete
Allows you to delete commands from the saved command area without executing the commands.
Place the cursor on the command to be deleted and press Enter. The command will be blanked
out. This process allows you to delete a command if you are running with Update mode set off.
The current setting is shown as an unavailable choice.
Functions
The Functions pull-down offers you this choice:
Compress List
Removes duplicate entries and blank spaces in the saved command area if you are running with
Update mode set off. Entries are compressed automatically in Update mode.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides general information on the TSO Command processor panel, including
line I/O mode and Session Manager mode, and restrictions on entering commands.
The saved command area
The ISPF Command Shell panel has a saved command area (the bottom portion of the screen) that
contains a list of up to 10 commands that you have saved; see Figure 215 on page 353 for an example.
These commands are point-and-shoot fields. What happens when you select a command depends on the
mode you specify from the Mode pull-down menu on the action bar.
Command (option 6)
352  z/OS: z/OS ISPF User's Guide Vol II

## Page 391

Figure 215. ISPF Command Shell panel with saved commands (ISRTSO)
Entering TSO commands, CLISTs, and REXX EXECs
You do not need to enter TSO before the command on this panel as you do on other panels, unless the
command exists in both ISPF and TSO and you want to process the TSO command. If you use TSO, your
processed command is blanked out when the ISPF Command Shell panel is displayed again.
TSO commands, CLISTs, and REXX EXECs entered are invoked using the ISPF SELECT CMD service.
Variable names starting with an ampersand (&) are evaluated by ISPF. If you want the underlying
command processor to see the ampersand you must specify 2 ampersands. For example:
DEF NONVSAM(NAME('MY.DATASET') DEVT(0000) VOLUME(&&SYSR2))
For example, the HELP, PRINT, and CANCEL commands are interpreted as the ISPF HELP, PRINT, and
CANCEL commands, unless you precede them with TSO. Therefore, to get TSO HELP information, enter:
===> TSO HELP xxx
Rules for entering TSO commands
Do not enter these commands under ISPF:
• LOGON and LOGOFF
• ISPF, PDF, or ISPSTART
• TEST
• Commands that are restricted by TSO or ISPF
• Commands that call a program authorized by the Authorized Program Facility (APF), except for the TSO
CALL command
• ISPEXEC service calls.
Command (option 6)
Chapter 8. Command (option 6)  353

## Page 392

Rules for entering CLISTs and REXX EXECs
You can enter a CLIST name or REXX exec name on this panel, but these restrictions apply:
• The CLIST or REXX exec cannot call the restricted commands shown in the preceding list. However, this
does not apply to ISPEXEC, which can be called in a CLIST or REXX exec.
• CLIST error exits are not entered for ABENDs.
• CLIST TERMIN command procedure statements may cause unwanted results.
Note: Remember that a command issued through an alias may contain some of the characteristics listed
here and thus may cause unwanted results.
Using the session manager
If the Session Manager licensed program is installed and available, you can use it by selecting Session
Manager mode on the ISPF Settings panel. For information on altering the PDF configuration table to
allow you to enter Session Manager mode, refer to z/OS ISPF Planning and Customizing.
If you select this option, any display output is displayed in the Session Manager TSOOUT stream.
Note: If GDDM/ISPF mode is active, Session Manager does not get control of the screen. GDDM/ISPF
mode is started when a GRINIT service has been issued, but a GRTERM service has not been issued. See
z/OS ISPF Services Guide for more information about these two services.
The function key definitions are not transferred to the Session Manager from ISPF. When the command
ends, the Session Manager prompts you to enter a null line to return to ISPF control and displays the TSO
Command Processor panel again when you do so.
If you do not select Session Manager mode, terminal I/O occurs as though the Session Manager were
not installed. The terminal operates in normal TSO fashion. Any communication with the command is in
line-I/O mode. When the command ends, three asterisks (***) are displayed. Press Enter to display the
TSO Command Processor panel again in full screen mode.
To interrupt a TSO command, CLIST, or REXX exec, press the PA1 key. The TSO command ends and
the TSO Command Processor panel is displayed again. If terminal input is inhibited, press the Reset key
before pressing the PA1 key. If you are in Session Manager mode, enter a null line to return to ISPF
full-screen mode.
When the TSO Command Processor panel is displayed again, the command that was just processed is
displayed to the right of the arrow. Enter another command or the END command to return to the ISPF
Primary Option Menu.
For terminals with primary and alternate screen sizes, ISPF does not check to make sure the same screen
settings are in effect when a command, CLIST, or REXX exec ends. If you call a CLIST, REXX exec, or
command that changes the screen settings, you are responsible for saving and restoring them before
control is returned to ISPF.
Command (option 6)
354  z/OS: z/OS ISPF User's Guide Vol II
