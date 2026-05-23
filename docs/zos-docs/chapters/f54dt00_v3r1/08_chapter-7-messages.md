# Chapter 7. Messages

Source file: f54dt00_v3r1.md
Start page: 169
Page span: 169-174

## Page 169

Chapter 7. Messages
You use messages to communicate information to users; that is, information that you, the application
developer, believe they need to know. Typically, this would be information regarding user actions, status,
or problems that need correction. Additionally, ISPF issues messages when needed to inform users of
situations that ISPF handles.
Dialog Tag Language provides you with a way to define application-provided messages. You use ISPF
services to handle the display of messages. When the application calls for a message to be displayed,
ISPF places it either in the message area of the application panel or within a pop-up window, known as a
message pop-up.
Messages are defined according to their purpose and severity. The four types of messages you can define
are:
Information
To provide information about a user-requested action.
Warning
To provide information about conditions the user may need to be aware of.
Action
To alert the user to an exception condition that requires a response from the user to correct the
situation.
Critical
To alert the user to an exception condition that requires a response from the user to correct the
situation. Critical messages are similar to action messages.
This chapter tells you how to define messages for your applications. For a complete description of ISPF
message processing, refer to z/OS ISPF Dialog Developer's Guide and Reference.
Defining messages
The messages you define using DTL are stored in message members. Each regular message member can
contain up to 10 messages. The conversion utility stores the message members in an ISPF message file
for the application. The DTL tags you use to define messages and message members are:
MSGMBR
Defines a message member. The MSGMBR tag requires an end tag.
MSG
Defines a message within a message member. The text of the MSG tag is the text that appears as
the message. Each message can be up to 512 bytes in length after variable substitution. “Variable
substitution” on page 141 describes variable substitution in messages.
You assign an identifier to each message within a message member. The message identifier is composed
of two required attribute values: the NAME attribute value of the MSGMBR tag and the SUFFIX attribute
value for the MSG tag.
The NAME attribute you specify for the MSGMBR tag can consist of 1-5 uppercase or lowercase
alphabetic characters and 2 numeric characters.
The SUFFIX attribute values for each of the MSG tags you code within a MSGMBR definition must consist
of either 1 numeric character (0-9) or a numeric character (0-9) and an optional suffix character as
defined for ISPF messages. Each of the values must be unique (a message suffix cannot be defined twice
in a message member).
<!doctype DM system>
<msgmbr name=maia00>
  <msg suffix=0>You cannot type a number in the Name field.
  <msg suffix=1>Please include your first name in the Name field.
Defining messages
© Copyright IBM Corp. 1989, 2024 137

## Page 170

<msg suffix=2>Unrecognized character in Name field.  Please correct.
  <msg suffix=3>Unrecognized character in Address field.  Please correct.
  <msg suffix=4>You cannot type a number in the City field.
  <msg suffix=5>Unrecognized character in City field.  Please correct.
  <msg suffix=6>You cannot type a number in the State field.
  <msg suffix=7>You must type two letters in the State field.
  <msg suffix=8>The Zip code exceeds the maximum length.
  <msg suffix=9>You cannot type an alphabetic character in the Zip field.
</msgmbr>
The value of the MSG SUFFIX attribute, when added to the MSGMBR NAME value, forms the message
identifier for that message. For example, the message identifier for this message: "You must type two
letters in the State field". is maia007. If you specify maia007 as the MSG value on a CHECKL tag, this
message is displayed when ISPF detects the error as a result of input validation.
In addition to SUFFIX, the MSG tag has an optional HELP attribute that allows you to identify a help panel
for the message. for information about defining help panels, see Chapter 6, “Information regions and help
panels,” on page 101.
Specifying message severity
The severity you assign a message determines if the alarm is sounded when the message is displayed.
You can specify the severity of a message with the MSGTYPE attribute of the MSG tag. ISPF accepts one
of four values for the MSGTYPE attribute:  INFO (the default value), WARNING, ACTION, or CRITICAL. The
value can be supplied as a variable name.
Information Messages
Use the default value INFO when you want to provide the user with feedback about the state of the
application.
<msgmbr name=orda00>
  <msg suffix=0>Your order is being processed.  Please wait...
</msgmbr>
Warning Messages
Warning messages tell users that a potentially undesirable situation could occur. Users only need to
respond to the message to continue, although corrective action may be required later. ISPF displays
warning messages with an alarm.
<msgmbr name=orda00>
  <msg suffix=0>Your order is being processed.  Please wait...
  <msg suffix=1 msgtype=warning>Your request for the engraving
  option is not valid.
  Please check your request, and correct it if necessary.
</msgmbr>
Action and Critical Messages
Action and critical messages both represent the highest degree of severity. They tell users about
exception conditions that require a response. The user must respond with a specific action to continue
with the application. ISPF displays these messages with an alarm.
Action messages may appear in a pop-up or in the panel message area. Critical messages always
appear in a pop-up.
<!doctype dm system>
<msgmbr name=orda00>
  <msg suffix=0>Your order is being received.  Please wait...
  <msg suffix=1 msgtype=warning>Your request for
  the engraving option is not valid.
Defining messages
138  z/OS: z/OS ISPF DTL Guide

## Page 171

Please check your request, and correct it if necessary.
  <msg suffix=2 msgtype=action>The data you have
  entered is incorrect.
  Please reenter the data.
</msgmbr>
Short messages
The SMSG attribute enables you to specify a short message. The short message does not conform to CUA
architecture, but it is supported for ISPF compatibility.
Assigning messages
Some of the DTL tags have an optional MSG attribute that you use to specify a message-identifier. The
message text associated with the message-identifier specified is displayed when conditions you define for
the tag are not met by the user.
This list contains the DTL tags that have MSG attributes associated with them, and describes the
conditions for each.
CHECKL
Use the MSG attribute of the CHECKL tag to specify a message ISPF displays when the user's input
fails the validation check defined for the check list.
CHOFLD
Use the MSG attribute of the CHOFLD tag to specify a message ISPF displays when the user does not
provide input for a required field. You can only assign a message to a data field when the REQUIRED
attribute has a value of YES.
DTAFLD
Use the MSG attribute of the DTAFLD tag to specify a message ISPF displays when the user does not
provide input for a required field. You can only assign a message to a data field when the REQUIRED
attribute has a value of YES.
LSTCOL
Use the MSG attribute of the LSTCOL tag to specify a message ISPF displays when the user does not
provide input for a required entry. You can only assign a message to a list column when the REQUIRED
attribute has a value of YES.
SELFLD
Use the MSG attribute of the SELFLD tag to specify a message ISPF displays when the user does not
provide input for a required single-choice selection field. You can only assign a message to a selection
field when the REQUIRED attribute has a value of YES.
Use the SELMSG attribute of the SELFLD tag to specify a message ISPF displays when the user selects
an invalid choice.
Use the SELMSGU attribute of the SELFLD tag to specify a message ISPF displays when the user
selects an unavailable choice.
VARCLASS
Use the MSG attribute of the VARCLASS tag to specify a message ISPF displays when the user's input
fails the validity check defined by the VARCLASS TYPE attribute.
Note: The message specified by the MSG attribute of a VARCLASS tag is also used if enclosed checks
(CHECKL tag) or translations (XLATL tag) do not include the MSG attribute.
XLATL
Use the MSG attribute of the XLATL tag to specify a message that ISPF displays when the user's input
fails a specified translation.
ISPF displays a default message for most of the situations listed above if you do not specify the MSG
attribute.
To show you how messages are associated with DTL tags, here is an example that defines a data field that
requires input from the user. It also defines a message member that contains the warning message ISPF
Defining messages
Chapter 7. Messages  139

## Page 172

displays if the user does not provide input for the field. Figure 79 on page 140 shows the displayed panel
and message.
<!doctype dm system>
<varclass name=namecls type='char 30'>
<varlist>
  <vardcl name=name varclass=namecls>
</varlist>
<msgmbr name=ordb00>
  <msg suffix=0 msgtype=warning>You must type your name in the Name field.
</msgmbr>
<panel name=msgxmp1>Application Panel
 <dtafld datavar=name pmtwidth=12 required=yes msg=ordb000>Name
<cmdarea>
</panel>
                             Application Panel
 
 Name . . . . ______________________________
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 You must type your name in the Name field.
 Command ===> ____________________________________________________________
Figure 79. Data field  and message
Displaying messages
You can specify how a message is to be displayed, either in the panel message area or a pop-up, using the
LOCATION attribute of the MSG tag.
There are five valid values you can assign to LOCATION: AREA (the default), MODAL, MODAL(L),
MODELESS, and MODELESS(L). AREA specifies that the message is to appear in the panel message
area, unless the text of the message exceeds the length of the message area. If the text of the message
exceeds the message area length, ISPF displays the message in a pop-up.
If you want a message that requires a response from the user to appear in a pop-up, specify the MODAL
or MODAL(L) value for the LOCATION attribute. This is useful for presenting warning and action messages
that have a good deal of text.
If you want a message that does not require a response from the user to appear in a pop-up, specify the
MODELESS or MODELESS(L) value for the LOCATION attribute.
For further discussion of these LOCATION values, see “MSG (Message)” on page 352.
Here is an example message member markup that contains three messages, each of them with a different
LOCATION value. The second and third messages display in pop-up windows.
Defining messages
140  z/OS: z/OS ISPF DTL Guide

## Page 173

<!doctype dm system>
<msgmbr name=orda01>
  <msg suffix=0>Your order is being received.  Please wait...
  <msg suffix=1 msgtype=warning location=modeless>Your request for
  the engraving option is not valid.
  Please check your request, and correct it if necessary.
  <msg suffix=2 msgtype=action location=modal>The data you have
  entered is incorrect.
  Please reenter the data.
</msgmbr>
Variable substitution
You can specify a variable in the text of a message by using the VARSUB (variable substitution) tag. When
the message is displayed, ISPF inserts the current value of the variable into the text of the displayed
message.
You code the VARSUB tag within the text of the message where you want the substitution to be made. You
use the required VAR attribute of the VARSUB tag to specify the name of a declared variable whose value
is substituted in the message text.
Here is an example that uses two variable substitutions in the text of the message "msga001". The first
VARSUB specifies the variable invvar, which provides an invoice number in the message. The second
VARSUB specifies the variable datevar, which provides a date in the message.
<!doctype dm system>
<varclass name=invoices type='char 10'>
<varclass name=updates  type='char 8'>
<varlist>
  <vardcl name=invvar varclass=invoices>
  <vardcl name=datevar varclass=updates>
</varlist>
<msgmbr name=msga00>
  <msg suffix=0>Your request is being processed.
  <msg suffix=1>The invoice number you have requested,
  <varsub var=invvar>, was last updated on
  <varsub var=datevar>.
</msgmbr>
Defining messages
Chapter 7. Messages  141

## Page 174

Defining messages
142  z/OS: z/OS ISPF DTL Guide
