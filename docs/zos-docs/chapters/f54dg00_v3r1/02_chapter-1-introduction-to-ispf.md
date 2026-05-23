# Chapter 1. Introduction to ISPF

Source file: f54dg00_v3r1.md
Start page: 29
Page span: 29-34

## Page 29

Chapter 1. Introduction to ISPF
This topic describes ISPF at an introductory level. It explains what ISPF is and what it does for you.
What is ISPF?
Consider the Interactive System Productivity Facility (ISPF) program product an extension of the MVS
Time Sharing Option (TSO) host system on which it runs. ISPF services complement those of the host
system to provide interactive processing. ISPF is similar to a control program or access method in that it
provides services to dialogs (applications) during their execution. The types of services provided by ISPF
are:
• Display services
• File-tailoring services
• Variable services
• Table services
• Miscellaneous services
• Dialog test facility, including:
– Setting breakpoints
– Tracing usage of dialog services and dialog variables
– Browsing trace output in the ISPF log data set
– Examining and updating ISPF tables
– Interactively invoking most dialog services.
A dialog receives requests and data from a user at a terminal. The dialog responds by using ISPF services
to obtain information from, or enter information into, a computer system.
What is a dialog?
To understand the dialog interface, you must first understand what a dialog is. A dialog is the interaction
between a person and a computer. It helps a person who is using an interactive display terminal to
exchange information with a computer.
The user starts an interactive application through an interface that the system provides. The dialog with
the user begins with the computer displaying a panel and asking for user interaction. It ends when the
task for which the interactions were initiated is completed.
A dialog developer creates the parts of a dialog, called dialog elements. Each dialog application is made
up of a command procedure or program, together with dialog elements that allow an orderly interaction
between the computer and the application user.
The elements that make up a dialog application are:
• Functions
• Variables
• Command tables
• Panel definitions
• Message definitions
• File-tailoring skeletons
• Tables
© Copyright IBM Corp. 1980, 2025 1

## Page 30

A dialog does not necessarily include all types of elements. For example, certain kinds of applications do
not use tables and skeletons.
Functions
A function is a command procedure or a program that performs processing requested by the user. It can
invoke ISPF dialog services to display panels and messages, build and maintain tables, generate output
data sets, and control operational modes.
A function can be coded in a command procedure language using CLIST or REXX or in a programming
language, such as PL/I, COBOL, FORTRAN, APL2, Pascal, or C.
You can use more than one language in a dialog application. For example, within a single application
containing three functions, each function could be written using a different language, such as PL/I,
COBOL, or FORTRAN. One or more of the functions can be written using a command procedure language
instead of a programming language.
Note:
1. ISPF functions written in PL/I should not be linked with the PL/I multitasking libraries.
2. ISPF functions written in FORTRAN should be linked in FORTRAN link mode. That is, include the
VLNKMLIB library ahead of the VFORTLIB library in the SYSLIB concatenation. See the VS FORTRAN
Programming Guide for additional information.
3. ISPF functions written in the C language should be linked with the C$START load module. For more
information, see the C Compiler User's Guide.
4. A function coded in a programming language can be designed for cross-system use, to be processed
by equivalent levels of ISPF running under VM and z/OS. Such a function would need to use equivalent
ISPF services available on both VM and z/OS.
Variables
ISPF services use variables to communicate information among the various elements of a dialog
application. ISPF provides a group of services for variable management. Variables can vary in length
from zero to 32K bytes and are stored in variable pools according to how they are to be used. A set of
variables whose names begin with the character Z are system variables. Z variables are reserved for ISPF
system-related uses.
Command tables
A system command table (ISPCMDS) is distributed with ISPF in the table input library. An application can
provide an application command table by including a table named xxxxCMDS in its table input library,
where xxxx is a 1- to 4-character application ID. In addition, you can specify up to three User command
tables and up to three Site command tables. The application IDs for the User and Site command tables
are specified in the ISPF Configuration table. You can also specify if the Site command tables are to be
searched before or after the system command table.
You can define an application command table either by using the Dialog Tag Language (DTL) and ISPF
conversion utility, or by using ISPF option 3.9.
When a user enters a command, the dialog manager searches the application command table, if any, and
then the system command table. If it finds the command, action is taken immediately. If it does not find
the command in the application or system tables, the command is passed to the dialog, unaltered, in the
command field. The dialog then takes appropriate action.
Note: You can use the TSO ISPCMDTB command to convert existing command tables to DTL. To
use ISPCMDTB, ensure the command table is in your table concatenation (ISPTLIB), then type TSO
ISPCMDTB applid (where applid is the application id of the command table). This will begin an edit
session containing the DTL version of the command table. Use the editor CREATE or REPLACE command
to save the table to your DTL source data set.
2  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 31

Panel definitions
A panel definition is a programmed description of the panel. It defines both the content and format of a
panel.
Most panels prompt the user for input. The user's response can identify which path is to be taken through
the dialog, as on a selection panel. The response can be interpreted as data, as on a data-entry panel.
Panels can invoke REXX statements, enabling the dialog developer to use the powers of the REXX
language to perform operations such as arithmetic, formatting of dialog variables, and verification,
transformation, and translation of data.
Message definitions
Message definitions specify the format and text of messages to users. A message can confirm that a
user-requested action is in progress or completed, or it can report an error in the user's input. Messages
can be superimposed on the display to which they apply, directed to a hardcopy log, or both.
File-tailoring skeletons
A file-tailoring skeleton, or simply a skeleton, is a generalized representation of sequential data. It can
be customized during dialog execution to produce an output data set. After a skeleton is processed, the
output data set can be used to drive other processes. File skeletons are frequently used to produce job
data sets for batch execution.
Tables
Tables are two-dimensional arrays that contain data and are created by dialog processing. They can be
created as a temporary data repository, or they can be retained across sessions. A retained table can also
be shared among several applications. The type and amount of data stored in a table depends on the
nature of the application.
Tables are generated and updated during dialog execution. The organization of each table is specified to
ISPF using ISPF table services.
What does a dialog do?
You can use ISPF to simplify the programming that provides interactive application operations.
Operations performed during dialog execution include:
• Identifying to the user choices of available processing routines
• Invoking a requested routine, based on the user's choice
• Prompting the user to enter data
• Reading the data into a work area
• Checking the data to verify that it is appropriate for the application
If the data is not appropriate for the application:
– Identifying the error to the user
– Prompting the user to enter new data and verifying that data
If the entered data is in the proper form:
– Displaying any information requested by the user
– Processing or storing the user's data, then advising the user of its disposition
• Creating sequential output data sets or reports
• Providing online messages, help, and tutorial displays to help users understand application processing.
Chapter 1. Introduction to ISPF  3

## Page 32

Developing a dialog
A developer, using an editor such as the PDF editor in Option 2 of ISPF, develops a dialog by creating
its various elements at a terminal and storing them in libraries. You can use any available editor when
creating dialog elements.
However, in addition to an editor, ISPF provides special facilities to aid dialog development. Examples of
these facilities are:
• A VIEW facility for displaying source data or output listings
• Utilities to simplify data handling
• Programming-language processing facilities
• Edit models for messages, file-tailoring skeletons, panels, and DTL source
• Library access services for accessing both ISPF libraries and other data sets.
Figure 2 on page 4 shows a developer using ISPF to create and test dialog elements. As shown in the
figure, panel definitions, message definitions, and file-tailoring skeletons are created before running the
dialog. These dialog elements are saved in libraries. The developer stores the program (after compilation)
or command procedure in an appropriate system program library. During dialog testing, tables of data, log
entries, and file-tailoring output data sets can be created by dialog processing. ISPF creates the log data
set the first time the user performs some action that results in a log message, such as saving edited data
or submitting a job to the batch machine. ISPF creates the list data set the first time a user requests a
print function or runs a dialog that issues a LIST service request.
When the developer completes the functions, panel definitions, and any other dialog elements required
by the application being developed, the dialog is ready to be processed under ISPF.
Figure 2. Using ISPF
4  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 33

How dialog elements interact
A dialog can be organized in a variety of ways to suit the requirements of the application and the needs of
the application user.
A typical dialog organization, shown in Figure 3 on page 5, starts with display of the highest menu,
called the primary option menu. User options selected from the primary option menu can result in the
call of a function or the display of a lower-level menu. Each lower-level menu can also cause functions to
receive control or still other menus to be displayed.
Eventually, a function receives control. The function can use any of the dialog services provided by ISPF.
Typically, the function can continue the interaction with the user by means of the DISPLAY service. The
function might also display data-entry panels to prompt the user for information. When the function ends,
the menu from which it was invoked is redisplayed.
Figure 3. Typical dialog organization starting with a menu
Figure 4 on page 6 shows another type of dialog organization in which a dialog function receives
control first, before the display of a menu. The function performs application-dependent initialization and
displays data-entry panels to prompt the user for basic information. It then starts the selection process
by using the SELECT service to display the primary option menu for the application.
The same figure also shows how a dialog function can invoke another function without displaying a
menu. It uses the SELECT service to do this, which provides a convenient way to pass control from a
program-coded function to a command-coded function, or vice versa. The invoked function then starts a
lower-level menu process, again by using the SELECT service.
Chapter 1. Introduction to ISPF  5

## Page 34

Figure 4. Typical dialog starting with a function
To relate your application design to CUA design models and principles, refer to the IBM Common User
Access Guidelines. It is recommended that you use DTL to design CUA-based panels. See the z/OS ISPF
Dialog Tag Language Guide and Reference for more information.
Dialog variables
ISPF uses dialog variables to communicate data between the dialog management services and the dialog
elements. A dialog variable's value is a character string that can vary in length from 0 to 32K bytes. Some
services restrict the length of dialog variable data.
Dialog variables are referred to symbolically. The name is composed of 1 to 8 characters (6 for FORTRAN).
Alphanumeric characters A-Z, 0-9, #, $, or @ can be used in the name, but the first character cannot be
numeric. APL variable names cannot contain #, $, or @.
Dialog variables can be used with panels, messages, and skeleton definitions, as well as within dialog
functions. For example, a dialog variable name can be defined in a panel definition, and then referred to in
a function of the same dialog. Or, the variable can be defined in a function, then used in a panel definition
to initialize information on a display panel, then later used to store data entered by the user on the display
panel.
For functions coded in a programming language other than APL2, the internal program variables that are
to be used as dialog variables can be identified to ISPF and accessed using the ISPF variable services. The
use of STEM or COMPOUND variables within a REXX procedure is not supported by ISPF. For a function
coded as CLIST or REXX command procedures or as an APL2 procedure, variables used in the procedure
are automatically treated as dialog variables. In this case, no special action is required to define them to
ISPF.
6  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
