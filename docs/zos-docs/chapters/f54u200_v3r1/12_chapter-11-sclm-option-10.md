# Chapter 11. SCLM (option 10)

Source file: f54u200_v3r1.md
Start page: 439
Page span: 439-440

## Page 439

Chapter 11. SCLM (option 10)
Option 10 gives you access to the Software Configuration and Library Manager (SCLM), which is an
extension of the ISPF library concept. You call SCLM functions by entering one of the options shown on
the panel in Figure 241 on page 401.
If SCLM does not appear on any of your menu panels or on the Menu pull-down, enter TSO SCLM on any
ISPF command line. If SCLM is available to your terminal session, the SCLM Main Menu is displayed.
For more information about SCLM, refer to z/OS ISPF Software Config ur ation  and Library Manager Guide
and Reference.
   Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                 SCLM Main Menu                    Enter option
 Enter one of the following options:
    1  View        ISPF View or Browse data
    2  Edit        Create or change source data in SCLM databases
    3  Utilities   Perform SCLM database utility/reporting functions
    4  Build       Construct SCLM-controlled components
    5  Promote     Move components into SCLM hierarchy
    6  Command     Enter TSO or SCLM commands
    6A Easy Cmds   Easy SCLM commands via prompts
    7  Sample      Create or delete sample SCLM project
    A  SCLM Admin  Maintaining SCLM administrators
    X  Exit        Terminate SCLM
 SCLM Project Control Information:
    Project . . . .           (Project high-level qualifier)
    Alternate . . .           (Project definition: defaults to project)
    Group . . . . .           (Defaults to TSO prefix)
 Option ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 241. SCLM Main Menu (FLMDMN)
The option names on this panel are point-and-shoot fields. See the Point-and-Shoot Text Fields section of
the ISPF User Interface topic in the z/OS ISPF User's Guide Vol I for more information.
SCLM Main Menu action bar
The SCLM Main Menu panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see details about the Utilities Action Bar Choice in the ISPF User Interface topic
in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides general information about SCLM topics as well as information about
each available choice on the SCLM Main Menu.
SCLM overview
SCLM is a library facility that supports projects in developing complex software applications. It does
this by providing software configuration and library management support. SCLM supports the software
development cycle of an application from the program design phase to release of the final product.
SCLM (option 10)
© Copyright IBM Corp. 1980, 2024 401

## Page 440

SCLM allows designers and programmers to define the architecture of an application (how the
components fit together) and ensures that the architecture definition is followed by automatically
controlling, maintaining, and tracking software components. By automatically enforcing guidelines
and procedures for developing software, SCLM enhances software quality and improves programmer
productivity. For complete information on using SCLM, refer to z/OS ISPF Software Config ur ation  and
Library Manager Guide and Reference.
SCLM addresses these software configuration and library management issues:
• Ensures that two programmers are not working on the same component at the same time.
• Allows users to integrate components only at the correct time and only by using the correct procedure.
• Logs and tracks software changes.
• Provides application integrity; all of the software components used to produce the final product are
available, but controlled.
• Documents the interfaces between the software components.
SCLM provides these facilities for automating software configuration and library management tasks:
Project Definition
Establishes the database.
Edit
Uses the ISPF editor to create and modify the software components.
Build
Integrates the software components.
Promote
Moves software components through the library hierarchy.
Utilities
Maintain the database.
Reports
Generate information about the build and promote activities, and about the contents of the database.
Interactive dialogs, batch interfaces, and callable services provide access to the functions and capabilities
of SCLM. These functions support the routine use of SCLM by:
• Allowing programmers to use the ISPF editor to create and modify software components
• Providing automated draw down and lockout functions without requiring special customizing to suit a
particular installation.
SCLM (option 10)
402  z/OS: z/OS ISPF User's Guide Vol II
