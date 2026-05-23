# Chapter 9. Defining key mapping lists

Source file: f54dt00_v3r1.md
Start page: 179
Page span: 179-182

## Page 179

Chapter 9. Defining key mapping lists
Every application panel has keys that map to valid actions for the panel. You define these key assignments
within key mapping lists. The key assignments map a key to a command defined within the application
command table or to an ISPF-provided command. You use the KEYLIST attribute of the HELP, HELPDEF,
PANEL or PANDEF tags to name the key mapping list to use for a panel. If a keylist is not specified, ISPF
provides the default key mapping list used for help panels. ISPF also provides a default key mapping list
used when application panels do not refer to an application-defined KEYLIST.
The tags you use to define key mapping lists are:
KEYL
To define a key mapping list. The required end tag ends the key mapping list definition.
KEYI
To define a key assignment and specify the command ISPF processes when the user presses the key,
and specify the label for the key if it is displayed in the function key area.
You can code multiple KEYI (key item) tags within a KEYL (key list) definition. You code a KEYI tag for
each key that is defined for the key mapping list.
Keylists are updated using ISPF table services. Input is obtained from the ISPTLIB DDname allocation
and output is written to the ISPTABL DDname allocation. See the description of how to allocate libraries
before starting ISPF in the z/OS ISPF User's Guide Vol I for more information about the use of ISPTLIB and
ISPTABL.
Assigning keys and actions
The KEYL tag starts a key mapping list definition and provides the name of the key mapping list. You
specify the key mapping list to be used with the KEYLIST attribute of the HELP, HELPDEF, PANEL, or
PANDEF tag.
Each KEYI definition within a key mapping list maps a key assignment with a command. The command
can be defined in the application command table, one of the user command tables or site command
tables, the system command table, or it can be one of the ISPF-provided commands. The required KEY
and CMD attributes of the KEYI tag match the key with the command.
The KEYI definition in this example maps the F2 key on the user's keyboard with the SEARCH command in
the application command table.
<!doctype dm system>
<cmdtbl applid=abcd>
 <cmd name=search>Search
   <cmdact action=passthru>
</cmdtbl>
<keyl name=panlkeys>
  <keyi key=f2 cmd=search>Search
</keyl>
<panel name=panl01 keylist=panlkeys>
⋮
</panel>
When the user presses the F2 key during the display of an application panel that refers to this key
mapping list, ISPF processes the SEARCH command.
ISPF default key list
ISPF provides a default key mapping list named ISPKYLST for application panels. If you do not specify
a key mapping list to be associated with a panel (using the KEYLIST attribute of the PANEL or PANDEF
Assigning keys and actions
© Copyright IBM Corp. 1989, 2024 147

## Page 180

tag), ISPF uses the keys defined for ISPKYLST to display in the function key area of the panel when it is
displayed. See “PANEL (Panel)” on page 376 for information about coding the PANEL tag.
The key mappings for ISPKYLST are:
Key
Command
F1
HELP
F2
SPLIT
F3
EXIT
F9
SWAP
F12
CANCEL
F13
HELP
F14
SPLIT
F15
EXIT
F21
SWAP
F24
CANCEL
ISPF provides a default key mapping list named ISPHELP for help panels. If you do not specify a key
mapping list to be associated with a panel (using the KEYLIST attribute of the HELP or HELPDEF tag), ISPF
uses the keys defined for ISPHELP to display in the function key area of the panel when it is displayed.
See “HELP (Help Panel)” on page 303 for information about coding the HELP tag and Table 38 on page
308 for key mappings of the ISPHELP keylist.
You can override the ISPF default key mapping list by specifying a KEYLIST attribute in the panel
definition. All keys that you want to be active, including those for ISPF-provided commands, must be
specified in the key mapping list referred to by the KEYLIST attribute.
Displaying keys
While all of the key assignments you define in a key mapping list are valid for the application panels that
refer to the list, they only appear in the function key area (FKA) of the panel under these conditions:
• You specify that the key is to be displayed by including FKA=YES in the KEYI tag, and
• The user has not turned off display of the function key area.
You use the FKA attribute of the KEYI tag to specify whether the key is to appear in the panel's function
key area. The default FKA value, NO, means that the key does not appear. You must specify FKA=YES for
the key to be displayed in the function key area.
When function keys are displayed in the function key area, the key you assign is displayed followed by an
equal sign and the FKA text defined for the KEYI tag.
Defining help for key list
The conversion utility supports a keys help panel name on the KEYL tag. This allows a keys help panel to
be associated with the key list. You can use the KEYLIST utility to add, change, or delete a keylist help
panel name.
Assigning keys and actions
148  z/OS: z/OS ISPF DTL Guide

## Page 181

Alternatively, the application can provide the help panel name in the ZKEYHELP variable. However, the
panel name specified as the keylist help panel either on the KEYL tag or by the KEYLIST utility overrides
the panel name supplied by the ZKEYHELP variable.
Here is an example where we want only the F2, F3, and F6 keys to appear in the panel function key area,
with F2 mapped to the SEARCH command defined in the application command table, F3 mapped to the
EXIT command, and F6 mapped to the KEYSHELP command. We also want F1 to be active to support
the ISPF HELP command. No other function keys are to be active for this key mapping list. To obtain this
result, we define the function key mapping list like this:
<!doctype dm system>
<cmdtbl applid=abcd>
  <cmd name=search>Search
    <cmdact action=passthru>
</cmdtbl>
<keyl name=panlkeys help=pnlkeyh>
  <keyi key=f1 cmd=help>
  <keyi key=f2 cmd=search fka=yes>Search
  <keyi key=f3 cmd=exit fka=yes>Exit
  <keyi key=f6 cmd=keyshelp fka=yes>Keyshelp
</keyl>
<panel name=panl01 keylist=panlkeys>
⋮
</panel>
This is how the function key area appears when panel "panl01" is displayed:
  F2=Search  F3=Exit  F6=Keyshelp
Figure 80. Displayed function key area
Assigning keys and actions
Chapter 9. Defining key mapping lists  149

## Page 182

Assigning keys and actions
150  z/OS: z/OS ISPF DTL Guide
