# Chapter 4. Using edit models

Source file: f54em00_v3r1.md
Start page: 101
Page span: 101-110

## Page 101

Chapter 4. Using edit models
This topic describes edit models and tells you how to use them.
What is an edit model?
An edit model is a predefined set of statements for a dialog element that you can include in the data you
are editing and then modify to suit your needs. When you enter the MODEL command, you can select the
correct segment for the data type being edited.
ISPF includes an initial set of models for panels, messages, skeletons, and command and program
processing of ISPF services. You can add more. There are no models of edit macro commands and
assignment statements.
A model has two parts:
Data lines
These are the actual lines that are placed in the data you are editing. For example, the data might be
a dialog service call or a panel format. You can update fields in the source statements by inserting
names, parameters, and so forth.
The models also include source statement comments for models of dialog service calls to document
the meanings of the possible return codes from the service. The comments are in a valid format
for the particular kind of model. These comments give you the information you need to develop
error-handling logic for your function. Sometimes they provide parameter descriptions for other kinds
of models.
Notes
Notes provide tutorial information about how to complete source code statements. You can specify
whether you want the notes displayed during the edit session by using the NOTES command or the
NOTES or NONOTES operand on the MODEL command. To remove notes from the panel, issue RESET.
To convert the notes to data so that they can be saved with your data set, use the MD (make dataline)
line command.
How models are organized
Models are organized and named according to a hierarchy based on the type and version of the dialog
element they represent. Each part of the model's name corresponds to a level in the hierarchy.
The first part of the logical name is the model class. There is a model class for each data set type qualifier
that can store a dialog element. The Model Classes panel, Figure 17 on page 70, lists the classes defined
for the models distributed with ISPF. This panel prompts you when you need to set the desired model
class, if you do not name the class explicitly.
Model hierarchy
© Copyright IBM Corp. 1984, 2024 69

## Page 102

Model Classes
 Enter number or Class of model.
 Enter END command to cancel MODEL command.
 1  CLIST    - ISPF services in CLIST commands
 2  COBOL    - ISPF services in COBOL programs
 3  EXEC     - ISPF services in EXEC commands
 4  FORTRAN  - ISPF services in FORTRAN programs
 5  MSGS     - Message format
 6  PANELS   - Panel formats and statements
 7  PLI      - ISPF services in PLI programs
 8  SKELS    - File tailoring control statements
 9  PASCAL   - ISPF services in PASCAL programs
 10 REXX     - ISPF services in TSO/REXX commands
 11 DTL      - ISPF Dialog Tag Language formats and statements
 12 C        - ISPF services in C/370 programs
 13 SCLM     - SCLM Project Definition Macros
 14 ARCHDEF  - SCLM Architecture Definition templates
 Option ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 17. Model Classes panel (ISREMCLS)
You can use the default for this part of the logical name whenever the edit profile name matches the class
of the model desired.
The second part of the logical name is the model name, which identifies the specific model within the
model class. Frequently, it uniquely identifies a model and completes the logical name. To uniquely
identify a model, you can define optional qualifiers . Qualifiers are used, for example, to differentiate
among the various kinds of panel verification (VER) statements.
A hierarchy of selection panels defines the hierarchy of models. The different parts of the logical name
of a model are selections on the panels that you can choose either by keyword name or option identifier.
This allows you to be prompted by selection panels if you do not know the logical name of the model you
want or to bypass the display of these panels if you do know the name.
Usually, you do not need to worry about the model class. You must specify it only if you want to use a
class that is different from the edit profile name. The model function of the editor recognizes PANELS as
a valid type qualifier for panel models, so you do not need to specify the class when requesting a panel
model from a data set with a type qualifier of PANELS (assuming you allow the edit profile name to default
to panels).
Assume, however, that you call your panels screens and maintain them in a data set with a type of
SCREENS. When you want to use a model to develop a new panel, you enter the MODEL command. The
model function does not recognize SCREENS as a model class, so you are prompted to identify the class
you want, which is the PANELS class in this situation.
Once you have specified a class, whether by panel selection or by use of the MODEL CLASS command,
that class remains in effect until you change it. The two ways to change the class specification are by
typing a data set name with a different type qualifier, or by leaving the Edit Entry panel.
How to use edit models
You use models to assist you in defining a dialog element. To use a model, first edit your data. Then
determine where you want to place the model. If you are editing existing data, define a label or use the
A (after) or B (before) line command to show where the model goes. You do not need to use the A or B
command when you have a new data set. Then type MODEL on the command line and press Enter.
If you know the logical name of the model you want, you can use it to directly access the model. Type
MODEL mmm, where mmm is the name of the model. For example, if you want the model for LMCLOSE,
you would specify MODEL LMCLOSE. If you enter MODEL with no parameters, PDF displays a series of
selection panels, from which you select the model name and any qualifiers.
How to use edit models
70  z/OS: z/OS ISPF Edit and Edit Macros

## Page 103

The original data is then displayed with the model in place. You can type over or use line commands to
change the data lines in the model to meet your needs.
As an example, assume that you are writing a dialog function using CLIST commands and
you want to have the CLIST display a panel. You are editing your CLIST member, called
USERID.PRIVATE.CLIST(DEMO1). Since your data set type, CLIST, matches the class of models you want,
you can allow the model class to default. If you enter MODEL without a model name, the CLIST Models
panel, Figure 18 on page 71, appears.
Figure 18. CLIST Models panel (ISREMCMF)
If you select option D1 (DISPLAY), the editor inserts the model for the DISPLAY service in your CLIST, as
shown in Figure 19 on page 72. The lines are inserted at the location you specify with a label or an A or B
line command. Notes are identified by the characters =NOTE= in the line command field.
How to use edit models
Chapter 4. Using edit models  71

## Page 104

File  Edit  Edit_Settings  Menu  Utilities  Compilers  Test  Help
 ───────────────────────────────────────────────────────────────────────────────
 EDIT       LSACKV.PRIVATE.CLIST(EDITOLD) - 01.01           Columns 00001 00072
 ****** ***************************** Top of Data ******************************
 000100   ISPEXEC  DISPLAY  PANEL(PANELNAM)     MSG(MSG-ID)        +
 000200                     CURSOR(FIELDNAM)    CSRPOS(POS#)       +
 000300                     COMMAND(COMMANDS)   RETBUFFR(BUF-NAME) +
 000400                     RETLGTH(LNG-NAME)   MSGLOC(MSG-FIELD)
 =NOTE=
 =NOTE=      PANELNAM  - OPTIONAL, NAME OF THE PANEL TO BE DISPLAYED.
 =NOTE=      MSG-ID    - OPTIONAL, IDENTIFIER OF A MESSAGE TO BE DISPLAYED ON
 =NOTE=                  THE PANEL.
 =NOTE=      FIELDNAM  - OPTIONAL, NAME OF THE FIELD WHERE THE CURSOR IS TO BE
 =NOTE=                  POSITIONED.
 =NOTE=      POS#      - OPTIONAL, POSITION OF CURSOR IN FIELD. DEFAULT IS 1.
 =NOTE=      COMMANDS  - OPTIONAL, NAME OF A VARIABLE WHICH CONTAINS THE CHAIN
 =NOTE=                  OF COMMANDS.
 =NOTE=      BUF-NAME  - OPTIONAL, NAME OF A VARIABLE WHICH CONTAINS THE
 =NOTE=                  REMAINING PORTION OF THE COMMAND CHAIN TO BE STORED
 =NOTE=                  IF AN ERROR OCCURS.
 =NOTE=      LNG-NAME  - OPTIONAL, NAME OF A VARIABLE WHICH CONTAINS THE LENGTH
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 19. DISPLAY Service Model
With the notes as a guide, you can edit the CLIST to change the DISPLAY service call parameters for your
function. The error-handling source code shown serves as a skeleton which you can update. Finally, use
RESET to eliminate the notes from the panel, leaving the service call, the error-handling logic, and the
comments. Some models also include examples in NOTE lines. Use the MD line command to turn NOTE
lines into data lines.
Adding, finding, changing, and deleting models
Models are implemented in a general fashion, so your installation can apply and use the concept for other
tasks besides dialog development. You can create a set of PL/I call models for your IMS applications, or a
set of report format models for your sales forecasting application. You can also create models for the JCL
statements that you use most frequently.
Similarly, you may find that the models provided for panel formats do not correspond to the standards for
your local installation or for your particular application. You can change the distributed panel models to
match your own requirements.
This topic describes how you can add a new model to your skeleton library, change an existing model, or
delete an existing model.
Adding models
To create a new model, you must:
1. Determine the data set name and member name for the model. For actual use, the model must be in a
skeleton library.
2. Create the source code for the model. Consider whether you should create all new source code or
change an existing model under a new name.
When you create a COBOL model, make sure number mode is on. Then, when you save the model, turn
number mode off.
3. Make the model accessible from a model selection panel by having its selection call the program
ISRECMBR with the actual model member name as its parameter. This involves:
• Changing an existing model selection panel to add the new panel.
• Creating a new model selection panel. If you do this, you must add the new panel to the hierarchy of
selection panels by changing one of the higher-level panels.
• No change, if you are replacing an existing model with an updated model with the same name.
Adding, finding, changing, and deleting models
72  z/OS: z/OS ISPF Edit and Edit Macros

## Page 105

• Adding the word NOSEQ after the model member name if you wish to check that model data is not
being overlaid by editor sequence numbers.
As an example of adding a model, assume that you want to create a model for multiple-line block letters.
Since you intend to use these block letters on panels, the model becomes part of the panel model class.
To build each model block letter, use the editor to create a new member in your skeleton library. For
example, you could create a member called BLKI containing this model for the letter I:
        IIIIIIIIII
            II
            II
            II
            II
            II
        IIIIIIIIII
)N
)N   the letter I for logo
Once the model for each letter is built, you must update the selection panel in the prompting sequence
that deals with panel model selection. This panel is named ISREMPNL and is stored in the system panel
library. Figure 20 on page 73 shows the last few lines in ISREMPNL:
                                Panel Models
 Option ===>                                                               
 Enter number or statement name.
 Enter END command to cancel MODEL command.
                                                                More:   -
 ⋮
 S18 CUAATTR  - CUA attributes
 S19 *REXX    - Rexx in panel procedures
 P0  PANSECT  - Panel Sections - Other definitions
 Panel Formats:
 F0  PANFORM
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 20. Panel Models panel (ISREMPNL)
Copy the panel shown in Figure 20 on page 73 into your panel data set and change it by adding a format
F1, BLOCKLTR. See Figure 21 on page 73 for an example.
                                Panel Models
 Option ===>                                                               
 Enter number or statement name.
 Enter END command to cancel MODEL command.
                                                                More:   -
 ⋮
 S18 CUAATTR  - CUA attributes
 S19 *REXX    - Rexx in panel procedures
 P0  PANSECT  - Panel Sections - Other definitions
 Panel Formats:
 F0  PANFORM
 F1  BLOCKLTR
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 21. Changed Panel Models panel (ISREMPNL)
If there are several new models, this panel should be updated so that when you select F2, a new Block
Letter selection panel is displayed. Therefore, you should change the )PROC section of panel ISREMPNL
to include item F2. See Figure 22 on page 74 for an example.
Adding, finding, changing, and deleting models
Chapter 4. Using edit models  73

## Page 106

Figure 22. Changed )PROC section of Panel Models panel (ISREMPNL)
This concept allows you and other users to have sets of individual models, and allows the installation to
have its own set of general models, without having multiple copies of the PDF model selection panels.
For each model class, the installation could provide two additional entries on the selection panel: one for
installation-wide models and one for your models. Each entry could point to a selection panel, with each
user having a copy of the selection panel to customize for individual use.
Note that the entry for F2, BLOCKLTR, points to a new panel, ISRBLOCK, which you would now build.
You can change an existing panel model to create the new panel. Figure 23 on page 75 shows how the
new panel might be typed. Note particularly the )INIT and )PROC sections of the coding. In the )PROC
section of panel ISRBLOCK, the target for all valid selections is the program ISRECMBR. The parameter
passed to this program is different for each separate, but valid, selection and is the name of the model for
that selection. Thus, for our example, the model name for selection 1 or I is BLKI.
You should follow the )INIT source code and the end source code in the )PROC section shown in Figure 23
on page 75 for all new panels.
Adding, finding, changing, and deleting models
74  z/OS: z/OS ISPF Edit and Edit Macros

## Page 107

)PANEL
/*  ISRBLOCK                                                        */
/*  5647-A01 (C) COPYRIGHT IBM CORP 1995, 2003                      */
/*  Sample source code for the Block Letter Model selection panel.  */
)ATTR
)BODY
%-------------------------  BLOCK LETTER  ------------------------
%OPTION  ===>_ZCMD                     +
%
%   1 +I           - Block letter I
%   2 +J           - Block letter J
%   3 +K           - Block letter K
%
%
+
+ Enter %END+command to cancel MODEL command. +
%
)INIT
  .CURSOR = ZCMD
  .HELP = ISRxxxxx
  IF (&ISRMDSPL = 'RETURN  ')
       .RESP = END
)PROC
  &ZSEL = TRANS(TRUNC (&ZCMD,'.')
          1,'PGM(ISRECMBR) PARM(BLKI)'
          I,'PGM(ISRECMBR) PARM(BLKI)'
          2,'PGM(ISRECMBR) PARM(BLKJ)'
          J,'PGM(ISRECMBR) PARM(BLKJ)'
          3,'PGM(ISRECMBR) PARM(BLKK)'
          K,'PGM(ISRECMBR) PARM(BLKK)'
          *,'?' )
  IF (&ZSEL = '?')
     .MSG = ISRYM012
  &ISRMMEND = 'N'                    /* SET THE END INDICATOR TO NO   */
  IF (.RESP = END )                  /* IF ENDING, WHY ... WHO CAUSED */
    IF (&ISRMONCL = 'Y')             /* MAKE SURE ITS NOT A CLASS OP. */
      IF (&ISRMDSPL = 'RETURN  ')    /* MAKE SURE ITS NOT END ON MBR. */
        &ISRMMEND = 'Y'              /* NO - ITS BECAUSE USER HIT END */
)END
Figure 23. Source code for Block Letter Model Selection panel
Finding models
Before you change or delete a model, you must determine the physical name of the model in the skeleton
library. See z/OS ISPF Planning and Customizing for a list of the names of the models of dialog elements
distributed with PDF. In addition, you can use the method shown here to find the member name for any
model.
You can find the member name for any model in the )PROC section of the final selection panel used to get
it. The member name is the parameter passed to ISRECMBR, the program called when you choose that
selection.
To determine the name of the model selection panel so that you can look at it to find the model member
name, use the PANELID command when that panel is displayed. Then use the Browse or Edit options to
look at the member of the panel library with that name.
Changing models
To change a model that currently exists, copy the existing model from the skeleton data set into your own
data set. Then use the editor to change the model in the same way you would change any text data set.
Note: Any lines that are to contain notes must have )N in positions 1 and 2, followed by one or more
blanks, as shown in this example.
    VARIABLE = VALUE
)N        VARIABLE - A DIALOG VARIABLE OR A CONTROL VARIABLE.
)N        VALUE    - A LITERAL VALUE CONTAINING: SUBSTITUTABLE
)N                   VARIABLES, A DIALOG VARIABLE, A CONTROL
)N                   VARIABLE, OR AN EXPRESSION CONTAINING A
Adding, finding, changing, and deleting models
Chapter 4. Using edit models  75

## Page 108

)N                   BUILT-IN FUNCTION.
)N        EXAMPLES:  &DEPT = 'Z59'   &A = &B    &C = ' '
When the model is later accessed using MODEL, the lines with )N indicators are flagged with =NOTE= in
the line command field (Figure 19 on page 72).
Deleting models
You can delete models by deleting the references to them. To delete the references, remove the entry
referencing the model in both the )BODY and )PROC sections of the model selection panel.
Generally, you can leave the model itself in the skeleton library. However, if you are deleting a substantial
number of models, you can delete those members from the library and then compress it.
Adding, finding, changing, and deleting models
76  z/OS: z/OS ISPF Edit and Edit Macros

## Page 109

Part 2. Edit macros
© Copyright IBM Corp. 1984, 2024 77

## Page 110

78  z/OS: z/OS ISPF Edit and Edit Macros
