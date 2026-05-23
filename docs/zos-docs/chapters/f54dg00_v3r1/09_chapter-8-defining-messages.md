# Chapter 8. Defining messages

Source file: f54dg00_v3r1.md
Start page: 293
Page span: 293-304

## Page 293

Chapter 8. Defining messages
This topic describes how to create and change ISPF messages. You can create messages in two ways:
• Using the existing message definition.
• Using the MSG and MSGMBR tags of the Dialog Tag Language (DTL). See the z/OS ISPF Dialog Tag
Language Guide and Reference for more information about these tags.
ISPF message definitions are stored in a message library and displayed by using the DISPLAY, TBDISPL,
or SETMSG service, written to the ISPF log file by the LOG service, or copied to variables specified in
a GETMSG service request. You create or change messages by editing directly into the message library.
ISPF interprets the messages during processing. No compilation or preprocessing step is required.
Note: When not in TEST mode, the most recently accessed message definitions are retained in virtual
storage for performance reasons. If you have modified a message, using TEST mode will ensure that the
updated version of the message will be picked up by ISPF services. See “ISPF test and trace modes” on
page 23 for more information.
Several messages can be within each member of the message library. When using the PDF editor to create
a message file, prevent numbers from appearing in the file by specifying NUMBER OFF.
The member name is determined by truncating the message ID after the second digit of the number.
For example:
Message ID
Member Name
G015
G01
ISPE241
ISPE24
XYZ123A
XYZ12
ABCDE965
ABCDE96
EMPX214
EMPX21
All messages that have IDs beginning with the characters G01, for example, must be in member G01.
Figure 74 on page 266 shows an example of a member in the message library. This member contains all
message IDs that begin with EMPX21.
© Copyright IBM Corp. 1980, 2025 265

## Page 294

EMPX210   'INVALID TYPE OF CHANGE' .HELP=PERSO33   .ALARM=YES
'TYPE OF CHANGE MUST BE NEW, UPDATE, OR DELETE.'
EMPX213    'ENTER FIRST NAME'       .HELP=PERSO34  .ALARM=YES
'EMPLOYEE NAME MUST BE ENTERED FOR TYPE OF CHANGE=NEW OR UPDATE.'
EMPX214    'ENTER LAST NAME'        .HELP=PERSO34  .ALARM=YES
'EMPLOYEE NAME MUST BE ENTERED FOR TYPE OF CHANGE=NEW OR UPDATE.'
EMPX215    'ENTER HOME ADDRESS'     .HELP=PERSO35  .ALARM=YES
'EMPLOYEE NAME MUST BE ENTERED FOR TYPE OF CHANGE=NEW OR UPDATE.'
EMPX216    'AREA CODE INVALID'      .ALARM=YES
'AREA CODE &PHA IS NOT DEFINED. PLEASE CHECK THE PHONE BOOK.'
EMPX217    '&EMPSER ADDED'
'EMPLOYEE &LNAME, &FNAME &I ADDED TO FILE'
EMPX218    '&EMPSER UPDATED'
'RECORDS FOR &LNAME, &FNAME &I UPDATED'
EMPX219    '&EMPSER DELETED'
'RECORDS FOR &LNAME, &FNAME &I DELETED'
Figure 74. Sample messages
How to define a message
Messages generally should appear in collating sequence by message ID. Each message within the library
consists of two required lines and (optionally) additional long message lines. The additional lines can
contain up to 512 bytes of long message text. These diagrams illustrate the syntax for defining messages: 
Line 1 syntax
msgid
' short message ' .HELP= panel
*
.ALARM=YES
NO
NOKANA
KANA
.WINDOW=RESP
NORESP
LRESP
LNORESP
.TYPE=NOTIFY
WARNING
ACTION
CRITICAL
Line 2 syntax
' long message '
+
Additional long message text lines – optional.
Line 3 syntax
' long message '
+
Line 4 syntax
' long message '
+
266  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 295

Line 5 syntax
' long message '
msgid
Required. Each message is referred to by a message identifier (ID). A message ID can be four to eight
characters long. It is defined as follows:
• Prefix: one to five alphabetic characters (A-Z, #, $, or @)
• Number: three numeric characters (0-9)
• Suffix (optional): one alphabetic character.
If the prefix is five characters long, the suffix must be omitted so that the total length does not exceed
eight characters. Use the message ID suffix if more than 10 messages are to be included in one
member.
short message
Optional. If a short message is specified on an ISPF panel, it is displayed first (before the long
message). Its maximum length is 24 bytes. The short message is displayed in a pop-up window
if the text is longer than will fit in the short message area or if you defined a message window
using the .WINDOW keyword for the message. Otherwise, the short messages are right-justified and
displayed, with a high intensity attribute, either:
• At the right end of the first line on the screen, if an action bar is not defined
• At the right end of the line following the action bar
If the user enters the HELP command, the long message is displayed, with a high intensity attribute. If
the user enters the HELP command again, tutorial mode is entered.
The location of the short and long messages in a user-designed panel is specified by the SMSG and
LMSG keywords. These keywords are defined under “Defining the body section” on page 169.
When messages are written to the ISPF log file, both the short message, if any, and the long message
are written in the same output line. The short message comes first, followed by the long message.
Note: For long or short messages in pop-up windows, if the message originates from panel
processing, such as a verification error message, the message pop-up window is placed adjacent
to the field that is the object of the validation.
.LOG=YES
Optional. Ensures that ISPF will write a copy of the message to the ISPF log, if it is allocated.
.HELP=panel | *
Optional. (Can be abbreviated to .H) If the user enters tutorial mode, the panel name specified
by .HELP is the first tutorial page displayed. If .HELP=* is specified, the first tutorial page is the one
specified in the panel definition, that is, the panel on which this message is being displayed. The
default is *.
NOKANA|KANA
Optional. The NOKANA keyword allows messages to contain lowercase characters, and still display
correctly on a Katakana terminal. Because hexadecimal codes for some lowercase characters overlap
those of some Katakana characters, they would display as meaningless characters on a Katakana
terminal. If the NOKANA keyword is present in a message definition, ISPF translates any lowercase
message characters to uppercase before displaying the message on a Katakana terminal.
In summary, if the terminal is Katakana, and:
• KANA is specified, all characters are left as is.
• NOKANA is specified, lowercase characters are translated to uppercase.
• If neither KANA nor NOKANA is specified, all characters are left as is.
If the terminal is not Katakana, and:
Chapter 8. Defining messages  267

## Page 296

• KANA is specified, lowercase characters are displayed as periods
• NOKANA is specified, all characters are left as is.
• If neither KANA nor NOKANA is specified, all characters are left as is.
Note:
1. On non-Katakana terminals, the KANA keyword can be used to display overlapping Katakana
characters as periods rather than as meaningless lowercase characters.
2. On Katakana terminals, the NOKANA keyword is necessary in messages containing lowercase
English characters.
3. See Chapter 10, “Extended code page support,” on page 299 for the discussion of the treatment of
the KANA or NOKANA keywords if a CCSID is specified.
.ALARM=YES|NO
Optional. (Can be abbreviated to .A) If .ALARM=YES is specified, the audible alarm sounds when the
message displays. If .ALARM=NO is specified, the alarm does not sound unless .ALARM is set to YES
in the panel definition. The default is NO.
.WINDOW=RESP|NORESP|LRESP|LNORESP
Optional. (Can be abbreviated to .W) The .WINDOW keyword tells ISPF to display the message in a
message pop-up window.
.WINDOW=RESP (R is a valid abbreviation for RESP) requests ISPF to display both long and short
messages in a message pop-up window that requires the user to press Enter before data can be
entered into the underlying panel. The user cannot enter data or interact with the underlying panel
until Enter (or some other attention key) is pressed.
.WINDOW=NORESP (N is a valid abbreviation for NORESP) requests ISPF to display both long and
short messages in a message pop-up window that does not require direct user response. The user can
enter data into the underlying panel while this message is being displayed.
.WINDOW=LRESP (LR is a valid abbreviation for LRESP) requests ISPF to display only long messages
in a message pop-up window that requires the user to press Enter before data can be entered into the
underlying panel. The user cannot enter data or interact with the underlying panel until Enter (or some
other attention key) is pressed.
.WINDOW=LNORESP (LN is a valid abbreviation for LNORESP) requests ISPF to display only long
messages in a message pop-up window that does not require direct user response. The user can enter
data into the underlying panel while this message is being displayed.
The MSGLOC parameter on the DISPLAY, TBDISPL, and SETMSG services controls the placement
of the message pop-up window. For messages that originate from panel processing, such as a
verification error message, the message pop-up window is placed adjacent to the field which is the
object of the validation. The window placement will be such that it does not overlay the object field, if
possible. If no correlation can be made between the validation and a field (such as when the variable
being validated is not a panel field name), the message pop-up window is displayed at the bottom of
the logical screen or below the active pop-up window, if one exists. See the sections on these services
in the z/OS ISPF Services Guide for a complete description of the MSGLOC parameter.
.TYPE=NOTIFY|WARNING|ACTION|CRITICAL
Optional. (Can be abbreviated to .T) The .TYPE keyword in the message definition identifies the type
of message. There are four types of messages, NOTIFY, WARNING, ACTION, and CRITICAL. N, W, A,
and C are valid abbreviations.
This table summarizes the characteristics of the different types of messages.
Table 24. Message characteristics
Type Color Intensity Placement Response Alarm
NOTIFY White High Message area or pop-up
window Optional Off
268  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 297

Table 24. Message characteristics (continued)
Type Color Intensity Placement Response Alarm
WARNING Yellow High Message area or pop-up
window Optional On
ACTION Red High Message area or pop-up
window Optional On
CRITICAL Red High Pop-up window Required On
The .TYPE keyword overrides any .ALARM value that can be specified. A .TYPE=CRITICAL message
is always displayed as though .WINDOW=RESP was specified. The defined color and highlighting
characteristics apply to messages displayed in the default short/long location and a pop-up message
window. The dialog application controls the field attributes for alternate message location fields.
long message
Required. If a short message is not specified, the long message is automatically displayed first, with a
high intensity attribute, in the long message area or in a message pop-up window. The long message
is displayed in a pop-up window if the text is longer than will fit in the long message area, if you
defined a message window using the .WINDOW keyword for the message, or if you have selected this
option on the Settings panel.
The location of the short and long messages in a user-designed panel is specified by the SMSG and
LMSG keywords. These keywords are defined under “Defining the body section” on page 169.
The maximum length of the long message text is 512 bytes. If the message text is greater than
512 bytes, it will be truncated. Messages greater than 78 bytes require multiple long message lines.
The continuation of the long message text into additional lines is indicated by one or more spaces
following the ending quote (') followed by a plus (+) sign. For example:
ISPX001 'short message text'
'Long message text' +
' continued over ' +
'multiple lines.  The maximum length is ' +
'512 bytes.'
For the best results, use the fewest number of message lines possible.
ISPX001 'short message text'
'Long message text continued over multiple lines.  The maximum' +
' length is 512 bytes.'
Consecutive SOSI characters resulting from multiple lines of DBCS data are automatically removed.
For example,
'Long messageSDBS' +
             O  I
'SCSSdata.'
 O  I
Result:  Long messageSDBCSSdata.
                     O    I
The ending SI in the first record and the beginning SO in the second record are automatically removed.
When messages are written to the ISPF log file, both the short message, if any, and the long message
are written in the same output line. The short message comes first, followed by the long message.
The long message text will be written to multiple records if the text is greater than 78 characters.
Existing dialogs which have VDEFINEd the system variable ZERRLM as 78 characters should be
updated to VDEFINE this variable as 512 characters.
Chapter 8. Defining messages  269

## Page 298

Note: For long or short messages in pop-up windows, if the message originates from panel
processing, such as a verification error message, the message pop-up window is placed adjacent
to the field which is the object of the validation.
Message display variations
The tables shown demonstrate various message display situations and the effect of the .TYPE keyword
and the PANEL DISPLAY CUA MODE field on the color and highlighting of the message text. The variations
are dependent on whether you used the Dialog Tag Language (DTL) or the panel definition statements to
define your panels.
If your dialog application panels are generated using the DTL, the dialog manager displays the messages
as shown in Table 25 on page 270.
Table 25. Message display using DTL
Message Definition Text Intensity
.TYPE=NOTIFY .ALARM=YES|NO White High
.TYPE=WARNING .ALARM=YES|NO Yellow High
.TYPE=ACTION .ALARM=YES|NO Red High
.TYPE=CRITICAL .ALARM=YES|NO Red High
.TYPE not specified .ALARM=NO White High
.TYPE not specified .ALARM=YES Yellow High
If your application panels are generated from the panel definition statements and you use the default
message placement, the dialog manager displays the messages as documented in Table 26 on page 270.
Table 26. Message display using panel definition  statements
Message Definition Text Intensity
.TYPE=NOTIFY .ALARM=YES|NO White High
.TYPE=WARNING .ALARM=YES|NO Yellow High
.TYPE=ACTION .ALARM=YES|NO Red High
.TYPE=CRITICAL .ALARM=YES|NO Red High
.TYPE not specified .ALARM=NO CUA mode=YES White High
.TYPE not specified .ALARM=YES CUA mode=YES Yellow High
.TYPE not specified .ALARM=NO CUA mode=NO White High
.TYPE not specified .ALARM=YES CUA mode=NO White High
If you define your panels using the panel definition statements and you use an alternate message
placement, the dialog (using the field attributes) controls the message text color and highlighting.
Messages tagged with CCSID
An ISPF message can be defined with .CCSID=xxxxx where xxxxx is the CCSID of the EXTENDED CODE
PAGE as defined by Character Data Representation Architecture. See “Supported CCSIDs” on page 303
for which CCSIDs are supported.
Panels or messages tagged with the CCSID keyword invoke the TRANS service. The to CCSID is the value
in ZTERMCID. This value is filled in during ISPF initialization as the result of the terminal query done by
ISPF. The from CCSID is the CCSID entered following the CCSID keyword.
270  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 299

If the CCSID keyword is used, the characters in the message are translated to the equivalent characters in
the terminal code page for display. This translation occurs only if the terminal has returned information to
allow ISPF to determine its CCSID and only if the code page indicated by the CCSID is different from the
code page of the terminal.
Note: The same CCSID is used for all messages within a message member. Therefore, this keyword
should be in the first record and start in the first column of the message member. If the .CCSID keyword
is not in the first record or does not start in the first column of the first record, it is ignored and character
translation does not occur.
  .CCSID=xxxxx
  ISPX001 'short message text'
  'Long message text' +
  ' continued over ' +
  'multiple lines.  The maximum length is ' +
  '512 bytes.'
All characters in the message member which are not short or long message text must be in the Syntactic
Character Set:
• A-Z
• a-z
• 0-9
• + < = > % & * " '
• ( ) , _ - . / : ; ?
The beginning and ending inhibited character tables are enhanced to include characters from the
extended code pages for the supported Asian Pacific languages in formatting message text. The CCSID
of the message is used to determine which tables to use. If no CCSID is specified, the session language
ID and terminal type determine the tables used. See Chapter 10, “Extended code page support,” on page
299 and “Message pop-up text formatting” on page 271.
Modeless message pop-ups
ISPF allows you to cancel a modeless message pop-up by positioning the cursor within the bounds of
the message pop-up and requesting CANCEL or ENTER. This allows you to remove the message pop-up
without submitting the underlying panel for processing.
For the cursor to be within the bounds of the message pop-up, it must be inside the window frame of
the message. Placing the cursor on the message window frame does not result in the message window
being canceled. Note that asynchronous command processing is not suspended when the cursor is placed
inside a message window. Therefore, commands such as PRINT and SPLIT are started when typed on the
command line and Enter pressed, even if the cursor is placed inside a modeless message pop-up window.
The HELP command will not display message help for a message window that has been canceled.
Message pop-up text formatting
The message text is retrieved from the message member. If it is more than one line (that is, if ISPF finds
at least one blank and a plus sign following the closing quote) the lines are concatenated, including blanks
within or at the end of the text. Trailing blanks are stripped from any variable values before the values are
substituted into the text string.
The width of the message pop-up window is determined based on the location where the window will
be placed. If the message is displayed as a result of a panel verification error, the message pop-up is
displayed relative to the field in error. If the MSGLOC parameter is specified on the DISPLAY or SETMSG
service, the message pop-up is displayed relative to the specified field name. If the MSGLOC parameter
is not specified, the message pop-up will be displayed at the bottom of the logical screen or below the
active ADDPOP pop-up window, if one exists.
Chapter 8. Defining messages  271

## Page 300

The width of the window will be the width from this determined location to the right edge of the screen.
Note that this width will vary based on the screen size the user is running with.
ISPF determines if the message text is to be formatted according to English rules or Asian rules based on
the type of data in the message text, MIXED or EBCDIC, together with the message CCSID or the current
ISPF session language variable, ZLANG.
If the data contains double-byte characters and the message CCSID is 00930, 00933, 00935, 00937, or
00939, the Japanese (Katakana), Korean, Simplified Chinese, Traditional Chinese, or Japanese (Latin) text
formatting rules are used, respectively. If the data contains double-byte characters and the message does
not have a CCSID or the CCSID is not 00930, 00933, 00935, 00937, or 00939 and the ZLANG value is
JAPANESE, CHINESET, CHINESES, or KOREAN, the Japanese, Traditional Chinese, Simplified Chinese, or
Korean text formatting rules are used, respectively. If the data contains double-byte characters and the
message does not have a CCSID, or if the message CCSID is not 00930, 00933, 00935, 00937, or 00939,
or if the ZLANG is not JAPANESE, CHINESET, CHINESES, or KOREAN, the Japanese text formatting rules
are used by default.
If the data is all single-byte data and there is no CCSID for the message, ISPF determines if the
application is running on a Japanese Katakana terminal and if the NOKANA keyword was specified on
the message definition. If so, ISPF uses the English formatting rules. If NOKANA was not specified, ISPF
uses the Japanese Katakana formatting rules. If the application is not running on a Katakana terminal and
there is no CCSID for the message, ISPF uses the English formatting rules.
English rules for message text formatting
Message text exceeding the width of the message window is wrapped to the next line. The text is split at
blanks only. If a word is longer than the message window width, the window is expanded to the width of
this word. However, if a word exceeds the maximum window size (screen width minus 3), the word will
be split and continued on the next line. Once the message formatting is complete, the message pop-up
window width will be decreased to the length of the longest line, excluding trailing blanks.
Asian rules for message text formatting
Some characters should not be placed at the beginning of a line, and some should not be placed at
the end of a line. These beginning-inhibited and ending-inhibited characters are different among the
languages, yet the required process is the same. Thus, ISPF uses the same text formatting process
for the Asian languages, but it uses a different beginning-and-ending-inhibited character table for each
language. The CCSID of the message is used to determine which tables to use. If no CCSID is specified,
the session language ID and terminal type determine the tables used. See Chapter 10, “Extended code
page support,” on page 299.
The message text is first split into words. An SBCS "word" is delimited by blanks, or SO/SI characters.
Then any beginning inhibitors are stripped from the beginning of the word and treated as separate words,
and any ending inhibitors are stripped from the end of the word and treated as separate words.
Adjoining DBCS alphanumeric characters (that is, Ward 42 characters) are treated as one DBCS "word".
Then any beginning inhibitors are stripped from the beginning of the word and treated as separate words,
and any ending inhibitors are stripped from the end of the word and treated as separate words. All other
non-Ward 42 double-byte characters are treated as separate DBCS words.
If a word is longer than the message window width, the window is expanded to the width of this word.
However, if a word exceeds the maximum window size (screen width = 3), the word will be split and
continued on the next line. If the text consists of mixed data and does not fit in one line within the
specified width, the first position will always be reserved for an SO character (if first word is double-byte)
or for a blank (if the first word is single byte). This will allow the text to be aligned properly.
Words that exceed the width of the message window are wrapped to the next line according to following
rules:
272  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 301

where:
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
Split as is.
Note: If words CE or CB are single-byte words and are more than one character, or if CE or CB are
double-byte words and are more than one double-byte character, no special processing is used; the line is
split as is.
SBCS and DBCS blanks that end or begin a line will be deleted.
Substitutable parameters in messages
A substitutable parameter, a dialog variable name preceded by an ampersand (&), can appear anywhere
within the short and long message text. For example:
'Volume &VOL not mounted'
Substitutable parameters can also be used to specify the value of .HELP or .ALARM, as follows:
'Volume &VOL not mounted'  .HELP = &H  .ALARM = &A
where variable H must contain a panel name or single asterisk, and variable A must contain YES or NO.
Substitutable parameters can also be used to specify the value of .TYPE and .WINDOW.
Chapter 8. Defining messages  273

## Page 302

Substitutable parameters in messages are normally replaced with values immediately before the
message displays. If the message is specified for display by using the SETMSG service, substitutable
parameters are replaced during SETMSG processing. When the GETMSG service is invoked, substitutable
parameters are replaced at the time of the GETMSG call. After substitution of the variables, the short
message is truncated to 24 characters and the long message is truncated to 512 characters.
Syntax rules for consistent message definition
These rules apply to the syntax of messages as they appear in the message library (Figure 74 on page
266):
• The message ID must begin in column 1 of the first line, and the long message must begin in column
1 of the second line. For readability, one or more blank lines can separate the two-line message
specifications within the member.
• Comments can precede or follow a two-line message specified within a member. A comment begins
with the characters /* starting in column one.
• In the first line, the fields must be separated by at least one blank. One or more blanks can optionally
occur on either side of an equal sign (=).
• The short message, if specified, and the long message must each be enclosed in single quotes ('). If the
short message is omitted, the enclosing single quotes are also omitted.
• Within the short or long message text, any non-alphanumeric character can terminate a variable name.
For example:
'Enter &X, &Y, or &Z'
where a comma terminates the variable names X and Y. The name Z is delimited by the single quote
that marks the end of the message.
• A period (.) at the end of a variable name has a special meaning. It causes concatenation with the
character string following the variable. For example, if the value of variable V is ABC, then:
'&V.DEF' yields 'ABCDEF'
• A single ampersand followed by a blank is interpreted as a literal ampersand character, not the
beginning of a substitutable variable. An ampersand followed by a nonblank is interpreted as the
beginning of a substitutable variable.
• A double ampersand can be used to produce a character string starting with an ampersand. The double
character rule also applies to single quotes within the delimiting single quotes required for the short and
long message text, and to a period, if it immediately follows a variable name. For example:
 &&  yields  &
 ‘’  yields  '  within delimiting single quotes
 ..  yields  .  immediately following a variable name.
DBCS-related variables in messages
These rules apply to substituting DBCS related variables in messages. These rules also apply to file
skeletons and file-tailoring operations.
• If the variable contains MIX format data, each DBCS subfield must be enclosed with shift-out and
shift-in characters.
Example:
eeee[DBDBDBDBDB]eee[DBDBDB]
ee... represents a field of EBCDIC characters
DBDB... represents a field of DBCS characters
-[ ]- represent shift-out and shift-in characters.
• If the variable contains DBCS format data only, the variable must be preceded by the ZE system
variable, without an intervening blank.
274  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 303

Example:
 ...text...&ZE&DBCSVAR..text...
 
• If the variable contains EBCDIC format data and is to be converted to the corresponding DBCS
format data before substitution, the variable must be preceded by the ZC system variable, without
an intervening blank.
Example:
 ...text...&ZC&EBCSVAR..text...
 
The ZC and ZE system variables can only be used for the two purposes described.
Chapter 8. Defining messages  275

## Page 304

276  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
