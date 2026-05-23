# Chapter 8. The application command table

Source file: f54dt00_v3r1.md
Start page: 175
Page span: 175-178

## Page 175

Chapter 8. The application command table
In addition to the commands in the ISPF system command table, DTL provides a way to define and store
commands that are specific to your application. You can also define commands that override the ISPF
system commands. You define and store these commands within a command table for your application.
These application-specific commands define the responses to commands entered by the user in the
command entry field and commands linked to pull-down choices and key mapping lists.
You can define only one command table for an application. ISPF locates the command table using the
defined applic ation -identifier  for the command table.
For a complete description of ISPF command processing and a list of the ISPF system commands, refer to
the z/OS ISPF User's Guide Vol I.
Note: You can use the TSO ISPCMDTB command to convert existing command tables to DTL. To
use ISPCMDTB, ensure that the command table is in your table concatenation (ISPCMDTB), type TSO
ISPCMDTB applid (where applid is the application id of the command table). This places you in an edit
session containing the DTL version of the command table. Use the editor CREATE or REPLACE commands
to save the table to your DTL source data set.
Defining the application command table
The tags you use to define an application command table are:
CMDTBL
Begins the definition of an application command table. The required end tag ends the definition.
CMD
Defines a command within an application command table. You code the CMD tags within a CMDTBL
definition (between the start and end tags).
CMDACT
Defines the action taken by ISPF when a user enters a command. You code the CMDACT tag following
the command (CMD) with which it is associated.
The CMDTBL tag has a required APPLID attribute that you use to define the application identifier  for the
command table. ISPF uses the value you assign with the APPLID attribute to identify the command table.
The value you assign to APPLID must be the same as the runtime application identifier specified when the
application starts.
The value you assign as an application identifier can have a maximum of 4 characters, and the first
character must be A-Z, a-z, @, #, or $.
Any remaining characters can be either A-Z, a-z, @, #, $ or 0-9. Lowercase characters are translated to
their uppercase equivalents. Additionally, ISPF reserves the application identifier ISPx, where x is any
character including the space character. Do not use any of these for an APPLID value.
The conversion utility uses the application identifier as a prefix to the string CMDS to form the name of the
command table library. For example, the APPLID value, demo, results in the application command table
name DEMOCMDS.
Command tables are updated using ISPF table services. Input is obtained from the ISPTLIB DDname
allocation and output is written to the ISPTABL DDname allocation. For the description of how to allocate
libraries before you start ISPF, and for more information about the use of ISPTLIB and ISPTABL, see the
z/OS ISPF User's Guide Vol I.
When a user enters a command in a command-entry field or through a pull-down choice or function key,
ISPF searches the command tables defined for the user. The tables are searched in this order (provided
that a table is present and defined):
1. Application command table
Defining the application command table
© Copyright IBM Corp. 1989, 2024 143

## Page 176

2. User command tables
3. Site command tables
4. System command table
Note: Up to three user and site command tables can be defined in the ISPF Configuration table. The
search order of the site and system command table can be reversed if specified as such in the ISPF
Configuration table.
If the command is found in a command table, ISPF performs the action defined in that command table for
that command. If the command is not found in any of the command tables, ISPF passes the command to
the application program for processing. If any of the command tables are not present, ISPF skips to the
next command table in the hierarchy.
Use the CMD tag to define each of the commands within the application command table. The CMD tag
has a required NAME attribute that you use to identify the internal-command-name for the command.
The value you assign as an internal-command-name must not exceed 8 characters, and the first character
must be alphabetic. Any remaining characters can be either alphabetic or numeric.
Here is a markup example that shows a source file that contains an application command table, a key
mapping list, and a panel with an action bar. The command table contains commands that are mapped to
the RUN attributes of the ACTION tags associated with the pull-down choices and to the CMD attributes of
the KEYI tags.
<!doctype dm system>
<cmdtbl applid=brws>
  <cmd name=quit>quit
    <cmdact action=...>
  <cmd name=send>send
    <cmdact action=...>
</cmdtbl>
<keyl name=panlkeys>
  <keyi key=f4 cmd=quit>
  <keyi key=f6 cmd=send>
</keyl>
<panel...>
  <ab>
    <abc>Actions
      <pdc>Quit
        <action run=quit>
      <pdc>Send
        <action run=send>
      <pdc>Exit
        <action run=exit>
  </ab>
⋮
</panel>
Because ISPF provides the EXIT command, it is not defined within the application command table. When
the EXIT command is entered, ISPF finds it in the system command table.
Specifying command actions
You must specify a CMDACT tag for each of the CMD tags you define within an application command table
so that ISPF can process these commands. You use the CMDACT tag to define the action taken for the
command. Code the CMDACT tag immediately after the CMD tag it is associated with.
The ACTION attribute
The CMDACT tag has a required attribute, ACTION, which you use to specify the ISPF command action.
Here is a list of ISPF command actions you can assign. You can also assign some of the ISPF-provided
system commands listed in “CMDACT (Command Action)” on page 234, and you can specify command
actions dynamically at run time as discussed in “Specifying command actions dynamically” on page 145.
Defining the application command table
144  z/OS: z/OS ISPF DTL Guide

## Page 177

ALIAS
To allow a command to have an alternate name, such as using QUIT as an alias for EXIT.
PASSTHRU
To pass the command to the application. The internal-command-name and any command parameters
are passed to the dialog in the ISPF ZCMD system variable.
SETVERB
To pass the command to the application. The internal-command-name is passed to the dialog in the
ZVERB system variable, and the parameters (if any) are passed to the dialog in the ZCMD system
variable.
The ALIAS command action provides you with a way to define synonyms for commands. The internal-
command-name you define for the ALIAS attribute value defines the command to be processed. You must
enclose the keyword ALIAS, the internal-command-name, and any optional parameters within quotes.
When you define an ALIAS command action, you must code that command's CMD and CMDACT tags
before the command the ALIAS represents. ISPF first searches the application-defined commands, and
then the ISPF system commands. It must locate the ALIAS definition before the aliased command.
Here is an example where we've added the commands PREV and NEXT to the application command table.
We want "PREV" and "NEXT" to be aliases for the ISPF system commands BACKWARD and FORWARD.
Because the BACKWARD and FORWARD commands are provided by ISPF, we do not need to define them
in the application command table. ISPF locates the aliases before the ISPF system commands they refer
to.
Additionally, this example shows the CMDACT for the SEND command set to PASSTHRU, because we want
the application program to process the SEND command.
<cmdtbl applid=brws>
  <cmd name=quit>quit
    <cmdact action='alias exit'>
  <cmd name=send>send
    <cmdact action=passthru>
  <cmd name=prev>
    <cmdact action='alias backward'>
  <cmd name=next>
    <cmdact action='alias forward'>
</cmdtbl>
Specifying command actions dynamically
You can also specify a variable as the value for the ACTION attribute of the CMDACT tag. ISPF substitutes
the value of the variable at run time when the command is processed. The runtime value of the variable
must be one of the ISPF-supported command actions. You specify the variable using the % notation in the
ACTION value.
Here is an example where we specified the variable scroll as a command action for the SCROLL command.
When the user issues the SCROLL command, ISPF obtains the value of the variable scroll from the
variable pool to determine the action to be taken. The application can then control the direction of
scrolling by setting the variable scroll to FORWARD or BACKWARD, or to NOP if no scrolling is possible.
<!doctype dm system>
<cmdtbl applid=abcd>
  <cmd name=scroll>scroll
    <cmdact action='%scroll'>
⋮
</cmdtbl>
Truncating commands
Instead of forcing the user to enter the full command name when typing a command in the command
area, you can a shortcut for the user by defining command truncations for commands. The user can issue
a truncated command in the command area by entering the minimum number of characters you specify
for the command.
Defining the application command table
Chapter 8. The application command table  145

## Page 178

To specify truncation for a command, you code the T (truncation) tag within the external-command-name
of the command.
For example, to specify "qu" as the minimum command for the QUIT command, you add the T tag to the
external-command-name, like this:
<cmdtbl applid=brws>
  <cmd name=quit>qu<t>it
    <cmdact action='alias exit'>
⋮
</cmdtbl>
The T tag follows the characters you specify as the minimum command.
With this truncation, the user can issue the QUIT command by typing the command in one of these ways:
qu
qui
quit
However, you should be careful to avoid adding truncations that duplicate other truncations in the
command table. For example, these two truncations define minimum commands ("co") that are identical:
<cmdtbl applid=brws>
  <cmd name=comp>co<t>mpare
    <cmdact action=passthru>
  <cmd name=copy>co<t>py
    <cmdact action=passthru>
</cmdtbl>
The preceding definition would cause the conversion utility to issue a warning message.
To avoid this type of duplication, place the T tag appropriately in the CMD tag content. The duplication
shown in the example can be avoided by coding the truncations in this way:
<cmdtbl applid=brws>
  <cmd name=comp>com<t>pare
    <cmdact action=passthru>
  <cmd name=copy>cop<t>y
    <cmdact action=passthru>
</cmdtbl>
Defining the application command table
146  z/OS: z/OS ISPF DTL Guide
