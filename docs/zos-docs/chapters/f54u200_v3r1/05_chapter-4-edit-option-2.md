# Chapter 4. Edit (option 2)

Source file: f54u200_v3r1.md
Start page: 121
Page span: 121-126

## Page 121

Chapter 4. Edit (option 2)
The Edit option (2) allows you to create, display, and change data stored in ISPF libraries, other
partitioned or single-volume or multivolume sequential data sets, or z/OS UNIX files with these
characteristics:
• Record Format (RECFM):
– Fixed or variable (non-spanned)
– Blocked or unblocked
– With or without printer control characters
• Logical Record Length (LRECL):
– From 1 to 32 760, inclusive, for fixed-length records
– From 5 to 32 756, inclusive, for variable-length records.
• VSAM data
– VSAM data can be edited if the ISPF Configuration table has been customized to enable VSAM
support (that is, VSAM_EDIT_ENABLED is set to "YES").
Note: When VSAM support is enabled, the default value for VSAM_EDIT_COMMAND is "FMNINV
DSE /". If the command is not available, IKJ56500I COMMAND FMNINV NOT FOUND, is issued as a
TSO message.
• z/OS UNIX files.
Editing a data set
When you select the Edit option, the Edit Entry Panel shown in Figure 57 on page 84 is displayed.
Edit (option 2)
© Copyright IBM Corp. 1980, 2024 83

## Page 122

Figure 57. Edit Entry panel (ISREDM01)
Edit Entry Panel action bar
The Edit Entry Panel action bar choices function as follows:
Menu
See the details about the Menu Action Bar Choice in the ISPF User Interface topic in the z/OS ISPF
User's Guide Vol I for information about the Menu pull-down.
RefList
See the Using Personal Data Set Lists and Library Lists topic in the z/OS ISPF User's Guide Vol I for
information about referral lists.
RefMode
See the details about Personal List Modes in the Using Personal Data Set Lists and Library Lists topic
in the z/OS ISPF User's Guide Vol I for information about referral list modes.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides general information about the Edit environment as well as information
about the main options and edit commands.
Edit Entry Panel fields
You can specify a concatenated sequence of up to four ISPF libraries, but the libraries must have been
previously allocated to ISPF with the Data Set utility (3.2).
The fields on this panel are:
Edit (option 2)
84  z/OS: z/OS ISPF User's Guide Vol II

## Page 123

Project
The common identifier for all ISPF libraries belonging to the same programming project.
Group
The identifier for the particular set of ISPF libraries; that is, the level of the libraries within the library
hierarchy.
You can specify a concatenated sequence of up to four existing ISPF libraries.
The editor searches the ISPF libraries in the designated order to find the member and copies it into
working storage. If the editor does not find the member in the library, it creates a new member with
the specified name.
When you save the edited member, the editor places or replaces it in the first ISPF library in the
concatenation sequence, regardless of which library it was copied from.
Type
The identifier for the type of information in the ISPF library.
Member
The name of an ISPF library or other partitioned data set member. Leaving this field blank or
entering a pattern causes PDF to display a member list. See z/OS ISPF User's Guide Vol I if you
need information about entering patterns.
Name
Any fully qualified data set name or z/OS UNIX file path name.
For more details about the Name field, see the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I.
Volume Serial
A real DASD volume or a virtual volume residing on an IBM 3850 Mass Storage System. To access
3850 virtual volumes, you must also have MOUNT authority, which is acquired through the TSO
ACCOUNT command.
PDSE Generation
Enter an absolute (positive) generation number or a relative (negative) generation number in this field
to edit a non-current generation of the member. This is valid only when the member is in a PDSE
Version 2 data set that is configured for member generations.
Initial Macro
You can specify a macro to be processed before you begin editing your sequential data set or
any member of a partitioned data set. This initial macro allows you to set up a particular editing
environment for the Edit session you are beginning. This initial macro overrides any IMACRO value in
your profile.
If you leave the Initial Macro field blank and your edit profile includes an initial macro specification,
the initial macro from your edit profile is processed.
If you want to suppress an initial macro in your edit profile, type NONE in the Initial Macro field. See
the topics about Initial Macros and the IMACRO primary command in the z/OS ISPF Edit and Edit
Macros for more details.
Profile Name
The name of an edit profile, which you can use to override the default edit profile. See the topics about
Edit Profiles and the edit environment in the z/OS ISPF Edit and Edit Macros.
Format Name
The name of a format definition or blank if no format is to be used.
Data Set Password
The password for OS password-protected data sets. This is not your RACF® password.
Record Length
Applicable when editing a z/OS UNIX file. ISPF normally treats z/OS UNIX files as having variable
length records. This field allows you to specify a record length which is used by the editor to load the
records from the file into the edit session as fixed-length records. When the file is saved, it is saved
Edit (option 2)
Chapter 4. Edit (option 2)  85

## Page 124

with fixed-length records. The Record Length field allows you to convert a variable-length file to fixed
length. The value specified in this field must be able to accommodate the largest record in the file. If
the editor finds a record that is larger than the length specified, an error message is displayed and the
edit session does not proceed.
Line Command Table
Use this field to define a set of user line commands that you can use during the edit session. The table
you specify can be generated using the ISPF table editor and contains the line commands that you
wish to have available and associates each line command with an edit macro that will be run if the line
command is entered during the edit session. For more information about EDIT line command tables,
see “Line command table support” on page 238.
Confirm Cancel/Move/Replace
When you select this field with a "/", a confirmation panel displays when you request one of these
actions, and the execution of that action would result in data changes being lost or existing data being
overwritten.
• For MOVE, the confirm panel is displayed if the data to be moved exists. Otherwise, an error
message is displayed.
• For REPLACE, the confirm panel is displayed if the data to be replaced exists. Otherwise, the
REPLACE command functions like the edit CREATE command, and no confirmation panel is
displayed.
• For CANCEL, the confirmation panel is displayed if any data changes have been made, whether
through primary commands, line commands, or typing.
Note: Any commands or data changes pending at the time the CANCEL command is issued are
ignored. Data changes are "pending" if changes have been made to the displayed edit data, but
no interaction with the host (ENTER, PF key, or command other than CANCEL) has occurred. If no
other changes have been made during the edit session up to that point, the confirmation panel is not
displayed.
Mixed Mode
When you select this field with a "/", it specifies that the editor look for shift-out and shift-in delimiters
surrounding DBCS data. If you do not select it, the editor does not look for mixed data.
Preserve VB record length
When you select this field with a "/", it specifies that the editor store the original length of each record
in variable-length data sets and when a record is saved, the original record length is used as the
minimum length for the record. The minimum length can be changed using the SAVE_LENGTH edit
macro command. The editor always includes a blank at the end of a line if the length of the record is
zero or eight.
Data Encoding
You can use this option to select whether to edit data as ASCII (CCSID 819) or UTF-8 (CCSID 1208).
When you select a value for this option, the editor uses the selected CCSID in converting the data to
the CCSID for the terminal.
For ASCII or UTF-8 z/OS UNIX files, the editor breaks up data into records using the ASCII linefeed
character (X'0A') and the ASCII carriage return character (X'0D') as the record delimiter. The linefeed
and carriage return characters are removed from the data loaded into the editor, but written back to
the file when the data is saved.
It is not necessary to use the Data Encoding option when the z/OS UNIX file is tagged with a CCSID
of 819 or 1208. If ISPF detects the file is tagged with CCSID 819 or 1208, it converts the data from
ASCII or UTF-8 to the CCSID of the terminal. When the file is saved, ISPF ensures the file is tagged
with a CCSID of 819 or 1208.
Double-byte character set support
The ISPF editor supports DBCS alphabets in two ways:
Edit (option 2)
86  z/OS: z/OS ISPF User's Guide Vol II

## Page 125

• Formatted data where DBCS characters are in the column positions specified in the format definition
created with the Format Utility (option 3.11)
• Mixed characters delimited with the special shift-out and shift-in characters.
If you are using mixed mode and the record length of a data set is greater than 72 bytes, there is
a possibility that a DBCS character might encroach on the display boundary. Here, PDF attempts to
display the other characters by replacing an unpaired DBCS character byte with an SO or SI character.
If there is a possibility that the replaced SO or SI character was erased, the line number of the line is
highlighted. If you change the position of the SO and SI characters on the panel, or if you delete the SO
and SI characters entirely, the DBCS character on the boundary is removed to keep the rest of the data
intact.
Edit (option 2)
Chapter 4. Edit (option 2)  87

## Page 126

Edit (option 2)
88  z/OS: z/OS ISPF User's Guide Vol II
