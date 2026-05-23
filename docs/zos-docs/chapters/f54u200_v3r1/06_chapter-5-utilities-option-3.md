# Chapter 5. Utilities (option 3)

Source file: f54u200_v3r1.md
Start page: 127
Page span: 127-346

## Page 127

Chapter 5. Utilities (option 3)
The Utilities option (3) provides a variety of functions for library, data set, and catalog maintenance, each
of which is described in this topic. The Utility Selection Panel is shown in Figure 58 on page 89.
Figure 58. Utility Selection Panel (ISRUTIL)
Utility Selection Panel action bar
The Utility Selection Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides information about each available choice on the Utilities Menu.
Library utility (option 3.1)
When you select this option, a panel is displayed (Figure 59 on page 90) that allows you to specify a data
set and an action to be performed. The Library utility is intended primarily for maintenance of partitioned
data sets. However, the print index listing (X), print entire data set (L), data set information (I), and short
data set information (S) functions also apply to sequential data sets.
Library utility (option 3.1)
© Copyright IBM Corp. 1980, 2024 89

## Page 128

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                Library Utility
                                                                    More:     +
 blank Display member list      I Data set information          B Browse member
     C Compress data set        S Short data set information    D Delete member
     X Print index listing      E Edit member                   R Rename member
     L Print entire data set    V View member                   P Print member
                                    Enter "/" to select option
 ISPF Library:                      /  Confirm Member Delete
    Project . . . MYPROJ               Enhanced Member List
    Group . . . . DEV      . . .          . . .          . . .         
    Type  . . . . SOURCE  
    Member  . . .             (If B, D, E, P, R, V, or blank selected)
    New name  . .             (If R selected)
 Other Partitioned or Sequential Data Set:
    Data Set Name . . .                                                        
    Volume Serial . . .           (If not cataloged)
 Option ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 59. Library Utility panel (ISRUDA1)
Library Utility panel action bar
The Library Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the details about Using Personal Data Set Lists and Library
Lists in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides information on the options available for processing libraries and
members, including compressing and printing partitioned data sets, displaying data set information
and member lists, and printing, renaming, deleting, browsing, editing, and viewing members.
Library Utility panel fields
All the fields on the Library Utility panel, with the exception of the "New name" field, are discussed in the
Libraries and Data Sets topic in the z/OS ISPF User's Guide Vol I. The "New name" field is required when
option R (rename member) is chosen; the field must contain the new member name. See “R — rename
member” on page 100 for more information about this option.
Library utility options for data sets
The topics listed here describe the options shown on the left side of the Library Utility panel shown in
Figure 59 on page 90. These options are used to work with data sets.
• “Blank — (display member list)” on page 91
• “C — compress data set” on page 91
• “X — print index listing” on page 92
• “L — print entire data set” on page 92
• “I — data set information” on page 92
• “S — short data set information” on page 96
Library utility (option 3.1)
90  z/OS: z/OS ISPF User's Guide Vol II

## Page 129

Blank — (display member list)
If you leave the Option field blank, you must specify a partitioned data set. ISPF displays a member list
when you press Enter. For more information, see the details about Using Member Selection Lists and
Library and Data Set List Utility Line Commands in the Libraries and Data Sets topic in the z/OS ISPF User's
Guide Vol I.
Note:
1. The column headers on a member list display (with the exception of Rename) are point-and-shoot sort
fields.
2. If you enter a slash in the 1-character or 9-character command field, the Member List Commands
pop-up window shown in Figure 60 on page 91 is displayed so that you can select the command you
want to use.
3. The 1-character or 9-character line command field is a point-and-shoot field. If you select the line
command field beside a member name, the Member List Commands pop-up window shown in Figure
60 on page 91 is displayed so that you can select the command you want to use. In addition, you can
enter commands (for example, TSO) directly in the 9-character field.
4. You can chain the P, R, D, V, E, and B commands; that is, you can select multiple members from a
member list for various processing tasks. Use the CANCEL command (from a View, Browse, or Edit
session) to break the chain and return to the member list. 
Figure 60. Member list commands pop-up window (ISRCMLEP)
C — compress data set
If you select option C, you can specify any partitioned data set. The compress function is not valid for a
PDSE. The compress is accomplished by calling either of these:
• The IEBCOPY utility
• An optional compress request exit routine, which can be specified by your installation.
Using this option can change an existing data set allocation to exclusive.
Library utility (option 3.1)
Chapter 5. Utilities (option 3)  91

## Page 130

ISPF allocates the IEBCOPY SYSUT3 and SYSUT4 data sets as one primary cylinder, one secondary
cylinder. If this is not sufficient for your compress request, these DDNAMES can be preallocated.
X — print index listing
If you select option X, you must specify either a DASD-resident sequential or partitioned data set. The
index listing is recorded in the ISPF list data set. For a partitioned data set, the index listing includes
general information about the data set followed by a member list. For a sequential data set, the index
listing includes general information only. See the topic about Listing Formats in z/OS ISPF User's Guide Vol
I for examples of the index listing format for source libraries and load libraries.
Note:
1. A volume serial is not allowed for multivolume data sets using option X.
2. If ISPF was entered in TEST mode, the listing also includes TTR data for each member of the data set.
This data is the track and record address, where the members reside on the volume.
L — print entire data set
If you select option L, you must specify either a DASD-resident sequential or partitioned data set. The
allowable data set characteristics are the same as for Browse, except that data sets with a logical record
length greater than 300 characters are not printed. Also, the data should not contain any printer control
characters. Use the Hardcopy utility (option 3.6) to print data sets that contain printer control characters.
A source listing of the complete data set (including all members of a partitioned data set), preceded by an
index listing, is recorded in the ISPF list data set.
Note:
1. A volume serial is not allowed for multivolume data sets using option L.
2. The page-numbering format of the ISPF list data set is PAGE: XX of YY. The YY value is calculated using
the data set member's current size statistic. When the member's current size is larger than the actual
member size, the result is PAGE: XX of YY, where YY is a page number greater than the last value of
XX. When the size statistic is smaller than the actual member size, the result is PAGE: XX of YY, until
the actual size number XX exceeds YY. Then the result is PAGE: XX, until the end of the member is
processed.
I — data set information
If you select option I, the location, characteristics, and current space utilization of the specified data set
are displayed. The format ISPF uses to display data set information when DFSMSdfp is not installed or is
not available, or when the Storage Management Subsystem is not active, is shown in “U — uncatalog data
set” on page 112. See “Information for managed data sets” on page 95 to see how ISPF displays data
set information when these products are installed, available, and active.
For sequential data sets, options I and S display the same information. For multivolume data sets, options
I and S display current allocation and utilization values that represent totals from all volumes used. You
may not enter a volume serial when you are requesting information on a multivolume data set.
Note:
1. The space for data sets allocated in blocks is calculated as if all of the tracks, including the last one,
contain only full blocks of data. Any partial "short" blocks are ignored.
2. The information shown for current space utilization is the actual data that the data set contains, based
on the number of allocation units (blocks, tracks, bytes, megabytes, and so on) that have been written.
For a data set allocated in units other than tracks and cylinders, it does not include the unused portion
of a track that is only partially filled.
For example, if a data set allocated in bytes with block size of 600 has one block written to a device
with a track size of 1000, 600 bytes of data are written and the remaining 400 bytes cannot be used
by a different data set. A track is the smallest possible unit of physical allocation to a data set on
DASD. ISPF reports 600 bytes used while other products (such as ISMF) report 1000 bytes used. ISPF
Library utility (option 3.1)
92  z/OS: z/OS ISPF User's Guide Vol II

## Page 131

reports the space occupied by data in the data set. ISMF reports the space used by this data set that is
not available for use by another data set. The difference is a relative indication of the effectiveness of
the block size used when the data set was created.
                              Data Set Information
 Data Set Name . . . . : ISPF.TEMP.PANELS
General Data                          Current Allocation                      
 Volume serial . . . : CPDLB0          Allocated blocks  . : 308              
 Device type . . . . : 3390            Allocated extents . : 1                
 Organization  . . . : PO              Maximum dir. blocks : 325              
 Record format . . . : FB                                                     
 Record length . . . : 80                                                     
 Block size  . . . . : 27920          Current Utilization                     
 1st extent blocks . : 308             Used blocks . . . . : 17               
 Secondary blocks  . : 69              Used extents  . . . : 1                
 Data set encryption : NO              Used dir. blocks  . : 1                
                                       Number of members . : 7                
                                                                              
                                      Dates                                   
                                       Creation date . . . : 2018/09/19       
                                       Referenced date . . : 2018/09/19       
                                       Expiration date . . : ***None***
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 61. Data Set Information panel (ISRUAIPO)
If the volume serial is followed by a plus (for example, HSM016+), the data set spans multiple volumes.
Press Enter to display a list of all allocated volumes that have been used, as shown in Figure 62 on page
94.
Library utility (option 3.1)
Chapter 5. Utilities (option 3)  93

## Page 132

Figure 62. Volume Information for a Multivolume Data Set (ISRUAMVI)
The "Allocated units" and "Used units" fields can vary, depending on the value that was specified in the
"Space units" field when you allocated the data set. For example, Figure 62 on page 94 shows what the
Data Set Information panel would look like if the data set was allocated by specifying Cylinders in the
"Space units" field.
If directory block information is not available, the Data Set Information panel shows a value of 0 * for the
"Maximum dir. blocks", "Used dir. blocks", and "Number of members" fields. The asterisk beside the zero
refers you to a note on the panel, which states that the directory is unavailable.
If the data set is a PDS, ISPF must open it to retrieve the directory information. This updates the
referenced date for the next time option I is displayed.
If the data set is a PDSE, the "Data set name type" field is LIBRARY and the "Maximum dir. blocks" field is
NOLIMIT. Because the used blocks, used extents, and used directory blocks are not applicable to a PDSE,
the Data Set Information panel replaces these values with "Used pages" and "% Utilized" (Figure 63 on
page 95). Other values that can appear in the "Data set name type" field are:
• HFS - MVS Hierarchical File System data set
• EXTENDED - DFSMSdfp Striped data set
• LARGE - Large format sequential data set
Note: When a PDSE data set is created, it sets aside five pages. This may cause a significant change to the
"% Utilized" value for a small data set.
Library utility (option 3.1)
94  z/OS: z/OS ISPF User's Guide Vol II

## Page 133

Data Set Information                              
                         
                                                       
 Data Set Name  . . . : ISPF.TEMP.EXEC                                            
                                                                                
 General Data                          Current Allocation                       
  Management class . . : **None**       Allocated cylinders : 35                
  Storage class  . . . : **None**       Allocated extents . : 3                 
   Volume serial . . . : CPDLB1         Maximum dir. blocks : NOLIMIT           
   Device type . . . . : 3390                                                   
  Data class . . . . . : **None**                                               
   Organization  . . . : PO            Current Utilization                      
   Record format . . . : FB             Used pages  . . . . : 6,254             
   Record length . . . : 80             % Utilized  . . . . : 99                
   Block size  . . . . : 27920          Number of members . : 79                
   1st extent cylinders: 15                                                     
   Secondary cylinders : 10                                                     
   Data set name type  : LIBRARY       Dates                                    
   Data set encryption : NO             Creation date . . . : 2018/03/08        
   Data set version  . : 1              Referenced date . . : 2018/09/19        
                                        Expiration date . . : ***None***        
                                                                                
                                                                                
                                                                                
                                                                                
 Command ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap      
 F12=Cancel                                                                
Figure 63. Data Set Information for PDSE Data Sets (ISRUAILE)
Information for managed data sets
The Library Utility option I (Figure 64 on page 95) displays information for data sets that reside on
Storage Management Subsystem Volumes (also called managed data sets) when:
• DFSMSdfp is installed and available.
• Storage Management Subsystem is active.
• Directory block information is available.
                             Data Set Information                              
                                                                    
 Data Set Name . . . . : MYDATA2.MULTI                                        
                                                                                
 General Data                           Current Allocation                      
  Management class . . : STANDARD        Allocated blocks  . : 14               
  Storage class  . . . : BASE            Allocated extents . : 1                
   Volume serial . . . : HSM016 +                                               
   Device type . . . . : 3390                                                   
  Data class . . . . . : **None**                                               
   Organization  . . . : PS             Current Utilization                     
   Record format . . . : FB              Used blocks . . . . : 0                
   Record length . . . : 80              Used extents  . . . : 0                
   Block size  . . . . : 3200                                                   
   1st extent blocks . : 14                                                     
   Secondary blocks  . : 10             Dates                                   
   Data set name type  :                 Creation date . . . : 2018/07/09       
   Data set encryption : NO              Referenced date . . : 2018/09/03       
                                         Expiration date . . : ***None***       
                                                                                
   SMS Compressible  . : NO                                                     
 To display multiple volumes press Enter or enter Cancel to Exit.
 Command ===> _________________________________________________________________
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap      
 F12=Cancel                                 
Figure 64. Data Set Information for Managed Data Sets (ISRUAIES)
Note: A "+" may be displayed beside the Volume serial field if the data set is a multiple volume data set.
This is determined from the number of volume entries in the catalog. Depending on the system set-up, a
"+" may not be displayed until the additional volumes have been accessed. For example, a data set with a
non-zero dynamic volume count in the SMS dataclass will not show multiple volume entries in the catalog
until the additional volumes have been accessed. Other vendor products which can dynamically expand
the volume list will also not show multiple volume entries in the catalog until the additional volumes have
been accessed.
Library utility (option 3.1)
Chapter 5. Utilities (option 3)  95

## Page 134

Press Enter to display a list of all allocated volumes as shown in Figure 62 on page 94.
The major difference between this information and the information that is displayed for data sets on
non-managed volumes is the addition of these classes:
• Management class
• Storage class
• Data class
S — short data set information
If you select option S, information about the selected data set is displayed. The information displayed
by option S is the same as that displayed by option S of the Data Set utility (option 3.2), but it differs
from option I in two respects. Information for partitioned data sets, when displayed by option S, lacks the
number of maximum and used directory blocks, and the number of members. For sequential data sets,
options I and S display the same information. You can not enter a volume serial when you are requesting
information on multivolume data sets.
Note:
1. The space for data sets allocated in blocks is calculated as if all of the tracks, including the last one,
contain only full blocks of data. Any partial "short" blocks are ignored.
2. The information shown for current space utilization is the actual data that the data set contains, based
on the number of allocation units (blocks, tracks, bytes, megabytes, and so on) that have been written.
For a data set allocated in units other than tracks and cylinders, it does not include the unused portion
of a track that is only partially filled.
For example, if a data set allocated in bytes with block size of 600 has one block written to a device
with a track size of 1000, 600 bytes of data are written and the remaining 400 bytes cannot be used
by a different data set. A track is the smallest possible unit of physical allocation to a data set on
DASD. ISPF reports 600 bytes used while other products (such as ISMF) report 1000 bytes used. ISPF
reports the space occupied by data in the data set. ISMF reports the space used by this data set that is
not available for use by another data set. The difference is a relative indication of the effectiveness of
the block size used when the data set was created.
Figure 65 on page 97 shows a short format example of data set information for a partitioned data set.
This is the short format ISPF uses to display data set information when DFSMSdfp is not installed or not
available, or when the Storage Management Subsystem is not active. See “Short information for managed
data sets” on page 97 to see how ISPF displays data set information when these products are installed,
available, and active.
Library utility (option 3.1)
96  z/OS: z/OS ISPF User's Guide Vol II

## Page 135

Data Set Information                              
                                                                                
 Data Set Name . . . . : ISPF.TEMP.PANELS                                    
                                                                                
 General Data                           Current Allocation                      
  Management class . . : **None**        Allocated blocks  . : 308              
  Storage class  . . . : **None**        Allocated extents . : 1                
   Volume serial . . . : CPDLB0                                                 
   Device type . . . . : 3390                                                   
  Data class . . . . . : **None**                                               
   Organization  . . . : PO             Current Utilization                     
   Record format . . . : FB              Used blocks . . . . : 17               
   Record length . . . : 80              Used extents  . . . : 1                
   Block size  . . . . : 27920                                                  
   1st extent blocks . : 308                                                    
   Secondary blocks  . : 69             Dates                                   
   Data set name type  : PDS             Creation date . . . : 2018/09/19       
   Data set encryption : NO              Referenced date . . : 2018/09/20       
                                         Expiration date . . : ***None***       
                                                                                
                                                                                
                                                                                
                                                                                
 Command ===>                                                                   
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap      
 F12=Cancel                                        
Figure 65. Short data set information (ISRUAIES)
The "Allocated units" and "Used units" fields can vary, depending on the value that was specified in the
"Space units" field when you allocated the data set. For example, Figure 65 on page 97 shows what the
short format of the Data Set Information panel would look like if the data set was allocated by specifying
CYLS in the "Space units" field.
Short information for managed data sets
The Library Utility option S displays information (Figure 66 on page 97) for data sets that reside on
Storage Management Subsystem volumes (also called managed data sets) when:
• DFSMSdfp is installed and available
• Storage Management Subsystem is active.
                       Data Set Information                              
                                                                    
 Data Set Name . . . . : MYDATA2.MULTI                                        
                                                                                
 General Data                           Current Allocation                      
  Management class . . : STANDARD        Allocated blocks  . : 14               
  Storage class  . . . : BASE            Allocated extents . : 1                
   Volume serial . . . : HSM016 +                                               
   Device type . . . . : 3390                                                   
  Data class . . . . . : **None**                                               
   Organization  . . . : PS             Current Utilization                     
   Record format . . . : FB              Used blocks . . . . : 0                
   Record length . . . : 80              Used extents  . . . : 0                
   Block size  . . . . : 3200                                                   
   1st extent blocks . : 14                                                     
   Secondary blocks  . : 10             Dates                                   
   Data set name type  :                 Creation date . . . : 2018/07/09       
   Data set encryption : NO              Referenced date . . : 2018/09/03       
                                         Expiration date . . : ***None***       
                                                                                
   SMS Compressible  . : NO                                                     
 Command ===> _________________________________________________________________
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap      
 F12=Cancel                                                          
Figure 66. Data Set Information (Short) for Managed Data Sets (ISRUAIES)
The major difference between this information and the information that is displayed for data sets on
non-managed volumes is the addition of these classes:
• Management class
Library utility (option 3.1)
Chapter 5. Utilities (option 3)  97

## Page 136

• Storage class
• Data class
If the data set is a PDSE, the "Data set name type" field is LIBRARY. Because the used blocks and used
extents are not applicable to a PDSE, the Data Set Information panel replaces these values with "Used
pages" and "% Utilized" (Figure 67 on page 98). Other values that can appear in the "Data set name
type" field are:
• HFS - MVS Hierarchical File System data set
• EXTENDED - DFSMSdfp Striped data set. When the Data Set Name Type is EXTENDED, the SMS
Compressible field indicates if the data set is compressible or not (YES or NO).
• LARGE - Large format sequential data set
                       Data Set Information                              
                                                                    
 Data Set Name . . . . : MYDATA.EXTENTS.TEST.PDSE                                      
                                                                                
 General Data                              Current Allocation                   
  Management class . . : STANDARD           Allocated cylinders : 13            
  Storage class  . . . : BASE               Allocated extents . : 1             
   Volume serial . . . : HSM019                                                 
   Device type . . . . : 3390                                                   
  Data class . . . . . : **None**          Current Utilization                  
   Organization  . . . : PO                 Used pages  . . . . : 6             
   Record format . . . : FB                 % Utilized  . . . . : 5             
   Record length . . . : 80                                                     
   Block size  . . . . : 27920                                                  
   1st extent cylinders: 13                                                     
   Secondary cylinders : 10                Dates                                
   Data set name type  : LIBRARY            Creation date . . . : 2018/10/25    
   Data set encryption : NO                 Referenced date . . : 2018/10/26    
   Data set version  . : 1                  Expiration date . . : ***None***    
                                                                                
                                                                                
 Command ===> _________________________________________________________________
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap      
 F12=Cancel                                        
Figure 67. Data Set Information (Short) for a PDSE (ISRUAISE)
Library utility options for members and library utility member list line
commands
The topics listed here describe the options on the Library Utility panel (shown in Figure 59 on page 90)
that you can use to work with members and the line commands that you can use on a member list that
you display from the Library Utility panel.
• “B — browse member” on page 99
• “C — copy member” on page 99
• “D — delete member” on page 99
• “E — edit member” on page 99
• “G — reset member statistics” on page 100
• “I — display member information” on page 100
• “J — submit member” on page 100
• “M — move member” on page 100
• “N — display generations” on page 100
• “P — print member” on page 100
• “R — rename member” on page 100
• “T — invoke TSO command for member” on page 101
• “V — view member” on page 101
Library utility (option 3.1)
98  z/OS: z/OS ISPF User's Guide Vol II

## Page 137

Note:
1. You can chain these commands; that is, you can select multiple members from a member list for
various processing tasks. Use the CANCEL command (from a View, Browse, or Edit session) to break
the chain and return to the member list.
2. With an enhanced member list, you can enter other commands. See “M — display member list” on
page 150.
B — browse member
You can specify B as an option on the Library Utility panel or as a line command on a member list that you
display from the Library Utility panel.
The specified member is displayed in Browse mode. You can use all the Browse commands.
If you select B as an option on the Library Utility panel, you must also specify a partitioned data set and a
member name on the Library Utility panel. When you exit Browse, the Library Utility panel reappears.
C — copy member
You can specify C as a line command on a member list that you display from the Library Utility panel.
If you enter line command C, the Copy Entry panel appears where you must specify a partitioned data set
and member name for the new member. You can also specify other options for the copy on this panel.
D — delete member
You can specify D as an option on the Library Utility panel or as a line command on a member list that you
display from the Library Utility panel.
You are prevented from deleting a PDS member that any user is currently editing.
If you select D as an option on the Library Utility panel:
• You must also specify a partitioned data set and a member name or pattern on the Library Utility panel.
• If you select Confirm Member Delete on the Library Utility panel, you are asked to confirm your intention
to delete this member. Note that Confirm Member Delete is forced on when you delete members using a
pattern.
• When the deleted member is a primary member, the primary member and all associated aliases are
deleted. When the deleted member is an alias, only the alias and its directory entry are deleted.
• When a member pattern is specified:
– Every primary member whose name matches the member pattern is deleted.
– Every alias that is associated with a primary member whose name matches the member pattern is
deleted, even if the alias name itself does not match the member pattern.
– Every alias whose name matches the member pattern is deleted, even if the alias is associated with a
primary member whose name does not match the member pattern.
If you enter line command D on a member list that you display from the Library Utility panel:
• If you have selected 1. Set Delete Confirmation On from the Confirm pull-down on the Library Utility -
Member List panel (ISRUDMM), then you are asked to confirm your intention to delete this member.
• When the deleted member is a primary member, the primary member and all associated alias names
are deleted. When the deleted member is an alias, only the alias and its directory entry are deleted.
E — edit member
You can specify E as an option on the Library Utility panel or as a line command on a member list that you
display from the Library Utility panel.
The specified member is displayed in Edit mode. You can use all EDIT commands.
Library utility (option 3.1)
Chapter 5. Utilities (option 3)  99

## Page 138

If you select E as an option on the Library Utility panel, you must also specify a partitioned data set and
member name on the Library Utility panel. When you exit Edit, the Library Utility panel reappears.
G — reset member statistics
You can specify G as a line command on a member list that you display from the Library Utility panel.
If you enter line command G, the Reset Member Statistics panel is displayed where you can enter the
action to be performed and any additional options for the reset action.
I — display member information
You can specify I as a line command on a member list that you display from the Library Utility panel.
If you enter line command I, the Member Information panel is displayed showing information about the
member.
J — submit member
You can specify J as a line command on a member list that you display from the Library Utility panel.
If you enter line command J, the member is submitted as JCL for batch processing.
M — move member
You can specify M as a line command on a member list that you display from the Library Utility panel.
If you enter line command M, the Move Entry panel is displayed where you must enter the destination
data set and member name. You can also specify other options for the move on this panel.
N — display generations
For members of PDSEs that support generations, you can display a generation list using N as a line
command. See z/OS ISPF User's Guide Vol I for more information about generation lists.
P — print member
You can specify P as an option on the Library Utility panel or as a line command on a member list that you
display from the Library Utility panel.
A source listing of the member is recorded in the ISPF list data set.
If you select P as an option on the Library Utility panel, you must also specify a partitioned data set and a
member name on the Library Utility panel.
Note: If any members are to be printed, the data set characteristics must conform to those for the L
option.
R — rename member
You can specify R as an option on the Library Utility panel or as a line command on a member list that you
display from the Library Utility panel.
You are prevented from renaming a member that is currently being edited by you or another user.
If you select R as an option on the Library Utility panel, you must also specify a partitioned data set and
member name on the Library Utility panel. You must also specify a new member name in the "New name"
field.
If you enter line command R on a member list that you display from the Library Utility panel, you can
specify the new member name in the Prompt field. If the new member name is not entered in the Prompt
field, the Member Rename panel is displayed where you must enter the new member name.
Library utility (option 3.1)
100  z/OS: z/OS ISPF User's Guide Vol II

## Page 139

Where the data set refers to a partitioned data set load library (RECFM=U), and the member to be
renamed is the name of a primary member, the user data component of any associated alias names will
be updated to refer to the renamed primary name.
T — invoke TSO command for member
You can specify T as a line command on a member list that you display from the Library Utility panel.
When you use the T line command, enter the name of the TSO command you want to execute in the
Prompt field to the right of the member name. The fully-qualified data set name, including the member
is passed as a parameter to the TSO command. If you want to execute a member that is a REXX exec
or CLIST, use the T line command on the line for that member, and enter EXEC in the Prompt field. If
you leave the Prompt field blank, the TSO Command Action panel appears, where you can enter the TSO
command to be run for the member and any additional parameters that are needed for the command.
V — view member
You can specify V as an option on the Library Utility panel or as a line command on a member list that you
display from the Library Utility panel.
The specified member is displayed in View mode. You can use all EDIT commands. For more information,
see the topic about View (option 1) in z/OS ISPF User's Guide Vol I.
If you select V as an option on the Library Utility panel, you must also specify a partitioned data set and
member name on the Library Utility panel. When you exit View, the Library Utility panel reappears.
Data set utility (option 3.2)
When you select this option, a panel is displayed (Figure 68 on page 101) that allows you to specify a data
set and an action to be performed.
  Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                               Data Set Utility
    A Allocate new data set                 C Catalog data set
    R Rename entire data set                U Uncatalog data set
    D Delete entire data set                S Short data set information
blank Data set information                  V VSAM Utilities
ISPF Library:
   Project  . .                  Enter "/" to select option
   Group  . . .                  /  Confirm Data Set Delete
   Type . . . .         
Other Partitioned, Sequential or VSAM Data Set:
   Data Set Name . . .                                                         
   Volume Serial . . .           (If not cataloged, required for option "C")
Data Set Password  . .           (If password protected)
Option ===>                                                                   
 F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
F10=Actions  F12=Cancel
Figure 68. Data Set Utility panel (ISRUDA2S)
Data Set Utility panel action bar
The Data Set Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  101

## Page 140

RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides information on:
• allocating new partitioned and sequential data sets
• processing existing data sets (renaming, deleting, cataloging, uncataloging, and displaying data set
information)
• The VSAM utilities
Data Set Utility panel fields
All the fields on the Data Set Utility panel are explained in the "ISPF Libraries and Data Sets" chapter of
the z/OS ISPF User's Guide Vol I. For option A you can specify any DASD-resident sequential or partitioned
data set. For the other options, you can specify any DASD-resident data set that is not VSAM. You can get
short information on a VSAM data set.
Data set utility options
These topics describe the options shown on the Data Set Utility panel:
• “A — allocate new data set” on page 102
• “Allocation errors” on page 109
• “C — catalog data set” on page 110
• “R — rename entire data set” on page 110
• “U — uncatalog data set” on page 112
• “D — delete entire data set” on page 112
• “S — data set information (short)” on page 114
• “Blank — (data set information)” on page 114
• “V — VSAM utilities” on page 115
A — allocate new data set
Use option A to allocate a new data set with or without the Storage Management Subsystem classes
(management class, storage class, and data class). A data set that is allocated on a volume that is
managed by the Storage Management Subsystem (SMS) is called a managed data set. A data set that is
allocated on a volume that is not managed by the SMS is called a non-managed data set.
To use option A, you must:
1. Enter one of these:
• An ISPF library name in the Project, Group, and Type fields
• Another partitioned or sequential data set name in the Data Set Name field.
See the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I for information on
how to enter the ISPF library name or the data set name.
2. If you entered an ISPF library name, the value in the Volume Serial field is ignored. However, if you
entered another data set name, you can specify the volume on which to allocate the data set in the
Volume Serial field. Do not enter a volume serial if you want to do one of these:
• Use the authorized default volume.
Data set utility (option 3.2)
102  z/OS: z/OS ISPF User's Guide Vol II

## Page 141

• Enter a generic unit address in the "Generic unit" field on the Allocate New Data Set panel.
Note that an SMS-eligible data set may be allocated on a volume different from any entered value.
For more information about Volume Serials, see the "ISPF Libraries and Data Sets" chapter of the z/OS
ISPF User's Guide Vol I.
3. If your ISPF libraries and data sets are password-protected, enter the password in the Data Set
Password field.
For more information about Data Set Passwords, see the "ISPF Libraries and Data Sets" chapter of the
z/OS ISPF User's Guide Vol I.
Note: You cannot assign a password to a managed data set. Therefore, the Data Set Password field is
ignored when you allocate a managed data set.
4. Press Enter.
The Allocate New Data Set panel is displayed. This panel enables you to specify data set allocation values.
The fields displayed on this panel depend upon the value of the ALLOWED_ALLOCATION_UNITS keyword
in the ISPF configuration table. When ALLOWED_ALLOCATION_UNITS is not 'A' the panel shown in Figure
69 on page 103 is displayed.
Figure 69. Allocate New Data Set panel (ISRUAAP2)
Otherwise, this panel is displayed: (Figure 70 on page 104).
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  103

## Page 142

Figure 70. Allocate New Data Set —managed data set support panel (ISRUAASE)
When you press Enter with this panel displayed, the new data set is allocated and cataloged. Entering the
END command returns you to the previous panel without allocating the data set.
An optional installation exit, the data set allocation exit, can control all data set creation, deletion,
allocation, and deactivation done directly by ISPF. This does not include allocations done by ISPF, the
TSO ALLOCATE command, or other TSO commands. See z/OS ISPF Planning and Customizing for more
information about the data set allocation exit.
Your installation must use DFSMSdfp to define the values that you enter in the "Management class",
"Storage class", and "Data class" fields. If you have no specific requirements, you can leave these fields
blank. However, be aware that your installation may provide default management, storage, and data
classes. These defaults would take effect if you leave any of the class fields blank and may even override
any classes that you specify.
Management class
Used to obtain data management-related information (migration, backup, and retention criteria, such
as expiration date) for the data set allocation.
If you have no specific management class requirements, you can leave this field blank. However, be
aware that your installation may provide a default management class. This default may even override
any management class that you specify.
Storage class
Used to obtain the storage-related information (volume serial) for the data set allocation. Any volume
serial that you enter in the " Volume serial" field is ignored unless the storage class that you use
includes the Guaranteed Space=Yes attribute (useful if you are allocating multivolume data sets).
Data set utility (option 3.2)
104  z/OS: z/OS ISPF User's Guide Vol II

## Page 143

Data class
Used to obtain the data-related information (space units, primary quantity, secondary quantity,
directory block, record format, record length, and data set name type) for the allocation of the data
set.
Default values are provided for the fields in Figure 69 on page 103, except for expiration date, based on
which of these occurred most recently:
• What you last entered on this panel
• The last display data set information request (options 3.1, 3.2, or 3.4).
You can type over the displayed defaults if you want to change them. Here is a list of the fields on this
panel and their definitions:
Volume serial
This field is one that you probably will not need to use very often. It is not required and is usually
ignored by the Storage Management Subsystem. Do not enter a volume serial if you want to do one of
these:
• Use the authorized default volume.
• Enter a generic unit address in the Generic unit field.
• Use the volume specified by the storage class you are using.
When a storage class is used, your installation and the SMS assume joint responsibility for
determining the volume on which the data set is allocated. The SMS enables the installation to select
the volumes that are eligible to contain the data set. It then chooses one of those volumes and
allocates the data set. The SMS's volume choice is based on:
• storage requirements
• The amount of space a volume has available.
Note: ISPF does not support allocation of tape data sets.
Multiple Volumes
Allows you to allocate sequential data sets that span multiple volumes. ISPF supports a maximum of
59 volumes. Place a slash in this field and press Enter to display a panel similar to the one shown in
Figure 71 on page 105. 
   Menu  RefList  Utilities  Help
 ─ ┌──────────── Multivolume Allocation ─────────────┐ ────────────────────────
   │ ISRUAMV                                         │
   │                                                 │              More:   - +
   │  Enter the number of volumes to allocate or     │  device address) **
 D │  the names of one or more volumes and           │ lt data class)
   │  press Enter to allocate or enter Cancel        │ S, KB, MB, BYTES
   │  command to exit.  If a number is entered,      │
   │  any volume names will be ignored.              │
   │                                   More:     +   │
   │  Number of volumes to allocate:                 │
   │                                                 │ tial data set) *
   │  Volume names:                                  │
   │   1. MVS8WF  2.         3.         4.           │
   │ Command ===>                                    │
   │  F1=Help         F2=Split        F3=Exit        │ DS, or blank)  *
   │  F7=Backward     F8=Forward      F9=Swap        │ MM/DD
   ⋘─────────────────────────────────────────────────┘ D in Julian form
 Enter "/" to select option             DDDD for retention period in days
 /  Allocate Multiple Volumes           or blank)
 Command ===>
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 71. Multivolume allocation panel (ISRUAMV)
This panel allows you to specify up to 59 volumes.
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  105

## Page 144

Note:
• Although the volume input fields are numbered consecutively, you may enter volume names in any
of the fields.
• The volume that you enter in the "Volume serial" field on the Allocate New Data Set panel will be
placed in the first field of the Multivolume Allocation panel.
• If you enter only one volume, standard data set allocation is invoked.
• If you enter a number in the "Number of volumes to allocate" field, any volume names left in the
name fields are ignored, and might or might not be the volumes the data set is allocated to.
• When displaying information about a multivolume data set, depending on your system setup, all
volumes might not be shown until they have been accessed.
Generic unit
The generic unit address for the direct access volume that is to contain the data set, such as 3380 or
3390. This field overrides the Volume Serial field on the Data Set Utility panel. Therefore, you should
leave this field blank if you want to do one of these:
• Use the authorized default volume
• Enter a volume serial in the Volume serial field.
Note:
1. Leave both the Volume serial and Generic unit fields blank to allow ISPF to select an eligible
volume. Eligibility is determined by the unit information in your user entry in the TSO User Attribute
Data Set (UADS) or the TSO segment of RACF.
2. At some installations, you are limited to eligible volumes even when an explicit volume serial is
specified. At other installations you can specify any mounted volume. This is an installation option.
3. To allocate a data set to a 3850 virtual volume, you must also have MOUNT authority, gained by
using the TSO ACCOUNT command or by using the RACF PERMIT command for the TSO AUTH
general resource class.
4. If you are allocating an SMS data set, you can enter either an installation defined group name or a
generic device type in the Generic unit field, but not a specific device number.
Space units
Any of these:
Track
Shows that the amounts entered in the primary and secondary quantity fields are expressed in
tracks.
Cylinder
Shows that the amounts entered in the primary and secondary quantity fields are expressed in
cylinders.
Block
Shows that the amounts entered in the primary and secondary quantity fields are expressed in
blocks.
Megabyte
Shows that the amounts entered in the primary and secondary quantity fields are expressed in
megabytes.
Kilobyte
Shows that the amounts entered in the primary and secondary quantity fields are expressed in
kilobytes.
Byte
Shows that the amounts entered in the primary and secondary quantity fields are expressed in
bytes.
Data set utility (option 3.2)
106  z/OS: z/OS ISPF User's Guide Vol II

## Page 145

Records
Shows that the amounts entered in the primary and secondary quantity fields are the average
number of records of the size specified by the block size field.
Note: "Space units" allows the shortest unique abbreviation for each attribute; for example, T for
TRKS, C for CYLS, K for KB, and M for MB, BY for BYTE, R for RECORDS, and BL for BLKS.
Average record unit
Shows the unit used when allocating average record length. U specifies single-record units (bytes). K
specifies thousand-record units (kilobytes). M specifies million-record units (megabytes). The default
value is U.
Primary quantity
The primary allocation quantity in tracks, cylinders, blocks, megabytes, kilobytes, bytes, or records,
as shown in the "Space units" field. This number can be zero for sequential data sets, but must
be greater than zero for PDSs. Also, if the primary quantity is zero, the secondary quantity must be
greater than zero.
Secondary quantity
The secondary allocation quantity in tracks, cylinders, blocks, megabytes, kilobytes, bytes, or records,
as shown in the "Space units" field. This quantity is allocated when the primary quantity is insufficient.
Directory blocks
Enter one of these:
• For partitioned data sets, you must specify the number of directory blocks. Each 256-byte block
accommodates these number of directory entries:
– Data sets with ISPF statistics: 6
– Data sets without ISPF statistics: 21
– Load module data sets: 4-7, depending on attributes
• ISPF requests a data set organization (DSORG) of PS when the value is zero or PO if the value is
greater than zero. Note that ISPF converts a blank value to zero.
Record format
Any valid combination of these codes:
F
Fixed-length records.
V
Variable-length records.
U
Undefined format records.
B
Blocked records.
A
ASA printer control characters.
M
Machine code printer control characters.
S
Standard (for F) or spanned (for V); use only with sequential data sets.
T
Track-overflow feature.
Note:
1. You must enter either F, V, or U.
2. You can specify S and T, but ISPF does not otherwise support them.
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  107

## Page 146

Record length
The logical record length, in bytes, of the records to be stored in the data set.
Block size
The block size, also called physical record length, of the blocks to be stored in the data set. Use
this field to specify how many bytes of data to put into each block, based on the record length. For
example, if the record length is 80 and the block size is 3120, 39 records can be placed in each block.
Note: The record length and block size are verified to be consistent with the record format. If you
need to use non-standard characteristics, use the TSO ALLOCATE command.
Data set name type
The type of data set to be allocated:
LIBRARY
Allocates a partitioned data set extended.
PDS
Allocates a partitioned data set.
LARGE
Allocates a large format sequential data set.
EXTREQ
Indicates that an extended data set is required.
EXTPREF
Indicates that an extended data set is preferred.
BASIC
Indicates that neither an extended nor a large format sequential data set is to be allocated.
blank
Allocates a partitioned or sequential data set based on the data set characteristics entered.
Note: If you specify LIBRARY and a zero directory size, ISPF allocates a PDSE and overrides the zero
directory size. If you specify blanks for the directory size, a sequential data set is allocated instead of
a PDSE.
Data set version
The version number when the Data set name type is LIBRARY. Valid values are:
1
Library version 1
2
Library version 2
blank
ISPF does not specify the library version and this is determined by system defaults.
Num of generations
This field is used only when when the Data set name type is LIBRARY and the Data set version is 2.
Specifies the maximum number of generations that are kept for members in the data set. Valid values
are from 0 to the system-defined maximum (MAXGENS_LIMIT in PARMLIB member IGDSMSxx). A
value of 0 indicates that no generations are kept.
Extended Attributes
Valid values are:
NO
Data set cannot have extended attributes or reside in EAS. This is the default for non-VSAM data
sets.
OPT
Data set can have extended attributes and reside in EAS. This is the default for VSAM data sets.
blank
Use default based on data type.
Data set utility (option 3.2)
108  z/OS: z/OS ISPF User's Guide Vol II

## Page 147

Expiration date
Allows you to protect valuable data by specifying a date, in your national language, when the data set
may be deleted. If you try to delete an unexpired data set, ISPF displays two panels: a Confirm Delete
panel, followed by a Confirm Purge panel. See “D — delete entire data set” on page 112 for more
information about deleting unexpired data sets.
An expiration date is not required, but if you enter one it should be in one of these formats:
YYYY/MM/DD
Date shown in year, month, and day, or your equivalent national format. The maximum expiration
date allowed is 2155/12/31.
YYYY.DDD
Date shown in Julian format, such as 2006.066 for March 7, 2006. The maximum expiration date
allowed is 2155.365.
You can specify a DDD value of up to 366 if the YYYY value represents a leap year.
DDDD
The number of days, starting with the creation date, after which the data set can be deleted. DDDD
has a range of 0 to 9999.
PERM, NOLIMIT, NEVER, 9999
Specifying any of these values causes ISPF to translate it to a value of 1999.365. This is treated
by ISPF as permanent retention.
Key Label
The key label field is used to encrypt data sets. It is equivalent to the DSKEYLBL parameter on the TSO
Allocate statement and the JCL DD statement. The DSKEYLBL parameter is only applied if the data set
is eligible to be encrypted, otherwise it is ignored.
Allocation errors
ISPF attempts to recognize inconsistent attributes for partitioned and sequential data sets before
allocating them. However, when conditions outside ISPF's control result in the allocation of such a data
set, the Allocation Error panel (Figure 72 on page 110) is displayed. These conditions are caused by:
• A data class that specifies inconsistent attributes
• Attributes entered on the Allocate New Data Set panel that create inconsistency by overriding other
attributes specified by the data class.
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  109

## Page 148

Figure 72. Allocation Error Panel (ISRUADCS)
The term inconsistent attributes refers to incompatible values that have been specified for one or more of
these items: Space units; Primary or Secondary quantity; Directory blocks; Record format; Record length;
Block size.
For example, if you allocate a data set with an undefined record format (RECFM=U) and a block size
of zero (BLOCKSIZE=0), some ISPF functions (such as Move and Copy) and services (such as LMMOVE,
LMCOPY, and LMINIT) cannot use the data set.
However, when either the linkage editor or the IEBCOPY utility has been called, these functions and
services determine the best block size for the data set. Then, when the data set has a block size greater
than zero, the ISPF functions and services listed can be used.
The Allocation Error panel gives you the opportunity to delete such a data set because other ISPF
functions, such as View (option 1) and Edit (option 2), may not be able to use it.
For information about allocation errors and how they affect data set promotion when using SCLM, refer to
z/OS ISPF Software Config ur ation  and Library Manager Guide and Reference.
C — catalog data set
If you select option C, the specified data set is cataloged. For this option, you must specify the volume
serial on which the data set resides, regardless of whether the data set is specified as project, library, and
type, or as another data set name. The data set must reside on the specified volume.
The preceding instructions for cataloging data sets do not apply to data sets that reside on Storage
Management Subsystem volumes. These data sets are automatically cataloged when you allocate them.
They cannot be cataloged by using option C.
R — rename entire data set
If you select option R, a panel is displayed to allow you to enter the new data set name.
Data set utility (option 3.2)
110  z/OS: z/OS ISPF User's Guide Vol II

## Page 149

Type the new data set name and press Enter to rename, or enter the END command to cancel. Either
action returns you to the previous panel.
If you specify a volume serial for a data set to be renamed, ISPF checks to see whether the data set is
cataloged on that volume. If it is, the Rename panel prompts you to specify whether to recatalog the data
set. If you specify a volume serial and the data set is not cataloged, it remains uncataloged after you
rename it. If a volume serial is not specified, the data set is recataloged to the new data set name and the
old data set name is uncataloged.
Note:
1. ISPF does not rename VSAM data sets or password-protected data sets.
2. A volume serial is not allowed for multivolume data sets using Rename.
3. Generation Data Group (GDG) data sets can only be renamed to something other than GDG names.
Attention: Trying to rename GDG data sets to a different generation or version number can
cause deletion of your GDG data set or group of GDG data sets.
4. When you rename a data set that resides on a Storage Management Subsystem volume, you cannot
specify a volume serial in the Volume Serial field. Both the cataloged entry and the VTOC entry are
renamed.
Rename processing with RACF
The normal order of processing when ISPF is asked to rename a data set is as follows:
1. The new data set name is cataloged using SVC 26
2. The data set is renamed using SVC 30
3. The old data set name is uncataloged using SVC 26
There are three occasions, however, when ISPF will deviate from this order of processing:
• If the data set is a System Managed (SMS) data set, the update of the catalog (both cataloging the new
name and uncataloging the old name) is handled by the operating system when the SVC 30 is issued. In
this case, ISPF does not issue either of the SVC 26 requests.
• If the data set is an uncataloged data set, no catalog update will be done. The data set is renamed using
the SVC 30 only.
• If the data set is cataloged, but the user specified both the data set name and volume, panel ISRUARP2
is displayed. The user has the option of specifying whether the catalog processing should be done. If the
user indicates (via a NO in the "Reply to uncatalog the data set" field) that no catalog processing should
be done, only the SVC 30 is used to rename the data set. If the reply is YES, the SVC 30 as well as both
SVC 26 requests are issued.
If an error is encountered during a rename request, an attempt is made to return the data set to its
original name, and to reset the catalog entries to their original status (remove the new name from the
catalog and leave the old name in the catalog).
This order of processing is intended to minimize the possibility that an uncataloged data set will
result if an error is encountered during the rename process. Errors may be encountered due to certain
combinations of RACF data set profiles and user access to the groups under which those data set profiles
fall. When an error occurs, the user receives a message indicating the status of the data set name, and of
the catalog entries.
See the z/OS Security Server RACF Security Administrator's Guide or equivalent documentation for your
security package, to determine the authorization levels required for each of these operations. The user
will need authorization first to catalog the new data set name, then to rename the data set, and then to
uncatalog the old data set name. This will require adequate authorization to any discrete or generic data
set profiles involved and to the catalogs involved. Be aware that a discrete data set profile is renamed
when the data set is renamed.
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  111

## Page 150

Renaming with expiration dates
If the data set has an expiration date in its catalog entry, the expiration date is not propagated forward to
the new catalog entry. In this case, a confirmation panel is displayed.
As directed by the panel, press Enter if you want to confirm the rename request. If you want the data
set to have an expiration date under its new name, use the TSO ALTER command or a similar function to
update the new catalog entry.
Renaming with aliases
The results of renaming a data set with an alias differ depending upon whether the data set is on a System
Managed Storage (SMS) volume or not. For an SMS data set, DFSMS ensures the alias is preserved and is
associated with the new data set name. For a non-SMS data set, the alias is removed.
U — uncatalog data set
If you select option U, the specified data set name is uncataloged. There is no need for the specified data
set to be allocated or for the volume on which it resides to be mounted.
If the catalog entry being removed contains an expiration date in the future, a confirmation panel is
displayed. Press Enter if you want to confirm the uncatalog request, otherwise press END to cancel the
request.
Note: Uncatalog is not allowed for multivolume data sets.
You cannot use option U to uncatalog a data set that resides on a Storage Management Subsystem
volume. However, the system uncatalogs these data sets when you delete them, which is done by using
option D of either the Data Set utility (option 3.2) or the Data Set List utility (option 3.4).
D — delete entire data set
If you select option D, a confirmation panel is displayed (Figure 73 on page 112) so you can make sure
you did not select this option by mistake.
   Menu  RefList  Utilities  Help
 ─ ┌───────────────────────────────────────────────────────────────────┐ ──────
   │                          Confirm Delete                           │
   │                                                      More:     +  │
   │ Data Set Name . : MYPROJ.DEV.SOURCE                               │
   │ Volume  . . . . : MVS8WF                                          │
   │ Creation date . : 2002/07/08                                      │
 b │                                                                   │
   │ Enter "/" to select option                                        │
 I │    Set data set delete confirmation off                           │
   │                                                                   │
   │ Instructions:                                                     │
   │   Press ENTER to confirm delete.                                  │
   │   (The data set will be deleted and uncataloged.)                 │
 O │                                                                   │
   │   Press CANCEL or EXIT to cancel delete.                          │
   │                                                                   │ "C")
   │ Command ===>                                                      │
 D │  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   │
   │  F9=Swap     F12=Cancel                                           │
   ⋘───────────────────────────────────────────────────────────────────┘
 Option ===> D
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 73. Confirm  Delete panel (ISRUADC1)
If you specify a volume serial for the data set to be deleted, ISPF checks to see whether the data set is
cataloged on that volume. If so, the Confirm Delete panel prompts you to specify whether to uncatalog
the data set. The displayed default is YES. If no volume serial is specified, and the data set does not have
an expiration date, the data set is deleted and uncataloged.
Note:
Data set utility (option 3.2)
112  z/OS: z/OS ISPF User's Guide Vol II

## Page 151

1. ISPF does not delete password-protected data sets or data sets allocated with an esoteric device type.
2. A volume serial is not allowed for multivolume data sets using Delete.
As directed on the panel, perform one of these actions:
• Press Enter to confirm the data set deletion.
• Enter the CANCEL or EXIT command to cancel. This action returns you to the previous panel.
If the data set has an expiration date that has not expired, ISPF displays a Confirm Purge panel (Figure 74
on page 113) after the Confirm Delete panel.
   Menu  RefList  Utilities  Help
 ─ ┌─────────────────────────────────────────────────────────────────────┐ ─────
   │                               Confirm Delete                        │
   │                                                                     │
   │ Data Set being deleted has an expiration date which has not expired │
   │ Data Set Name . : MYPROJ.DEV.SOURCE                                 │
   │ Volume  . . . . : MVS8WF                                            │
 b │ Creation date . : 2002/07/08                                        │
   │ Expiration Date : 2002/10/01                                        │
 I │                                                                     │
   │ Enter "/" to select option                                          │
   │    Purge Data Set                                                   │
   │                                                                     │
   │ Instructions:                                                       │
 O │   Enter "/" to confirm the purge request.                           │
   │   (The data set will be deleted and uncataloged.)                   │
   │                                                                     │
   │   Press CANCEL or EXIT to cancel the purge request.                 │
   │                                                                     │
   │ Command ===>                                                        │
 D │  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward     │
   │  F9=Swap     F12=Cancel                                             │
   ⋘─────────────────────────────────────────────────────────────────────┘
 F10=Actions  F12=Cancel
Figure 74. Confirm  purge panel (ISRUADC3)
Use a slash to select Purge Data Set if you want ISPF to purge the data set. The statement that is enclosed
in parentheses on the Confirm Purge panel shows whether the data set to be purged will be uncataloged.
When you delete a data set, the volume name is compared to the volume name in the configuration table.
If the names match, the command specified in the configuration table is used in place of the ISPF delete
processing. This lets you delete migrated data sets without first causing them to be recalled.
Delete processing with RACF
If the data set is an SMS data set, it is deleted using SVC 29. The update of the catalog entry is handled by
the operating system.
If the data set is not an SMS data set and either it is not cataloged or the user indicates on panel
ISRUADC2 (panel ISRUADC2 is shown if the volume is supplied) that it is not to be uncataloged, it is
deleted using SVC 29.
If the data set is not an SMS data set and it is cataloged and/or the user indicates on panel ISRUADC2
(panel ISRUADC2 is shown if the volume is supplied) that it is to be uncataloged, this process is followed:
1. The data set is deleted using SVC 29 (SCRATCH).
2. The data set is uncataloged using SVC 26.
This order of processing is intended to minimize the possibility that an uncataloged data set will result if
an error is encountered during the delete process. Some combinations of RACF generic and discrete data
set profiles and user access to the groups under which those profiles fall can cause this process to fail. If
an error is encountered in this process the user is notified via a message of the status of the data set and
catalog entries.
See the z/OS Security Server RACF Security Administrator's Guide or equivalent documentation for your
security package, to determine the authorization levels required for each of these operations. The user
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  113

## Page 152

will need authorization first to delete the data set and then to uncatalog the data set name. This will
require adequate authorization to any discrete or generic data set profiles involved and to the catalogs
involved. Be aware that a discrete data set profile is deleted when the data set is deleted.
S — data set information (short)
If you select option S, information about the selected data set is displayed. The information displayed by
option S is the same information displayed by option S on the Library Utility panel (option 3.1). See “S —
short data set information” on page 96 for more information and Figure 65 on page 97 for an example. To
return to the previous panel, press Enter or enter the END command.
The space for data sets allocated in blocks is calculated as if all of the tracks, including the last one,
contain only full blocks of data. Any partial "short" blocks are ignored.
Note:
1. The information shown for current space utilization is the actual data that the data set contains, based
on the number of allocation units (blocks, tracks, bytes, megabytes, and so on) that have been written.
For a data set allocated in units other than tracks and cylinders, it does not include the unused portion
of a track that is only partially filled.
For example, if a data set allocated in bytes with block size of 600 has one block written to a device
with a track size of 1000, 600 bytes of data are written and the remaining 400 bytes cannot be used
by a different data set. A track is the smallest possible unit of physical allocation to a data set on
DASD. ISPF reports 600 bytes used while other products (such as ISMF) report 1000 bytes used. ISPF
reports the space occupied by data in the data set. ISMF reports the space used by this data set that is
not available for use by another data set. The difference is a relative indication of the effectiveness of
the block size used when the data set was created.
2. Space utilization values are not displayed for VSAM or BDAM data sets.
See “Short information for managed data sets” on page 97 to learn more about the data set information
that is displayed for managed data sets.
Blank — (data set information)
If you leave the Option field blank, information about the selected data set is displayed. The information
displayed is the same information displayed by option I on the Library Utility panel (option 3.1). See “I —
data set information” on page 92 for more information and “U — uncatalog data set” on page 112 for an
example. To return to the previous panel, press Enter or enter the END command.
Note:
1. For multivolume data sets, options I and S display current allocation and utilization values that
represent totals from all volumes used.
2. You can not enter a volume serial when you are requesting information on a multivolume data set.
3. The space for data sets allocated in blocks is calculated as if all of the tracks, including the last one,
contain only full blocks of data. Any partial "short" blocks are ignored.
4. The information shown for current space utilization is the actual data that the data set contains, based
on the number of allocation units (blocks, tracks, bytes, megabytes, and so on) that have been written.
For a data set allocated in units other than tracks and cylinders, it does not include the unused portion
of a track that is only partially filled.
For example, if a data set allocated in bytes with block size of 600 has one block written to a device
with a track size of 1000, 600 bytes of data are written and the remaining 400 bytes cannot be used
by a different data set. A track is the smallest possible unit of physical allocation to a data set on
DASD. ISPF reports 600 bytes used while other products (such as ISMF) report 1000 bytes used. ISPF
reports the space occupied by data in the data set. ISMF reports the space used by this data set that is
not available for use by another data set. The difference is a relative indication of the effectiveness of
the block size used when the data set was created.
5. Space utilization values are not displayed for VSAM or BDAM data sets.
Data set utility (option 3.2)
114  z/OS: z/OS ISPF User's Guide Vol II

## Page 153

See “Information for managed data sets” on page 95 for information about the data set information that
is displayed for managed data sets.
V — VSAM utilities
Use option V to create the IDCAMS commands to define, delete, and list catalog information for VSAM
data sets. Before the command is issued, you will be allowed to edit it in an ISPF Edit session. The
command will process in the foreground.
Note: The VSAM utilities function builds a command that is syntactically correct; the utility does not do
any compatibility checking of the fields used to build the command.
When you select option V, the panel shown in Figure 75 on page 115 is displayed.
   ┌────────────────────────────────────────────────────────────────────────┐
 ─ │   Menu  Utilities  Help                                                │ ─
   │ ─────────────────────────────────────────────────────────────────────  │
   │                            VSAM Utilities                              │
   │                                                           More:     +  │
   │  Process Request                 Data Type                             │
   │     1. Define                        1.  Alias                         │
 b │     2. Delete                        2.  Alternate Index               │
   │     3. Information (Listcat)         3.  Cluster                       │
 I │                                      4.  Generation Data Group         │
   │                                      5.  Non-VSAM                      │
   │                                      6.  Page Space                    │
   │                                      7.  Path                          │
   │                                      8.  User Catalog                  │
 O │                                      9.  Data     *                    │
   │                                      10. Index    *                    │
   │                                      11. NVR      **                   │
   │                                      12. Truename **                   │
 D │                                      13. VVR      **                   │
   │                       * Listcat Only                                   │
   │ Command ===>                                                           │
 O │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward    │
   │  F9=Swap      F10=Actions   F12=Cancel                                 │
 F ⋘────────────────────────────────────────────────────────────────────────┘
Figure 75. VSAM Utilities panel (ISRUVSAM)
VSAM Utilities panel action bar
The VSAM Utilities panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides information on the VSAM utilities, including the VSAM profile data set
and the GET, SAVE, and CHANGE commands.
VSAM Utilities panel fields 
There are two fields on the VSAM Utilities panel:
Process Request
Required field. Indicates what is going to be done to the VSAM data set:
1
Define. Process an IDCAMS define request
2
Delete. Process an IDCAMS delete request against one or more data sets.
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  115

## Page 154

3
Listcat. Process an IDCAMS list catalog request.
VSAM Data Type
Required field. Indicates what kind of data set is to be defined, deleted or listed:
1
Alias. Define, delete, or list an alternate name for a non-VSAM data set or a user catalog
2
Alternate Index. Specify that an alternate index is to be defined, deleted, or listed or that an
alternate index entry is to be recataloged
3
Cluster. Specify that a cluster is to be defined, deleted, or listed or that a cluster entry is to be
recataloged
4
Generation Data Group. Specify that a generation data group entry is to be defined, deleted, or
listed
5
Non-Vsam. Specify that a non-VSAM, non-SMS-managed data set is to be defined, deleted, or
listed
6
Page Space. Specify that a page space is to be defined, deleted, or listed
7
Path. Specify that a path is to be defined, deleted, or listed or that a path entry is to be
recataloged
8
User Catalog. Specify that a catalog is to be defined, deleted, or listed
9
Data. List data level information (Listcat request only)
10
Index. List index level information (Listcat request only)
11
NVR. Delete an SMS-managed non-VSAM volume record entry (Delete request only)
12
Truename. Delete the truename entry for a data or index component of a cluster or alternate
index or the name of an alternate index (Delete request only)
13
VVR. Delete an unrelated VSAM volume record entry (Delete request only).
Example usage – defining  a cluster
To define a cluster, on the VSAM Utilities panel, type 1 in the Process Request field and 3 in the VSAM
Data Type field. The Define Cluster panel is displayed as shown in Figure 76 on page 117.
Data set utility (option 3.2)
116  z/OS: z/OS ISPF User's Guide Vol II

## Page 155

Menu  Function  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                 Define Cluster
                                                Enter "/" to select option
                                                /  Edit IDCAMS command
                                                /  Browse errors only
  Cluster Name  . . . MYPROJ.DEV.SOURCE3                         
                                                                    More:     +
                           Cluster Level Information:
  Space Units  . . . . . . .    1. Cylinders   Primary Quantity . . .         
                                2. Tracks      Secondary Quantity . .         
                                3. Records
                                4. Kilobytes
                                5. Megabytes
  Volumes  . . . . . . . . .         . . .         . . .         . . .       
  Buffer Space . . . . . . .         
  Control Interval Size  . .       
  Data Class . . . . . . . .         
  Management Class . . . . .         
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 76. Define  Cluster panel (ISRUVPC3)
Note:
1. Select the Edit IDCAMS command option to edit the IDCAMS command that this process generates
before the command is issued. If you do not select this option, the command will be issued when you
press Enter.
2. Select the Browse errors only option to browse the output from IDCAMS only when a nonzero return
code is returned by IDCAMS.
3. A Key label input field is included in the Define Cluster section and can be used to add a KEYLABEL
parameter value to the IDCAMS define cluster command.
Fill in the required fields or use a VSAM profile data set as described in “Building a VSAM profile data set”
on page 118. When you press Enter, the screen shown in Figure 77 on page 117 is displayed.
Note: If you try to use a profile that was defined for a different request type (for example, Generation Data
Group), you will receive a "Type mismatch" error.
   Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                                            Columns 00001 00072
     Instructions:
       Enter EXECute command to issue request.
       Enter CANcel, END, or RETURN command to cancel request.
 ****** ***************************** Top of Data ******************************
 ==MSG> -Warning- The UNDO command is not available until you change
 ==MSG>           your edit profile using the command RECOVERY ON.
 000001  /* IDCAMS COMMAND */
 000002  DEFINE CLUSTER (NAME(MYPROJ.DEV.SOURCE3) -
 000003         ) -
 000004         DATA (NAME(MYPROJ.DEV.SOURCE3.DATA) -
 000005         ) -
 000006         INDEX (NAME(MYPROJ.DEV.SOURCE3.INDEX) -
 000007         )
 ****** **************************** Bottom of Data ****************************
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 77. Editing the IDCAMS command (ISRUVEDT)
When you are ready to process the command, type EXEC on the Command line and press Enter. If the
command processes with a nonzero return code, the panel shown in Figure 78 on page 118 is displayed.
Data set utility (option 3.2)
Chapter 5. Utilities (option 3)  117

## Page 156

Figure 78. Browsing IDCAMS Errors (ISRUVBRO)
Press Exit (F3) to return to the panel shown in Figure 77 on page 117, make the necessary changes, and
resubmit the command.
Building a VSAM pr o file  data set
You can build a VSAM profile data set, each member of which can be used to store input fields on a VSAM
input panel for later retrieval to the same panel. If you try to use a profile that was defined for a different
request type (for example, Generation Data Group), you will receive a "Type mismatch" error.
When you have filled in a VSAM input panel, select the Save to Profile choice from the Functions pull-
down on the action bar. ISPF displays the Profile Member Name panel.
Type in a member name for the profile data set member. When you press Enter, the data set is created
with the attributes RECFM=variable blocked, LRECL=203, Type=PDS.
Using a VSAM pr o file  data set
When you have displayed a VSAM input panel, select the Get from Profile choice from the Functions
pull-down on the input panel action bar to display the panel shown in Figure 79 on page 119.
Data set utility (option 3.2)
118  z/OS: z/OS ISPF User's Guide Vol II

## Page 157

Menu  Functions  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
 GET      USERID.VSAM.PROFILE                               Row 00001 of 00001
    Name     Prompt          Size    Created           Changed            ID
 . PMNTEST                     78   2002/08/05   2002/08/05 11:56:23    USERID
   **End**
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 79. Using a VSAM pr o file  data set (ISRVMLGT)
When you select a profile and press Enter, the fields on the entry panel will fill with the values stored in
the profile data set member.
Changing the VSAM pr o file  data set
To change the name of the active VSAM profile data set, select the Change Profile Data Set choice from
the Functions pull-down on an input panel action bar to display the panel shown in Figure 80 on page
119.
   Menu  Function  Utilities  Help
 ─ ┌────────────────────────────────────────────────────────────────────────┐ ─
   │   Menu  Utilities  Help                                                │
   │ ─────────────────────────────────────────────────────────────────────  │
   │                           Profile Data Set                             │
   │                                                                        │
   │  Profile Data Set . . 'USERID.VSAM.PROFILE'                            │
   │                                                                        │
   │                                                                        │ +
   │ Command ===>                                                           │
   │  F1=Help       F2=Split      F3=Exit       F9=Swap      F10=Actions    │
   │ F12=Cancel                                                             │
   └────────────────────────────────────────────────────────────────────────┘
                                3. Records
                                4. Kilobytes
   ⋮
Figure 80. Panel for changing the name of the VSAM pr o file  data set (ISRUVGET)
You can type the name of a different profile data set. When you press Enter, the data set is created if it
does not exist, and this data set becomes the active profile data set.
Move/Copy utility (option 3.3)
When you select this option, a panel is displayed (Figure 81 on page 120) that allows you to specify the
"From" data set (and member if it is partitioned) and an option to be performed. The Move/Copy Utility
prevents you from moving or copying a PDS member that you or another user is currently editing.
Move/Copy utility (option 3.3)
Chapter 5. Utilities (option 3)  119

## Page 158

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                              Move/Copy Utility
 C  Copy data set or member(s)          CP Copy and print
 M  Move data set or member(s)          MP Move and print
Specify "From" Data Set below, then press Enter key
From ISPF Library:
   Project . . . ________      (--- Options C and CP only         ---)
   Group . . . . ________ . . . ________ . . . ________ . . . ________
   Type  . . . . ________
   Member  . . . ________         (Blank or pattern for member list,
                                   "*" for all members)
From Other Partitioned or Sequential Data Set:
   Data Set Name . . . _______________________________________________________
   Volume Serial . . . ________   (If not cataloged)
Data Set Password  . .            (If password protected)
Option ===> __________________________________________________________________
 F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
F10=Actions  F12=Cancel
Figure 81. Move/Copy Utility panel (ISRUMC1)
Move/Copy Utility panel action bar
The Move/Copy Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides information on how to specify the "From" and "To" data sets, how to
select members to copied, and the rules relating to how different data types are moved or copied.
Move/Copy Utility panel fields
All the fields on the Move/Copy Utility panel are explained in the "ISPF Libraries and Data Sets" chapter of
the z/OS ISPF User's Guide Vol I. On this panel, you specify the data set that you want to copy, move, lock,
or promote. This is called the "From" data set.
If you request a member list or specify an asterisk (*) in the Member field on the "From" panel, ISPF does
not display a Member field on the "To" panel. See the Member Selection List Commands section of the
"ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I for information about primary
and line commands that are available for the Move/Copy utility member list display.
You can copy or move load modules stored in partitioned data sets with undefined record formats, but
you cannot print them.
The deletion of any member because of a move is recorded in your ISPF log data set, if allocated.
When you complete the panel and press Enter, ISPF displays another panel that is determined by the
option you selected. This panel allows you to specify the "To" or "Target" data set or controlled library.
The "From" data set must already exist. If the "Target" data set does not exist the user is prompted to see
if the data set should be allocated. Choices are to allocate the "Target" data set using the characteristics
of the "From" data set as a model, or to allocate the new data set by specifying the characteristics for it. If
the user uses the "From" data set as a model, then that data set must be cataloged and the volume field
Move/Copy utility (option 3.3)
120  z/OS: z/OS ISPF User's Guide Vol II

## Page 159

is ignored. This function can be suppressed through the ISPF Configuration table. If it is suppressed, an
allocate request for a nonexistent data set fails.
Move/Copy utility options
These topics describe the options shown on the Move/Copy Utility panel:
• “C and CP — copying data sets” on page 121
• “M and MP — moving data sets” on page 123
• “Using the move/copy utility with load modules” on page 124
• “Moving or copying alias entries” on page 125
• “Member list processing when using IEBCOPY” on page 126
C and CP — copying data sets
When you use the C and CP options, ISPF supports library concatenation. This allows you to specify up to
four input libraries as the "From" data set. The libraries are searched from left to right as they are entered
on the panel. The member to be copied, which is either specified in the Member field or selected from a
member list, is copied from the first library in which it is found.
If you select C or CP, the panel shown in Figure 82 on page 122 is displayed. This panel allows you to
specify the "To" data set—the library or data set name that you want the copied data to be stored under.
Note: The Move/Copy utility does not support:
• Supplying a volume serial when attempting to copy a multivolume data set
• Copying unmovable data sets (data set organization POU or PSU).
C — copy data set or member(s)
Use option C to copy a data set. You can specify either a DASD-resident sequential or partitioned data set
for both the "From" or "To" data sets. The "From" data set is not deleted.
CP — copy and print
Use this option as you would use option C, except that source listings are recorded in the ISPF list data
set, as follows:
• If the "To" data set is partitioned, a listing of each new or replaced member is recorded.
• If the "To" data set is sequential, a listing of its complete contents is recorded.
Move/Copy utility (option 3.3)
Chapter 5. Utilities (option 3)  121

## Page 160

Menu  RefList  Utilities  Help
───────────────────────────────────────────────────────────────────────────────
COPY     From MYPROJ.DEV.SOURCE
                                                                   More:     +
Specify "To" Data Set Below
To ISPF Library:                 Options:
   Project  . . MYPROJ              Enter "/" to select option
   Group  . . . DEV                 _  Replace like-named members
   Type . . . . SOURCE              /  Process member aliases
To Other Partitioned or Sequential Data Set:
   Data Set Name . . . _______________________________________________________
   Volume Serial . . . ______    (If not cataloged)
Data Set Password  . .           (If password protected)
To Data Set Options:
   Sequential Disposition        Pack Option         SCLM Setting
   _  1. Mod                     3  1. Yes           3  1. SCLM
      2. Old                        2. No               2. Non-SCLM
Command ===>  ________________________________________________________________
 F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
F10=Actions  F12=Cancel
Figure 82. Move/Copy Utility - "To" panel for copying (ISRUMC2B)
All the fields on the Move/Copy Utility "To" panels for copying data sets are explained in the Libraries and
Data Sets topic in the z/OS ISPF User's Guide Vol I, except these general Options and To Data Set Options:
Replace like-named PDS members
Select this option to allow replacement of a member in the "To" data set with a like-named member in
the "From" data set.
Process member aliases
Select this option to allow the primary member and all alias members to be copied together.
Sequential Disposition
If the "To" data set is sequential, enter:
1
To add the "From" data set to the end of the "To" data set (Mod).
2
To replace the "To" data set's entire contents with the contents of the "From" data set (Old).
If the "From" data set consists of several members of an ISPF library or a partitioned data set to be
moved or copied to a sequential data set, the members are written to the "To" data set one after
another. The "To" data set disposition (Old or Mod) controls only the beginning location of the "To"
data set after the copy or move is completed.
Pack Option
To indicate how the data is to be stored in the "To" data set, enter:
1
If you want the data in the "To" data set to be packed.
2
If you do not want the data in the "To" data set to be packed.
3
If you want the data to be stored in the same format in the "To" data set as it is in the "From" data
set.
If you are copying data to a sequential data set with disposition of MOD, you cannot mix packed
and unpacked data, nor can you copy multiple packed members.
The technique used to pack data is an internal algorithm used only by ISPF. If the data is packed,
attempts to access or process the data outside ISPF can cause unwanted results. See the description
of the PACK primary command in z/OS ISPF Edit and Edit Macros for more information.
Move/Copy utility (option 3.3)
122  z/OS: z/OS ISPF User's Guide Vol II

## Page 161

SCLM Setting
The SCLM setting is a bit that ISPF uses to determine what type of edit the file last had performed
upon it.
1 SCLM
This bit is ON to specify that the last edit of this file was under SCLM control.
2 Non-SCLM
This bit is ON to specify that the last edit of this file was under control of something other than
SCLM.
3 As-is
This bit is ON to specify that this operation leaves the current setting unchanged.
M and MP — moving data sets
When you use the M and MP options, ISPF does not provide library concatenation support. You can
specify up to four input libraries as the "From" data set. However, only the first library in the sequence is
searched. Therefore, the member to be moved, which is either specified in the Member field or selected
from a member list, is moved only if it is found in the first library. However, the other three library names
remain on the panel and can be used with the C and CP options.
If you select M or MP, the panel shown in Figure 83 on page 124 is displayed. This panel allows you to
specify the "To" data set—the library or data set name that you want the moved data stored under.
Note: The Move/Copy utility does not support:
• Supplying a volume serial when attempting to copy a multivolume data set
• Copying unmovable data sets (data set organization POU or PSU).
M — move data set or member(s)
Use option M to move a data set. You can specify either a DASD-resident sequential or partitioned data
set for both the "From" or "To" data sets.
Option M causes data sets to be deleted after they have been successfully moved to the "To" data set, as
follows:
• If the "From" data set is partitioned, the selected members are deleted from it.
• If the "From" data set is sequential, the complete "From" data set is deleted.
MP — move and print
Same as option M, except source listings are recorded in the ISPF list data set, as follows:
• If the "To" data set is partitioned, a listing of each new or replaced member is recorded.
• If the "To" data set is sequential, a listing of its complete contents is recorded.
Move/Copy utility (option 3.3)
Chapter 5. Utilities (option 3)  123

## Page 162

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
 MOVE     From MYPROJ.DEV.SOURCE
                                                                    More:     +
 Specify "To" Data Set Below
 To ISPF Library:                 Options:
    Project  . . MYPROJ              Enter "/" to select option
    Group  . . . DEV                 _  Replace like-named members
    Type . . . . SOURCE              /  Process member aliases
 To Other Partitioned or Sequential Data Set:
    Data Set Name . . . ______________________________________________________
    Volume Serial . . . ______    (If not cataloged)
 Data Set Password  . .           (If password protected)
 To Data Set Options:
    Sequential Disposition        Pack Option         SCLM Setting
    1  1. Mod                     3  1. Yes           3  1. SCLM
       2. Old                        2. No               2. Non-SCLM
 Command ===> ________________________________________________________________
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 83. Move/Copy Utility - "To" panel for moving (ISRUMC2B)
All the fields on the Move/Copy Utility "To" panels for moving data sets are explained in the "ISPF
Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I, except these general Options and To
Data Set Options:
• Replace like-named PDS members.
• Process member aliases
• Sequential Disposition
• Pack Option
• SCLM Settings
See “C and CP — copying data sets” on page 121 for descriptions of these fields.
Using the move/copy utility with load modules
For a move or copy of load modules, these rules apply:
• Both data sets must be partitioned and must have an undefined record format (RECFM=U).
• Load modules that were created for planned overlay cannot be moved or copied.
• The print option, if specified, is ignored.
• If the "To" library is LLA-managed, it must be in NOFREEZE mode.
• For Move or Copy, reblocking can be done for load modules only, and is done by using the IEBCOPY
COPYMOD function. Whether the load module is reblocked depends on the block sizes for the "To"
and "From" data sets, as well as the value of the USE_IEBCOPY_COPY_OR_COPYMOD_OPTION and
WHEN_TO_USE_IEBCOPY keyword settings in the ISPF Configuration table (see z/OS ISPF Planning and
Customizing for more information).
– If the WHEN_TO_USE_IEBCOPY setting is 0, IEBCOPY is only used:
- When copying from a data set with a larger block size to a data set with a smaller block size.
- When a PDSE has been specified in the "From" data set concatenation or as the "To" data set.
– If the WHEN_TO_USE_IEBCOPY setting is 1, IEBCOPY is always used to copy load modules.
– If the WHEN_TO_USE_IEBCOPY setting is 2, IEBCOPY is only used when a PDSE has been specified in
the "From" data set concatenation or as the "To" data set.
– If the WHEN_TO_USE_IEBCOPY setting indicates that IEBCOPY should be used, these rules apply
when determining whether to reblock or not:
Move/Copy utility (option 3.3)
124  z/OS: z/OS ISPF User's Guide Vol II

## Page 163

- When the USE_IEBCOPY_COPY_OR_COPYMOD_OPTION setting is 1:
• If the "To" and "From" block sizes are the same, no reblocking occurs. ISPF uses IEBCOPY COPY.
• If the "To" block size is larger than the "From" block size, no reblocking occurs. ISPF uses
IEBCOPY COPY.
• If the "To" block size is smaller than the "From" block size, reblocking occurs. ISPF uses IEBCOPY
COPYMOD.
- When the USE_IEBCOPY_COPY_OR_COPYMOD_OPTION setting is 2:
• If the "To" and "From" block sizes are the same, no reblocking occurs. ISPF uses IEBCOPY COPY.
• If the "To" block size is larger than the "From" block size, reblocking occurs. ISPF uses IEBCOPY
COPYMOD.
• If the "To" block size is smaller than the "From" block size, reblocking occurs. ISPF uses IEBCOPY
COPYMOD.
– When the USE_IEBCOPY_COPY_OR_COPYMOD_OPTION setting is 3:
- Reblocking occurs. ISPF uses IEBCOPY COPYMOD.
• If IEBCOPY is used to process the copy, ISPF allocates these data sets:
zprefix.zuser.SPFnnn.IEBCOPY
IEBCOPY SYSPRINT data set
SYSIN
IEBCOPY SYSIN data set
SYSUT3 and SYSUT4
IEBCOPY work data sets
The SYSPRINT data set is deleted when the copy ends successfully. If errors are encountered, it is
kept to help you diagnose errors. SYSIN, SYSUT3, and SYSUT4 are temporary data sets that use VIO if
available, and are freed upon completion of the copy. All allocations use the value of ISPF Configuration
table keyword PDF _DEFAULT_UNIT as the unit. The sizes for the SYSUT3 and SYSUT4 data sets are
calculated dynamically, based on the number of members to be copied. If this is not sufficient for your
move/copy request, these DDNAMES can be preallocated. If they are preallocated, ISPF does not free
them when the copy is finished.
Moving or copying alias entries
Alias entries can be moved or copied from one partitioned data set to another under these conditions:
• If the "To" library is LLA-managed, it must be in NOFREEZE mode
• If the "Process member aliases" option has been selected (ALIAS mode), these rules apply:
– Either the Primary member or any alias member may be selected to copy the primary member and
all of its aliases. This will occur even if a single member is specified or some of the members are not
displayed in the current member selection list.
– Alias members are copied for both load and non-load data sets, as well as for PDS and PDSE data
sets.
– Copying to the same data set is not supported when aliases are automatically selected, as this would
result in the from and to member name being the same.
• If ISPF is not using IEBCOPY and the "Process member aliases" option has not been selected (NOALIAS
mode):
– After the move or copy is successfully completed for the main member or members, then the alias
entry or entries can be copied.
– From a member list:
- When the main member or members are selected first, are not renamed, and are successfully
moved or copied, then the alias entry or entries can be copied if they are selected without leaving
the member list.
Move/Copy utility (option 3.3)
Chapter 5. Utilities (option 3)  125

## Page 164

- If the target data set is a PDSE, alias entries cannot be copied.
• If IEBCOPY is being used and NOALIAS is in effect:
– The method described for copying when not using IEBCOPY will also work when using IEBCOPY. In
addition, if all main members and aliases are selected at the same time they are processed by the
same invocation of IEBCOPY and are copied correctly.
– If the target data set is a PDSE, alias entries must be selected and processed together with the main
member.
In all other cases for move and copy where NOALIAS is in effect, you can select alias names, but they
are not preserved as aliases in the "To" data set. That is, the members to which they refer are moved or
copied, and the alias entries are stored in the "To" data set with the alias flags turned off.
Member list processing when using IEBCOPY
When copying load modules using the IEBCOPY interface, all selected members are processed as a group.
This means that the processing does not stop on the first failure but will attempt to process all selected
members before the member list is redisplayed. The Prompt field will be updated to indicate the result for
each individual member.
No error message is displayed if two or more members are not processed successfully because they
may have failed for different reasons. Reselecting a member and processing it individually will display
a specific error message if the processing for that member fails again. These values can appear in the
Prompt field:
*COPIED
Member was copied or copied/locked successfully
*MOVED
Member was moved successfully
*REPL
Member was replaced in the output library (Moved or Copied)
*NO DATA
Member was not found in the input library or BLDL error
*INUSE-I
ENQ failed on input member
*INUSE-O
ENQ failed on output member
*NO-COPY
Member was not copied successfully
*MIXED
You are attempting to mix load and non-load data
*NO-DEL
The delete step failed on a Move request
*NO-REPL
Member exists in the output library and replace not requested
*ALIAS
Member is a PDSE Program Object alias and cannot be copied individually. It will be copied when the
main member is copied.
Data set list utility (option 3.4)
When you select this option, the Data Set List Utility panel (Figure 84 on page 127) is displayed. You can
either display or print lists of ISPF libraries, data sets, or volume table of contents (VTOC) information.
Data set list utility (option 3.4)
126  z/OS: z/OS ISPF User's Guide Vol II

## Page 165

Menu  RefList  RefMode  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                             Data Set List Utility
    blank Display data set list               P Print data set list
        V Display VTOC information           PV Print VTOC information
 Enter one or both of the parameters below:
    Dsname Level . . .    PDFTOOL.COMMON                                      
    Volume serial  . .          
 Data set list options
    Initial View                 Enter "/" to select option
    1  1. Volume                 /  Confirm Data Set Delete
       2. Space                  /  Confirm Member Delete
       3. Attrib                 /  Include Additional Qualifiers
       4. Total                  /  Display Catalog Name
                                 /  Display Total Tracks
                                    Prefix Dsname Level
 When the data set list is displayed, enter either:
   "/" on the data set list command field for the command prompt pop-up,
   an ISPF line command, the name of a TSO command, CLIST, or REXX exec, or
   "=" to execute the previous command.
 Option ===>__________________________________________________________________
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 84. Data Set List Utility panel (ISRUDLP)
Data Set List Utility panel action bar
The Data Set List Utility Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Note: When you use a referral list from within the Data Set List Utility, these functions are performed
before the referral list is processed:
• The quotes are removed from the data set name.
• The value in ZPREFIX is added preceding the non-quoted data set name if the first qualifier is not
ZPREFIX.
• The member name is removed.
RefMode
For information about referral list modes, see the details about Personal List Modes in the Using
Personal Data Set Lists and Library Lists topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides information about displaying and printing data set lists and VTOC
information.
Data Set List Utility panel fields
The fields on this panel are:
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  127

## Page 166

Dsname level
This field is used to specify the level or levels of any data set that you want ISPF to list or print for you. An
optional installation exit, called the data set list (DSLIST) exit, can control whether a data set name should
appear in the list. See z/OS ISPF Planning and Customizing for more information about this exit.
When you specify the Dsname Level, you are defining the level qualifiers for the data set names to be
included in the list. Therefore, in Figure 84 on page 127, the value PDFTOOL.COMMON represents the first
two levels of a data set name. An ISPF library typically has a three-level name: project, group, and type.
The Dsname Level field supports the inclusion of system symbols.
ISPF retains the information you put in this field and displays it the next time you use this panel.
Except for the first level, you can specify the level qualifiers fully, partially, or use defaults. Do not enclose
the value in the Dsname Level field in quotes.
Asterisks and percent signs may be used to filter the list of data sets that is displayed. For ICF catalog
lists and volume lists, asterisks and percent signs may be used in the high-level qualifier. Asterisks may be
used anywhere in a qualifier, not just in the first and last positions. However, one qualifier must be at least
partially qualified.
A single asterisk by itself indicates that at least one qualifier is needed to occupy that position. A single
asterisk within a qualifier indicates that zero or more characters can occupy that position. A double
asterisk by itself indicates that zero or more qualifiers can occupy that position. A double asterisk is
invalid within a qualifier.
In this example, all data set names with SYS1 as the first qualifier and at least one other qualifier will be
listed.
SYS1.*
In this example, all data set names with SYS1 as the first qualifier will be listed.
SYS1 or SYS1.**
In this example, all data set names that have a qualifier of CLIST and are in catalogs that you have
authority to will be listed. A VTOC list will contain all data set names that have a qualifier of CLIST.
**.CLIST
Note:
1. If you enter a high-level qualifier of '*' or '**', ISPF displays a pop-up window to warn you that the
search will be for all catalogs on the system and will take time. If there are many catalogs, this
search could take a considerable amount of time. You can press Enter to continue the search, or you
can enter Cancel or End from the pop-up window to cancel the search. Be aware that if you have
mount authority, a catalog search with '*' or '**' as the high-level qualifier can require that volumes be
mounted for the catalogs to be searched.
2. The ISPF Configuration table contains a selectable option, named DISALLOW_WILDCARDS_IN_HLQ, to
disallow the use of the '*' or '%' in the high-level qualifier. 
3. If the first character of the dsname level is a dot(.), tilde (~) or forward slash (/), the string is passed
unchanged to UDLIST. No exits or other processing normally associated with the Data Set List Utility is
performed.
A single percent sign indicates that any one single alphanumeric or national character can occupy that
position. One to eight percent signs can be specified in each qualifier. This example is valid for Dsname
Level:
AAA%*.B*%%%B.C
In this example, the list will contain all data sets that start with AAA and one or more other characters,
have a second qualifier that starts and ends with B and has at least three other characters between the
Data set list utility (option 3.4)
128  z/OS: z/OS ISPF User's Guide Vol II

## Page 167

B's, and have a third qualifier of 'C'. The list will contain entries from catalogs that you have authority to. A
VTOC list will contain entries that match these characteristics.
In this example, the list will contain all data sets that start with SYS and one other character, such as SYS1
or SYS2.
SYS%
If you enter a SYS% alias for a SYS1 data set as the Dsname Level (for example, SYSP as a single
qualifier), you see SYSP as an ALIAS because this single qualifier is an alias for SYS1. The data set names
pointed to by a SYS% alias can be displayed in a data set list by entering any of these:
• a Dsname Level of SYS1 and a volume
• a Dsname Level of SYS%
• a Dsname Level of the fully qualified data set name (such as SYSP.PARMLIB)
PRO**CT is not valid as a data set name level because a double asterisk (**) is not valid in the middle of a
qualifier.
Alias names that match the specified Dsname Level will be displayed as the alias name itself. The volume
field for all alias names will contain the characters '*ALIAS' to indicate this. Real names that match the
Dsname Level will also be displayed.
If you enter ISPFTEST as a Dsname Level and you have real data set names that start with ISPFTEST and
aliases for those real names that start with ISPFTEST, you would see a list of this format:
ISPFTEST.BASE.CLIST                         TSOPK1
ISPFTEST.BASE.CLIST.ALIAS                   *ALIAS
ISPFTEST.BASE.SOURCE                        TSOPK1
ISPFTEST.BASE.SOURCE.ALIAS                  *ALIAS
A VSAM cluster entry is flagged in the volume field as '*VSAM*'. A VSAM path entry is flagged in the
volume field as '*PATH*'. A VSAM alternate index entry is flagged in the volume field as '*AIX®*'.
Note:
1. A catalog search may result in the DSLIST containing duplicate names. This can occur when the
definition of user catalog aliases results in multiple catalogs being searched when the data set list is
built. Line commands against duplicate data sets in the DSLIST are supported. Selecting the "Display
Catalog Name" option will display the name of the catalog associated with each data set on the Total
view. This can identify where duplicate data set names were found. The existence of duplicates may
be inconsistent when changing the DSLEVEL qualifiers. For example, SYS1.PARM.* may have different
results than SYS1.PAR*. Duplicate entries may or may not display in a consistent manner, however the
DSLIST will always be complete, with no omissions.
2. If a VSAM cluster matches the Dsname Level, all parts of the cluster are listed even if the data and
index portions do not match the Dsname Level.
When a multicluster (key-range) data portion of a VSAM cluster is displayed on a catalog list, no
information is shown except for the volume and device. The information comes from the VTOC and the
catalog name does not match the VTOC name. When using a VTOC list the information is displayed.
Volume serial
Use this field to specify the volume serial whose VTOC is to be used by ISPF to display or print a list of
data set names or VTOC information. ISPF retains the information you put in this field and displays it the
next time you use this panel.
If you want to display a list of only the data sets that reside on a particular volume, leave the Dsname
Level field blank and enter the volume serial in the Volume field.
The Volume serial field supports the inclusion of system symbols.
You can enter a single volume name or a generic volume name to list data sets from more than one
volume. The volume name can be partially specified using asterisks as global volume name characters
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  129

## Page 168

and percent signs as placeholders in the volume name. A single asterisk within a volume name indicates
that zero or more characters can occupy that position. A single percent sign indicates that any one
alphanumeric or national character can occupy that position. Examples follow.
*
Lists data set names matching the Dsname Level from all volumes
PRM*
Lists names from all volumes beginning with 'PRM'
M%C*
Lists names from volumes beginning with 'M', followed by any single character, a 'C', and any three
other characters
Note:
1. During pre-allocation verification processing for a data set list line command, ISPF issues a LOCATE
(SVC 26) for the data set name. This occurs even when you specify a volume serial on the Data Set
List Utility panel. If this LOCATE fails (for example, an SMS data set by the same name exists and the
volume for the SMS data set is not available), ISPF issues an error message and the line command
fails.
2. Specifying a single asterisk as a volume name will require more time to display of print the VTOC list.
3. A generic volume name can not be used to display VTOC information.
Initial view
Use this field to tell ISPF which view of the data set list you would like to see. ISPF retains the information
you put in this field and displays it the next time you use this panel.
All the scroll commands function normally from these displays, except for the LEFT and RIGHT
commands. These commands switch from one view to another, because the panels used to show the
different views are connected as if they formed a ring. Each time you enter the LEFT or RIGHT command,
another view is displayed in the sequence shown in Figure 85 on page 130, starting from the current view.
Figure 85. Sequence of data set list display views
If you enter the RIGHT command with the Total view in the sequence displayed, ISPF displays the Volume
view. If you enter the LEFT command with the Volume view displayed, ISPF displays the Total view. The
available views are:
1. Volume
This view shows a data set list that contains data set names and the volumes on which they reside.
Figure 86 on page 134 shows a typical data set list display using the Volume view.
Data set list utility (option 3.4)
130  z/OS: z/OS ISPF User's Guide Vol II

## Page 169

2. Space
The Space view shows a data set list that contains data set names, tracks, percentages used, extents,
and devices. An additional header line, displayed above the column headings and showing the total
tracks of all data sets, the total tracks of all non-excluded data sets, the number of data sets listed
and the number of non-excluded data sets listed, is displayed if the Display Total Tracks option is
selected. Figure 87 on page 135 shows a typical data set list display using the Space view with the
Total Tracks header line.
3. Attrib
This view shows a data set list that contains data set names, data set organizations, record formats,
logical record lengths, and block sizes. Figure 88 on page 136 shows a typical data set list display
using the Attributes view.
Note: For each of the views, the list is sorted by data set name. See the list under Figure 89 on page
136 for descriptions of the fields shown on this panel.
4. Total
This view shows a data set list that contains all the information displayed by the Volume, Space, and
Attributes views, plus the created and expired or referred dates. (The Display Expiration Date option
on the DSLIST Settings of the options pull-down on the Data Set List utility (option 3.4) allows you
to display either the expiration date or the referred date of the data set.) The list is sorted by data
set name and has two lines per data set. Figure 89 on page 136 shows a typical data set list display
using the Total view.
The catalog name can also be displayed if the Display Catalog Name option is selected and no value
is entered into the Volume Serial field. If the Display Catalog Name option is selected, three lines per
data set are displayed. Figure 90 on page 136 shows a typical data set list display using the Total view
with the Catalog name. See the list under the figure for descriptions of the fields shown on this panel.
An additional header line, displayed above the column headings and showing the total tracks of all
data sets, the total tracks of all non-excluded datasets, the number of data sets listed, and the
number of non-excluded data sets listed, is displayed if the Display Total Tracks option is selected.
Confirm data set delete
This field controls whether the Confirm Delete panel appears when you use the D (delete data set) line
command or the TSO DELETE command from the displayed data set list. Use a slash to select this option.
If you select this option, ISPF displays the Confirm Delete panel (Figure 73 on page 112), giving you an
opportunity to change your mind and keep the data set. If you try to delete an unexpired data set, the
Confirm Purge panel (Figure 74 on page 113) is displayed following the Confirm Delete panel. Follow the
directions on the panel to either confirm or cancel the data set purge.
Attention:
If you deselect the option and the data set is deleted, it cannot be retrieved.
See “D — delete data set” on page 151 for more information about the D line command.
Confirm member delete
This field controls whether the Confirm Member Delete panel is displayed when you use the D (delete)
command for a member in the displayed data set list. Use a slash to select this option.
If you select this option, ISPF displays the Confirm Member Delete panel. This panel gives you an
opportunity to change your mind and keep the member.
Include additional qualifiers
This field is used to generate the data set list with all data sets matching the qualifiers in the Dsname
Level field, including data sets with additional qualifiers.
If this field is not selected, the data set list will include only data sets that match the qualifiers entered in
the Dsname Level field.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  131

## Page 170

Examples
Assume that these data sets exist:
  PDFTOOL.COMMON.ASM
  PDFTOOL.COMMON.CLIST
  PDFTOOL.COMMON.CLIST.OLD
  PDFTOOL.COMMON.CLIST.VB
  PDFTOOL.COMMON.CNTL
  PDFTOOL.COMMON.CNTL.INPUT
  PDFTOOL.COMMON.EXEC
1. List data sets whose name starts with PDFTOOL.COMMON. The data set can include additional
qualifiers:
Dsname Level . . . PDFTOOL.COMMON / Include Additional Qualifiers
PDFTOOL.COMMON.ASM
PDFTOOL.COMMON.CLIST
PDFTOOL.COMMON.CLIST.OLD
PDFTOOL.COMMON.CLIST.VB
PDFTOOL.COMMON.CNTL
PDFTOOL.COMMON.CNTL.INPUT
PDFTOOL.COMMON.EXEC
2. List data sets whose name is PDFTOOL.COMMON, with no additional qualifiers:
Dsname Level . . . PDFTOOL.COMMON Include Additional Qualifiers
(No data set names found)
3. List data sets whose name starts with PDFTOOL.COMMON and whose third qualifier starts with "C".
The data set can include additional qualifiers:
Dsname Level . . . PDFTOOL.COMMON.C* / Include Additional Qualifiers
PDFTOOL.COMMON.CLIST
PDFTOOL.COMMON.CLIST.OLD
PDFTOOL.COMMON.CLIST.VB
PDFTOOL.COMMON.CNTL
PDFTOOL.COMMON.CNTL.INPUT
4. List data sets whose name starts with PDFTOOL.COMMON and whose third qualifier starts with "C".
The data set must not have additional qualifiers after the third qualifier:
Dsname Level . . . PDFTOOL.COMMON.C* Include Additional Qualifiers
PDFTOOL.COMMON.CLIST
PDFTOOL.COMMON.CNTL
Display catalog name
Use this option to have the Total view display for each data set in the list the name of the catalog in which
the data set was located.
The option is only applicable when a catalog search is used to build the Data Set List, therefore, it is
ignored when a value is entered in the Volume Serial field.
Display total tracks
Use this option to display an additional header line on the Space or the Total view, showing the total
tracks of all data sets, the total tracks of all non-excluded data sets, the number of data sets listed, and
the number of non-excluded data sets listed.
Depending on the size of the data set list, processing time increases because the tracks information for all
data sets has to be collected before the list is displayed. When the list comprises 50 data sets or more,
a pop-up panel is displayed, indicating the progress of the data collection. The keyboard locks when this
pop-up panel appears and stays locked until the data set list is displayed.
Data set list utility (option 3.4)
132  z/OS: z/OS ISPF User's Guide Vol II

## Page 171

Prefix Dsname Level
Use this option to have ISPF automatically add your TSO user prefix as the first qualifier of the Dsname
Level. When this option is selected and you have created a TSO user prefix, that prefix is added to the
beginning of the Dsname Level provided the Dsname Level is not enclosed in single quotes. If the Dsname
Level is entered enclosed in quotes, ISPF will not add your TSO user prefix. When this option is not
selected ISPF will not accept the Dsname Level enclosed in quotes.
Data set list utility options
Sub-sections describe the options shown on the Data Set List Utility panel.
Blank — display data set list
Leave the Option line blank to display a data set list. You can use these parameters to control what data
set information is displayed and how delete requests are processed:
1. Enter one or more data set name level qualifiers in the Dsname Level field. See “Dsname Level” for
more information.
2. Enter a volume serial in the Volume field if you want ISPF to create a data set list from the VTOC. If
you leave this field blank, the list is created from the catalog. See “Volume serial” on page 129 for
more information.
3. In the Initial View field, enter the view of the data set list (Volume, Space, Attributes, or Total) that
you want to see first. Examples of these views are shown in Figure 86 on page 134, Figure 87 on page
135, Figure 88 on page 136, and Figure 89 on page 136, respectively.
4. Enter a slash (/) in the Confirm Data Set Delete field to tell ISPF to display a confirmation panel if
you enter the D (delete data set) line command or the TSO DELETE command. See “Confirm data set
delete” on page 131 for more information.
5. Enter a slash (/) in the Confirm Member Delete field to tell ISPF to display a confirmation panel if you
enter the D (delete) command for a member in a data set list.
6. Enter a slash (/) in the Include Additional Qualifiers field to tell ISPF to list all data sets that match
the qualifiers in the Dsname Level field, including data sets with additional qualifiers.
7. Enter a slash (/) in the Display Catalog Name field to tell ISPF to display the name of the catalog
associated with each data set in the Total view.
8. Enter a slash (/) in the Display Total Tracks field to tell ISPF to display an additional header line above
the column headings, showing the total tracks of all data sets, the total tracks of all non-excluded
data sets, the number of data sets listed and the number of non-excluded data sets listed. Depending
on the size of the data set list, processing time increases because the tracks information has to be
collected for the whole list up front. When the list comprises 50 data sets or more, a pop-up panel is
displayed, indicating the progress of the data collection.
9. Enter a slash (/) to prefix the data set name level qualifiers in the Dsname Level field.
10. Press Enter to display the data set list, as shown in Figure 86 on page 134.
Note: If a plus displays after the volume serial (for example, HSM020+) on a list obtained from the
catalog, the data set spans multiple volumes. Information displayed about that data set by selecting
Information or Short Information or by using the Space or Total view will represent the total amounts
across all used volumes. For further information, see the description for Volume at “Volume” on page
139.
When a VTOC list is displayed and a multivolume data set is included on that volume, there will not be an
indicator that this data set spans multiple volumes, and the information on a space or total view will be for
that volume only. The information displayed on a VTOC list is only information obtained from the VTOC of
that volume. When the multivolume data set is selected for information or for short information, the space
information will be for all volumes that the data set spans.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  133

## Page 172

Menu  Options  View  Utilities  Compilers  Help
  ───────────────────────────────────────────────────────────────────────────────
 DSLIST - Data Sets Matching HANKO3                                 Row 1 of 14
 Command - Enter "/" to select action                  Message           Volume
 -------------------------------------------------------------------------------
          HANKO3                                                         *ALIAS
          HANKO3.DDIR                                                    *VSAM*
          HANKO3.DDIR.D                                                  D$US50
          HANKO3.DDIR.I                                                  D$US50
          HANKO3.EXEC                                                    D$US08
          HANKO3.ISD1.ISPF.ISPPROF                                       D$US23
          HANKO3.ISD1.ISPVCALL.TRACE                                     D$US48
          HANKO3.ISPF.ISPPROF                                            D$US26
          HANKO3.ISPVCALL.TRACE                                          D$US14
          HANKO3.LOAD                                                    D$US08
          HANKO3.MAKEDSNS.OUTPUT                                         D$US35
          HANKO3.SYS2.BRODCAST                                           D$US04
          HANKO3.TASID.SNAPSHOT                                          D$US05
          HANKO3.TEST                                                    D$US08
 ***************************** End of Data Set list ****************************
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 86. Data set list - volume view (ISRUDSL0)
Data set list panel action bar
The Data Set List panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Options
The Options pull-down offers these choices:
DSLIST Settings
The settings to control the behavior of the data set list display. Options are:
• Display Edit/View entry panel
• Display Browse entry panel
• Automatically update reference lists
• List pattern for MO, CO, D, and RS actions
• Show status for MO, CO, D, and RS actions
• Confirm Member delete
• Confirm Data Set delete
• Do not show expanded command
• Enhanced member list for Edit, View, and Browse
Selecting this choice causes the enhanced member list to be used when the E,V, or B commands
are used. De-selecting this choice causes traditional member list processing to occur.
• Display Total Tracks
• Execute Block Commands for excluded Data Sets
• Display Expiration Date
Refresh List
Refresh the display of the data set list.
Append to List
Select a Personal data set list to append to the existing DSLIST. The DSLIST is rebuilt, including
the data sets or data set name levels from the personal list selected.
Data set list utility (option 3.4)
134  z/OS: z/OS ISPF User's Guide Vol II

## Page 173

Note: The APPEND is based on the selected personal data set list. If an entry in the list is not
quoted, your TSO prefix is added as the first level of the data set name. If the entry contains a
member, the member is ignored. Duplicate personal list entries are ignored. If the entry contains
a volume and "Include volume on retrieve" is selected on the Referral List Settings panel, a VTOC
search is used instead of the catalog. A catalog search is recommended for best performance. A
volume should be used only if the data set is not cataloged.
Enter the DSLIST primary command REFRESH on the DSLIST display panel to erase all appended
personal lists.
Save List
Saves the data set list to a file.
Reset
Resets the data set list.
View
The View pull-down offers these choices:
Note: The current display view is shown as an unavailable choice; that is, it is displayed in blue (the
default) with an asterisk as the first digit of the selection number.
1
Volume Changes the display to the Volume view, as shown in Figure 86 on page 134.
2
Space Changes the display to the Space view. 
   Menu  Options  View  Utilities  Compilers  Help
 ───────────────────────────────────────────────────────────────────────────────
 DSLIST - Data Sets Matching HANKO3                                 Row 1 of 14
 Total Tracks:          86 non-x:         86   Data Sets:     14 non-x:     14
 -------------------------------------------------------------------------------
 Command - Enter "/" to select action                        Tracks %Used   XT
 -------------------------------------------------------------------------------
          HANKO3
          HANKO3.DDIR
          HANKO3.DDIR.D                                          45    ?     1
          HANKO3.DDIR.I                                           1    ?     1
          HANKO3.EXEC                                            15   12     1
          HANKO3.ISD1.ISPF.ISPPROF                                1  100     1
          HANKO3.ISD1.ISPVCALL.TRACE                              2  100     1
          HANKO3.ISPF.ISPPROF                                     1  100     1
          HANKO3.ISPVCALL.TRACE                                   2  100     1
          HANKO3.LOAD                                            15   13     1
          HANKO3.MAKEDSNS.OUTPUT                                  1  100     1
          HANKO3.SYS2.BRODCAST                                    1    0     1
          HANKO3.TASID.SNAPSHOT                                   1  100     1
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 87. Data set list - space view (ISRUDSL0)
3
Attributes Changes the display to the Attributes view. 
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  135

## Page 174

Menu  Options  View  Utilities  Compilers  Help
 ───────────────────────────────────────────────────────────────────────────────
 DSLIST - Data Sets Matching HANKO3                                 Row 1 of 14
 Command - Enter "/" to select action                 Dsorg  Recfm  Lrecl  Blksz
 -------------------------------------------------------------------------------
          HANKO3
          HANKO3.DDIR                                  VS
          HANKO3.DDIR.D                                VS    ?          ?      ?
          HANKO3.DDIR.I                                VS    ?          ?      ?
          HANKO3.EXEC                                  PO-E  FB        80  32720
          HANKO3.ISD1.ISPF.ISPPROF                     PO    FB        80  27920
          HANKO3.ISD1.ISPVCALL.TRACE                   PS    FB        80  27920
          HANKO3.ISPF.ISPPROF                          PO    FB        80   6160
          HANKO3.ISPVCALL.TRACE                        PS    FB        80  27920
          HANKO3.LOAD                                  PO    U          0  32760
          HANKO3.MAKEDSNS.OUTPUT                       PS    FB        80  27920
          HANKO3.SYS2.BRODCAST                         PS    FB       150   1500
          HANKO3.TASID.SNAPSHOT                        PS    VBA      255  27998
          HANKO3.TEST                                  PS    VBA      138  13800
 ***************************** End of Data Set list ****************************
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 88. Data set list - attributes view (ISRUDSL0)
4
Total Changes the display to the Total view. 
   Menu  Options  View  Utilities  Compilers  Help
 ───────────────────────────────────────────────────────────────────────────────
 DSLIST - Data Sets Matching HANKO3                                 Row 1 of 14
 Total Tracks:          86 non-x:         86   Data Sets:     14 non-x:     14
 -------------------------------------------------------------------------------
 Command - Enter "/" to select action                  Message           Volume
       Tracks  %     XT Device  Dsorg Recfm Lrecl Blksz  Created    Referred
 -------------------------------------------------------------------------------
          HANKO3                                                         *ALIAS
 -------------------------------------------------------------------------------
          HANKO3.DDIR                                                    *VSAM*
                                 VS
 -------------------------------------------------------------------------------
          HANKO3.DDIR.D                                                  D$US50
           45   ?     1 3390     VS   ?         ?     ? 2007/02/21 2007/02/21
 -------------------------------------------------------------------------------
          HANKO3.DDIR.I                                                  D$US50
            1   ?     1 3390     VS   ?         ?     ? 2007/02/21 ***None***
 -------------------------------------------------------------------------------
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 89. Data set list - total view (ISRUDSL0)
   Menu  Options  View  Utilities  Compilers  Help
  ───────────────────────────────────────────────────────────────────────────────
 DSLIST - Data Sets Matching HANKO3                                 Row 1 of 14
 Total Tracks:          86 non-x:         86   Data Sets:     14 non-x:     14
 -------------------------------------------------------------------------------
 Command - Enter "/" to select action                  Message           Volume
       Tracks  %     XT Device  Dsorg Recfm Lrecl Blksz  Created    Referred
           Catalog
 -------------------------------------------------------------------------------
          HANKO3                                                         *ALIAS
           CATALOG.MASTER.SYSPLEXD
 -------------------------------------------------------------------------------
          HANKO3.DDIR                                                    *VSAM*
                                 VS
           CATALOG.USER1.SYSPLEXD
 -------------------------------------------------------------------------------
          HANKO3.DDIR.D                                                  D$US50
           45   ?     1 3390     VS   ?         ?     ? 2007/02/21 2007/02/21
           CATALOG.USER1.SYSPLEXD
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 90. Data set list - total view with catalog name (ISRUDSL0)
Data set list utility (option 3.4)
136  z/OS: z/OS ISPF User's Guide Vol II

## Page 175

5
Sort You can sort the data set list by any of these fields:
1. Name
2. Message
3. Volume
4. Tracks
5. Percent Used
6. Extents
7. Dsorg
8. Recfm
9. Lrecl
10. Blksz
11. Creation date
12. Expiration date
13. Referenced date
14. Device
15. Volume indicator
16. Catalog
You can also specify the sort sequence (ascending or descending) or accept the default sequence
for the associated sort field. By default, character fields are sorted alphabetically and numeric
fields are sorted in descending order.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Compilers
The Compilers pull-down offers you these choices:
1
Foreground Compilers. Displays the Foreground Selection Panel.
2
Background Compilers. Displays the Batch Selection Panel.
3
ISPPREP Panel Utility  Displays the Preprocessed Panel Utility panel.
4
DTL Compiler  Displays the ISPF Dialog Tag Language Conversion Utility panel.
Help
The Help pull-down provides general information about the data set list, including the format of the
displayed list and the available line commands and primary commands.
Data set list panel fields 
The fields listed here can appear on the data set list panels. The fields displayed will vary depending on
the view that you select.
Total Tracks
Total number of tracks of all data sets in the list.
non-x
Total number of tracks of all data sets in the list, not including the tracks of all excluded data sets.
When the number of total tracks or total non-excluded tracks exceeds 10 digits (the maximum
provided in the header line), the display changes as follows:
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  137

## Page 176

nnnnnnnnkB
Kilobyte. The total number is divided by 1000.
nnnnnnnnMB
Megabyte. The total number is divided by 1,000,000.
nnnnnnnnGB
Gigabyte. The total number is divided by 1,000,000,000.
nnnnnnnnTB
Terabyte. The total number is divided by 1,000,000,000,000.
The unchanged number of tracks and non-excluded tracks is available in the shared pool variables
ZDLSIZET and ZDLSIZTX .
Data Sets
Total number of data sets in the list.
non-x
Total number of data sets in the list, not including excluded data sets.
Command
Field used to enter a line command, TSO command, CLIST, or REXX EXEC when displaying a data set
list. See “Data set list utility line commands” on page 147 for more information.
Name
Data set name, as in the VTOC or catalog.
Message
This field is initially blank. After you perform an operation on a data set using one of the built-in line
commands, one of these messages is displayed in this field:
LineCommand
Message
B
Browsed
C
Cataloged
E
Edited
U
Uncataloged
D
Deleted
P
Printed
PX
Index Printed
R
Renamed
I
Info - I
M
Member List
S
Info - S
Z
Compressed
Data set list utility (option 3.4)
138  z/OS: z/OS ISPF User's Guide Vol II

## Page 177

F
Free Completed
=
(message shown for last command entered)
V
Viewed
RA
Refadd
CO
Copied
MO
Moved
RS
Reset
X
– 1 data set(s) not displayed
NX
(no message)
NXF
(no message)
NXL
(no message)
If you enter a TSO command, CLIST, or REXX exec on the Command line, a default message appears in
the Message field. The message you see can be one of these:
• In this format, depending on the results of the TSO command, CLIST, or REXX exec:
XXXXXXXX  RC=#
where:
XXXXXXXX
The command entered
#
The return code.
• "ERROR MSG LOGGED".
This may occur with PDSE or HFS data sets. A fully formatted message appears in the ISPF log,
provided one has been allocated.
Note: See “Data set list utility line commands” on page 147 for a description of the Data Set List
Utility line commands.
Volume
Volume serial number.
An indicator may be displayed beside the Volume field:
+ (plus sign)
May be displayed beside the Volume serial field if the data set is a multiple volume data set. This
is determined from the number of volume entries in the catalog. Depending on the system set-up,
a "+" may not be displayed until the additional volumes have been accessed. For example, a data
set with a non-zero dynamic volume count in the SMS dataclass will not show multiple volume
entries in the catalog until the additional volumes have been accessed. Other vendor products
which can dynamically expand the volume list will also not show multiple volume entries in the
catalog until the additional volumes have been accessed.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  139

## Page 178

1
Migrated to disk
2
Migrated to tape
C
Migrated to cloud
Tracks
Number of tracks allocated to the data set.
%Used
Percentage of allocated tracks used, expressed in whole numbers, not rounded. If any track is used,
the minimum percentage is 1. If the data set is a PDSE, the % refers to the percentage of allocated
pages used.
See “F — free unused space” on page 153 for information about freeing track space manually.
Note: Space utilization values are not displayed for VSAM or BDAM data sets.
XT
Number of extents allocated to the data set.
Device
Device type on which the volume that contains the data set is mounted.
Dsorg
One of the data set organizations shown. In the definitions of these data set organizations, unmovable
means the data set contains absolute addresses instead of relative addresses. These data sets are not
moved to any other DASD storage location during read/write operations.
PS
Sequential
PS-E
Sequential Extended Format
PS-L
Large Format Sequential
PSU
Sequential unmovable
PO
Partitioned
POU
Partitioned unmovable
PO-E
Partitioned extended (PDSE)
DA
Direct
DAU
Direct unmovable
HFS
MVS Hierarchical File System
VS
VSAM
VS-E
VSAM Extended Format
blank
None of the preceding data set organizations.
Data set list utility (option 3.4)
140  z/OS: z/OS ISPF User's Guide Vol II

## Page 179

Recfm
Record format specified when the data set was allocated. See “A — allocate new data set” on page
102 for more information about record formats.
Lrecl
Logical record length, in bytes, specified when the data set was allocated.
Blksz
Block size, in bytes, specified when the data set was allocated.
Created
Creation date in the national format.
Expires
Expiration date in the national format, specified when the data set was allocated. If no expiration
date was specified, ***None*** is displayed. If a "never expire" date (1999/12/31 or its equivalent) is
specified, ***Perm*** is displayed. See “A — allocate new data set” on page 102 for more information
about expiration dates.
Note: The expiration date is only available with the I and S line commands.
Referred
Date, in the national format, that this data set was last accessed.
Catalog
The name of the catalog in which the data set was located. Only displayed in the Total view when the
Display Catalog Name option is selected and no value is entered in the Volume Serial field.
Actions you can take from the data set list panel
These topics describe actions you can take from the Data Set List panel:
• “Line commands” on page 141
• “TSO commands, CLISTs, and REXX EXECs” on page 141
• “Using the slash ( / ) character” on page 141
• “TSO command/CLIST/REXX exec variables” on page 143
Line commands
Line commands can be entered in the Command field to the left of the data set names. See “Data set list
utility line commands” on page 147 for definitions of these line commands.
TSO commands, CLISTs, and REXX EXECs
Besides the ISPF-supplied line commands, you can also enter TSO commands, CLISTs, and REXX EXECs
that use a fully qualified data set name as an operand. You can type over the field containing the data
set name to enter commands that require more space than is provided in the Command field. ISPF
determines the end of the command by scanning the Command field and the field containing the data set
name from right to left. The first character found that differs from the original is considered to be the last
character of the command. Therefore, it is best to enter a blank after the last character of your command
if it extends into the field containing the data set name.
TSO commands, CLISTs, and REXX EXECs entered are invoked using the ISPF SELECT CMD service.
Variable names starting with an ampersand (&) are evaluated by ISPF. If you want the underlying
command processor to see the ampersand you must specify 2 ampersands. For example:
DEF NONVSAM(NAME(/) DEVT(0000) VOLUME(&&SYSR2))
Note: If the TSO command, CLIST, or REXX exec issues a return code greater than or equal to 8,
processing stops and an error message is displayed.
Using the slash ( / ) character
If a command, CLIST, or REXX exec requires the data set name in a position other than the first operand or
if other operands are needed, you can use the slash ( / ) character to represent the quoted data set name.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  141

## Page 180

If no operands are specified after the command, ISPF uses the name of the data set being acted on as the
command's first operand.
To specify a member of a partitioned data set, enclose the member name or pattern in parentheses
immediately following the / character. You can use this format with the V (view data set), B (browse data
set), D (delete data set), E (edit data set), and M (display member list) line commands. For information
about these line commands, see “Data set list utility line commands” on page 147.
You may find it helpful to call the SHOWCMD primary command before using the slash ( / ) for the first
time. After you call SHOWCMD, a special Data Set List Utility panel appears each time you enter a line
command, TSO command, CLIST, or REXX exec on a data set list display. The panel shows you the
command you entered and how ISPF expanded, and thus interpreted, that command. See “SHOWCMD
command” on page 160 for more information about and an example of the SHOWCMD primary command.
Rules for substituting data set names in line commands
The rules shown apply to substituting the slash ( / ) character for a data set name or adding the data set
name as the last operand. Each rule is followed by one or more examples that prove the rule by using
either a CLIST or a line command.
In each example, the data set being acted on is USER.TEST.DATA, which always appears, either
completely or partially, in uppercase. However, the CLIST or line command is typed in lowercase to
differentiate between the CLIST or line command and USER.TEST.DATA when this data set name is either
completely or partially typed over.
Each example also shows:
Original
The line as it appears before the CLIST or line command is entered.
As typed
The line as it appears after the CLIST or line command is typed.
After
The line as it appears after the CLIST or line command is expanded to show the placement of quotes
and data set name substitution for the slash (/) character.
1. You can type over the data set name. Expanded commands can contain a maximum of 255 characters
and are converted to uppercase. This example shows how rule “1” on page 142 would apply if you
typed %clist1 da(/):
(Original)           USER.TEST.DATA
(As typed)  %clist1 da(/).TEST.DATA
(After)     %CLIST1 DA('USER.TEST.DATA')
2. The data set name substitution character (/) is replaced with the quoted, fully qualified data set name
if the character following the / is not a number, letter, or national character. This example shows how
rule “2” on page 142 would apply if you typed %clist2 / newdate(1986/03/15):
(Original)           USER.TEST.DATA
(As typed)  %clist2 / newdate(1986/03/15)
(After)     %CLIST2 'USER.TEST.DATA' NEWDATE(1986/03/15)
3. If a slash ( / ) is followed immediately by a member name in parentheses, the ending quote for the data
set is placed after the closing parenthesis that follows the member name. This example shows how
rule “3” on page 142 would apply if you typed %clist3 da(/(xyz)):
(Original)           USER.TEST.DATA
(As typed)  %clist3 da(/(xyz)).DATA
(After)     %CLIST3 DA('USER.TEST.DATA(XYZ)')
Data set list utility (option 3.4)
142  z/OS: z/OS ISPF User's Guide Vol II

## Page 181

4. If the first operand is the unquoted data set name as it appears in the list, quotes are added around
it or after a closing parenthesis following a member name. This example shows how rule “4” on page
143 would apply if you typed b (the B (browse) line command) and added member (abc):
(Original)           USER.TEST.DATA
(As typed)  b        USER.TEST.DATA(abc)
(After)     B 'USER.TEST.DATA(ABC)'
5. If the line command does not have any operands or if the data set name has not been substituted as
specified by either rule “3” on page 142 or rule “4” on page 143, the quoted, fully qualified data set
name is added to the end of the line command. This example shows how rule “5” on page 143 would
apply if you typed %clist4 user.test.fortran:
(Original)           USER.TEST.DATA
(As typed)  %clist4 user.test.fortran
(After)     %CLIST4 USER.TEST.FORTRAN 'USER.TEST.DATA'
This example shows how rule “5” on page 143 would apply if you typed %clist4
'user.test.fortran'. The purpose of this example is to show that if you enclose the CLIST
operand in quotes, ISPF still puts quotes around the data set name being acted on. The results are the
same.
(Original)           USER.TEST.DATA
(As typed)  %clist4 'user.test.fortran'
(After)     %CLIST4 'USER.TEST.FORTRAN' 'USER.TEST.DATA'
This example shows how rule “5” on page 143 would apply if you typed %clist5 member1(abc).
The purpose of this example is to show that the results do not change if the CLIST operand contains a
member name enclosed in parentheses.
(Original)           USER.TEST.DATA
(As typed)  %clist5 member1(abc)ATA
(After)     %CLIST5 MEMBER1(ABC) 'USER.TEST.DATA'
This example shows how rule “5” on page 143 would apply if you partially over typed the data set
name to get a new data set name. Adding the quotation marks fully qualifies the new data set name.
(Original)          USER.TEST.DATA
(As typed)  al 'USER.TEmp.DATA'
(After)     AL 'USER.TEMP.DATA' 'USER.TEST.DATA' 
If quotation marks are not used, the operand is truncated at the last changed character.
(Original)          USER.TEST.DATA
(As typed)  al USER.TEmp.DATA
(After)     AL USER.TEMP 'USER.TEST.DATA'
TSO command/CLIST/REXX exec variables
If you use a TSO command, CLIST, or REXX exec, ISPF puts the variables described in Table 9 on page
143 in the shared pool for the TSO command, CLIST, or REXX exec to use. 
Table 9. TSO command/CLIST/REXX exec variables (output)
Variable Name Description
Length in
Characters
ZDLBLKSZ Data set block size 5
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  143

## Page 182

Table 9. TSO command/CLIST/REXX exec variables (output) (continued)
Variable Name Description
Length in
Characters
ZDLCAT Cataloged status; one of these:
0
Data set is cataloged on volume ZDLVOL.
2
Data set is cataloged on a volume other than ZDLVOL and is
either:
• on volume ZDLCAT but uncataloged
• on volume ZDLCAT and defined in a user catalog that is
connected to the master catalog, but not in the normal
catalog search path
The name of the user catalog is in ZDLCATNM.
4
Data set is uncataloged on volume ZDLVOL.
6
Data set is not cataloged on any volume and is uncataloged
on volume ZDLVOL.
8
Data set is not available on volume ZDLVOL. This status is
returned for data sets that have been either migrated or
deleted.
(1)
ZDLCATNM Name of the catalog in which the data set was located 44
ZDLCDATE Creation date 10
ZDLCMD Line command 9
ZDLCONF Delete confirmation (Y | N) 1
ZDLDEV Device type 8
ZDLDSN Data set name 44
ZDLDSNTP Data set name type 8
ZDLDSORG Data set organization 4
ZDLEDATE Expiration date 10
ZDLEXT Number of extents used 3
ZDLEXTX Number of extents used, long format 5
ZDLLCMD Expanded line command 255
ZDLLRECL Data set logical record length 5
ZDLMIGR Whether the data set is migrated (YES or NO) 3
ZDLMVOL Multivolume indicator 1
Data set list utility (option 3.4)
144  z/OS: z/OS ISPF User's Guide Vol II

## Page 183

Table 9. TSO command/CLIST/REXX exec variables (output) (continued)
Variable Name Description
Length in
Characters
ZDLOVF Whether variables ZDLEXTX and ZDLSIZEX should be used to
obtain the 'number of extents used' and 'data set size in tracks'
values (YES or NO). The value is YES when the 'number of
extents used' value exceeds the size of variable ZDLEXT or
the 'data set size in tracks' value exceeds the size of variable
ZDLSIZE.
3
ZDLRDATE Date last referenced 10
ZDLRECFM Data set record format 5
ZDLSIZE Data set size in tracks 6
ZDLSIZEX Data set size in tracks, long format 12
ZDLSPACU Space units: either BLOCKS, TRACKS, CYLINDERS, BYTES,
KILOBYTES, or MEGABYTES
10
ZDLUSED Percentage of used tracks 3
ZDLVOL Volume 6
ZDLXSTAT Exclude status 1
When you select the Display Total Tracks option, and the data set list is displayed either in SPACE view or
in TOTAL view, ISPF also puts the variables described in Table 10 on page 145 in the shared pool for the
TSO command, CLIST, or REXX exec to use. 
Table 10. TSO command/CLIST/REXX exec additional variables (output)
Variable Name Description
Length in
Characters
ZDLSIZET Total tracks of all data sets in the list 19
ZDLSIZTX Total tracks of all data sets in the list, not including the tracks of
excluded data sets
19
ZDLDST Total number of data sets in the list (available for all display
views)
6
ZDLDSX Total number of data sets in the list, not including the excluded
data sets
6
Note: ISPF cannot calculate reliable space utilization values for VSAM or BDAM data sets. Therefore,
question marks (?) are returned in variables that report space utilization for these data sets.
A TSO command, CLIST, or REXX exec can set these variables and place them in the shared pool to
communicate with the Data Set List utility (option 3.4). 
Table 11. TSO command/CLIST/REXX exec variables (input)
Variable Name Description
Length in
Characters
ZDLNDSN New data set name to appear in list 44
ZDLMSG Message to appear in list 16
ZDLREF Refresh data set information; Y | N 1
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  145

## Page 184

P — print data set list
Use option P to print a data set list. You must:
1. Enter one of these:
• One or more data set name level qualifiers in the Dsname Level field and a volume serial in the
Volume field. The list will contain all data sets for the specified levels and volume. Only the specified
volume is searched. See “Dsname Level” and “Volume serial” on page 129 for more information.
• One or more data set name level qualifiers in the Dsname Level field, but leave the Volume field
blank. The list will contain all data sets for the specified levels that are cataloged.
• A volume serial in the Volume field, but leave the Dsname Level field blank. The list will contain only
the data sets on the specified volume. Only the specified volume is searched.
Note: All data set lists are formatted the same when they are printed. Therefore, values entered in the
Initial View field have no effect when you use option P.
2. Press Enter to print the data set list. The data set list is stored in the ISPF list data set.
ISPF displays a progress status pop-up panel when the necessary information to perform a P (print data
set list) command has to be retrieved and the data set list comprises 50 or more data sets. The keyboard
locks when this pop-up panel appears and stays locked until the P command is completed. This happens
regardless of the setting of Display Total Tracks option and the value entered in the Initial View field.
V — display VTOC information
Option V is used to display VTOC (volume table of contents) information. To use option V:
1. In the Volume field, specify the volume serial for which you want ISPF to display information.
Note: VTOC information is formatted the same, whether displayed or printed. Therefore, values
entered in the Initial Display View field have no effect when you use option V.
2. Press Enter to display the VTOC information.
Note: The Dsname Level field is not applicable for the V or PV command. Only the Volume field is
relevant.
Figure 91 on page 146 shows an example of a VTOC display.
   Menu  RefList  RefMode  Utilities  Help
┌────────────────────────── VTOC Summary Information ──────────────────────────┐
│ Volume . : MVS8WF                                                            │
│                                                                              │
│ Unit . . : 3390                                                              │
│                                                                              │
│  Volume Data             VTOC Data              Free Space  Tracks   Cyls    │
│  Tracks . :   50,085     Tracks  . :      59    Size  . . :   1,146       1  │
│  %Used  . :       97     %Used . . :      60    Largest . :      22       0  │
│  Trks/Cyls:       15     Free DSCBS:   1,187                                 │
│                                                 Free Extents . :     323     │
│                                                                              │
│ Command ===>                                                                 │
│  F1=Help    F2=Split   F3=Exit    F9=Swap   F12=Cancel                       │
⋘───────────────────────────────────────────────────────────────────────────────┘
       3. Attrib                 /  Include Additional Qualifiers
       4. Total                  /  Display Catalog Name
                                 /  Display Total Tracks
 When the data set list is displayed, enter either:
   "/" on the data set list command field for the command prompt pop-up,
   an ISPF line command, the name of a TSO command, CLIST, or REXX exec, or
      "=" to execute the previous command.
 Option ===> V
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 91. VTOC summary information panel (ISRUDSLV)
Track values do not include the remaining alternate tracks for the volume. The free space track values are
the number of tracks for the free cylinders plus any additional free tracks.
The fields shown on the VTOC Summary Information panel are:
Data set list utility (option 3.4)
146  z/OS: z/OS ISPF User's Guide Vol II

## Page 185

Unit
Shows the type of DASD device the volume is on, such as 3380 or 3390.
Volume Data
Describes general information about the volume:
Tracks
Total tracks on the volume.
%Used
Percentage of total tracks or pages not available for allocation.
Trks/Cyls
Number of tracks per cylinder for this volume.
VTOC Data
Describes general information about the VTOC on the volume:
Tracks
Total tracks allocated to the VTOC.
%Used
Percentage of allocated tracks or pages used by data set control blocks (DSCBs).
Free DSCBS
Number of unused DSCBs.
Free Space
Describes the free space available for data set allocation on the volume under the headings Tracks
and Cyls, showing:
Size
Total number of free tracks and cylinders.
Largest
The largest number of contiguous free tracks and cylinders.
Free Extents
The number of free areas with free cylinders.
PV — print VTOC information
Option PV is used to print VTOC information. To use option PV:
1. Blank out the Dsname Level field.
2. In the Volume field, specify the volume serial for which you want ISPF to print information.
Note: VTOC information is formatted the same, whether displayed or printed. Therefore, values
entered in the Initial View field have no effect when using option PV.
3. Press Enter to print the VTOC information. The VTOC information is stored in the ISPF list data set.
Note: The Dsname Level field is not applicable for the PV or V command. Only the Volume field is
relevant.
Data set list utility line commands
This section documents the line commands that you can enter in the Data Set List Utility when a data set
list is displayed. For information on the line commands that you can enter in the Data Set List Utility when
a member list is displayed, see the information about Using Member Selection Lists and Library and Data
Set List Utility Line Commands in the ISPF Libraries and Data Sets topic in z/OS ISPF User's Guide Vol I.
After you display a data set list by leaving the Option field blank, you can enter a line command to the left
of the data set name. You can also enter TSO commands, CLIST names, or REXX exec names. If a '>' is
used before the CLIST or REXX exec name, the parameters passed to the command are not translated to
upper case. The z/OS UNIX commands OGET and OPUT can be entered and the parameters are also not
translated to upper case.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  147

## Page 186

The slash ( / ) character, which can be used with TSO commands, CLISTs, and REXX EXECs, can also
be used with the B (browse data set), CO (copy data set), D (delete data set), E (edit data set), M
(display member list), MO (move data set), and V (view data set) line commands to specify a member
name or a pattern. You can type over the field containing the data set name to enter commands that
require more than the space provided. For more information about using this symbol, see “Using the slash
( / ) character” on page 141. For more information about member name patterns, see the details about
Displaying Member Lists in the "ISPF Libraries and Data Sets" chapter in the z/OS ISPF User's Guide Vol I.
You can also enter line commands in block command format to execute the same line command for
several data sets at once. You mark the block by typing a "/ /" at the beginning of a block of rows and
another "/ /" at the end of the block of rows. You must type the line command either immediately after
the / / on the first row of the block, or immediately after the / / on the last row of the block. You can
enter several blocks of commands at the same time, but you cannot nest them. Single line commands
are not allowed within a block command. You can execute all line commands, including TSO commands,
Clists and REXX execs as block commands. If you have selected the DSLIST settings option Execute
Block Commands for excluded Data Sets, all applicable excluded rows are unexcluded before the block
commands are executed.
Line commands that are valid for aliases may be used with any alias data sets that are listed. Uncatalog,
delete, and rename commands are not valid for alias data sets. A line command such as 'B' for browse or
'I' for information will display the real name of the data set.
The Data Set List Utility always supports the U (uncatalog) line command for tape data sets. The Data Set
List Utility can support additional line commands for data sets stored on tape and other removable media,
by calling external commands such as DFSMSrmm. This interface is configured in the ISPF configuration
table and enabled by setting the configuration table keyword DSLIST_RM_ENABLED to YES.
Depending on the removable media interface, these line commands may be supported:
I
Information
S
Short Information
D
Delete
R
Rename
C
Catalog
M
Member List
P
Print
X
Print Index
CO
Copy
MO
Move
Which line commands are actually supported by a particular interface depends on the capabilities of the
external command.
For more information about configuring the Data Set List Utility removable media interface, see z/OS ISPF
Planning and Customizing.
If a CLIST, REXX exec, or program is issued against a data set, ISPF gathers information on the data set
and makes it available through dialog variables. See Table 9 on page 143 for the list of those variables. If
the data set being processed is on an unmounted file system, a temporary mount is issued, file system.
Data set list utility (option 3.4)
148  z/OS: z/OS ISPF User's Guide Vol II

## Page 187

The Command field and the field containing the data set name fields make up a single point-and-shoot
field. If you enter a slash in the Command field or if you select any part of the combined point-and-shoot
field, the Data Set List Actions pop-up shown in Figure 92 on page 149 is displayed so that you can select
the command you want to use.
   Menu  Options  View  Utilities  Compilers  Help
 ─ ┌───────────────────────────────────────────────────────────────┐ ──────────
 D │                    Data Set List Actions                      │ ow 1 of 12
   │                                                  More:        │
   │ Data Set: BILLSWA                                             │
 C │                                                               │     Volume
 - │ DSLIST Action                                                 │ -----------
 / │     1.  Edit                     15. Reset                    │     *ALIAS
   │     2.  View                     16. Move                     │     A$US05
   │     3.  Browse                   17. Copy                     │     A$US08
   │     4.  Member List              18. Refadd                   │     A$US03
   │     5.  Delete                   19. Exclude                  │     A$US07
   │     6.  Rename                   20. Unexclude 'NX'           │     A$US12
   │     7.  Info                     21. Unexclude first 'NXF'    │     A$US02
   │     8.  Short Info               22. Unexclude last 'NXL'     │     A$US07
   │     9.  Print                    23. SuperC 'SC'              │     A$US06
   │     10. Catalog                  24. SuperCE 'SCE'            │     A$US08
   │     11. Uncatalog                25. Search-For 'SF'          │     A$US08
   │     12. Compress                 26. Search-ForE 'SFE'        │     A$US06
   │     13. Free                     27. Allocate                 │     A$US01
   │     14. Print Index                                           │     A$US08
   │                                                               │     A$US06
   │ Select a choice and press ENTER to process data set action.   │     A$US01
   │  F1=Help        F2=Split       F3=Exit        F7=Backward     │
 C │  F8=Forward     F9=Swap       F12=Cancel                      │  ===> PAGE
   ⋘───────────────────────────────────────────────────────────────┘ 9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 92. Data set list commands pop-up (ISRUDABC)
E — edit data set
For the E command, the processing is similar to selecting the Edit option (2) and entering the library or
data set name on the Edit Entry Panel, except that mixed mode is the assumed operation mode.
Note: Edit is not allowed for multivolume data sets from a VTOC list.
If you select a library or other partitioned data set, an Edit member list is displayed. For more information
about using member selection lists, see the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's
Guide Vol I.
The E line command uses the values from a prompt panel to specify items including the initial macro,
profile name, panel name, format, and mixed mode editing. These values are stored in the profile and are
used on subsequent edits.
To change these values or other edit settings, use the "DSLIST settings" panel on the data set list Options
pull-down. Check both the "Enhanced member list for Edit, View, and Browse" and the "Display Edit/View
entry panel" options. The prompt panel is always shown when you edit a sequential file, or when you
directly edit a member of a partitioned data set. When you are using a member list you can force the
display of the panel by placing a slash mark (/) in the Prompt field next to the member you select.
If the editor appears to be invoking an unexpected initial macro, or it appears to be using an unexpected
profile, follow the process described to check the values on the prompt panel.
V — view data set
For the V command, the processing is similar to selecting the View option (1) and entering the library or
data set name on the View Entry Panel. If you have set your DSLIST options to not show the edit/view
entry panel:
• Mixed mode is the assumed operation mode.
• You cannot specify a data set format, an edit profile, or an initial macro.
Note: From a VTOC list, you can view a single volume of a multivolume non-SMS data set.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  149

## Page 188

If you select a library or other partitioned data set, a View member list is displayed. For more information
about using member selection lists, see the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's
Guide Vol I.
The V line command uses the values from a prompt panel to specify items including the initial macro,
profile name, panel name, format, and mixed mode. These values are stored in the profile and are used on
subsequent edits.
To change these values or other view settings, use the "DSLIST settings" panel on the data set list Options
pull-down. Check both the "Enhanced member list for Edit, View, and Browse" and the "Display Edit/View
entry panel" options. The prompt panel is always shown when you view a sequential file, or when you
directly view a member of a partitioned data set. When you are using a member list you can force the
display of the panel by placing a slash mark in the Prompt field next to the member you select.
If view appears to be invoking an unexpected initial macro, or it appears to be using an unexpected
profile, follow the process described to check the values on the prompt panel.
B — browse data set
For the B command, processing is the same as if you specify Browse Mode from View (option 1), except
that mixed mode is the assumed operation mode and you cannot specify a data set format.
If you select a library or other partitioned data set, a Browse member list is displayed. For more
information about using member selection lists, see the "ISPF Libraries and Data Sets" chapter of the
z/OS ISPF User's Guide Vol I.
To change the mixed mode or the PDSE generations value, use the "DSLIST settings" panel on the data
set list Options pull-down. Check both the "Enhanced member list for Edit, View, and Browse" and the
"Display Browse entry panel" options. The prompt panel is shown when you:
• Browse a member list of a PDSE version 2 data set, and force the display of the panel by placing a slash
mark in the Prompt field next to a member you select which has non-current generations.
• Directly browse a member of a PDSE version 2 data set which has non-current generations.
• Use a DBCS code page and browse a member list of a PDS, and force the display of the panel by placing
a slash mark in the Prompt field next to a member you select.
• Use a DBCS code page and browse a sequential data set or directly browse a member of a partitioned
data set.
Note: From a VTOC list, you can browse a single volume of a multivolume non-SMS data set.
M — display member list
For the M command, a member selection list of a partitioned data set is displayed. This member list
provides an expanded line command field in the area to the left of the list. The line command field in other
member lists has room for only one character, unless the browse, view, or edit enhanced member list is
selected.
From the member list, you can use the same primary commands and line commands that are valid for
Library utility (option 3.1) member selection lists. See the information about Using Member Selection Lists
and Library and Data Set List Utility Line Commands in the Libraries and Data Sets topic in the z/OS ISPF
User's Guide Vol I.
Note:
1. From a VTOC list, you can browse a single volume of a multivolume non-SMS data set.
2. You can chain the line commands; that is, you can select multiple members from a member list for
various processing tasks. Use the CANCEL command (from a View, Browse, or Edit session) to break
the chain and return to the member list.
You can also enter TSO commands, CLISTs, or REXX EXECs in the Line Command field. If you enter a
line command other than B (browse member), C (copy member), D (delete member), E (edit member),
G (reset member statistics), I (display member information), J (submit member), M (move member), N
Data set list utility (option 3.4)
150  z/OS: z/OS ISPF User's Guide Vol II

## Page 189

(display member generation list), P (print member), R (rename member), T (invoke TSO command for
member), or V (view member), ISPF interprets it as a TSO command, CLIST, or REXX EXEC.
If the prompt field contains non-blank data that does not start with "*" then the prompt field data is
passed as an argument:
COMMAND 'DSN(MEMBER)' prompt
See “TSO commands, CLISTs, and REXX EXECs” on page 141 for more information.
Note: If the TSO command, CLIST, or REXX exec issues a return code greater than or equal to 8,
processing stops and an error message is displayed.
Figure 93 on page 151 shows an example of a member list with statistics and an expanded line command
field.
   Menu  Functions  Confirm  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
 DSLIST            MYPROJ.DEV.SOURCE                     Row 0000001 of 0000373
 Command ===>                                                  Scroll ===> PAGE
            Name     Prompt       Size   Created          Changed          ID
 _________ FL@SPCGB                 21  2003/12/10  2003/12/10 02:58:01 LSACKV
 _________ FL@SPCIM                 21  2003/12/15  2003/12/15 09:37:51 LSACKV
 _________ FL@SPCLO                 21  2003/12/05  2003/12/05 22:52:24 LSACKV
 _________ FL@SPCMI                 21  2003/12/10  2003/12/10 06:22:13 LSACKV
 _________ FL@SPCNG                 21  2003/12/01  2003/12/02 23:09:25 LSACKV
 _________ FL@SPCPR                 21  2003/12/12  2003/12/12 01:46:48 LSACKV
 _________ FL@SPCRA                 21  2003/12/12  2003/12/12 04:03:30 LSACKV
 _________ FL@SPCSC                 23  2004/04/21  2005/12/23 11:54:27 BBAGG
 ⋮
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 93. Member list display - expanded line command field  (ISRUDSM)
Figure 94 on page 151 shows load module library statistics with an expanded line command field.
   Menu  Functions  Confirm  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
 DSLIST            PDFTDEV.SVT.LOAD                      Row 0000001 of 0000505
 Command ===>                                                  Scroll ===> PAGE
            Name     Prompt        Alias-of     Size      TTR     AC   AM   RM
 _________ FLM$CP                  FLMIO24    0000A3E8   089B0F   00    24   24
 _________ FLM$CPI                            000000E8   00F80A   00    31  ANY
 _________ FLM$DE                  FLMIO24    0000A3E8   089B0F   00    24   24
 _________ FLM$DT                  FLMIO24    0000A3E8   089B0F   00    24   24
 _________ FLM$99                  FLMIO24    0000A3E8   089B0F   00    24   24
 _________ FLMA                               00008278   076E0D   00    31  ANY
 _________ FLMB                               000AA8B8   084A10   00    31  ANY
 _________ FLMBCMD                 FLMDDL     00140A68   087906   00    31  ANY
 ⋮
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 94. Load module library display - expanded line command field  (ISRUDSM)
D — delete data set
For the D command, the processing is the same as if you had selected option D from the Data Set utility
(option 3.2) without specifying a volume serial. This command deletes and uncatalogs the entire data set.
If a member name or pattern is supplied then a member delete will occur.
Note: Delete is not allowed for multivolume data sets from a VTOC list.
If you select the Confirm Delete option on the Data Set List Utility panel, the Confirm Delete panel (Figure
73 on page 112) is displayed to allow you to continue or cancel the operation. Note that Confirm Delete
is forced on when deleting members by pattern. If you are trying to delete an unexpired data set, the
Confirm Purge panel (Figure 74 on page 113) is also displayed.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  151

## Page 190

When you delete a data set the volume name is compared to the volume name in the configuration table.
If the names match, the command specified in the configuration table is used in place of the ISPF delete
processing. This allows you to delete migrated data sets without first causing them to be recalled.
R — rename data set
For the R command, the processing is the same as if you had selected option R from the Data Set utility
(option 3.2). The Rename Data Set panel is displayed to allow you to specify the new name.
Note: Rename is not allowed for multivolume data sets from a VTOC list.
See “R — rename entire data set” on page 110 for more information.
I — data set information
For the I command, the processing is the same as if you had selected option I from the Library utility
(option 3.1) or left the Option field blank with the Data Set utility (option 3.2). See “I — data set
information” on page 92 and “Information for managed data sets” on page 95 for more information.
Note:
1. For multivolume data sets, options I and S display current allocation and utilization values that
represent totals from all volumes used.
2. Space utilization values are not displayed for VSAM or BDAM data sets.
S — information (short)
For the S command, the processing is the same as if you had selected option S from the Library utility
(option 3.1) or the Data Set utility (option 3.2). See “S — short data set information” on page 96 and
“Short information for managed data sets” on page 97 for more information.
Note:
1. For multivolume data sets, options I and S display current allocation and utilization values that
represent totals from all volumes used.
2. Space utilization values are not displayed for VSAM or BDAM data sets.
P — print data set
For the P command, the processing is the same as if you had selected option L from the Library utility
(option 3.1). This command formats the contents of a source data set for printing and records the output
in the ISPF list data set. It also produces an index listing, which appears at the beginning of the output.
Note: The Print command is not allowed for multivolume data sets from a VTOC list.
C — catalog data set
For the C command, the processing is the same as if you had selected option C from the Data Set utility
(option 3.2). See “C — catalog data set” on page 110 for more information.
Note: Multivolume data sets are always cataloged.
U — uncatalog data set
For the U command, the processing is the same as if you had selected option U from the Data Set utility
(option 3.2). See “U — uncatalog data set” on page 112 for more information.
Note: The U command is not supported for multivolume data sets.
Z — compress data set
For the Z command, the processing is the same as if you had selected option C from the Library utility
(option 3.1). This command recovers wasted space that was formerly occupied by deleted or updated
Data set list utility (option 3.4)
152  z/OS: z/OS ISPF User's Guide Vol II

## Page 191

members and is now available for use again. You do not need to compress a PDSE. If you use the Z
command on a PDSE, the data is not reorganized.
The Z command calls either the IEBCOPY utility or the compress request exit routine. See z/OS ISPF
Planning and Customizing for more information.
F — free unused space
For the F command, space that is not being used by the data set is released. For example, if a data set is
allocated with 100 tracks but is only using 60 tracks, the F command releases the 40 tracks that are not
being used.
Note: The F command for non-SMS multivolume data sets only releases space on the last volume written
to. Volumes after the last write position may still have unused allocated space after the command
completes.
However, if the data set has been allocated with CYLS (cylinders) specified as the space units, only the
tracks beyond the last cylinder used are freed. For example, if a data set occupies 1.2 of 3 allocated
cylinders, the F command frees all tracks beyond the last used cylinder, leaving 2 cylinders allocated.
PX — print index listing
For the PX command, the processing is the same as if you had selected option X from the Library utility
(option 3.1). The index listing is recorded in the ISPF list data set. See “X — print index listing” on page 92
for more information.
Note: The Print command is not allowed for multivolume data sets from a VTOC list.
RS — reset
For the RESET command, a panel is displayed that prompts you to reset or delete ISPF statistics, and to
enter a new user ID, version number, or modification level.
MO — move
For the MOVE command, a panel is displayed that prompts you for a library or data set name for the to
data set.
Note: How aliases are handled by the MO and CO line commands depends on how the Process member
aliases option is set. For more information see “Moving or copying alias entries” on page 125.
CO — copy
For the COPY command, a panel is displayed that prompts you for a library or data set name for the to
data set.
Note: How aliases are handled by the MO and CO line commands depends on how the Process member
aliases option is set. For more information see “Moving or copying alias entries” on page 125.
RA — RefAdd
For the REFADD command, you are provided with an interface to referral lists, where you can add a data
set and a volume to a Personal Data Set List.
X — exclude data set
For the EXCLUDE command, one data set from a data set list is excluded from the list.
NX — unexclude data set
For the Unexclude command, one data set, or a set of data sets that have been excluded from a data set
list are re-shown.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  153

## Page 192

NXF — unexclude first data set
For the UNEXCLUDE FIRST command, the first of a set of excluded data sets is re-shown.
NXL — unexclude last data set
For the UNEXCLUDE LAST command, the last of a set of excluded data sets is re-shown.
SC — SuperC
The SC command invokes the SuperC Compare Utility with the data set predefined in the "New" Data Set
field. These keyword parameters can be entered after the SC command:
NDSN(new data set)
NVOL(volume for NDSN)
ODSN(old data set)
OVOL(volume for ODSN)
M(member mask)
PROMPT
By default no prompting for SuperC information happens.
See “SuperC utility (option 3.12)” on page 183 for more information.
SCE — SuperCE
The SCE command invokes the SuperCE Compare Utility with the data set predefined in the New DS Name
field. These keyword parameters can be entered after the SCE command:
NDSN(new data set)
NVOL(volume for NDSN)
ODSN(old data set)
OVOL(volume for ODSN)
M(member mask)
PROMPT
By default no prompting for SuperC information happens.
See “SuperCE utility (option 3.13)” on page 192 for more information.
SF — Search-For
The SF line command invokes the Search-For Utility on the selected data set.
If the selected data set is a PDS or PDSE then the SRCHFOR Member List function is invoked. You can
provide a single search string with the SF line command. (Example: SF string1). If no search string is
provided the Srchfor Options popup window is displayed. Use this panel to enter multiple search strings,
process options, and output options. You can use the process options "Set EDIT FIND string" and "Set
BROWSE FIND string" to initialize the FIND string in Edit and Browse from the first SRCHFOR string. Use
the output option "Filter list" to list only the subset of members that contain one of the search strings.
An option E, V, or B can be entered immediately after the SF command. This will set the default action
(Edit, View, or Browse) for when the S line command is used to select a member in the enhanced member
list. (Example: SF B sets the default action in the member list to Browse.)
See “Search-For utility (option 3.14)” on page 203 for more information.
SFE — Search-ForE
The SFE line command invokes the Extended Search-For Utility on the selected data set.
Data set list utility (option 3.4)
154  z/OS: z/OS ISPF User's Guide Vol II

## Page 193

If the selected data set is a PDS or PDSE then the SRCHFOR Member List function is invoked. You can
provide a single search string with the SFE line command. (Example: SFE string1). If no search string is
provided the Srchfor Options popup window is displayed. Use this panel to enter multiple search strings,
process options, and output options. You can use the process options "Set EDIT FIND string" and "Set
BROWSE FIND string" to initialize the FIND string in Edit and Browse from the first SRCHFOR string. Use
the output option "Filter list" to list only the subset of members that contain one of the search strings.
An option E, V, or B can be entered immediately after the SFE command. This will set the default action
(Edit, View, or Browse) for when the S line command is used to select a member in the enhanced member
list. (Example: SFE B sets the default action in the member list to Browse.)
See “Search-ForE utility (option 3.15)” on page 209 for more information.
AL — Allocate
The AL line command uses a new data set name as a parameter. If no parameter is supplied, then the
displayed data set must have been previously deleted by another command. When a new data set name is
provided, then the displayed data set can be used as a model for allocation attributes.
= — repeat last command
For the = command, the most recently used line command is repeated. This command is most helpful
when the same TSO command, CLIST, or REXX EXEC is to be called for more than one data set in a data
set list. For example, suppose you have a CLIST named TESTABC and two data sets named USER.DATA1
and USER.DATA2. To run the CLIST with the two data sets consecutively from a data set list, you could:
1. Type TESTABC in the Command field beside USER.DATA1.
2. Type = in the Command field beside USER.DATA2.
3. Press Enter.
This procedure saves keystrokes because you type the CLIST name only once and you press Enter only
once.
Data set list utility primary commands
Primary commands are available when you use the Data Set List utility. These commands, which you enter
on the command line, are:
• APPEND
• CONFIRM
• EXCLUDE
• FIND and RFIND
• LC
• LOCATE
• MEMBER
• REFRESH
• RESET
• SAVE
• SHOWCMD
• SORT
• SRCHFOR
• VA, VS, VT, and VV
These topics describe these commands:
• “APPEND command” on page 156
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  155

## Page 194

• “CONFIRM command” on page 156
• “EXCLUDE command” on page 157
• “FIND and RFIND commands” on page 157
• “LC command” on page 158
• “LOCATE command” on page 158
• “MEMBER command” on page 158
• “REFRESH command” on page 159
• “RESET command” on page 159
• “SAVE command” on page 159
• “SHOWCMD command” on page 160
• “SORT command” on page 161
• “SRCHFOR command” on page 162
• “VA, VS, VT, and VV commands” on page 163
APPEND command
The APPEND primary command appends additional data sets to an existing displayed DSLIST. Use this
format:
APPEND
list_name
DSname_level
You can use the APPEND command with no parameters to get a list of your personal data sets. Then
select the one you want to append to the current list.
If you give a list_name with the command, the list given is appended.
By specifying DSname_level as a parameter, you can use the resulting list to select which list to append to
the current one. For example, entering APPEND Userid.C* gives you a list of all personal lists that begin
with C as the second-level identifier. Then you can select the one to append.
By specifying DSname_level in quotes, data sets beginning with DSname_level are appended to the data
set list.
The APPEND primary command accepts system symbols. For example:
APPEND 'SYS2.**.&SYSPLEX'
CONFIRM command
The CONFIRM primary command controls display of the Confirm Delete panel. Use this format:
CONFIRM
ON
OFF
You can use these operands with the CONFIRM command:
ON
Tells ISPF to display the Confirm Delete panel when you enter the D (delete data set) line command or
TSO DELETE command. This is the default setting.
OFF
Tells ISPF not to display the Confirm Delete panel.
For example, this command would tell ISPF not to display the Confirm Delete panel:
Data set list utility (option 3.4)
156  z/OS: z/OS ISPF User's Guide Vol II

## Page 195

CONFIRM OFF
EXCLUDE command
The EXCLUDE primary command excludes data sets from a list based on a character string. Use this
format:
EXCLUDE
character string ALL PREFIX NEXT
CHARS FIRST SUFFIX LAST WORD
PREV
You can use these operands with the EXCLUDE command:
character string
Tells ISPF which data set to exclude from the list.
ALL
Tells ISPF to exclude every data set in the list.
NEXT| FIRST | LAST | PREV
Operands that define the starting point, direction, and extent of the lines to exclude.
PREFIX | CHARS | SUFFIX | WORD
Operands that set the conditions for a character string match.
For example, this command tells ISPF to exclude a data set that includes BILBO3 in the name from a list:
EXCLUDE BILBO3
FIND and RFIND commands
The FIND primary command finds and displays a character string within the data set name. Use this
format:
FIND string
NEXT CHARS ALL PREFIX X
FIRST SUFFIX NX LAST WORD
PREV
For example, this command would tell ISPF to find all occurrences of the character string ELSE:
FIND ELSE ALL
The operands X and NX can be used to limit your search to excluded (X) or unexcluded (NX) data sets.
For more information about the operands used with this command, see “FIND—find character strings” on
page 73. NEXT and CHARS are the default operands.
ISPF automatically scrolls to bring the character string to the top of the list. To repeat the search without
reentering the character string, use the RFIND command.
Note: RFIND search starts from the second data set in the list. It is not cursor-sensitive.
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  157

## Page 196

LC command
The LC primary command invokes the color change utility from the command line of a data set list display,
as shown in Figure 95 on page 158.
                     Data Set List Color Change Utility
   Change one or more of the Data Set List Field Attributes and press Enter
   to immediately see the effect.  Clearing a field restores defaults.
                                                                    More:     +
   Field:             Color:       Intens:      Hilite:        Defaults:
   Volume . . . . . . BLUE   . . . LOW    . . . NONE           Blue
 * Data Set Name  . . GREEN  . . . LOW    . . . NONE           Green
   Data Set Stats . . TURQ   . . . LOW    . . . NONE           Turquoise
   Create,Expire,
   Catalog  . . . . . YELLOW . . . LOW    . . . NONE           Yellow
   Message  . . . . . WHITE  . . . LOW    . . . NONE           White
   Tracks,Device  . . WHITE  . . . LOW    . . . NONE           White
   Marked Data Set  . YELLOW . . . HIGH   . . . NONE           Yellow
 * _ Use Point-and-Shoot field attributes on Data Set Name field
 --------------------------------- Example ------------------------------------
 Command - Enter '/' to select action                     Message      Volume
       Tracks  %     XT Device  Dsorg Recfm Lrecl Blksz  Created    Referred
          Catalog
 ------------------------------------------------------------------------------
          ISR.V5R5M0.ISRLOAD                              Browsed      HSM001
 Command ===> ____________________________________________________     Defaults
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 95. Data Set List Color Change Utility panel (ISRDLCP)
LOCATE command
The LOCATE primary command scrolls the list of data sets based on the field on which the data set list is
sorted, as described under “SORT command” on page 161. Use this format:
LOCATE lparm
You can use the lparm operand with the LOCATE command for either of these situations:
• If the list is sorted by data set name, specify a data set name.
• If the list is sorted by another field, specify a value for the field by which the list is sorted.
For example, for a data set list sorted by volume, you could enter:
LOCATE TSOPK1
This command locates the first data set in the list on volume TSOPK1. If the value is not found, the list is
displayed starting with the entry before which the specified value would have occurred.
MEMBER command
The MEMBER primary command is used to search for a member name or pattern in all of the partitioned
data sets in the data set list. It can be abbreviated as M or MEM. The parameters, X, EX, NX, RECALL1,
and RECALL2 are optional. X and EX limit the search to excluded data sets. NX limits the search to
non-excluded data sets. RECALL1 includes data sets migrated to DASD in the search. RECALL2 includes
all migrated data sets in the search. Use this format:
MEMBER string
X
EX NX RECALL1 RECALL2
The data set list is scrolled so that the first data set containing the member or pattern is at the top of
the list. The MEMBER command finds any occurrence of the specified member name or pattern within a
partitioned data set.
Data set list utility (option 3.4)
158  z/OS: z/OS ISPF User's Guide Vol II

## Page 197

REFRESH command
The REFRESH primary command updates the display of the data set list to whatever the list's current
state is. For example, after deleting several items on the list, REFRESH causes the list to be displayed
without the deleted items. If you have appended to the list, REFRESH restores the list to its status before
the append operation.
RESET command
The RESET primary command unexcludes data sets that were excluded from a list, and removes any
pending line commands and messages from the data set list.
SAVE command
The SAVE primary command writes the data set list to the ISPF list data set or to a sequential data set.
ISPF writes the data set list in its current sort order. If the Display Catalog Name option is selected and
Volume Serial was not entered, the catalog name associated with each data set is included in the Data Set
List written to the sequential file. Use this format:
SAVE
list-id
where list-id is an optional user-specified qualifier of the data set to which the member list will be written.
ISPF names the data set:
prefix.userid.list-id.DATASETS
where:
prefix
Your data set prefix, as specified in your TSO user profile. If you have no prefix set, or if your prefix is
the same as your user ID, the prefix is omitted and the data set name will be: userid.list-id.DATASETS.
userid
Your TSO user ID.
The data set is created if it does not exist, or written over if it exists, and has compatible attributes. If
you omit the list-id operand, the list is written to the ISPF list data set and includes the list and column
headings and this data set information:
• Data set name
• Volume
• Org
• Recfm
• Lrecl
• Blksz
• Trks
• %Used
• XT
• Created
• Catalog Name (depending on the setting of the Display Catalog Name option)
If you enter SAVE without a list-id and the Display Total Tracks option is selected, an additional header
line with the accumulated tracks of all data sets and the number of all data sets in the list is written above
the column headings. If you provide the list-id operand, the list does not include the column headings and
contains all the data set information of the list without the listid provided, plus this information:
• Device
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  159

## Page 198

• Expires
• Referred
This command would tell ISPF to write the list to a sequential data set named either
pr efix .userid.MY.DATASETS or userid.MY.DATASETS.
SAVE MY
If the sequential data set already exists, ISPF writes over it; if not, ISPF creates it.
ISPF displays a progress status pop-up panel when the necessary information to perform a SAVE or SORT
primary command has to be retrieved and the data set list comprises 50 or more data sets. The keyboard
locks when this pop-up panel appears and stays locked until the SAVE or SORT command is completed.
This happens regardless of the setting of Display Total Tracks option and the value entered in the Initial
View field.
Note: When Display Total Tracks is ON and Initial View = 2 (Space) or 4 (Total), the pop-up panel appears
during the data set list display when it comprises 50 or more data sets and all the track information is to
be retrieved. When the SAVE command is subsequently issued the pop-up is not displayed as the data is
already available.
SHOWCMD command
The SHOWCMD primary command controls the display of line commands and their operands as they are
called. Use this format:
SHOWCMD
ON
OFF
where:
ON
Tells ISPF to display line commands. This is the default setting.
OFF
Tells ISPF not to display line commands. Though the SHOWCMD default is ON, SHOWCMD is initially
set to OFF.
After you enter SHOWCMD ON, a panel (Figure 96 on page 161) is displayed each time you enter a line
command, TSO command, CLIST, or REXX exec on a data set list display.
On this panel, you see the command as you typed it and then, a few lines down, you see the command
as ISPF interpreted it. Seeing these commands displayed can be especially useful when you use the
slash ( / ) character to substitute for the data set name because the panel shows the line command
after expansion occurs. Therefore, you can tell immediately whether you need to add operands to the
command.
For example, suppose you have a data set list displayed on the screen and decide to browse member
MEMB1 of data set USER.TEST.DATA. To see how ISPF interprets the B (browse) line command, type
SHOWCMD ON on the Command line and press Enter. Then, enter this line command in the Line
Command field to the left of USER.TEST.DATA:
B /(MEMB1)
When you press Enter, the panel shown in Figure 96 on page 161 is displayed.
Data set list utility (option 3.4)
160  z/OS: z/OS ISPF User's Guide Vol II

## Page 199

Data Set List Utility
 Data Set Name. : USER.TEST.DATA
 Command before expansion:
       B /(MEMB1)
 Command after expansion:
 ===> B 'USER.TEST.DATA(MEMB1)'                                              
                                                                               
                                                                               
                      
 The expanded command field shown here can be modified,
 but the data set name field may not be changed for built-in commands.
 Press ENTER key to process the command.
 Enter END command to return without processing the command.
 Command ===>                                                                 
  F1=Help    F2=Split   F3=Exit    F9=Swap   F12=Cancel
Figure 96. Data Set List Utility - SHOWCMD panel (ISRUDSLS)
Note:
1. The data set name and commands shown in Figure 96 on page 161 are for illustrative purposes only.
These values are determined by the command you enter and the data set acted on by that command.
2. SHOWCMD must be entered from a data set list. It is invalid if you use a line command, such as M, to
display a member list before calling it.
When the panel showing the commands is displayed, you can perform one of these actions:
• Press Enter to call the command displayed in the "Command after expansion" field.
• Change the command displayed in the "Command after expansion" field and then press Enter to call the
changed command.
• Enter the END command to return to the data set list display.
For information about using line commands, TSO commands, CLISTs, REXX EXECs, and the / character on
a data set list display, see “Blank — display data set list” on page 133.
SORT command
The SORT primary command sorts the data set list by the specified field. Use this format:
SORT
field1
A
D
field2
A
D
where:
field1
The major sort field. If only one operand is used, ISPF treats it as field1 . If both operands are used,
ISPF sorts the list by field1  first, then by field2  within field1 .
field2
The minor sort field.
A|D
The direction in which values are sorted for this field (A=ascending, D=descending).
For example, to sort a data set list by volume and block size within each volume, use this command:
Data set list utility (option 3.4)
Chapter 5. Utilities (option 3)  161

## Page 200

SORT VOLUME BLKSZ
If you do not specify a field, ISPF sorts the list by data set name. The keywords described in Table 12 on
page 162 tell ISPF by which fields to sort the data set list. 
Table 12. Sort fields  for source libraries
Field Default Sequence Description
NAME Ascending Data set name
MESSAGE Ascending Command completion message
VOLUME Ascending Volume serial
DEVICE Ascending Device type
DSORG Ascending Data set organization
RECFM Ascending Record format
LRECL Descending Logical record length
BLKSZ Descending Block size
TRACKS Descending Data set size
%USED Descending Percentage used
XT Descending Extents used
CREATED Descending Creation date
EXPIRES Ascending Expiration date
REFERRED Descending Last accessed data
MVOL Ascending Multivolume or migration level
CATALOG Ascending Catalog Name
Automatic scrolling is performed, if necessary, to bring the major sort field into view. ISPF displays
a progress status pop-up panel when the necessary information to perform a SAVE or SORT primary
command has to be retrieved and the data set list comprises 50 or more data sets. The keyboard locks
when this pop-up panel appears and stays locked until the SAVE or SORT command is completed. This
happens regardless of the setting of Display Total Tracks option and the value entered in the Initial View
field.
Note: When Display Total Tracks is ON and Initial View = 2 (Space) or 4 (Total), the pop-up panel appears
during the data set list display when it comprises 50 or more data sets and all the track information is to
be retrieved. When the SORT command is subsequently issued the pop-up is not displayed as the data is
already available.
SRCHFOR command
Use the SRCHFOR primary command to search the data sets in the data set list for one or more strings
of data using the SuperC Utility (see Option 3.14). You may limit the search to excluded or non-excluded
data sets, and control whether migrated data sets are recalled and searched or not. Use this format:
SRCHFOR string
The string parameter is optional but always converted to uppercase. If specified it is used to prefill the
first search string on the subsequent DSLIST Srchfor Options panel.
Data set list utility (option 3.4)
162  z/OS: z/OS ISPF User's Guide Vol II

## Page 201

WORD, SUFFIX, and PREFIX are available operands for search string specification. Note that the search
strings are case sensitive and must match exactly as specified. Consider the 'Any case' process option if
you want to disregard case.
Select the "ASCII" process option to cause ISPF to process the data in the data sets as ASCII. The data
read from the data sets is converted from ASCII to EBCDIC. Any search string given in hexadecimal
notation is assumed to be in ASCII, matching the original input data. The ASCII code page is assumed to
be ISO 8859-1 (CCSID 819). The terminal code page is used as the EBCDIC code page. If the terminal
code page cannot be determined code page 1047 is used.
You can use the C (continuation) operand to specify that both the current and previous string must be
found on the same line to constitute a match. Otherwise, lines with either string are treated as matching.
Table 13. SRCHFOR command search string examples
Example Search strings Explanation
===> ABC
===> EFG
Either string ABC or EFG may be found in the search data set.
===> ABC WORD
===> EFG C
The two strings (ABC and EFG) must be found on the same line. ABC must
be a complete word, while EFG (a continuation definition) can be part of
any word.
===> ABcD prefix The string (ABcD) is detected if the case of each letter matches and it is a
prefix of a word.
===> X'7b00' The hex string is specified as the search string. The listing must be
browsed with 'HEX ON'.
===> 'AB C''D' The string (AB C'D) is specified.
To start the search, press the Enter key from the DSLIST Srchfor Options panel. To cancel the request and
return to the Data Set List, enter END or CANCEL.
Output is in the listing DSN you specify and in the MESSAGE field in the DSLIST. Sort on this field to
consolidate results.
VA, VS, VT, and VV commands
The VA, VS, VT, and VV commands change the data set list display to the Attributes, Space, Total, and
Volume views, respectively.
Reset ISPF statistics utility (option 3.5)
If you have set STATS mode on, the ISPF editor automatically generates statistics for each member of a
partitioned data set. You might want to reset these statistics for these reasons:
• The program you are developing has been completed and you would like to reset all version numbers
before starting on the next release.
• A person has left the project, and you wish to reassign some of the members to the user ID of the
person who is taking over the work.
• You would like to create ISPF statistics for some members that were created or modified on a system
other than ISPF.
• You want to delete existing statistics from a partitioned data set to save space in the directory.
This option allows you to create, update, or delete statistics and to reset sequence numbers.
The Reset ISPF Statistics utility handles only partitioned data sets whose record length is in this range:
• From 1 to 32 760, inclusive, for fixed-length records
• From 5 to 32 756, inclusive, for variable-length records.
Reset ISPF statistics utility (option 3.5)
Chapter 5. Utilities (option 3)  163

## Page 202

For more information about ISPF Member Statistics, see the "ISPF Libraries and Data Sets" chapter in the
z/OS ISPF User's Guide Vol I.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                             Reset ISPF Statistics
 R Reset (create/update) ISPF statistics   D Delete ISPF statistics
 New Userid  . . . . . . ________  (If userid is to be changed)
 New Version Number  . . __        (If version number is to be changed)
 SCLM Setting                               Enter "/" to select option
 3  1. SCLM   2. Non-SCLM   3. As is        /  Reset Mod Level
                                            /  Reset Sequence Numbers
 ISPF Library:                              /  Reset Date/Time
    Project . . . JOHNLEV                   /  Reset Number of Lines
    Group . . . . TEST                        
    Type  . . . . DATA     
    Member  . . .                      (Blank or pattern for member selection
                                        list, "*" for all members)
 Other Partitioned Data Set:
    Name  . . . . . . .                                                       
    Volume Serial . . .          (If not cataloged)
 Data Set Password  . .          (If password protected)
 
 Option ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 97. Reset ISPF Statistics panel (ISRURSP)
Reset ISPF statistics panel action bar
The Reset ISPF Statistics panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers these choices:
1
General
2
Why you might want to Reset ISPF statistics
3
Filling in the reset utility panel
4
Using the Member list
6
ISPF statistics
7
Appendices
Reset ISPF statistics utility (option 3.5)
164  z/OS: z/OS ISPF User's Guide Vol II

## Page 203

8
Index
Reset ISPF statistics panel fields
All the fields on this panel are described in the Libraries and Data Sets topic in the z/OS ISPF User's Guide
Vol I, except these fields:
New Userid
This field is used to set the ID field in the statistics. Enter a new user ID here if you want to change the
user ID the statistics are recorded under. It is required if you do not specify a new version number.
If you are updating the user ID but not resetting the sequence numbers, the statistics are updated but
the data is not scanned or renumbered.
New Version Number
Enter a number here if you want to change the version number. This field is required if you do not
specify a new user ID when resetting statistics. It is ignored if you are deleting statistics.
Reset Mod Level
Use a slash to select this option and reset the modification level. Deselect this option if you do not
want to reset the modification level. A new version number is required to reset the modification level.
Reset Sequence Numbers
Use a slash to select this option and reset the sequence numbers. Deselect this option if you do
not want to reset the sequence numbers. A new version number is required to reset the sequence
numbers. Only standard (STD) sequence numbers are reset.
If the data is in packed format, there can be no sequence number processing. However, statistics for
members in packed format can be created or updated if the sequence numbers are not being reset.
SCLM Settings
The SCLM setting is a bit that ISPF uses to determine what type of edit the file last had performed
upon it.
1 SCLM
This bit is ON to specify that the last edit of this file was under SCLM control.
2 Non-SCLM
This bit is ON to specify that the last edit of this file was under control of something other than
SCLM.
3 As-is
This bit is ON to specify that this copy operation transfers the current setting of this file as it
already is.
Reset Date/Time
The setting of this option determines whether to reset the Last Modified Date or Time and the Creation
Date of the file.
Reset Number of Lines
The setting of this option determines whether to reset the Current Number of Lines, the Initial
Number of Lines, and the Number of Modified Lines settings. If this option is selected, the Current
Number of Lines and Initial Number of Lines settings are set to the actual number of lines of the
member.
The Number of Modified Lines setting is dependent on the Reset Mod Level and Reset Sequence
Numbers options. If either of those are reset and the Reset Date/Time field is selected, then the value
of the Number of Modified Lines is set to zero. Otherwise, the Number of Modified Lines remains as is.
Extended statistics are automatically generated if you select this option and extended statistics are
enabled in the site configuration and any of the line count values exceed 65535. More space is
occupied in the PDS directory by each member with extended statistics.
Reset ISPF statistics utility (option 3.5)
Chapter 5. Utilities (option 3)  165

## Page 204

Reset ISPF statistics utility options
These topics describe the options shown on the Reset ISPF Statistics panel:
• “R — reset (create/update) ISPF statistics” on page 166
• “D — delete ISPF statistics” on page 166
• “Results of resetting statistics” on page 166
R — reset (create/update) ISPF statistics
Use option R either to create statistics in a library that does not currently have them, or to update
statistics in a library.
The New Userid field is optional for option R. If you specify a user ID, it is placed in the ID field of the
statistics. If you leave the New Userid field blank and select a member without statistics, the ID field of
the statistics is set to the current user ID.
Either a new user ID or a new version number is required when you use this option. When you specify a
version number, the statistics are created or reset as follows:
Version Number
Set to the specified value.
Modification Level
Set to zero if requested; otherwise, unchanged.
Creation Date
Set to current date in the national format.
Change Date
Set to current date, in the national format, and time.
Current No. Lines
Set to the current number of data records.
Initial No. Lines
Set to the current number of data records.
No. Modified Lines
Set to zero if the Reset Sequence Numbers field is selected.
If you have requested updating of the modification level and resetting of the sequence numbers, the last
two digits of each sequence number are set to zeros. Otherwise, they are not changed.
If you have requested updating of sequence numbers, the data is scanned to determine if valid, ascending
sequence numbers are present in all records. If so, the data is renumbered. Otherwise, the data is
assumed to be unnumbered and renumbering is not done.
D — delete ISPF statistics
Use option D to delete ISPF statistics for an ISPF library or other partitioned data set. The New Userid and
New Version Number fields are ignored when you use option D.
Results of resetting statistics
What you specify for the New Version Number, Reset Mod Level, and Reset Sequence Numbers fields
controls the resetting of the sequence numbers, the modification flags within the data, and the statistics.
A new version number is required to reset the modification level and sequence numbers. Therefore, if a
new version number is entered and the data is not in packed format, Table 14 on page 167 shows the
various combinations you can use for the Reset Mod Level and Reset Sequence Numbers fields and the
results of those combinations.
Reset ISPF statistics utility (option 3.5)
166  z/OS: z/OS ISPF User's Guide Vol II

## Page 205

Table 14. Reset mod level and reset sequence numbers combinations
Reset Mod Level Selected Reset Mod Level Deselected
Reset Sequence Numbers
Selected
RESET MOD FLAGS=UNCHANGED SEQ
#'s=RESET
Reset Sequence Numbers
Deselected
Unchanged Unchanged
Processing of alias entries
If statistics are updated or created for members of a data set by entering a wildcard as part of the
member name and no member selection list is displayed, statistics for alias members are not created,
thus leaving the alias bit untouched.
If alias members are selected from a member selection list, a confirmation pop-up panel is displayed for
each alias selected, before creating ISPF statistics. If statistics are created for an alias member, the alias
bit is turned off, effectively creating a non-alias member using the same TTR as the original member for
which the alias was created.
Hardcopy utility (option 3.6)
The Hardcopy utility allows you to specify a sequential data set or a member of a partitioned data set to
be printed, and the destination of the output. It also allows you to specify whether a sequential data set is
to be kept or deleted after printing. Partitioned data set members are always kept.
You can use the Hardcopy utility to print any DASD-resident data set except ISPF list and log data sets;
use the ISPF LIST command to print log and list data sets during an ISPF session.
An optional print utility exit can be specified by your installation. If this exit is installed, it may cause
the Hardcopy utility's response to differ from the descriptions shown here. See z/OS ISPF Planning and
Customizing for more information about the print utility exit.
Another factor that can affect the Hardcopy utility's performance is whether the TSO/E Information
Center Facility is installed. If the TSO/E Information Center Facility is installed, your installation can
optionally allow ISPF to display a panel for submitting TSO/E Information Center Facility information with
the print request. See “Using the TSO/E information center facility” on page 170 for more information.
If the TSO/E Information Center Facility is not installed, the Hardcopy utility first displays the panel shown
in Figure 98 on page 167.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                Hardcopy Utility
                                                                    More:     +
 Process option     1. Print and keep data set or member
                    2. Print and delete sequential data sets
   Data Set Name  . .                                                        
   Volume Serial  . . . .              (If not cataloged)
   Data Set Password  . .              (If password protected)
 Print Mode . . . . . . . BATCH        (Batch or Local)
 Batch Sysout class . . .                    (BATCH only)
 Local printer ID or
 writer-name  . . . . . .                    (LOCAL only)
 Local Sysout class . . .                    (LOCAL only)
 Job statement information: (If not to local printer/external writer, verify
 before proceeding)
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 98. Hardcopy Utility panel - before JCL generation (ISRUHCP)
Hardcopy utility (option 3.6)
Chapter 5. Utilities (option 3)  167

## Page 206

Hardcopy utility panel action bar
The Reset ISPF Statistics panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers these choices:
1
General
2
Function of the hardcopy utility
3
Selecting a print mode
4
Submitting a background job to print a data set or member
5
Routing a data set to a printer local to your terminal group
6
Printing a data set using TSO/E Information Center Facility
7
Appendices
8
Index
Hardcopy utility panel fields
The Data Set Name, Volume Serial, Data Set Password, and Job statement information fields, shown in
Figure 98 on page 167, are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's
Guide Vol I. The other fields on the panel are:
Print Mode
Lets you specify one of these print modes:
BATCH
Submits your print request as a background job.
LOCAL
Routes your data to a local printer, such as an IBM 328x printer that is connected to your terminal
group.
Batch Sysout Class
Destination of printed data set. Used only if the data set is to be printed and Batch SYSOUT class is
specified.
Local Printer ID or Writer name
Destination of printed data set. Used only if the data set is to be printed and Local Printer ID or
external writer name is specified.
Note: If you specify a Local Printer ID or writer name and you have selected the Edit PRINTDS
Command option on the ISPF Settings panel (option 0), ISPF displays the Local Print Command Edit
Hardcopy utility (option 3.6)
168  z/OS: z/OS ISPF User's Guide Vol II

## Page 207

panel to allow you to intercept and edit the PRINTDS command before it is processed. For more
information on editing the PRINTDS command, see the "ISPF Libraries and Data Sets" chapter in the
z/OS ISPF User's Guide Vol I.
Local Sysout Class
Used in conjunction with the Local Printer ID or Writer Name. Specifies the output class to use for
output processing.
Generating and submitting JCL
Follow these steps to generate and submit JCL for your print jobs:
1. Choose one of the options listed at the top of the panel and type its code, 1 (for PK) or 2 (for PD), in the
Option field.
2. Specify a fully qualified data set name and member name.
This is a required field. If you are entering a fully qualified TSO data set name, you must enclose the
name in quotes. If you omit the quotes, the data set prefix from your TSO user profile is automatically
added to the beginning of the data set name.
3. If the data set is not cataloged, specify the volume serial.
4. If your data set is password protected, type the password in the Data Set Password field. For more
information on data set passwords, see the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I.
5. Specify either BATCH or LOCAL in the Print Mode field.
6. Specify one of these:
• If you chose BATCH in the previous step, type a Batch SYSOUT class and any job statement
information you need.
• If you chose LOCAL in the previous step, type the name of a local printer or writer name in the Local
Printer ID field. Job statement information is ignored.
7. Press Enter.
What happens next depends on your choice in step “5” on page 169. If you chose BATCH, see step
“7.a” on page 169. If you chose LOCAL, see step “7.b” on page 170.
a. If you chose BATCH, ISPF generates the JCL and displays the panel shown in Figure 99 on page
169, with the message JCL generated in the upper-right corner. 
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                Hardcopy Utility                  JCL generated
                                                                    More:     +
 Process option  1  1. Print and keep data set or member
                    2. Print and delete sequential data sets
                    3. Exit without submitting job
 Enter End command to submit job.
   Data Set Name  . . 'MYPROJ.DEV.SOURCE(TESTA)'                              
   Volume Serial  . . . .              (If not cataloged)
   Data Set Password  . .              (If password protected)
 Batch Sysout class . . . A            
 Print Mode . . . . . . : BATCH        (Batch or Local)
 Local printer ID or
 writer-name  . . . . . :
 Local Sysout class . . :
 Job statement information:
 Command ===>                                                                 
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 99. Hardcopy Utility panel - after JCL generation (ISRUHCJP)
Hardcopy utility (option 3.6)
Chapter 5. Utilities (option 3)  169

## Page 208

At this point you can either:
• Cancel the job by typing the CANCEL command in the Option field and pressing Enter.
• Submit the job by typing the END command and pressing Enter. ISPF displays this message at the
bottom of the panel:
IKJ56250I JOB jobname(jobid) SUBMITTED
***
Press Enter. For more information about BATCH printing, see “Additional batch printing
information” on page 170.
• Specify another data set name for printing.
b. If you chose LOCAL, ISPF calls the PRINTDS TSO command processor to print the data set on the
specified local printer.
A message is displayed in the short message area to show that PRINTDS has accepted the request.
At this point, you can:
• Specify another option and press Enter
• Enter the END command
• Enter the CANCEL command
8. If you entered CANCEL or END, ISPF determines the next panel you see as follows:
• If you entered the Hardcopy utility from the ISPF Primary Option Menu or through the jump function
(=), ISPF displays the ISPF Primary Option Menu.
• If you entered the Hardcopy utility from the Utility Selection Panel, ISPF returns you to that panel.
Additional batch printing information
When you enter the desired information and press Enter, ISPF generates JCL that contains the job
statement operands and a job step that prints the specified data set, using the IBM IEBGENER utility.
Note:
1. IEBGENER does not support packed data. If you try to print packed data, you may get unwanted
results. IEBGENER prints the data set one logical record per print line. If the logical record length is
greater than the printer width, the logical record is truncated.
2. ISPF does not unpack data automatically before printing it. Therefore, if you need to unpack data
before printing it, edit the data set and enter the PACK primary command with the OFF operand. See
z/OS ISPF Edit and Edit Macros for more information about the PACK command.
Once the JCL for the first job step is generated, the job statement operands are shown for information
aboutly. They are no longer highlighted and you cannot type over them, since the job statement has
already been generated. You can then select another data set name to cause another job step to be
generated.
Using the TSO/E information center facility
If the TSO/E Information Center Facility is installed, your installation can allow ISPF to display the panel
shown in Figure 100 on page 171.
Hardcopy utility (option 3.6)
170  z/OS: z/OS ISPF User's Guide Vol II

## Page 209

Hardcopy Utility
   Process option  1  1. Print and keep data set or member
                      2. Print and delete sequential data set
 Data Set Name . . . .                                                        
 Volume Serial . . . .              (If not cataloged)
 Data Set Password . .              (If password protected data set)
 Printer location  . .               
 Printer format  . . .         
 Number of copies  . .    
 Command ===>                                                                 
  F1=HELP      F2=          F3=END       F4=DATASETS  F5=FIND      F6=CHANGE
  F9=SWAP     F10=LEFT     F11=RIGHT    F12=SUBMIT
Figure 100. Hardcopy Utility panel - with the TSO/E information center facility installed (ISRUHCPI)
Follow these steps to use the TSO/E Information Center Facility to submit your print jobs:
1. Choose one of the options listed at the top of the panel and type its code, PK or PD, in the Option field.
2. Enter a fully qualified data set name and member name.
You must specify at least the low-level qualifier, such as LIST. If you enter your user prefix as part of
the data set name, you must enclose the complete data set name in quotes. However, if you omit the
user prefix and quotes, your user prefix is automatically added to the beginning of the data set name.
3. If the data set is not cataloged, enter the volume serial.
4. If your data set is password-protected, enter the password in the Data Set Password field. For more
information about data set passwords, see the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I.
5. You can either leave the Printer location field blank or enter the location ID of the printer to be used.
The location ID is assigned by your installation.
You can also enter a partial location name followed by an asterisk (*).
6. You can either leave the "Printer format" field blank or enter the format ID of the printer to be used.
The format ID is assigned by your installation.
You can also enter a partial format name followed by an asterisk (*). If your printer location and format
entries do not identify a specific printer, a printer selection list is displayed. From this list, which is
similar to a member selection list, you can select a printer.
7. Specify the number of copies you want.
8. Press Enter.
The values entered in the fields on this panel are passed directly to the TSO/E Information Center
Facility for processing.
Outlist utility (option 3.8)
This utility gives you the ability to browse, print, delete, or requeue job output that is in a held SYSOUT
queue. When you select this option, a panel is displayed (Figure 101 on page 172) that allows you to
select an option and enter the appropriate operands.
Outlist utility (option 3.8)
Chapter 5. Utilities (option 3)  171

## Page 210

Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                Outlist Utility
                                                                    More:     +
     L List job names/id(s) via the TSO STATUS command
     D Delete job output from SYSOUT hold queue
     P Print job output and delete from SYSOUT hold queue
     R Requeue job output to a new output class
 blank Display job output
 For Job to be selected:
    Jobname  . .         
    Class  . . .  
    JobID  . . .         
 For Job to be requeued:
    New Output class  . .  
 For Job to be printed:
    Printer Carriage Control  . .           (A for ANSI    )
                                            (M for machine )
 Option ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 101. Outlist Utility panel (ISRUOLP1)
Outlist Utility panel action bar
The Outlist Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down offers you these choices:
1
General
2
Listing the status of jobs
3
Deleting the output of a held job
4
Printing the output of a held job
5
Requeueing the output of a held job
6
Displaying the Output of a held job
7
Appendices
8
Index
Outlist Utility panel fields
The fields on this panel are:
Jobname
The held SYSOUT job. It is required for all options except option L.
Outlist utility (option 3.8)
172  z/OS: z/OS ISPF User's Guide Vol II

## Page 211

Class
The SYSOUT hold queue. If you omit the CLASS operand, all SYSOUT queues are searched for the
specified job.
JobID
Required only if more than one job exists with the same job name.
New Output class
When requeuing a job (option R), enter the new SYSOUT hold class here.
Printer Carriage Control
When printing a data set (option P), enter a value here that corresponds to the type of carriage control
characters in the data set. Valid values are:
A
If the data contains American National Standard Institute (ANSI) carriage control characters.
M
If the data contains machine control characters.
Blank
If the data contains no carriage control characters.
The record formats for the corresponding data sets are FBA, FBM, and FB, respectively.
Outlist utility options
These topics explain the options listed at the top of the Outlist Utility panel:
• “L — list job names/ID(s) via the TSO STATUS command” on page 173
• “D — delete job output from SYSOUT hold queue” on page 173
• “P — print job output and delete from SYSOUT hold queue” on page 173
• “R — requeue job output to a new output class” on page 174
• “Blank — display job output” on page 174
L — list job names/ID(s) via the TSO STATUS command
If you select option L, a list of job names and job IDs is displayed. If you leave the job name blank, or if the
job name is your user ID plus one identifying character, the status is listed for all jobs having job names
consisting of your user ID followed by that identifying character. If you supply any other job name, the
status for that exact job is displayed.
The list of job names is displayed on the lower portion of the panel. If the list is too long to fit on the
screen, three asterisks are displayed on the last line of the screen. You can display the remainder of the
list by pressing Enter.
D — delete job output from SYSOUT hold queue
If you select option D, the held output for a specific job is deleted from the specified SYSOUT queue.
P — print job output and delete from SYSOUT hold queue
If you select option P, the held output for a specific job is removed from the SYSOUT queue and placed
in an ISPF-defined data set for printing. You can choose the record format for this data set by putting an
entry in the Printer Carriage Control field.
An optional print utility exit can be installed by your system programmer. If this exit is installed, it may
cause the Outlist utility's response to differ from the descriptions provided here. See z/OS ISPF Planning
and Customizing for more information about the print utility exit.
Another factor that can affect the performance of the Outlist utility is whether the TSO/E Information
Center Facility is installed. If the TSO/E Information Center Facility is installed, your installation can
optionally allow ISPF to display a panel for submitting the TSO/E Information Center Facility information
Outlist utility (option 3.8)
Chapter 5. Utilities (option 3)  173

## Page 212

with the print request. See Figure 187 on page 318 for an example of this panel and “Using the TSO/E
information center facility” on page 170 for information about the fields on this panel.
If the TSO/E Information Center Facility is not installed, the Outlist utility displays the panel shown in
Figure 186 on page 312 when you press Enter. Use this panel to tell ISPF how and where the job output
is to be printed. This option does not honor multiple copies for output on hold queue. To print multiple
copies use option R.
ISPF uses temporary data sets named pr efix .userid.SPFnnn.OUTLIST (if your data set prefix in your TSO
user profile is different from your TSO userid) or userid.SPFnnn.OUTLIST (if your prefix and userid are the
same), where nnn is a number between 100 and 999.
Attention: If you keep or use all data sets through 999, ISPF resets to 100 and uses the existing
data sets. Also, ISPF can use the data sets that you allocate using the temporary data set naming
convention.
R — requeue job output to a new output class
If you select option R, the held output for a specific job is requeued to another SYSOUT class from
the specified SYSOUT queue. You must enter the new SYSOUT class on the panel in the "New Output
class" field. You can use this option to print output with multiple copies by requeuing to a SYSOUT class
predefined to print multiple copies.
Blank — display job output
If you leave the Option field blank, the held output for the specified job is displayed in Browse mode.
You can use all Browse commands. The data remains in the SYSOUT queue. When you enter the END or
RETURN command to end Browse, the Outlist Utility panel is displayed again, and you can then choose to
print, requeue, or delete the job output.
Command table utility (option 3.9)
The Command Table utility (option 3.9) enables you to create or change ISPF application command
tables. When you select this option, a panel is displayed (Figure 102 on page 175) to prompt you for an
application ID. The name of the command table is then derived by adding CMDS to the application ID. If
the table exists in the table input library, ISPTLIB, it is displayed and can be modified. If the table does
not exist in the table input library, a new table is generated.
The command table displays the search order of commands for a particular logical screen. The order is
from top to bottom of those commands displayed. The "User table" and "Site table" fields are blank if no
values are set for them in the ISPF Configuration table, or if values have been set but the tables do not
exist in the "ISPTLIB" concatenation.
You cannot use this utility to change a command table that is currently in use. Command table ISPCMDS,
the system command table, is always in use by the Dialog Manager component. If you enter ISP in the
Application ID field, ISPF displays the ISPCMDS command table in read-only mode.
While you are using this utility to change a command table, the table cannot be used for other purposes.
For example, you cannot use split screen and select a function with NEWAPPL(XYZ) if you are changing
command table XYZCMDS.
Command table utility (option 3.9)
174  z/OS: z/OS ISPF User's Guide Vol II

## Page 213

Menu  Help
 ─ ┌─────────────────────────────── Commands ────────────────────────────────┐
 I │ ISPUCMA                 Command Table Utility                           │
   │                                                                         │
 1 │   Specifications                       Command table search order       │
   │   Application ID . . ISR               Application table  . : ISR       │
 2 │   Enter "/" to select option           User table 1 . . . . : USER      │
   │      Show description field            User table 2 . . . . :           │
 3 │                                        User table 3 . . . . :           │
 4 │                                        Site table 1 . . . . : SITE      │
   │                                        Site table 2 . . . . :           │
 5 │                                        Site table 3 . . . . :           │
 6 │                                        System table . . . . : ISP       │
 7 │                                                                         │
 8 │ If no application ID is specified, the current application ID will be   │
 9 │ used. The name of the command table to be processed is formed by        │
 1 │ prefixing the application id to the string 'CMDS'.  For example:        │
 1 │ Application ID  . .  TST results in a command table name of 'TSTCMDS'.  │
 1 │                                                                         │
 1 │ Command ===>                                                            │
 1 │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward     │
 O │  F9=Swap      F12=Cancel                                                │
   ⋘─────────────────────────────────────────────────────────────────────────┘
 F10=Actions  F12=Cancel
Figure 102. Command Table Utility panel (ISPUCMA)
Command Table Utility panel fields
The fields on the Command Table Utility panel function as follows:
Application ID
Contains the name of an application for which you want to define commands.
Show description field
Allows you to display the descriptions as well as the commands and definitions.
The command table for the named application is displayed on a Command Table editing panel (Figure 103
on page 175). This panel can be scrolled up and down using the scroll commands.
   File  Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                                  Update TSTCMDS                Row 1 to 4 of 4
 Insert (I), delete (D), repeat (R) and edit (E) command entries.
 Enter END command to save changes or CANCEL to end without saving.
      Verb      T  Action
      SORT      0  SELECT PGM(PQRSORT) PARM(&ZPARM)
      PREPARE   4
      QUIT      2  ALIAS END
      EXPLAIN   4
 ******************************* Bottom of data ********************************
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 103. Command table editing panel (ISPUCMD)
The column headings on the panel are:
Verb
The command verb, which is the name of the command you are defining in the command table. A
command verb must be 2 to 8 characters long, inclusive, and must begin with an alphabetic character.
The content of this column is assigned to the ZCTVERB system variable.
Command table utility (option 3.9)
Chapter 5. Utilities (option 3)  175

## Page 214

T (truncation)
The minimum number of characters that you must enter to find a match with the command verb.
If this number is zero or equal to the length of the command verb, you must enter the complete
command verb. For example, in Figure 103 on page 175 the PREPARE command has a truncation
value of 4. Therefore, for the TST application used as the example in the figure, only the first four
letters, PREP, must be entered to call this command. The content of this column is assigned to the
ZCTTRUNC system variable.
Action
The actual coding of the action to be carried out when you enter the command. The action length must
not be greater than 240 characters. The content of this column is assigned to the ZCTACT system
variable.
To enter or edit the coding for the action:
1. Enter the E command table line command to display the Extended Command Entry panel
(ISPUCMX).
2. Type the required coding in the Action lines.
Normally, any text you type in lowercase is translated to uppercase before it is saved.
To define some of the parameters in lowercase select the Allow mixed-case in Action field option
on the Extended Command Entry panel. The case of the text you type is not translated and is saved
as you input it.
Note that when you select the Allow mixed-case in Action field option:
a. The first word must be input in uppercase.
b. If you use &ZPARM to obtain parameters from the command line, the parameters may be
translated to uppercase (regardless of the setting of the Allow mixed-case in Action field
option).
3. Optionally, type a brief description of the purpose of the command in the Description lines.
4. Press PF3 to return to the Command Table Editing panel.
Note:
1. Do not use ACTIONS, CANCEL, CRETRIEV, CURSOR, EXIT, PRINT, PRINTG, PRINTHI, PRINTL,
PRINTLHI, RESIZE, RETF, RETP, RETRIEVE, SPLIT, SPLITV, SWAP, WINDOW, or WS as keywords
in the Action column. These keywords are intended only for use in the system command table
distributed with ISPF. They are not intended for use in application command tables.
2. Take care with ACTIONs that use ZPARM, as the ISPF parser will add a matching parenthesis if
one appears to be missing. Consider an entry of "SELECT CMD(%CMD &ZPARM) NEWAPPL(ISR)". If
"(XYZ" is passed then the command will receive "(XYZ) NEWAPPL(ISR)" as a parameter.
The valid actions are:
SELECT
Causes the selected dialog (command, program, or selection panel) to be given control
immediately. See z/OS ISPF Dialog Developer's Guide and Reference for more information about
the SELECT statement and its keywords.
ALIAS
Allows one command verb to carry out the action defined for another. For example, in Figure 103
on page 175, QUIT is an alias for END. Therefore, for the TST application used as the example in
the figure, entering QUIT causes the same action to occur as entering END.
An ALIAS command must be defined before the command for which it is an ALIAS.
PASSTHRU
Causes the command to be passed through to the dialog as if it had not been found in the
command table.
Command table utility (option 3.9)
176  z/OS: z/OS ISPF User's Guide Vol II

## Page 215

SETVERB
Causes the command to be passed through to the dialog, with the command verb stored
separately from the operands.
NOP
Causes the command to be inoperative. An inactive command message is displayed.
Blank
Causes the command table entry to be ignored. ISPF continues to search for additional entries for
the same command verb. If the command is not found in either the application command table or
the system command table, an invalid command message is displayed.
xxxxx
A variable name, beginning with an ampersand (&), allows dynamic specification of the command
action.
DESCRIPTION
An optional, brief description of the action the command verb is to perform. Since this column is
offset three spaces under the Action column, the description length must not be greater than 80
characters. The content of this column is assigned to the ZCTDESC system variable.
For a new table, this panel initially contains dummy entries with all fields shown as underscores. The
underscores are pad characters and need not be blanked out. However, any null entries where at least the
verb contains all underscores are automatically deleted when the table is saved.
Scrolling a command table
You can scroll the table entries, using the ISPF UP and DOWN scroll commands, and change one or more
entries simply by typing over them.
Saving a command table
The END command causes the table to be saved in the table output library, ISPTABL, and ends the utility.
Canceling a command table
The CANCEL command ends the command table display without saving the table.
Using command table line commands
The line commands you can enter at the left of any entry (by typing over the four quotation marks) are
described in these topics:
• “D — deleting lines” on page 177
• “E — editing lines” on page 178
• “I — inserting lines” on page 178
• “R — repeating lines” on page 178
• “V — viewing lines” on page 178
Multiple line commands or changes can be entered in a single interaction. Line commands followed by
a number, such as D3, are repeated that number of times. The lines are processed in the order in which
they appear on the screen. Any line commands or changes that are entered concurrently with the END
command are processed before the table is saved.
D — deleting lines
The D command deletes one or n lines
D
1
n
Command table utility (option 3.9)
Chapter 5. Utilities (option 3)  177

## Page 216

E — editing lines
The E command displays the Extended Command Entry panel (ISPUCMX) where you can edit the action
and description fields for a line.
E
I — inserting lines
The I command inserts one or n lines.
I
1
n
The inserted lines contain underscores (pad characters) in all field positions.
R — repeating lines
The R command repeats a line one or n times. The repeated lines contain underscores (pad characters) in
the Verb and T (truncation) fields, but the Action and Description fields are copied from the line on which
the R command was entered.
R
1
n
V — viewing lines
The V command views one or n lines. You can look at the entire command entry including the command
action and description fields, but you cannot change them.
V
1
n
Format specifications utility (option 3.11)
The Format Specifications utility (option 3.11) is provided to support the IBM 5550 terminal using the
Double-Byte Character Set (DBCS). It is used to maintain formats that are used when viewing, browsing,
and editing to display data sets that contain predefined formatted records.
The purpose of a format is to structure data from a record into fields, and to define the order these fields
are to be physically displayed on the screen when you are viewing, browsing, and editing.
When you select this option, a panel is displayed (Figure 104 on page 179) that allows you to add, copy,
delete, or update a format. You can also display the format list.
Format specifications utility (option 3.11)
178  z/OS: z/OS ISPF User's Guide Vol II

## Page 217

Menu  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                             Format Specifications
                                                                    More:     +
    A            Add a new format
    C            Copy formats
    D            Delete a format
    U            Update format
    L or BLANK   Display format list
 Format Name . . .         
 For COPY operations, specify the following:
     From Format . . .           (Blank for format list, * for all formats)
     From Table  . . .           (Default is "ISRFORM" )
 Note: The Format Utility is provided for support of the IBM 5550 terminal
 Option ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 104. Format Specific ations  panel (ISRFM01)
Format Specifications panel action bar
The Format Specifications panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down introduces formats and provides information about how to add, copy, delete,
update, and display formats.
Format Specifications panel fields
The fields on this panel are:
Format Name
The name of the format that you want to add, delete, or update. When copying a format (option C), this
is the name you want the copied format stored under.
From Format
When copying a format (option C), you can:
• Enter the name of a format you want to copy
• Enter an asterisk (*) to copy all formats
• Leave the field blank to display a copy format selection list.
See Figure 106 on page 181 for an example of a Copy Format Selection List display.
From Table
When copying a format (option C), you can:
• Enter the name of a table from which you want to copy a format
• Leave the field blank if you want to copy a format from the ISRFORM table.
Note: The ISRFORM table is the default location in which all of your user-defined formats are
stored. If you have not yet defined any formats, this table will be empty.
Format specifications utility (option 3.11)
Chapter 5. Utilities (option 3)  179

## Page 218

Format Specifications panel options
These topics describe the options shown at the top of the Format Specifications panel:
• “A — add a new format” on page 180
• “C — copy formats” on page 181
• “D — delete a format” on page 182
• “U — update a format” on page 182
• “L or BLANK — display format list” on page 182
A — add a new format
If you specify option A and a format name, the Format Definition panel (Figure 105 on page 180) is
displayed.
                          Format Definition (FORM01)
                                                                    More:     +
  Field   Start   Field   Field        Field   Start   Field   Field
  Number  Column  Length  Type         Number  Column  Length  Type
     1    00000     00                    2    00000     00       
     3    00000     00                    4    00000     00       
     5    00000     00                    6    00000     00       
     7    00000     00                    8    00000     00       
     9    00000     00                   10    00000     00       
    11    00000     00                   12    00000     00       
    13    00000     00                   14    00000     00       
    15    00000     00                   16    00000     00       
    17    00000     00                   18    00000     00       
    19    00000     00                   20    00000     00       
  Field Number: Identifies the field position on the screen.
  Start Column: From 1 to 32760; Specifies column position in the record.
  field Length: From 1 to 71; Fields must not overlap.
  Field Type  : E - single-byte, D - double-byte,  M - mixed data
  Enter the END command to exit and save the format.
 Command ===>                                                                 
  F1=Help    F2=Split   F3=Exit    F9=Swap   F12=Cancel
Figure 105. Format Definition  panel (ISRFM02)
A field definition includes:
Field Number
The number of the field for which you are defining a format. You can define up to 20 fields.
Start Column
Starting column position in the record.
Field Length
Field length in bytes; the maximum is 71 bytes.
Field Type
The type of data that can be entered in the field. Valid types are:
E
EBCDIC (single-byte)
D
DBCS (double-byte)
M
Mixed data
Note: All three of these field types can contain extended graphics characters. CAPS ON processing is
not possible because of context dependencies. Therefore, it is ignored when you are editing formatted
data.
Format specifications utility (option 3.11)
180  z/OS: z/OS ISPF User's Guide Vol II

## Page 219

The format definition information applies to both existing records and inserted records in a data set.
Note: It is recommended that you avoid using STD or COBOL formats with numbered data. The results
can be different from using formats with unnumbered data. If you must use numbered data, do not define
the columns the sequence numbers will appear in, or define an EBCDIC or mixed data field for them.
C — copy formats
If you specify option C on the Format Specifications panel:
• If you specify both an asterisk (*) in the From Format field and a table name other than ISRFORM in the
From Table field, all formats stored in the "From" table are copied to ISRFORM.
Note: If you specify a table name in the From Table field, and that table does not have the same format
as ISRFORM, a severe error occurs.
• If you specify both a format name and a "From" format, the format is copied. If you specified a "From"
table (other than ISRFORM), the format is copied from that table. Otherwise, the format is copied
from ISRFORM. The Format Definition panel for the newly created format, containing the currently
defined fields, is displayed. You can add, delete, and update field definitions. When you enter the END
command, the format definition is stored in ISRFORM under the format name you specified.
• If you specify a format name but no "From" format, the Copy Format Selection List panel (Figure 106 on
page 181) is displayed.
If you did not specify a "From" table, the formats listed are those stored in ISRFORM, the default format
table. Otherwise, the formats listed are those stored in the table you specified.
Note: The ISRFORM table is the default location in which all of your user-defined formats are stored. If
you have not yet defined any formats, the table will be empty and you will receive a "No formats found"
message.
You can select a format to copy by entering the S line command to the left of that format name. Other
commands you can enter are U (Update), R (Rename), D (Delete), SELECT (which is similar to S), SORT,
and LOCATE. See “Format selection list commands” on page 182 for a description of these commands.
The format is copied, and the Format Definition panel for the newly created format, containing the
currently defined fields, is displayed. You can add, delete, and update field definitions. When you enter
the END command, the format definition is stored in ISRFORM under the format name you specified on
the Format Specifications panel.
• If you specify neither a format name nor a "From" format, but you do specify a "From" table (other than
ISRFORM), the Copy Format Selection List panel is displayed. You can select one or more formats to
copy by entering the S line command to the left of each format names. Each of these formats is copied
under the same name from the specified "From" table to the ISRFORM table.
Note: If you do not specify option C but specify a name in the From Format field, the From Format field is
ignored.
                     Copy Format Selection List (ISRFORM)       Row 1 to 4 of 4
        Name      Rename    Created   Last Modified       ID
        COMMON01            02/11/19  02/11/19  11:08     USERID
        COMMON02            02/11/19  02/11/19  11:16     USERID
        COMMON03            02/11/19  02/11/19  11:16     USERID
        COMMON04            02/11/19  02/11/19  11:16     USERID
 ******************************* Bottom of data ********************************
 ⋮
 Command ===>                                                 Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 106. Copy Format Selection List panel (ISRFM04)
Format specifications utility (option 3.11)
Chapter 5. Utilities (option 3)  181

## Page 220

D — delete a format
If you specify option D and a format name on the Format Specifications panel, the format is deleted.
U — update a format
If you specify option U and a format name on the Format Specification panel, the Format Definition panel
containing the currently defined fields is displayed. You can add, delete, and update field definitions.
L or BLANK — display format list
If you specify option L or leave the Option line blank on the Format Specifications panel, the Format
Selection List panel (Figure 107 on page 182) is displayed.
                             Format Selection List              Row 1 to 4 of 4
        Name      Rename    Created   Last Modified       ID
        COMMON01            02/11/19  02/11/19  11:08     USERID
        COMMON02            02/11/19  02/11/19  11:16     USERID
        COMMON03            02/11/19  02/11/19  11:16     USERID
        COMMON04            02/11/19  02/11/19  11:16     USERID
 ******************************* Bottom of data ********************************
 ⋮
 Command ===>                                                 Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 107. Format Selection List panel (ISRFM03)
Format selection list commands
These topics describe the commands you can use on a Format Selection List panel:
• “Locating format names” on page 182
• “Renaming a format” on page 183
• “Sorting format names” on page 183
• “Updating or selecting a format” on page 183
Deleting a format
If you specify the D line command beside a format name, the format is deleted.
Locating format names
The LOCATE command is another useful tool, especially if you have a long format list. To use the LOCATE
command, ensure that the list is sorted by name. Next, enter the LOCATE command on the Command line.
The syntax is:
LOCATE name
where:
name
The name of the format you want to find.
For example, this command would find a format named FORM03:
LOCATE FORM03
Format specifications utility (option 3.11)
182  z/OS: z/OS ISPF User's Guide Vol II

## Page 221

If the format exists, the entry for the specified format name appears as the second line following the
header lines. If the specified name is not found, the existing format name that would immediately
precede the specified name appears as the first line following the header lines.
Renaming a format
If you specify the R line command beside a format name, you must also specify its new name in the
Rename field before you press Enter. If you do not, the Enter required field message appears in
the upper-right corner of the screen and the cursor moves to the Rename field.
Sorting format names
You can sort the name list on this panel by entering the SORT command on the Command line. The syntax
of the SORT command is:
SORT NAME
TIME
where:
NAME
Sort by name.
TIME
Sort by time last modified.
For example, this command would sort a format selection list by time:
SORT TIME
Updating or selecting a format
If you specify the U or S line command beside a format name, the Format Definition panel containing the
currently defined fields is displayed. You can add, delete, and update field definitions.
You can specify that multiple operations be done at the same time. However, if you specify U or S with
other line commands, any commands after the first U or S are ignored.
The SELECT command provides you with another way to specify a format. This command is entered on
the Command line.
The syntax of the SELECT command is:
SELECT name
where:
name
The name of the format you want to select.
If the format exists, the Format Definition panel containing the currently defined fields is displayed. You
can add, delete, and update field definitions.
If no format exists for that name, a new format is created, and the Format Definition panel is displayed to
allow you to define fields.
SuperC utility (option 3.12)
Note: For an introduction to the SuperC and SuperCE utilities (options 3.12 and 3.13), see Appendix A,
“SuperC reference,” on page 431.
SuperC utility (option 3.12)
Chapter 5. Utilities (option 3)  183

## Page 222

The SuperC utility (option 3.12) is a dialog that uses the SuperC program to compare data sets of
unlimited size and record length at the file, line, word, or byte level. The panel shown in Figure 108 on
page 184 is used to specify the name of a new data set.
Note: In this context, a new data set is an updated version of a previously created data set, such as a data
set in your private library that has been modified but has not yet been promoted.
   Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                     SuperC Compare Utility - New Data Set
 Specify "New" Data Set to be compared, then press the ENTER key.
                                                                    More:     +
    Project . . .         
    Group . . . .          . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .                  (Blank or pattern for member selection list,
                                     "*" for all members)
 "New" Other Partitioned, Sequential or VSAM Data Set:
    Data Set Name . . .                                                        
    Volume Serial . . .            (If not cataloged)
 Profile DS Name  . . .                                                        
 Data Set Password  . .            (If New data set password protected)
 Enter "/" to select option      Execution Mode          Output Mode
    Mixed Mode                   1  1. Foreground        1  1. View
    Bypass selection list           2. Batch                2. Browse
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 108. SuperC Utility panel (ISRSSNEW)
This panel requires only the names of the input data sets and a foreground or batch mode setting.
Note:
1. For DBCS searches and compares to function properly, ISPF must be invoked with the JAPANESE
keyword, on a terminal that supports DBCS, and the MIXED process option must be supplied to
SuperC.
2. When a member of a PDSE version 2 data set that is configured for member generations is specified as
the old or new data set, the current generation of the member is used for the comparison.
SuperC Compare Utility panel action bar
The SuperC Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides general information about SuperC topics, as well as information about
specifying the data sets and options and interpreting listings.
SuperC Compare Utility panel fields
All the fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF
User's Guide Vol I, except:
SuperC utility (option 3.12)
184  z/OS: z/OS ISPF User's Guide Vol II

## Page 223

Profile DSN
The name of an optional data set that can contain a compare type, listing type, sequence numbers
setting, Browse setting, process options, and process statements. All these elements, when combined
in one data set or member, are called a pr o file . See “Profiles and defaults - activate profiles and
defaults” on page 200 for information about using the SuperCE utility (option 3.13) to create a profile
data set.
The listing type and sequence numbers setting of the profile are copied onto the panel used to specify
the old data set name (Figure 109 on page 186), but can be typed over or blanked out. However, other
elements of the profile are in effect, even though they are not shown on the panel.
Mixed Mode
Select this field to have SuperC scan and parse the input data set lines for DBCS text strings.
Note: Mixed Mode is not valid for the File or Byte compare.
Bypass Selection List
When a member pattern is entered in the PDS Member List field or the member name portion of
the data set field (such as MY.DATA.SET(pattern)), selecting this field causes SuperC to process all
members matching that pattern without displaying a member selection list. Leaving this field blank
causes the member list to be displayed.
Execution Mode
The processing mode you want to use when comparing the data sets. Choose one of these:
1
Foreground. After the old data set panel and member selection, if any, are completed, foreground
mode compares the new and old data sets and stores the results in the data set specified in the
Listing DS Name field, which you can browse at the terminal.
2
Batch. After the old data set panel and the member list, if any, are completed, batch mode causes
the display of the SuperC Utility - Submit Batch Jobs panel, so you can specify job card and print
disposition information or edit the JCL. Then, the batch job is submitted to compare the new and
old data sets. See “Submitting a SuperC job in batch mode” on page 189 for more information.
Note: You cannot specify a data set password in batch mode. If your data sets are password
protected, use foreground mode.
Output Mode
The output mode you want to use when displaying the listing file. Choose one of these:
1
View. This enables the listing file to be displayed in view mode. All View functions are enabled in
this mode.
2
Browse. This enables the listing file to be displayed in the browse mode. All Browse functions are
enabled in this mode.
3
Eview. This option only appears on non-English panels. It operates exactly the same as View
except that Superc is invoked with an English language constants module. All titles and headings
are in English. This facilitates use of hiliting of Superc listings on non-DBCS terminals.
When you complete the New Data Set panel and press Enter, ISPF displays the panel shown in Figure 109
on page 186. Of the five fields shown at the bottom of the panel (Volume Serial, Listing DS Name, Data Set
Password, Listing Type, and Sequence Numbers), all except Listing Type may not appear, depending on
the mode you choose (foreground or batch) and the contents of the profile data set. Also, if you request a
member list or specify an asterisk (*) in the Member field on the new data set panel, ISPF does not display
a Member field on the old data set panel.
Note: In this context, an old data set is a base version of a data set, such as a data set in a production
library.
SuperC utility (option 3.12)
Chapter 5. Utilities (option 3)  185

## Page 224

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                     SuperC Compare Utility - Old Data Set
 Specify "Old" Data Set to be compared, then press the ENTER key to compare to
 "New" Data set . . : MYPROJ.DEV.SOURCE
                                                                    More:     +
    Project . . . MYPROJ  
    Group . . . . TEST     . . .          . . .          . . .         
    Type  . . . . SOURCE  
    Member  . . .         
 "Old" other Partitioned, Sequential or VSAM Data Set:
    Data Set Name . . .                                                        
    Volume Serial . . .            (If not cataloged)
 Listing DS Name  . . . SUPERC.LIST                                            
 Data Set Password  . .            (If Old data set password protected)
 Listing Type . . . . 1  1. Delta   2. CHNG    3. Long    4. OVSUM   5. Nolist
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 109. SuperC Utility - Old Data Set panel (ISRSSOLD)
Specify the name of an old data set. The type of old data set that you can specify depends on the type of
new data set you specified on the previous panel. For example, you can compare:
• A complete new PDS to a complete old PDS
• A new sequential data set to:
– An old sequential data set
– An old membered PDS
• A new membered PDS to an old sequential data set.
In this context, the term membered PDS refers to a PDS for which a single member has been specified,
such as:
'USERID.TEST.SCRIPT(NEWDATA)'
SuperC treats a membered PDS as a sequential data set because the comparison is done on a one-to-one
basis. However, SuperC cannot compare a sequential data set to a complete PDS because it cannot
compare one data set to more than one member of another data set.
When you press Enter, ISPF either displays a member selection list or begins the comparison. All the
fields on this panel are explained in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's
Guide Vol I, except:
Update DS Name
Tells SuperC the name of the data set that will contain column-oriented results of the comparison.
Note: This field is not displayed unless your profile data set contains an update (UPDxxxx) process
option.
This data set is normally used as input to post processing programs and can be specified in addition
to the normal listing data set. See the Process Options selection in “Process options - select process
options” on page 199 for information about the SuperC process options.
If you leave this field blank, SuperC uses this default name:
prefix.userid.SUPERC.UPDATE
where pr efix  is your TSO prefix and userid is your user ID. If your prefix and user ID are identical, only
your prefix is used. Also, if you do not have a prefix, only your user ID is used.
SuperC utility (option 3.12)
186  z/OS: z/OS ISPF User's Guide Vol II

## Page 225

Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is
included before the SUPERC qualifier.
If you enter a fully qualified data set name SuperC uses it as specified. Otherwise, SuperC only
appends your TSO prefix to the front of the data set name specified. If you run with TSO PROFILE
NOPREFIX, SuperC uses the name as you entered it, which can result in an attempt to catalog the
name in the master catalog.
If you enter the name of a data set that already exists, the contents of that data set are replaced by
the new update output.
If you enter the name of a data set that does not exist, SuperC allocates it for you. The data set
is allocated as a sequential data set unless you enter a member name after it, in which case it is
allocated as a partitioned data set.
Note: For the UPDMVS8, UPDCMS8, UPDSEQ0, and UPDPDEL process options, the update data set
contains valid data but only after a successful compare when differences are detected. The data set
is always empty after a comparison that shows the data sets or members being compared have no
differences.
Listing Type
The type of listing you want SuperC to create when it compares the data sets. This is a required field,
so you must choose one of the listing types shown here. See Appendix B, “Understanding the listings,”
on page 477 for sample listings.
DELTA
Lists the differences between the source data sets, followed by the overall summary.
CHNG
Lists the differences between the source data sets, plus up to 10 matching lines before and after
the differences. This listing is a variation of the DELTA listing; the matching lines before and after
help you recognize changed areas of the source data sets.
LONG
Lists all the new data set source lines, plus old data set deleted lines. Both inserted and deleted
lines are flagged.
OVSUM
Lists only the overall summary of the comparison. However, a PDS comparison generates an
individual summary line for each PDS member.
NOLIST
Produces no listing output. In foreground mode, only a message is returned to show the outcome
of the compare.
Listing DS Name
The name of the list data set to which SuperC writes the results of the comparison. However, if you
enter NOLIST in the Listing Type field, SuperC does not create an output listing, so this name is
ignored. Also, if you chose batch mode, this field does not appear on the panel. The SuperC Utility -
Submit Batch Jobs panel is used instead.
If you leave this field blank, SuperC allocates a list data set, using default data set attributes and this
data set name:
prefix.userid.SUPERC.LIST
where pr efix  is your TSO prefix and userid is your user ID. If your prefix and user ID are identical, only
your prefix is used. Also, if you do not have a prefix, only your user ID is used.
Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is
included before the SUPERC qualifier.
If you enter a fully qualified data set name SuperC uses it as specified. Otherwise, SuperC only
appends your TSO prefix to the front of the data set name specified. If you run with TSO PROFILE
SuperC utility (option 3.12)
Chapter 5. Utilities (option 3)  187

## Page 226

NOPREFIX, SuperC uses the name as you entered it, which can result in an attempt to catalog the
name in the master catalog.
If you enter the name of a data set that already exists, the contents of that data set are replaced by
the new output listing. However, if the data set is sequential, you can add this listing to the data set
instead of replacing it by including the APNDLST process option in your profile data set.
If you enter the name of a data set that does not exist, SuperC allocates it for you. The data set
is allocated as a sequential data set unless you enter a member name after it, in which case it is
allocated as a partitioned data set.
Sequence Numbers
A value that tells SuperC whether to exclude sequence number fields from its comparison of your data
sets. This field is not displayed if the compare type is FILE or BYTE. You can choose one of these:
blank
Exclude Sequence Number fields from the comparison if the data set is F 80 or V 255 and the
compare type is
Line
Otherwise, treat as data.
SEQ
Exclude Sequence Number fields from the comparison. Sequence numbers are assumed in
columns 73-80 in F 80 and in columns 1-8 in V 255 data sets.
NOSEQ
Treat F 80/V 255 standard sequence number columns as data.
COBOL
Ignore columns 1-6 in F 80 data sets. Data in columns 1-6 is assumed to be sequence numbers.
SuperC member lists
The panel shown in Figure 110 on page 189 is displayed after you specify the old data set name, but only
if all these statements are true:
• The new data set is partitioned.
• The Member field, shown on the SuperC Utility panel (see Figure 108 on page 184), or the PDS Member
List field, shown on the SuperCE Utility panel (see Figure 113 on page 193) was left blank or a pattern
was used, and Bypass Selection List was not selected. For more information on Displaying Member
Lists, see the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I.
• The profile data set or statements data set being used does not contain any SELECT process
statements.
SuperC utility (option 3.12)
188  z/OS: z/OS ISPF User's Guide Vol II

## Page 227

Menu  Functions  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
         COMPARE    USERID.COPYBOOK                          Row 00001 of 00027
 Enter END command to process selections or CANCEL to leave the member list.
 Enter Old member(Oldmem) name if it is different from New member(Newmem) name.
   Newmem   Oldmem           Size    Created           Changed            ID
 . BIGCHAR                      4   2001/06/29   2002/02/25 10:42:27    USERID
 . BIGKSDS                      3   2001/08/10   2001/08/10 13:15:59    USERID
 . CONVT1                      24   2001/06/18   2001/06/18 16:04:26    USERID
 . COPYCONC                    12   2001/07/05   2001/07/05 17:33:41    USERID
 . COPYMM                       2   2001/06/11   2001/06/11 10:57:01    USERID
 . COPY01                       9   2001/02/24   2001/06/13 16:09:28    USERID
 . COPY0102                    15   2000/05/11   2001/06/11 11:08:49    USERID
 . COPY02                       7   2001/02/24   2001/02/24 17:09:50    USERID
 . DITTST1                     27   2001/06/13   2001/06/13 10:38:16    USERID
 . FLMLDATE                   443   2001/12/12   2001/12/12 12:41:44    USERID
 . FLMUDU                     415   2001/12/10   2001/12/10 20:44:55    USERID
 . FMNCCPY1                    35   2000/10/18   2002/09/10 17:18:42    USERID
 . FMNCCPY2                    35   2000/10/18   2002/09/10 17:19:11    USERID
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 110. SuperC member list panel (ISRSSML)
The members displayed in this list are members in the new data set. If the OLDMEM column is blank,
SuperC assumes each member in the new data set is to be compared with a member of the same name in
the old data set.
If you enter a member name in the OLDMEM column, SuperC compares this member to the one listed
beside it in the NEWMEM column.
To compare your selections, enter the END command. If you have not selected any members, ISPF
returns you to the previous panel.
To cancel your selections, enter either:
• The RESET command to remove all unprocessed selections without ending the member list display
• The CANCEL command to end the member list display without processing selections that are still on the
screen.
Note: Both the jump function (=) and the RETURN command cause an implied cancellation of selections
before they are carried out.
For more information about member lists, see the Using Member Selection Lists section of the "ISPF
Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I.
Submitting a SuperC job in batch mode
If you selected Batch Mode (2) on the SuperC Utility panel, the panel shown in Figure 111 on page 190 is
displayed before the job is submitted. This panel allows you to specify one of these:
• The SYSOUT class, which determines the printer to which your job is sent and the format used for the
printed output.
• The name of a listing data set.
• Output data definitions that you can use to give the printer additional instructions, such as an output
destination that is not defined by a SYSOUT class.
SuperC utility (option 3.12)
Chapter 5. Utilities (option 3)  189

## Page 228

SuperC Utility - Submit Batch jobs
                                                                    More:     +
 Press ENTER to continue submit
 Enter "/" to select option                   Generate Output Type:
 /  Edit JCL before user submit               1  1. SYSOUT Class
                                                 2. Data Set Name
                                                 3. //OUTDD DD
 SYSOUT Class . . . . A              
 Data Set Name  . . .                                                        
 //OUTDD DD . . . . .                                                        
 // . . . . . . . . .                                                        
                      LRECL for the Listing Output will be 133
   Job statement information: (Required - Enter/Verify JOB control statement)
 ===>                                                                        
 ===>                                                                        
 ===>                                                                        
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 111. SuperC Utility - Submit Batch Jobs panel (ISRSCSUB)
The "Job statement information" field is explained in the Job Statement Information section of the "ISPF
Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I. The other fields on the panel shown
in Figure 111 on page 190 are:
SYSOUT class
A system output classification defined by your installation, which defines certain print characteristics,
such as the printer and the format that is used to produce the output. You can enter any valid SYSOUT
parameters. This field is required if you leave the Option field blank.
If you enter either option 1 or option 2, the "SYSOUT class" field is ignored. However, for option 2, you
can include the SYSOUT= operand in an OUTDD DD field.
Data Set Name
The name of the listing data set that you want ISPF to store your compare results in. This data set can
be either partitioned or sequential.
The Data Set Name field is required if you use option 1 on this panel. This field serves the same
purpose as the Listing DS Name field, which is used when running the SuperC utility in foreground
mode.
The logical record length (LRECL) of the listing data set is displayed under the blank OUTDD lines on
the SuperC Utility - Submit Batch Jobs panel. SuperC creates listings with one of four LRECLs:
132
Standard listing for the NOPRTCC process option; printer control characters are omitted.
133
Standard listing.
202
Wide listing for the NOPRTCC process option; printer control characters are omitted.
203
Wide listing.
If you specify an existing sequential data set with an incorrect LRECL, SuperC overrides the data set
specifications. This applies to any listing and update data sets in both foreground and batch.
A separate operation, such as using the Hardcopy utility (option 3.6), is needed to print the listing data
set.
If you leave the Option field blank or enter option 2, the Data Set Name field is ignored. Therefore, to
specify an output data set in either of these two situations, you must include the DSN= operand in an
OUTDD DD field.
SuperC utility (option 3.12)
190  z/OS: z/OS ISPF User's Guide Vol II

## Page 229

When you are specifying the name of an existing data set, these rules apply:
• When you submit JCL for processing, the output listing produced by that JCL usually replaces the
contents of the specified data set, if any exist. Therefore, be careful when specifying the name of an
existing data set.
You can keep a history of changes by using the APNDLST compare option when you run the
comparison. This compare option adds the new output listing to the contents of the specified
sequential data set instead of replacing it.
Note: Using the APNDLST process option with a packed output listing file may cause unpredictable
results in the output listing file.
• Use standard TSO data set naming conventions.
When you are specifying the name of a data set that does not exist, these rules apply:
• If you include a member name in the data set specification, ISPF allocates a partitioned data set
with suitable attributes for the listing.
• If you do not specify a member name, ISPF allocates a sequential data set.
/ /OUTDD DD
Output data definitions that are used to specify additional printer instructions in job control language
(JCL). This field is required if you use this panel. Otherwise, it is ignored.
The OUTDD DD fields are provided so you can pass to your printer all the JCL needed to format
special types of output that may not be supported by your installation's SYSOUT class definitions. The
example shown in Figure 111 on page 190 specifies a wide format for printing on 14 3/4-inch forms.
The "SYSOUT class" and Data Set Name fields are ignored. If you need to specify this information,
be sure to include it in your OUTDD DD job card. If you specify a data set name in your OUTDD DD
job card, the output data set is printed and kept. Otherwise, it is printed and deleted. Here are some
examples:
• To specify a SYSOUT class, enter:
//OUTDD DD SYSOUT=X
where X is the SYSOUT class, such as A, B, or C.
• To specify a data set name, enter:
//OUTDD  DD  DSN=fully.qualified.name
//           DISP=XXXXX...
where XXXXX... is one of these:
– For an old data set:
OLD
– For a new sequential data set:
(NEW,CATLG),SPACE=(3325,(50,100),RLSE),UNIT=SYSDA
– For a new partitioned data set
(NEW,CATLG),SPACE=(3325,(50,100,25)),UNIT=SYSDA
– For a sequential data set that will be modified by, instead of replaced by, the comparison results:
MOD
Note: These three fields are independent of one another. Also, none of them requires you to provide an
OUTDD card in the "Job statement information" field.
SuperC utility (option 3.12)
Chapter 5. Utilities (option 3)  191

## Page 230

Using the NOLIST listing type in batch mode
If you enter the NOLIST listing type and choose batch mode, the options on the SuperC Utility - Submit
Batch Jobs panel shown in Figure 111 on page 190 are not valid because no listing is produced.
Therefore, an alternate panel is displayed, which blanks out the fields that are not valid but still allows you
to submit job statement JCL. This panel is shown in Figure 112 on page 192.
                             Batch Submit - Nolist
 Press ENTER to continue submit job or END to Cancel
 Enter "/" to select option
 /  Edit JCL before user submit
   NOLIST listing type was specified.  There will be no output generated.
   Job statement information: (Required - Enter/Verify JOB control statement)
 ===>                                                                        
 ===>                                                                        
 ===>                                                                        
 ===>                                                                        
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 112. SuperC Utility - submit batch jobs panel using NOLIST (ISRSCSB1)
When this panel is displayed, you can either:
• Type the job statement JCL and press Enter to submit the job
• Enter the END command to cancel the job.
SuperCE utility (option 3.13)
The SuperCE utility (option 3.13) is a dialog that uses the SuperC program to compare data sets of
unlimited size and record length at the file, line, word, or byte level. It is appropriate if you need more
flexibility than the standard SuperC utility (option 3.12) provides.
Note: For an introduction to the SuperC and SuperCE utilities (options 3.12 and 3.13), see Appendix A,
“SuperC reference,” on page 431.
The panel shown in Figure 113 on page 193 is the first panel of the SuperCE utility. It requires only the
names of the input data sets, which are entered using standard TSO naming conventions, such as:
New DS Name  . . . . 'USERID.TEST2.SCRIPT'
Note: When a member of a PDSE version 2 data set that is configured for member generations is specified
as the old or new data set, the current generation of the member is used for the comparison.
SuperCE utility (option 3.13)
192  z/OS: z/OS ISPF User's Guide Vol II

## Page 231

Menu  Utilities  Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                                SuperCE Utility
 New DS Name  . . .                                                        
 Old DS Name  . . .                                                        
 PDS Member List              (blank/pattern - member list, * - compare all)
    (Leave New/Old DSN "blank" for concatenated-uncataloged-password panel)
    Compare Type               Listing Type               Display Output
    2  1. File                 2  1. OVSUM                1  1. Yes
       2. Line                    2. Delta                   2. No
       3. Word                    3. CHNG                    3. Cond
       4. Byte                    4. Long                    4. UPD
                                  5. Nolist
 Listing DSN  . . . . SUPERC.LIST                                            
 Process Options  . .                                          
                                                               
 Statements Dsn . . .                                                        
 Update DSN . . . . .                                                        
 Enter "/" to select option      Execution Mode          Output Mode
    Bypass selection list        1  1. Foreground        1  1. View
                                    2. Batch                2. Browse
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 113. SuperCE Utility panel (ISRSEPRI)
SuperCE Utility panel action bar
The SuperCE Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Options
The Options pull-down offers you these choices:
1
Edit Statements
2
Process Options
3
Profiles and Defaults
Help
The Help pull-down provides general information about SuperCE topics, including how to specify the
input data sets and options.
SuperCE Utility panel fields
A default compare type, listing type, listing data set name, and Browse option are provided if you choose
not to specify your own. The fields on the SuperCE Utility panel are:
New DS Name and Old DS Name
Specify the name of a sequential data set, PDS, or membered PDS. Use standard TSO naming
conventions, including quotes for fully qualified names. Leave either or both of these fields blank
to display a panel on which you can specify concatenated, uncataloged, and password-protected data
sets. These panels are shown in Figure 114 on page 197 (foreground compare) and Figure 115 on
page 197 (batch compare).
SuperCE utility (option 3.13)
Chapter 5. Utilities (option 3)  193

## Page 232

PDS Member List
Leave this field blank to display a member selection list for the new data set. Otherwise, enter either a
pattern or an asterisk (*). See “SuperC member lists” on page 188 for more information.
pattern
Entering a pattern causes ISPF to display a list of the members in the new data set that match
the pattern, unless Bypass Selection List has been specified. For more information about using
patterns, see the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I. For
example:
PDS Member List  . . ISR*
*
Entering an asterisk causes all the members in the new data set to be compared to any like-
named members in the old data set. A member list is not displayed. For example:
PDS Member List  . . *
Members in either data set not having like-named members in the other data set are not
compared, but are listed in the output list data set.
When entire data sets are compared by using an asterisk for a member name pattern, each real
member that appears in both the old and new data sets is compared once. Alias entries are
processed but only to determine if they have matching alias and/or real entries.
Note: You can also use SELECT process statements in the statements data set to specify an optional
set of PDS members to be searched. However, the SELECT statement turns off the PDS member list
function.
Compare Type
The type of comparison you want SuperC to perform. Choose one of these:
File
Compares source data sets for differences, but does not show what the differences are. This is
the simplest and fastest method with the least amount of processing overhead. For this compare
type, SuperC prepares summary information only and causes all listing types to produce the same
output, except NOLIST, which does not produce any output listing. A message is returned to notify
you of the compare results.
Line
Compares source data sets for line differences. Reformatted lines (that is, lines with blanks
inserted or deleted) are automatically detected for lines less than or equal to 256 characters. This
compare type is the default. It is most useful for comparisons of program source code because it
is record-oriented and points out inserted or deleted lines of code. Lines can be of unlimited size.
Word
Compares source data sets for word differences. In this context, a word is a group of characters
that begins and ends with a blank or other line delimiter. If you use the XWDCMP process option,
all non-alphanumeric characters are considered to be delimiters. Also, a word cannot be longer
than 256 characters.
The Word compare type is most useful for comparing text data sets. If two data sets contain the
same words in the same order, SuperC considers them to be identical, even if those words are not
on the same lines.
Byte
Compares source data sets for byte differences. The output listing data set consists of a
hexadecimal printout with character equivalents listed on the right. A BYTE compare with a LONG
listing of a data set against itself results in a hexadecimal dump of that data set. This compare
type is most useful for comparing machine readable data.
SuperCE utility (option 3.13)
194  z/OS: z/OS ISPF User's Guide Vol II

## Page 233

Listing Type
The type of listing you want SuperC to create when it compares the data sets. Listing Type is not a
required field in SuperCE. If you do not specify a listing type, the default is DELTA. See the topic about
Listing Formats in the z/OS ISPF User's Guide Vol I for sample listings.
OVSUM
Lists only the general summary of the comparison. However, a PDS comparison generates an
individual summary line for each PDS member.
Delta
Lists the differences between the source data sets, followed by the general summary.
CHNG
Lists the differences between the source data sets, plus up to 10 matching lines before and after
the differences. This listing is a variation of the DELTA listing; the matching lines before and after
help you recognize changed areas of the source data sets.
Long
Lists all the new data set source lines, plus old data set deleted lines. Both inserted and deleted
lines are flagged.
Nolist
Produces no listing output. In foreground mode, a message is returned to show the outcome of
the comparison.
Listing Dsn
The name of the list data set to which SuperC writes the results of the comparison. However, if you
enter NOLIST in the Listing Type field, SuperC does not create an output listing, so this name is
ignored.
If you leave this field blank, SuperC allocates a list data set, using default data set attributes and this
data set name:
prefix.userid.SUPERC.LIST
where pr efix  is your TSO prefix and userid is your user ID. If your prefix and user ID are identical, only
your prefix is used. Also, if you do not have a prefix, only your user ID is used.
Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set to
YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER keyword is
included before the SUPERC qualifier.
If you enter a fully qualified data set name SuperC uses it as specified. Otherwise, SuperC only
appends your TSO prefix to the front of the data set name specified. If you run with TSO PROFILE
NOPREFIX, SuperC uses the name as you entered it, which can result in an attempt to catalog the
name in the master catalog.
If you enter the name of a data set that already exists, the contents of that data set are replaced by
the new output listing. However, if the data set is sequential, you can add this listing to the data set
instead of replacing it by using the APNDLST process option.
If you enter the name of a data set that does not exist, SuperC allocates it for you. The data set
is allocated as a sequential data set unless you enter a member name after it, in which case it is
allocated as a member of a partitioned data set.
Process Options
Keywords that tell SuperC how to process the compare operation. You can type these keywords in the
Process Options fields or select them from a panel. See “Process options” on page 434 for a table of
keywords.
Statements Dsn
The name of the data set that contains your process statements. All statements data sets must be
fixed block with 80-byte records (FB 80). See “Edit statements - edit statements data set” on page
199 for more information.
SuperCE utility (option 3.13)
Chapter 5. Utilities (option 3)  195

## Page 234

Update Dsn
Tells SuperC the name of the data set that will contain column-oriented results of the comparison.
This data set is normally used as input to post processing programs and can be specified besides the
normal listing data set.
If you leave this field blank and use an update (UPDxxxx) option, SuperC uses this default name:
prefix.userid.SUPERC.UPDATE
where pr efix  is your TSO prefix and userid is your user ID. If your prefix and user ID are identical, only
your prefix is used. Also, if you do not have a prefix, only your user ID is used.
Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is
included before the SUPERC qualifier.
If you enter a fully qualified data set name SuperC uses it as specified. Otherwise, SuperC only
appends your TSO prefix to the front of the data set name specified. If you run with TSO PROFILE
NOPREFIX, SuperC uses the name as you entered it, which can result in an attempt to catalog the
name in the master catalog.
If you enter the name of a data set that already exists, the contents of that data set are replaced by
the new update output. However, if the data set is sequential, you can add this listing to the data set
instead of replacing it by using the APNDUPD process option.
If you enter the name of a data set that does not exist, SuperC allocates it for you. The data set
is allocated as a sequential data set unless you enter a member name after it, in which case it is
allocated as a partitioned data set.
Note: For the UPDMVS8, UPDCMS8, UPDSEQ0, and UPDPDEL process options, the update data set
contains valid data, but only after a successful compare when differences are detected. The data set
is always empty after a comparison that shows the data sets or members being compared have no
differences.
Display Output
Tells ISPF whether you want to display the output listing in Browse mode. Enter one of these:
Note: The NOLIST listing type overrides Yes, No, and Cond.
Yes
Call Browse to display the listing data set after processing the comparison. This is the default.
No
Do not call Browse to display the SuperC listing data set.
Cond
Do not call Browse unless SuperC finds differences between the data sets.
UPD
Browse the update data set instead of the list data set. This parameter is not valid unless you
create an update data set by using one or more of the SuperC process options that begin with UPD
(UPDxxxx).
Bypass Selection List
When a member pattern is entered in the PDS Member List field, selecting this field causes SuperC
to process all members matching that pattern without displaying a member selection list. Leaving this
field blank causes the member list to be displayed.
Execution Mode
Foreground
If you choose Foreground, SuperC processes the data sets in foreground mode, so you can
browse the results of the compare. This choice locks your keyboard until SuperC processing is
complete.
SuperCE utility (option 3.13)
196  z/OS: z/OS ISPF User's Guide Vol II

## Page 235

The panel shown in Figure 114 on page 197 is displayed if you specify Foreground in the
Execution Mode field and you leave the New DS Name or Old DS Name field blank on the SuperCE
Utility panel.
                     SuperCE - Concatenation Foreground Entry
                               "New" Concatenation
    DS1 . . .                                                        
    DS2 . . .                                                        
    DS3 . . .                                                        
    DS4 . . .                                                        
                 Other "New" Partitioned, Sequential or VSAM Data Set
 Data Set Name . . .                                                        
 Volume Serial . . .           (If not cataloged)
 Password  . . . . .           (Password allowed only in foreground mode)
                               "Old" Concatenation
    DS1 . . .                                                        
    DS2 . . .                                                        
    DS3 . . .                                                        
    DS4 . . .                                                        
                 Other "Old" Partitioned, Sequential or VSAM Data Set
 Data Set Name . . .                                                        
 Volume Serial . . .           (If not cataloged)
 Password  . . . . .           (Password allowed only in foreground mode)
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 114. SuperCE - concatenation interactive entry panel (ISRSECAT)
Batch
If you choose Batch, SuperC processes the data sets in batch mode. This choice frees the
keyboard, allowing you to continue using ISPF while waiting for SuperC to compare the data
sets. The output listing is sent to the destination specified on the SuperC Utility - Submit Batch
Jobs panel (Figure 111 on page 190).
The panel shown in Figure 115 on page 197 is displayed if you specify Batch in the Execution
Mode field and you leave the New DS Name or Old DS Name field blank on the SuperCE Utility
panel. You can concatenate up to four data sets that have like attributes. For example, all must be
either sequential or partitioned.
                     SuperCE - Concatenation Batch Entry
                               "New" Concatenation
    DS1 . . .                                                        
    DS2 . . .                                                        
    DS3 . . .                                                        
    DS4 . . .                                                        
                 Other "New" Partitioned, Sequential or VSAM Data Set
 Data Set Name . . .                                                        
 Volume Serial . . .           (If not cataloged)
 Password  . . . . .           (Password allowed only in foreground mode)
                               "Old" Concatenation
    DS1 . . .                                                        
    DS2 . . .                                                        
    DS3 . . .                                                        
    DS4 . . .                                                        
                 Other "Old" Partitioned, Sequential or VSAM Data Set
 Data Set Name . . .                                                        
 Volume Serial . . .           (If not cataloged)
 Password  . . . . .           (Password allowed only in foreground mode)
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 115. SuperCE - Concatenation Batch Entry panel (ISRSECAT)
SuperCE utility (option 3.13)
Chapter 5. Utilities (option 3)  197

## Page 236

This panel is the same as the panel shown in Figure 114 on page 197, except the Password field
is used only in foreground mode. If your data sets are password protected, compare the data sets
in foreground mode by specifying Foreground in the Execution Mode field on the SuperCE Utility
panel.
Printing a SuperCE listing in batch mode:
If you specify Batch in the Execution Mode field on the SuperCE Utility panel, the panel shown
in Figure 111 on page 190 is displayed before the job is submitted. This panel allows you to
determine whether to print your SuperC listing or write it to a list data set.
Output Mode
The output mode for displaying the listing file. Choose one of these:
1
View. This enables the listing file to be displayed in view mode. All View functions are enabled in
this mode.
2
Browse. This enables the listing file to be displayed in the browse mode. All Browse functions are
enabled in this mode.
3
Eview. This option only appears on non-English panels. It operates exactly the same as View
except that SuperC is invoked with an English language constants module. All titles and headings
are in English. This facilitates use of hiliting of SuperC listings on non-DBCS terminals.
SuperCE Utility primary commands
The SuperCE utility provides the functions described in these topics, each of which is controlled by a
command that you can type on the command line:
A – Profile Manager
When you enter primary command A on the SuperCE Utility panel, the Profile Manager panel is
displayed. See “Profiles and defaults - activate profiles and defaults” on page 200 for information
related to the Profile Manager panel.
B – Batch
When you enter primary command B on the SuperCE Utility panel, processing is the same as when
you press Enter with Batch specified in the Execution Mode field. The value specified in the Execution
Mode field is ignored. See the section describing the Batch option in the Execution Mode field for
information on batch processing.
E – Edit statements
When you enter primary command E on the SuperCE Utility panel, the statements data set that you
specified in the Statements Dsn field is displayed in Edit mode. See “Edit statements - edit statements
data set” on page 199 for information related to the edit statements data set.
P – Process options
When you enter primary command P on the SuperCE Utility panel, a Compare Process Options panel
is displayed. This panel contains the process options that are available for the compare type (File,
Line, Word, or Byte) that is selected. See “Process options - select process options” on page 199 for
information related to the Compare Process Options panel.
S – Extended Search-For Utility
When you enter primary command S on the SuperCE Utility panel, the Extended Search-For Utility
panel is displayed. See “Search-ForE utility (option 3.15)” on page 209 for information related to the
Extended Search-For Utility.
SuperCE utility options
These topics describe the options that are available in the Options pull-down on the SuperCE Utility panel
action bar:
• “Process options - select process options” on page 199
SuperCE utility (option 3.13)
198  z/OS: z/OS ISPF User's Guide Vol II

## Page 237

• “Edit statements - edit statements data set” on page 199
• “Profiles and defaults - activate profiles and defaults” on page 200
Process options - select process options
When you select Process Options from the Options pull-down menu, a Compare Process Options panel
is displayed. This panel contains the process options that are available for the compare type (File, Line,
Word, or Byte) that is selected. You can also access the Compare Process Options panel by entering the
primary command P on the SuperCE Utility panel.
The compare type that you select determines the available process options:
Line Compare
ALLMEMS ANYC    APNDLST APNDUPD ASCII   CKPACKL CNPML   COBOL   COVSUM
Cpnnnnn DLMDUP  DLREFM  DPACMT  DPADCMT DPBLKCL DPCBCMT DPCPCMT DPFTCMT
DPMACMT DPPLCMT DPPSCMT EMPTYOK FMVLNS  GWCBL   LOCS    LONGLN  MIXED
NARROW  NOPRTCC NOSEQ   NOSUMS  REFMOVR SEQ     UPDCMS8 UPDCNTL UPDLDEL
UPDMVS8 UPDPDEL UPDREV  UPDREV2 UPDSEQ0 UPDSUMO VTITLE  WIDE    Y2DTONLY
Word Compare
ALLMEMS ANYC    APNDLST APNDUPD ASCII   CKPACKL COBOL   COVSUM  Cpnnnnn 
DPACMT  DPADCMT DPBLKCL DPCBCMT DPCPCMT DPFTCMT DPMACMT DPPLCMT DPPSCMT 
EMPTYOK GWCBL   LOCS    MIXED   NOPRTCC NOSEQ   NOSUMS  SEQ     UPDCNTL    
UPDREV  UPDREV2 UPDSUMO VTITLE  XWDCMP
Byte Compare
ALLMEMS APNDLST APNDUPD ASCII   COVSUM  Cpnnnnn EMPTYOK LOCS    NOPRTCC
NOSUMS  UPDCNTL UPDSUMO VTITLE
File Compare
ALLMEMS APNDLST ASCII   COVSUM  Cpnnnnn EMPTYOK FMSTOP  LMCSFC  LOCS
NOPRTCC
To select one or more SuperCE process options, perform either of these actions:
• Type any nonblank character to the left of the process options you want to select. Use the Backward
and Forward keys, as necessary, to move through the panel. Press Enter when you have finished. This
causes the options you chose to be displayed in the Process Options fields on the SuperCE Utility panel.
If you select two options that cannot be chosen together, or if you enter an option name incorrectly, an
error message is displayed.
• Use the CANCEL command to return to the SuperCE Utility panel without processing selections.
SuperC process options can affect how the input data is processed, and determine the format and content
of the output listing data set. They can also help you save processing time by avoiding comments and
blank lines. A separate group of options, called update data set options (UPDxxxx), allow you to create
update data sets, examples of which are shown in Appendix C, “Update files,” on page 493.
All these options can be chosen from the XXXX Compare Process Options panels, where XXXX is the
compare type (FILE, LINE, WORD, or BYTE) that you are using, or you can type any of them in the Process
Options field on the SuperCE Utility panel. Errors caused by mistyping process options are detected when
you call the SuperCE utility.
For definitions of the SuperC process options, see “Process options” on page 434.
Edit statements - edit statements data set
A statements data set consists of process statements that contain instructions for the SuperC program.
They are similar to the process options, but are composed of a keyword and one or more operands. See
“Process options - select process options” on page 199 for information about SuperCE process options.
SuperCE utility (option 3.13)
Chapter 5. Utilities (option 3)  199

## Page 238

When you select the Edit Statements option from the SuperCE Utility Options pull-down menu, the
SuperCE utility displays the statements data set you specified in the Statements Dsn field. You can also
display the statements data set by entering the primary command E on the SuperCE Utility panel. The
statements data set is always displayed in Edit mode, allowing you to add, change, or delete SuperC
process statements as needed. Only one process statement can appear on each line of the statements
data set.
The size of the Edit window depends on the number of lines your terminal can display. The sample panel
shown in Figure 116 on page 200 shows how the Edit window appears on a 24-line display. Examples
of some common process statements are listed below the Edit window so you can easily compose the
proper input line.
            USERID.SUPERC.STMTS                             Columns 00001 00072
    Enter or change Process Statements in the EDIT window below:
 ****** ***************************** Top of Data ******************************
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ****** **************************** Bottom of Data ****************************
    Examples                    Explanation
 CMPCOLM 5:60  75:90         Compare using two column compare ranges
 LSTCOLM 25:90               List columns 25:90 from input
 DPLINE 'PAGE '              Exclude line if "PAGE " found anywhere on line
 SELECT  MEM1,NMEM2:OMEM2    Compare MEM1 with MEM1 and NMEM2 with OMEM2
 CMPLINE NTOP 'MACRO'        Start comparing after string found in new DSN
 LNCT    66                  Set lines per page to 66
                         - - - - -
 Others: CHNGV    CMPBOFS CMPCOLMN CMPCOLMO CMPSECT DPLINEC NCHGT
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 116. SuperC process statements panel (ISRSEPRS)
The SuperC program validates process statements at run time. Invalid process statements are not used
and are noted at the bottom of the listing. Unless a higher return code is required by some other
condition, a return code of 4 is generated.
See “Process statements” on page 445 for process statement syntax, definitions, and examples.
Profiles and defaults - activate profiles and defaults
A SuperC profile is a data set that can contain a compare type, a listing type, a Browse setting, and various
combinations of process options and process statements that you select.
SuperC profiles are useful for a wide range of users. Beginners can use profiles created by others as a
simple method of running SuperC. Experienced SuperC users can create profiles for the groups of options
they use often so that they do not have to remember individual process options and statements. Also,
profiles give system programmers a mechanism for setting up complex compare tools that others can
simply call by profile name.
Some other characteristics of profiles are:
• A profile can be either a sequential data set or a member of a PDS.
• Data set names are not represented in a profile.
• Profiles can be created only with the SuperCE utility (option 3.13). However, once they are created, they
can be used in the standard SuperC utility (option 3.12).
• To change a profile, activate it by selecting the Activate option on the Profile Manager panel and make
the necessary changes to the information in the fields on the SuperCE Utility panel. Then select the
Create option on the Profile Manager panel, entering in the Activate/Create Profile DS Name field the
name of the profile data set that you want to modify.
SuperCE utility (option 3.13)
200  z/OS: z/OS ISPF User's Guide Vol II

## Page 239

• You can modify the SuperC default settings by selecting the Defaults option on the Profile Manager
panel. See Figure 119 on page 203 for an example of the SuperC - Defaults panel.
• You can display the contents of a profile data set using View, Browse, or Edit. Figure 117 on page 201
shows a Browse display of a profile data set.
   Menu  Utilities  Compilers  Help
 ───────────────────────────────────────────────────────────────────────────────
            USERID.TESTPROF                           Line 00000000 Col 001 080
********************************* Top of Data **********************************
.* PROF PREFIX CTYP=LINE,LTYP=DELTA ,BRW=YES
.* PROF PREFIX PROC1=                                          * MARGIN*
.* PROF PREFIX PROC2=                                          * MARGIN*
******************************** Bottom of Data ********************************
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 117. Browse a SuperCE pr o file 
When you select Profiles and Defaults from the Options pull-down menu, the Profile Manager panel is
displayed. You can also access the Profile Manager panel by entering the primary command A on the
SuperCE Utility panel. The panel is used to activate and create profiles and to modify SuperC default
values.
                           SUPERCE - Profile Manager
 A  Activate    Reads the specified input profile data set:
                1.  Establishes the process and compare options from the
                    profile prefix lines.
                2.  Establishes the profile as the process statement data set
                    if any process statements are detected.
 C  Create      Creates an output profile data set:
                1.  Combines process and compare options from the Primary Panel
                    and any process statements from the Statements Data Set:
                    SUPERC.STMTS
                2.  Rewrites the profile data set (if the data set exists) or
                    allocates a new data set before generating the profile.
 D  Defaults    Presents panel for modifying SuperC defaults.
 Activate/Create
 Profile DS Name . . .                                                        
 Option ===>                                                                  
  F1=Help    F2=Split   F3=Exit    F9=Swap   F12=Cancel
Figure 118. SuperCE - P r o file  Manager panel (ISRSEPMG)
The only field on this panel is:
Activate/Create Profile DS Name
The name of the profile data set that you want to either activate or create. This field is required when
you choose option A (Activate) or C (Create).
These topics describe the options shown at the top of the SuperCE - Profile Manager panel:
SuperCE utility (option 3.13)
Chapter 5. Utilities (option 3)  201

## Page 240

• “A — activate” on page 202
• “C — create” on page 202
• “D — defaults” on page 202
A — activate
Option A (Activate) uses the contents of the profile data set specified in the Activate/Create Profile DS
Name field to populate fields on the SuperCE Utility panel. For example, process options stored in the
profile appear in the Process Options fields. When you choose option A, the profile data set that you enter
in the Activate/Create Profile DS Name field must be cataloged.
C — create
Option C (Create) causes SuperCE to copy data entered on the SuperCE Utility panel and place it in the
profile data set specified in the Activate/Create Profile DS Name field. Be sure the correct information is
displayed on that panel and that the statements data set, if you specify one, contains the correct process
statements before you create the profile.
If the profile data set that you specify does not already exist, SuperCE allocates it for you. Data stored in
the profile data set can include:
• These values taken from the fields on the SuperCE Utility panel. The abbreviations in parentheses show
how these values are identified in a profile data set:
– Compare type (CTYP)
– Listing type (LTYP)
– Browse setting (BRW)
– Process options (PROC1 and PROC2).
• Process statements copied from the statements data set that was specified in the Statements Dsn field.
This data set name is displayed and highlighted on the SuperCE - Profile Manager panel. For example,
the sample panel shown in Figure 118 on page 201 displays the name SUPERC.STMTS.
If you leave the Statements Dsn field blank, the data set name is not displayed on the SuperCE - Profile
Manager panel and SuperCE does not include any process statements in your profile. See these topics
about process options and process statements, respectively:
– “Process options - select process options” on page 199
– “Edit statements - edit statements data set” on page 199
D — defaults
Option D (Defaults) brings up the SUPERC – Defaults panel, shown in Figure 119 on page 203, that allows
you to:
• Specify SuperC output data set default allocation parameters
The first extent and secondary space values are used whenever Options 3.12, 3.13, or 3.14 create
a new output data set such as a listing or statements data set. If you specify a new data set with a
member name, the directory space value is used to create a PDS. If you blank out any of the values,
SuperC will supply defaults.
Space values are applicable only if you select "Invoke SuperC via PROGRAM interface".
Note: New data set allocation block size parameters are controlled by the ISPF Configuration Table. See
z/OS ISPF Planning and Customizing for details.
• Specify your own Statements data set initial edit macro name
• Enable or disable a high performance program interface to SuperC. If you select "Invoke SuperC via
PROGRAM interface", ISPF invokes SuperC directly. Otherwise, ISPF invokes SuperC via a CLIST named
ISRSFORG (ISRSSRCH for Search-For). The CLIST interface may be useful if you need to customize
SuperCE utility (option 3.13)
202  z/OS: z/OS ISPF User's Guide Vol II

## Page 241

the allocations or wish to post-process the result. The PROGRAM interface is more efficient and is the
default.
                               SUPERC - Defaults
         Verify entries below. End or Enter to exit.
 New List data set allocation in blocks:
 1st Extent . . 50     Secondary . . 100    Directory . . 5  
 New Update data set allocation in blocks:
 1st Extent . . 15     Secondary . . 30     Directory . . 5  
 New Profile data set allocation in blocks:
 1st Extent . . 5      Secondary . . 5      Directory . . 5  
 New Statements data set allocation in blocks:
 1st Extent . . 5      Secondary . . 5      Directory . . 5  
 Statements data set initial edit macro name . . !ISRSMAC
 Enter "/" to select option
 /  Invoke SuperC via PROGRAM interface
 Command ===>                                                                 
  F1=Help    F2=Split   F3=Exit    F9=Swap   F12=Cancel
Figure 119. SuperC - Defaults panel (ISRSDFLT)
Search-For utility (option 3.14)
Note: For an introduction to the Search-For and Extended Search-For utilities (options 3.14 and 3.15), see
Appendix A, “SuperC reference,” on page 431.
The Search-For utility (option 3.14) is a dialog that uses the SuperC program to search your data sets or
PDS members for one or more character strings. The Search-For Utility panel, shown in Figure 120 on
page 204, is the first panel of the Search-For utility. The only requirements for this panel are:
• A string to be searched for, unless you select "Specify additional search strings"
• A data set to search, along with a volume serial and password if necessary.
A default listing data set name is provided if you choose not to enter your own.
Note: When member generations of a PDSE version 2 data set are searched for character strings, only
members of the current generation are searched.
Search-For utility (option 3.14)
Chapter 5. Utilities (option 3)  203

## Page 242

Menu  RefList  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
                               Search-For Utility
                                                                    More:     +
 Search String  . .                                                            
 ISPF Library:
    Project . . .         
    Group . . . .          . . .          . . .          . . .         
    Type  . . . .         
    Member  . . .                 (Blank or pattern for member selection list,
                                    "*" for all members)
 Other Partitioned, Sequential or VSAM Data Set:
    Data Set Name . . .                                                        
    Volume Serial . . .           (If not cataloged)
 Listing Data Set . . . SRCHFOR.LIST                                           
 Data Set Password  . .           (If Search-For data set password protected)
 Enter "/" to select option               Execution Mode        Output Mode
    Specify additional search strings     1  1. Foreground      1  1. View
    Mixed Mode                               2. Batch              2. Browse
    Bypass selection list
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 120. Search-For Utility panel (ISRSFSPR)
Search-For Utility panel action bar
The Search-For Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Help
The Help pull-down provides general information about Search-For topics, including how to specify
the input data sets, search string, and options.
Search-For Utility panel fields
All the fields on this panel are explained in the Libraries and Data Sets topic in the z/OS ISPF User's Guide
Vol I, except these:
Search String
A string to be searched for. No distinction is made between uppercase and lowercase characters. Use
the Extended Search-For utility (option 3.15) to specify case-sensitive searches.
Four keywords—C, PREFIX, SUFFIX, and WORD—can help you narrow the scope of a search. See
“Search-For strings and keywords” on page 206 for information about these keywords and the rules
that govern search string entry.
Specify additional search strings
Select this field to have the Search-For utility search for more than one string. The Search-For utility
displays the panel shown in Figure 121 on page 206, on which you can specify additional search
strings. This panel precedes a member list request.
If you do not select this option, the Search-For utility searches only for the string entered in the
Search String field.
Search-For utility (option 3.14)
204  z/OS: z/OS ISPF User's Guide Vol II

## Page 243

Mixed Mode
Select this field to have the Search-For utility scan and parse the input data set lines for DBCS text
strings.
Note: The Word, Prefix, and Suffix Search-For qualifiers have no effect on DBCS strings.
Bypass Selection List
When a member pattern is entered in the PDS Member List field or the member name portion of
the data set field (such as MY.DATA.SET(pattern)), selecting this field causes SuperC to process all
members matching that pattern without displaying a member selection list. Leaving this field blank
causes the member list to be displayed.
Execution Mode
The processing mode you want to use when searching the data sets. Specify one of these:
1
Foreground. Searches the data sets and stores the results in the data set specified in the Listing
Data Set Name field. You can browse the listing data set at the terminal.
2
Batch. Causes the display of the Search-For Utility - Submit Batch Jobs panel so that you can
specify job card and print disposition information or edit the JCL statements. Then, Search-For
submits the batch job to search the data sets. See “Submitting a Search-For job in batch mode” on
page 208 for more information.
Note: You cannot specify a data set password in batch mode. If your data sets are password
protected, use foreground mode.
Output Mode
The output mode you want to use when displaying the listing file. Choose one of these:
1
View. This enables the listing file to be displayed in view mode. All View functions are enabled in
this mode.
2
Browse. This enables the listing file to be displayed in the browse mode. All Browse functions are
enabled in this mode.
Listing Data Set
The name of the listing data set to which the SuperC program writes the results of the search. If
you leave this field blank, the Search-For utility allocates a listing data set, using default data set
attributes and this data set name:
prefix.userid.SRCHFOR.LIST
where pr efix  is your TSO prefix and userid is your user ID. If your prefix and user ID are identical, only
your prefix is used. Also, if you do not have a prefix, only your user ID is used.
Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is
included before the SRCHFOR qualifier.
If you enter a fully qualified data set name SuperC uses it as specified. Otherwise, SuperC only
appends your TSO prefix to the front of the data set name specified. If you run with TSO PROFILE
NOPREFIX, SuperC uses the name as you entered it, which can result in an attempt to catalog the
name in the master catalog.
If you enter the name of a data set that does not exist, the Search-For utility allocates it for you. The
data set is allocated as a sequential data set unless you enter a member name after it, in which case it
is allocated as a partitioned data set.
Search-For utility (option 3.14)
Chapter 5. Utilities (option 3)  205

## Page 244

Specifying additional search strings
The panel shown in Figure 121 on page 206 is displayed if you select "Specify additional search strings"
on the Search-For Utility panel. You can specify:
• Additional strings to be searched for
• Optional scan-type and continuation keywords.
   Menu  RefList  Utilities  Help
 ─ ┌───────────────────────────────────────────────────────────────────────┐ ──
 I │                          Search-For Strings                           │
   │                                                                       │
 S │                   Specify 1 or more Search Strings below:             │
   │                                                                       │
 I │   ==>                                                                 │
   │   ==>                                                                 │
   │   ==>                                                                 │
   │   ==>                                                                 │
   │   ==>                                                                 │ ,
   │   ==>                                                                 │
 O │   ==>                                                                 │
   │   ==>                                                                 │
   │   ==>                                                                 │
   │   ==>                                                                 │
 L │                                                                       │
 D │            Press ENTER to start search or END command to exit.        │
   │ Command ===>                                                          │
 E │  F1=Help       F2=Split      F3=Exit       F7=Backward   F8=Forward   │
 / │  F9=Swap      F10=Actions   F12=Cancel                                │
 C └───────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 121. Additional Search Strings panel (ISRSFSST)
From this panel, pressing Enter either:
• Displays a member list, if requested
• Runs the search if no member list is needed.
Entering the END command returns you to the Search-For Utility panel.
Search-For strings and keywords
Enter the strings that you want SuperC to find. When you press Enter, SuperC looks for the strings without
regard to whether they appear in uppercase or lowercase in the original data set. If you are searching a
partitioned data set concatenation, SuperC will inform you (in the CONCAT# field in the listing) in which
group it first found the string. If you want SuperC to search all the groups for each member, you can use
the SDUPM process option of Extended Search-For (option 3.15). If you want the search to distinguish
between uppercase and lowercase, you must use Extended Search-For (option 3.15). If any of the strings
are found (string-1 OR string-2 OR string-3) on the line, SuperC considers the condition met. You can
restrict SuperC searches further by using one of the SuperC Search-for keywords discussed in “Using
keywords” on page 207.
Entering search strings
Enclose the string in single quotation marks if it contains embedded blanks or apostrophes. Two
consecutive apostrophes must be entered to specify a single apostrophe within a search string.
If you need to specify a DBCS string that contains a hexadecimal '7D' (x'7D', the hexadecimal
representation of a single quotation mark) as half of a DBCS pair, you must use the Enhanced SearchFor
option (option 3.15) with the MIXED process option.
This example searches for the string IT'S A LIVING.
==> 'IT'S A LIVING'
Search-For utility (option 3.14)
206  z/OS: z/OS ISPF User's Guide Vol II

## Page 245

Using keywords
These keywords can help you narrow the range of the search. If you do not use a keyword, SuperC will
find the string wherever it exists, even if that happens to be in the middle of a word.
PREFIX
Shows the string is preceded by a non-alphanumeric character, such as a blank space. It cannot be
used on the same line with SUFFIX or WORD. For example, you can do this:
==> ELSE PREFIX
==> ELSE SUFFIX
but not this:
==> ELSE PREFIX SUFFIX
SUFFIX
Shows the string is followed by a non-alphanumeric character. It cannot be used on the same line with
PREFIX or WORD. See the examples under PREFIX.
WORD
Shows the string is both preceded and followed by a non-alphanumeric character. It cannot be used
on the same line as PREFIX or SUFFIX. See the examples under PREFIX.
C
Continuation. Shows continuation of the previous line(s). Continuation lines generate additional
strings, all of which must be found in the same line of an input data set.
Also, the C keyword can be entered on the same line as one of the other keywords. This example tells
SuperC to find ELSE and to also find IF, but only when IF is on the same line as ELSE.
==> ELSE WORD
==> IF WORD C
Search-For member lists
A panel similar to the one shown in Figure 122 on page 208 is displayed only if:
• The search data set is partitioned.
• The Member field on the Search-For Utility panel (Figure 120 on page 204) or the PDS Member List field
on the Extended Search-For Utility panel (Figure 124 on page 209) was left blank or a pattern was used
and Bypass Selection List was not selected. For more information on Displaying Member Lists, see the
"ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I.
• For the Extended Search-For utility (option 3.15), the statements data set being used does not contain
any SELECT process statements.
Note: When member generations of a PDSE version 2 data set are searched for character strings, only
members of the current generation are searched.
Search-For utility (option 3.14)
Chapter 5. Utilities (option 3)  207

## Page 246

Menu  Functions  Utilities  Help
 ───────────────────────────────────────────────────────────────────────────────
         SEARCH      USERID.DATASET                          Row 00001 of 00026
 Enter END command to process selections or CANCEL to leave the member list.
    Name     Prompt          Size    Created           Changed            ID
 . COPY01T                     10   2002/09/06   2002/09/17 12:56:47    USERID
 . COPY01TM                    10   2002/08/26   2002/08/28 18:23:19    USERID
 . COPY0102                    17   2002/09/05   2002/09/06 09:59:12    USERID
 . DCAR                         7   2002/08/06   2002/08/06 13:59:01    USERID
 . DT1                          5   2002/08/08   2002/08/08 17:09:42    USERID
 . FMNCCPY1                    17   2002/09/12   2002/09/18 14:45:55    USERID
 . FMNCCPY2                    17   2002/09/12   2002/09/12 15:10:06    USERID
 . NEWCPPYT                    17   2002/09/05   2002/09/05 18:24:29    USERID
 . TEMP0102                    17   2002/08/28   2002/08/28 18:38:02    USERID
 . TESTMD1                     10   2002/08/29   2002/08/29 17:02:17    USERID
 . TEST0102                    17   2001/03/08   2002/09/02 15:18:53    USERID
 . TEST3                       17   2002/09/06   2002/09/06 10:04:59    USERID
 . TEST4                       17   2002/09/06   2002/09/06 11:26:50    USERID
 Command ===>                                                  Scroll ===> PAGE
  F1=Help    F2=Split   F3=Exit    F5=Rfind   F7=Up      F8=Down    F9=Swap
 F10=Left   F11=Right  F12=Cancel
Figure 122. Search member list panel (ISRSSML)
To start the search, enter the END command.
To cancel your selections, enter either:
• The RESET command to remove all unprocessed selections without ending the member list display
• The CANCEL command to end the member list display without processing selections still on the screen.
Note: Both the jump function (=) and the RETURN command cause an implied cancellation of selections
before they are carried out.
For more information about member lists, see the Using Member Selection Lists section of the "ISPF
Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I.
Submitting a Search-For job in batch mode
If you selected Batch Mode (2) on the Search-For Utility panel, the panel shown in Figure 123 on page
208 is displayed before the job is submitted.
                     Search-For Utility - Submit Batch jobs
 Press ENTER to continue submit
 Enter "/" to select option                   Generate Output Type:
 /  Edit JCL before user submit               1  1. SYSOUT Class
                                                 2. Data Set Name
                                                 3. //OUTDD DD
 SYSOUT Class . . . . A              
 Data Set Name  . . .                                                        
 //OUTDD DD . . . . .                                                        
 // . . . . . . . . .                                                        
                      LRECL for the Listing Output will be 133
 Job statement information: (Required - Enter/Verify JOB control statement)
 ===>                                                                        
 ===>                                                                        
 ===>                                                                        
 ===>                                                                        
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 123. Search-For Utility - Submit Batch Jobs panel (ISRSFSUB)
This panel allows you to specify one of these Generate Output types:
Search-For utility (option 3.14)
208  z/OS: z/OS ISPF User's Guide Vol II

## Page 247

• The SYSOUT class, which determines the printer to which your job is sent and the format used for the
printed output
• The name of a listing data set
• Output data definitions that you can use to give the printer additional instructions, such as an output
destination that is not defined by a SYSOUT class.
The Job Statement information field is explained in the details about Job Statement Information in the
Libraries and Data Sets topic of the z/OS ISPF User's Guide Vol I. The other fields on this panel, as well
as the options listed at the top of the panel, are described in “Submitting a SuperC job in batch mode” on
page 189.
Search-ForE utility (option 3.15)
Note: For an introduction to the Search-For and Extended Search-For utilities, see Appendix A, “SuperC
reference,” on page 431.
If you select option 3.15, the Extended Search-For Utility panel, shown in Figure 124 on page 209,
is displayed. This utility is a dialog that uses the SuperC program to search your data sets or PDS
members for one or more character strings. It is appropriate if you need more flexibility than the standard
Search-For utility (option 3.14) provides.
Note: When member generations of a PDSE version 2 data set are searched for character strings, only
members of the current generation are searched.
   Menu  Utilities  Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                          Extended Search-For Utility
                                                                    More:     +
 Search DS Name  . .                                                        
 PDS Member List . .           (blank/pattern - member list, * - search all)
 (Leave Search DSN "blank" for concatenated-uncataloged-password panel)
 Enter Search Strings and Optional operands (WORD/PREFIX/SUFFIX,C)
    Caps . .                                                            
    Caps . .                                                            
    Caps . .                                                            
    Asis . .                                                            
    Asis . .                                                            
 Listing DSN . . . . SRCHFOR.LIST                                           
 Process Options . .                                                        
 Statements Dsn  . .                                                        
 Enter "/" to select option      Execution Mode          Output Mode
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 124. Extended Search-For Utility panel (ISRSFPRI)
Search-ForE Utility panel action bar
The Search-ForE Utility panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Options
1
Edit statements
Search-ForE utility (option 3.15)
Chapter 5. Utilities (option 3)  209

## Page 248

2
Process options
Help
The Help pull-down provides general information about Extended Search-For topics, including how to
specify data sets, search strings, process options and process statements.
Search-ForE Utility panel fields
The panel requires only the entry of character string(s). The fields on this panel are:
Search DS Name
Specify the name of a sequential data set, PDS, or membered PDS. Use standard TSO naming
conventions, including quotes for fully qualified names. Leave this field blank to display a panel on
which you can specify concatenated, uncataloged, and password-protected data sets. This panel is
shown in Figure 125 on page 212.
PDS Member List
Leave this field blank to display a list of all the members in the search data set. Otherwise, enter a
pattern or an asterisk (*). See “Search-For member lists” on page 207 for more information.
pattern
Entering a pattern causes ISPF to display a list of the members in the search data set that match
the pattern unless Bypass Selection List was selected. See the topic about Displaying Member
Lists in the "ISPF Libraries and Data Sets" chapter of the z/OS ISPF User's Guide Vol I for more
information about using patterns. For example:
PDS Member List . . . ISR*
*
Entering an asterisk causes all the members in the search data set to be searched.
You can also use SELECT process statements in the statements data set to specify an optional set
of PDS members to be searched. However, the SELECT statement turns off the PDS member list
function.
CAPS
A search string that you want the Extended Search-For utility to find. This search string is converted to
uppercase before the search begins and is found only if it exists in the search data set in uppercase.
The ANYC process option causes the string to be found in any case, (uppercase, lowercase, or mixed
case) even if you enter the string in the CAPS field.
You can enter up to three uppercase search strings, one in each CAPS field. Here are some examples:
example 1
Either of these strings may be found in the search data set:
CAPS . . . . THEN
CAPS . . . . IF
example 2
The two strings shown must be found on the same line because of the continuation (C) keyword.
THEN must be a complete word, while ISR must be the prefix of a word.
CAPS . . . . THEN WORD
CAPS . . . . ISR PREFIX C
example 3
In the next example, a hexadecimal string is specified as the search string. Use this to find
unprintable characters.
CAPS . . . . X'7B00'
Search-ForE utility (option 3.15)
210  z/OS: z/OS ISPF User's Guide Vol II

## Page 249

example 4
This example searches for the string JOE'S CLIST. Notice that the string is enclosed in single
quotation marks and the apostrophe following Joe's name has been doubled.
CAPS . . . . 'JOE''S CLIST'
ASIS
A search string that you want the Extended Search-For utility to find. This search string is searched for
as it is when you enter it in the ASIS field. Therefore, the Extended Search-For utility does not find the
string unless it exists in the data set exactly as you enter it in an ASIS field. You can enter one search
string in each ASIS field.
The examples following the CAPS field definition apply to the ASIS field as well.
See “Search-For strings and keywords” on page 206 for a list of rules that determine the format
required for entering search strings and for definitions of the keywords that are shown in the
examples.
The SRCHFOR and SRCHFORC process statements override any strings entered in the CAPS and ASIS
fields.
Listing DSN
The name of the list data set to which the Extended Search-For utility writes the listing information.
If you leave this field blank, Extended Search-For allocates a list data set, using default data set
attributes and this data set name:
prefix.userid.SRCHFOR.LIST
where pr efix  is your TSO prefix and userid is your user ID. If your prefix and user ID are identical, only
your prefix is used. Also, if you do not have a prefix, only your user ID is used.
Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is
included before the SRCHFOR qualifier.
If you enter a fully qualified data set name SuperC uses it as specified. Otherwise, SuperC only
appends your TSO prefix to the front of the data set name specified. If you run with TSO PROFILE
NOPREFIX, SuperC uses the name as you entered it, which can result in an attempt to catalog the
name in the master catalog.
If you enter the name of a data set that already exists, the contents of that data set are replaced by
the new listing output. However, if the data set is sequential, you can add this listing to the data set
instead of replacing it by using the APNDLST process option.
If you enter the name of a data set that does not exist, Search-For allocates it for you. The data set
is allocated as a sequential data set unless you enter a member name after it, in which case it is
allocated as a partitioned data set.
Process Options
Keywords that tell SuperC how to process the search-for operation. You can type these keywords in
the Process Options field or select them from a panel. See “Process options” on page 434 for tables of
keywords.
Bypass Selection List
When a member pattern is entered in the PDS Member List field, selecting this field causes SuperC
to process all members matching that pattern without displaying a member selection list. Leaving this
field blank causes the member list to be displayed.
Statements Dsn
The name of the data set that contains your search-for process statements, which you can create or
change by using primary command E on the Extended Search-For Utility panel. SuperC reads these
process statements before conducting the search. All statements data sets must be fixed block with
80-byte records (FB 80).
Search-ForE utility (option 3.15)
Chapter 5. Utilities (option 3)  211

## Page 250

Execution Mode
Foreground
If you choose option 1, Foreground, and you leave the Search DS Name field blank, the Extended
Search-For - Concatenation Data Set Entry panel, shown in Figure 125 on page 212, is displayed. 
                Extended Search-For Concatenation Foreground Entry
                               "Search" Concatenation
    DS1 . . .                                                        
    DS2 . . .                                                        
    DS3 . . .                                                        
    DS4 . . .                                                        
                        Other "Search" Partitioned, Sequential or VSAM Data Set
 Data Set Name . . .                                                        
 Volume Serial . . .           (If not cataloged)
 Password  . . . . .           (Password allowed only in foreground mode)
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 125. Extended Search-For - concatenation data set entry panel (ISRSFCON)
For fields DS1 through DS4, use normal TSO naming conventions. You can specify a series of
concatenated data sets, an uncataloged or password-protected data set, or a cataloged data set
name.
Up to four data sets can be concatenated. Make sure the data sets are concatenated in the proper
sequence, as follows:
1. If two or more sequential data sets are concatenated as one input data set, the data set
attributes, such as block size, must be identical.
2. PDS concatenations must have the data set with the largest block size as the first in any
concatenation.
3. Search-For uses only the first occurrence of a member in the concatenated series of PDSs as
source input for a search. Any other occurrences of the member are ignored. You may specify
the SDUPM process option to cause SuperC to search for and report all occurrences of the
string for the entire concatenated series of PDS members.
Other partitioned or sequential data sets, volume serials, and data set passwords are specified
as on any other data entry panel. For more information, see the "ISPF Libraries and Data Sets"
chapter in the z/OS ISPF User's Guide Vol I.
Note: The Password field applies only to the other partitioned or sequential data set. TSO prompts
you if any concatenated data sets are password-protected.
Batch
Option 2 causes SuperC to process the data sets in batch mode. This choice frees the keyboard,
allowing you to continue using ISPF while waiting for SuperC to search the data sets. The output
listing is sent to the destination specified on the Search-For Utility - Submit Batch jobs panel
(Figure 123 on page 208).
The panel shown in Figure 126 on page 213 is displayed if you select option 2, Batch, and leave
the Search DS Name field blank on the Extended Search-For Utility panel. You can concatenate
up to four data sets that have like attributes. For example, all must be either sequential or
partitioned.
Search-ForE utility (option 3.15)
212  z/OS: z/OS ISPF User's Guide Vol II

## Page 251

Extended Search-For Concatenation Batch Entry
                               "Search" Concatenation
    DS1 . . .                                                        
    DS2 . . .                                                        
    DS3 . . .                                                        
    DS4 . . .                                                        
                        Other "Search" Partitioned, Sequential or VSAM Data Set
 Data Set Name . . .                                                        
 Volume Serial . . .           (If not cataloged)
 Password  . . . . .           (Password allowed only in foreground mode)
 Command ===>                                                                 
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F12=Cancel
Figure 126. Extended Search-For - concatenation batch entry panel (ISRSFCON)
This panel is the same as the panel shown in Figure 125 on page 212, except the Password
field is used only in foreground mode. If your data sets are password-protected, search the data
sets in foreground mode by specifying Foreground in the Execution Mode field on the Extended
Search-For Utility panel.
If you selected the Batch option on the Extended Search-For Utility panel, the panel shown in
Figure 123 on page 208 is displayed before the job is submitted. Use this panel to specify whether
your Search-For listing is to be printed or written to a list data set.
Output Mode
The output mode you want to use when displaying the listing file. Choose one of these:
View
This enables the listing file to be displayed in view mode. All View functions are enabled in this
mode.
Browse
This enables the listing file to be displayed in the browse mode. All Browse functions are enabled
in this mode.
Search-ForE Utility primary commands
The SuperCE utility provides the functions described in these topics, each of which is controlled by a
command that you can type on the command line:
B – Batch
When you enter primary command B on the Extended Search-For Utility panel, processing is the same
as when you press Enter with Batch specified in the Execution Mode field. The value specified in the
Execution Mode field is ignored. See the section describing the Batch option in the Execution Mode
field for information on batch processing.
E – Edit statements
When you enter primary command E on the Extended Search-For Utility panel, the statements data
set that you specified in the Statements Dsn field is displayed in Edit mode. See “Edit statements -
edit Search-For statements data set” on page 214 for information related to the edit statements data
set.
P – Process options
When you enter primary command P on the Extended Search-For Utility panel, the Extended Search-
For Process Options panel is displayed. This panel contains the Extended Search-For process options.
See “Process options - select Search-For process options” on page 214 for information related to the
Extended Search-For Process Options panel.
Search-ForE utility (option 3.15)
Chapter 5. Utilities (option 3)  213

## Page 252

Search-ForE Utility options
These topics describe the options that are available in the Options pull-down on the Extended Search-For
Utility panel action bar:
• “Process options - select Search-For process options” on page 214
• “Edit statements - edit Search-For statements data set” on page 214
Process options - select Search-For process options
When you select Process Options from the Options pull-down menu, the Extended Search-For Process
Options panel is displayed. This panel contains the Extended Search-For process options. You can also
access the Extended Search-For Process Options panel by entering the primary command P on the
Extended Search-For Utility panel.
Table 15 on page 214 lists all of the process options for Search-For.
Table 15. Search-For process options
ALLMEMS ANYC    APNDLST ASCII   CKPACKL COBOL   Cpnnnnn
DPACMT  DPADCMT DPBLKCL DPCBCMT DPCPCMT DPFTCMT DPMACMT
DPPLCMT DPPSCMT EMPTYOK FINDALL FMSTOP  IDPFX   LMTO    
LNFMTO  LONGLN  LPSF    LTO     MIXED   NOPRTCC NOSEQ   
NOSUMS  SDUPM   SEQ     XREF
These rules govern the selection of Search-For process options:
• Type any nonblank character to the left of one or more process options. Then press Enter. This causes
the options you specify to be displayed in the Process Options field on the Extended Search-For Utility
panel. If you select two options that cannot be specified together, or if you enter an option name
incorrectly, an error message is displayed. Use the Backward and Forward keys, as necessary, to move
through the panel.
• Use the CANCEL command to return to the Extended Search-For Utility panel without processing
selections.
Search-For process options can affect how the input data is processed, and determine the format and
content of the output listing data set. They can also help you save processing time by avoiding comments
and blank lines.
All these options can be chosen from the Search-For Process Options panel or you can type them in
the Process Options field on the Extended Search-For Utility panel. Errors caused by mistyping process
options are detected when you call the Extended Search-For utility.
For definitions of the Search-For process options, see “Process options” on page 434.
Edit statements - edit Search-For statements data set
A statements data set consists of process statements that contain instructions for the SuperC program.
They are similar to the process options, but are composed of a keyword and one or more operands. See
“Process options - select Search-For process options” on page 214 for information about Search-For
process options.
When you select Edit Statements from the Options menu on the Extended Search-For Utility panel, the
Extended Search-For utility displays the statements data set you specified in the Statements Dsn field.
You can also display the statements data set by entering the primary command E on the Extended
Search-For Utility panel. The statements data set is always displayed in Edit mode, allowing you to add,
change, or delete search-for process statements as needed.
The size of the Edit window depends on the number of lines your terminal can display. The sample panel
shown in Figure 127 on page 215 shows how the Edit window appears on a 24-line display. Examples
of some common process statements are listed below the Edit window so you can easily compose the
proper input line.
Search-ForE utility (option 3.15)
214  z/OS: z/OS ISPF User's Guide Vol II

## Page 253

USERID.SRCHFOR.STMTS                           Columns 00001 00072
    Enter or change Process Statements in the EDIT window below:
 ****** ***************************** Top of Data ******************************
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ''''''
 ****** **************************** Bottom of Data ****************************
    Examples                    Explanation
 SRCHFOR  'ABCD',W           Search for the word "ABCD"
 SRCHFORC 'DEFG'             "DEFG" must be on same line as word "ABCD"
 CMPCOLM  1:60  75:90        Search columns 1:60 and 75:90 for string(s)
 DPLINE 'PAGE ',87:95        Exclude line if "PAGE " found in columns 87:99
 DPLINE   'PAGE '            Exclude if "PAGE " found anywhere on line
 SELECT   MEM1,MEM2          Search only members MEM1 and MEM2 of PDS
                         - - - - -
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit      F5=Rfind     F6=Rchange   F7=Up
  F8=Down      F9=Swap     F10=Left     F11=Right    F12=Cancel
Figure 127. Search-For process statements panel (ISRSFPRS)
The SuperC program validates the process statements at run time. Invalid process statements are not
used and are noted at the bottom of the listing. Unless a higher return code is required by some other
condition, a return code of 4 is returned.
For the syntax and examples of the Search-For process statements, see “Process statements” on page
445.
ISPF table utility (option 3.16)
The ISPF Table Utility (Option 3.16) provides functions for processing ISPF tables. When you select this
option, the ISPF Table Utility entry panel is displayed. This panel allows you to specify a table data set or
DD, a table name, and an option to be performed.
The Edit and Browse functions allow you to view the data in the rows of an ISPF table in full-screen
mode (that is, multiple rows are displayed on a screen). Line commands allow you to work with individual
or multiple table rows. Primary commands are provided to support processing against the entire table,
including changing the format of the displayed data. Table data can be scrolled in any direction (up, down,
left, or right). All table column values are displayed in scrollable fields, allowing columns to be scrolled
left or right, and individual column values to be expanded and displayed in a popup window. The values
for any extension variables associated with a particular table row can be displayed.
The Edit function allows you to change the data in a table simply by overtyping the displayed value. Edit
function line commands are available to insert new table rows, repeat rows, and delete rows. Extension
variables for a table row can be created, modified, or deleted.
The Export function writes the data in an ISPF table to a sequential file so that it can be browsed or
edited. You can customize the format of the data written to the sequential file.
The Import function uses the data in a sequential file to either create a new ISPF table or replace an
existing table. The data in the sequential file is required to be in a special format generated by the ISPF
Table Utility Export function.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  215

## Page 254

Menu  RefList  Utilities  Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                               ISPF Table Utility
 Option ===>                                                                   
    blank Display table list                  E Edit table
        B Browse table                        I Import table data
 Enter one of the parameters below:
   Table Data Set . .                                  
   or Table DD  . . . ISPTLIB  (Default is ISPTLIB)
   Table Name . . . . ________  (Blank or pattern for table selection list)
   Import Data Set    ______________________________________________
   Enter "/" to select option
   _  Open table in SHARE mode
   _  Table is an EDIT line command table
 F1=Help      F3=Exit     F12=Cancel   F13=Help     F15=End      F16=Return
F17=Rfind    F18=Rchange  F22=Left     F23=Right    F24=Cretriev
Figure 128. ISPF Table Utility panel (ISRUTBP0)
Table Utility panel action bar
The Table Utility Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, see the topic about Using Personal Data Set Lists and Library Lists
in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Options
The Options pull-down offers these choices:
 1
Table Utilities Options
 2
Export Report Options
 3
Export Data Set Attributes
Help
The Help pull-down provides general information about Table Utility topics as well as information
about each of the main panels and options.
Table Utility panel fields
The fields on the ISPF Table Utility panel are:
Table Data Set
The name of the data set containing the table you wish to process.
Table DD
The name of the DD allocated to your ISPF session which contains the table you wish to process.
The default is the ISPTLIB DD if you do not enter data for either the Table Data Set or Table DD. If you
enter data in both the Table Data Set and Table DD fields, the Table Data Set takes precedence.
ISPF table utility (option 3.16)
216  z/OS: z/OS ISPF User's Guide Vol II

## Page 255

Table Name
The name of the table you wish to process.
If you leave this field blank or supply a pattern the table selection list will be displayed showing the
matching tables in the table data set or DD.
Import Data Set
The name of the sequential data set containing the data used to create or replace a table through the
Import function.
Open table in SHARE mode
Select this option if the table you choose to process is already open on another logical screen, or if you
might need to share the table with another logical screen.
Table is an EDIT line command table
Select this option to create a table that can be used as an Edit line command table. The utility creates
predefined columns. This option also formats unique headings to be used with an Edit line command
table.
Table utility entry panel options
These are the options shown on the ISPF Table Utility entry panel:
Blank - (Display Table List)
If you leave the Option field blank, a list of tables for the Table Data Set or Table DD is displayed when
the Table Name is either blank or contains a pattern. If a valid Table Name is entered, the table list is
bypassed and the Edit/Browse panel is displayed.
B - (Browse Table)
If a valid Table Name for the Table Data Set or Table DD is entered, the Browse Table panel is
displayed. If the Table Name is either blank or contains a pattern, the table list is displayed allowing
you to select the table to be browsed.
E - (Edit Table)
If a valid Table Name for the Table Data Set or Table DD is entered, the Edit Table Display panel is
displayed. If the Table Name is either blank or contains a pattern, the table list is displayed allowing
you to select the table to be edited.
I - (Import Table Data)
The Import function uses data from a sequential data set to create a new ISPF table or update an
existing ISPF table. You must supply a Table Data Set and Table Name for the new or updated table.
The sequential data set containing the data that will be used to create or update the table must be
specified in the Import Data Set field.
Table data set selection list
This selection list is displayed when you enter a table data set name and either no table name or a table
name pattern on the table utility entry panel.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  217

## Page 256

Menu  Utilities  Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                                ISPF Table List               Row 1 to 12 of 29
 Command ===> ________________________________________________ Scroll ===> CSR 
 List of tables in table library PDFTOOL.COMMON.TABLES
    Name
    --------
 _  BLG0CMDS
 _  BLG0KEYS
 _  BLG0PROF
 _  BLSGEDIT
 _  BLSGEDRT
 _  BLSGPROF
 _  BLSLPROF
 _  DAFCMDS
 _  ECXPDFPC
 _  HSOCMDS
 _  MOSCMDS
 _  MVS8CMDS
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 129. Table data set selection list panel (ISRUTBP1)
If no table name is supplied, all members in the table data set are shown in the selection list. If a table
name pattern is supplied, all members in the table data set that have a name matching the pattern are
shown in the selection list.
These line commands are available on the table data set selection list panel:
E
The Edit line command displays the EDIT table panel. It is available if you did not enter an option on
the table utility entry panel.
B
The Browse line command displays the BROWSE table panel. It is available if you did not enter an
option on the table utility entry panel.
S
When you use the Select line command against a table, either the BROWSE table or EDIT table panel
is displayed:
• BROWSE table is displayed if you entered option B on the entry panel.
• EDIT table is displayed if you entered option E on the entry panel.
• If you did not enter option B or E on the entry panel, the panel is determined by how the option "Use
EDIT as default to process selected table" is set. To set this option, select Table Utility Options panel
from the Options pull-down.
These primary commands are available on the table data set selection list panel:
L string
The Locate command scrolls the selection list and positions at the top of the display the entry which
either matches or precedes (in alphabetic sequence) the value of string.
S tblname
The Select command searches the selection list for an entry that matches tblname. If a matching
entry is found the table is displayed in either the BROWSE table or EDIT table panel, following the
same rules as for the Select line command.
E tblname
The Edit command is available if you did not enter an option on the table utility entry panel. The
selection list is searched for an entry that matches tblname. If a matching entry is found the EDIT
table panel is displayed.
ISPF table utility (option 3.16)
218  z/OS: z/OS ISPF User's Guide Vol II

## Page 257

B tblname
The Browse command is available if you did not enter an option on the table utility entry panel. The
selection list is searched for an entry that matches tblname. If a matching entry is found the BROWSE
table panel is displayed.
Table DD selection list
This selection list is displayed when you enter a table DD and either no table name or a table name
pattern on the table utility entry panel.
   Menu  Utilities  Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                                ISPF Table List              Row 1 to 12 of 443
 Command ===> ________________________________________________ Scroll ===> CSR 
 List of tables in data sets allocated to DD ISPTLIB
               Concat.
    Name       Number   Table Data Set
    --------   -------  ----------------------------------------------
 _  $ISRPROF         1  PDFTDEV.LSACKV.TABLES
 _  #ISRPROF         1  PDFTDEV.LSACKV.TABLES
 _  ABCPROF          7  LSACKV.ISPF.ISPPROF
 _  ACBKEYS         17  SYS1.DGTTLIB
 _  ADB2DB2D         9  SYS2.TABLES.SYSPLEXD
 _  ADB2PARM        23  DB2.ADMIN.V2R1M0.SADBTLIB
 _  ADB21D          23  DB2.ADMIN.V2R1M0.SADBTLIB
 _  ADB21DI2        23  DB2.ADMIN.V2R1M0.SADBTLIB
 _  ADB21S          23  DB2.ADMIN.V2R1M0.SADBTLIB
 _  ADB21SP         23  DB2.ADMIN.V2R1M0.SADBTLIB
 _  ADB21T          23  DB2.ADMIN.V2R1M0.SADBTLIB
 _  ADB21X          23  DB2.ADMIN.V2R1M0.SADBTLIB
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 130. Table DD selection list panel (ISRUTBP2)
If no table name was supplied, all members in the data sets allocated to the table DD are shown in the
selection list. If a table name pattern was supplied, all members in the data sets allocated to the table DD
which have a name matching the pattern are shown in the selection list.
The table DD selection list is sorted in member name order. Along with the member name, the selection
list displays the name of the table data set where the member was found, and the concatenation number
for that data set within the table DD.
These line commands are available on the table DD selection list panel:
E
The Edit line command displays the EDIT table panel. It is available if you did not enter an option on
the table utility entry panel.
B
The Browse line command displays the BROWSE table panel. It is available if you did not enter an
option on the table utility entry panel.
S
When you use the Select line command against a table, either the BROWSE table or EDIT table panel
is displayed:
• BROWSE table is displayed if you entered option B on the entry panel.
• EDIT table is displayed if you entered option E on the entry panel.
• If you did not enter option B or E on the entry panel, the panel is determined by how the option "Use
EDIT as default to process selected table" is set. To set this option, select Table Utility Options panel
from the Options pull-down.
These primary commands are available on the table data set selection list panel:
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  219

## Page 258

L string
The Locate command scrolls the selection list and positions at the top of the display the entry which
either matches or precedes (in alphabetic sequence) the value of string.
S tblname
The Select command searches the selection list for an entry that matches tblname. If a matching
entry is found the table is displayed in either the BROWSE table or EDIT table panel, following the
same rules as for the Select line command.
E tblname
The Edit command is available if you did not enter an option on the table utility entry panel. The
selection list is searched for an entry that matches tblname. If a matching entry is found the EDIT
table panel is displayed.
B tblname
The Browse command is available if you did not enter an option on the table utility entry panel. The
selection list is searched for an entry that matches tblname. If a matching entry is found the BROWSE
table panel is displayed.
Edit/browse table panel
The table display panel used for the Edit and Browse functions of the table utility shows multiple rows on
the one screen. Each row occupies one line on the screen. The UP and DOWN primary commands allow
you to scroll through the rows in a table.
   Options  Help
 ───────────────────────────────────────────────────────────────────────────────
 BROWSE                  ISPF Table BLSGEDIT                  Row 1 to 15 of 17
 Command ===> ________________________________________________ Scroll ===> CSR 
                                                                Shift ===> PAGE
     ZEDPTYPE ZEDPLRCL ZEDPRCFM ZEDPFLAG                 ZEDPBNDL ZEDPBNDR
     ----+--- ----+--- ----+--- ----+----1----+----2---- ----+--- ----+---
 __  TRACE    128      F        000000101000000000010000 0        0
 __  CLIST    251      V        010000000000100000010000 0        0
 __  PANELS   80       F        000000101000100000010000 0        0
 __  TRACE    72       F        000000101000000000010000 0        0
 __  CNTL     80       F        000000001000100000010000 0        0
 __  JCL      80       F        010000000000100000010000 0        0
 __  VCALL    80       F        000000001000000000000000 0        0
 __  TRACE    121      V        000000001000000000000000 0        0
 __  F02      80       F        000000001000000000000000 0        0
 __  F03      121      V        000000001000000000000000 0        0
 __  PRINT1   129      V        000000001000000000000000 0        0
 __  TEXT     251      V        010000000000000000000000 0        0
 __  ISPVCALL 80       V        000000000000000000000000 0        0
 __  TRACE    80       F        000000000000000000000000 0        0
 __  LOG      121      V        000000000000000000000000 0        0
  F1=Help      F3=Exit      F4=Expand    F5=Rfind    F12=Cancel   F13=Help
 F15=End      F16=Return   F17=Rfind    F22=Left     F23=Right    F24=Cretriev
Figure 131. Table display panel, edit mode (ISRUTBP3)
The dialog variables for the table rows are displayed in columns across the screen, with the dialog
variable names shown as column headings. The RIGHT and LEFT primary commands allow you to view
any columns that are not currently visible.
Two options on the Table Utility Options panel control how key values are displayed:
• Color used to display table key values specifies the color (BLUE, RED, PINK, GREEN, TURQ, YELLOW, or
WHITE).
• Intensity used to display table key values specifies the intensity (HIGH or LOW).
The default color is GREEN and the default intensity is HIGH. For the Edit function, key values are always
underscored. For the Browse function, key values are not underscored.
To determine the width required for each column field, the table utility must scan the table rows and
check the length of the table variable values. While the utility uses an efficient method to scan a table,
this process can be time consuming for a table with an extremely large number of rows. You can limit the
ISPF table utility (option 3.16)
220  z/OS: z/OS ISPF User's Guide Vol II

## Page 259

number of rows scanned through the "Maximum rows searched to determine column width" option on the
Table Utility Options panel.
All table variables are displayed in scrollable fields, with a scale indicator displayed below each column
heading. Using scrollable fields allows the EXPAND primary command to be used to display the value
of a table variable in a popup window. This popup window can display and edit data in HEX mode. The
scrollable fields also allow you to use the RIGHT and LEFT primary commands to horizontally scroll
column values.
For the Browse function, all the fields displaying table variable values are protected. For the Edit function,
all these fields are unprotected and you can make changes to the table variable values by overtyping the
displayed data.
For the Edit function, when you press Exit (F3) the changes are saved to a table output library. Normally
the changes would be saved to the originating data set.
If you specified the table name and a Table DD on the ISPF Table Utility panel, and the "Always save table
in originating data set" check box on the Table Utility Options panel is not selected, ISPF prompts you to
specify the output data set. See “Table output data set selection” on page 230 for more information.
Line commands
This topic describes the line commands available on the Edit/Browse panel.
E
Extension Variables. Use this command to display the extension variables for the table row. When
using the Edit function, the values of the extension variables can be changed, new extension variables
can be created, and existing extension variables can be deleted. See “Extension Variables panel” on
page 221.
In
Insert Row After. Use this command to insert one or more rows after the row where the line command
was entered. The table variable values for an inserted row are initialized with blanks.
Bn
Insert Row Before. Use this command to insert one or more rows before the row where the line
command was entered. The table variable values for an inserted row are initialized with blanks.
Rn
Repeat Row. Use this command to create one or more copies of the table row. The copied rows
are inserted after the row where the line command was entered. For the copied rows, all variables
excluding keys are initialized using the values from the corresponding variables in the row where the
line command was entered. Key variables are initialized with blanks.
Dn
Delete Row. Use this command to delete one or more table rows.
Note:
1. The E command is available in both the Edit and Browse functions. The I, B, R, and D commands are
only available in the Edit function.
2. For all line commands except E, an optional number from 1 to 9 can be entered as a suffix to the line
command character. This causes the command to operate on multiple rows starting with the row on
which the command was entered.
3. When processing a keyed table, the optional number is ignored for the line commands I, B, and R.
Extension Variables panel
The Extension Variables panel shows the names and values of the extension variables defined for a table
row. To display the extension variables panel, enter the E line command against a table row on the table
display screen.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  221

## Page 260

Options  Help
  ───────────────────────────────────────────────────────────────────────────────
                 BLSGEDIT Extension Variables for Row 1         Row 1 to 3 of 3
 Command ===> ________________________________________________ Scroll ===> CSR 
 Extension variable values scrollable width:    65
 S   Name      Value
 --  --------  -----------------------------------------------------------------
               ----+----1----+----2----+----3----+----4----+----5----+----6----+
 __  ZEDPIMAC
 __  ZEDPFLG2  01000011
 __  ZEDPFLG3  00000001
 ******************************* Bottom of data ********************************
  F1=Help    F2=Split   F3=Exit    F4=Expand  F5=Rfind   F7=Up      F8=Down
  F9=Swap   F10=Left   F11=Right  F12=Cancel
Figure 132. Extension Variables Panel (ISRUTBP4)
When you edit a table, the extension variable names and their values and the "Extension variable values
scrollable width" are displayed in unprotected fields. You can change the extension variable names and
values by overtyping the displayed data. You can use the selection field to enter a line command against
an extension variable. When you browse a table, the extension variable names and values are protected,
and the selection field is unavailable.
The extension variable values are displayed in scrollable fields with a scale indicator displayed below the
column heading. You can use the RIGHT and LEFT primary commands to horizontally scroll through one of
the values. You can enter the EXPAND primary command to display the value of an extension variable in a
popup window. This popup window also enables you to display and edit data in HEX mode.
The "Extension variable values scrollable width" field initially displays the length of the scrollable width of
the field that displays the extension variable values. This length will be the maximum of either:
• The length of the field displaying the values. This length depends on the width of the screen. For
example, if the screen has a width of 80 characters the field will have a length of 65 characters. For a
screen with a width of 132 characters the field will have a length of 117 characters.
• The length of the largest value for the extension variables displayed.
If you need to lengthen the value for an extension variable beyond the scrollable limit, you can use this
field to enter a numeric value to increase the scrollable width of the field. You can then use the EXPAND
primary command (F4) to update the value of the extension variable.
Line commands
This topic describes the line commands available on the Extension Variables panel.
In
Insert Extension Variable After. Use this command to insert one or more extension variables after the
row where the line command was entered. The name and value for the inserted extension variable are
initialized with blanks.
Bn
Insert Extension Variable Before. Use this command to insert one or more extension variables before
the row where the line command was entered. The name and value for an inserted extension variable
are initialized with blanks.
Rn
Repeat Extension Variable. Use this command to create one or more copies of the extension variable.
The extension variables are inserted after the row where the line command was entered. For the
ISPF table utility (option 3.16)
222  z/OS: z/OS ISPF User's Guide Vol II

## Page 261

new extension variables, the names and values are copied from the extension variable where the line
command was entered.
Dn
Delete Extension Variable. Use this command to delete one or more extension variables.
Note:
1. Line commands on the extension variables panel are only available when using the Edit function.
2. For all line commands, an optional number from 1 to 9 can be entered as a suffix to the line command
character. This causes the command to operate on multiple extension variables starting with the
extension variable against which the command was entered.
Primary commands
This topic describes the primary commands available on the Table Utility Edit/Browse panel:
Navigating through the table
• UP
• DOWN
• LEFT
• RIGHT
• FIND
• RFIND
Changing the data or how it is displayed
• INSERT
• EXPAND
• SORT
• STATS
• STRUCT
Saving or exporting table data
• SAVE
• CANCEL
• EXPORT
• FEXPORT
Browse and Edit primary commands are entered in the Command field. All the primary commands except
SAVE are available in both the Edit and Browse functions. The SAVE command is only available in Edit.
CANCEL
Terminate Edit without Saving Changes. The CANCEL command (F12) terminates table editing without
saving the table data to the output data set.
CAN can be used as an abbreviation for the CANCEL command.
DOWN
Scroll Down. The standard ISPF DOWN command (F8|F20) can be used to vertically scroll the table
display towards the bottom of the table.
ISPF supported scroll amount values used for the DOWN command can be entered in the Scroll field.
You can also enter a valid scroll amount in the Command field.
EXPAND
Expand Display of Scrollable Field. The standard ISPF EXPAND command (F4) can be used to display
a table variable value in a popup window containing a scrollable dynamic area. To do this, enter
EXPAND while the cursor is placed on the field displaying a table variable value.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  223

## Page 262

EXPORT
Display Table Export Layout. The Export Layout panel is displayed when the EXPORT primary
command is entered on the Edit/Browse panel. This panel shows the structure used to format the
table data written to the export output data set. You can make changes to the structure to alter the
format of the data written to the output data set. 
   Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                          Table BLG0CMDS Export Layout          Row 1 to 9 of 9
 Command ===> ________________________________________________ Scroll ===> CSR 
 Overtype the Order column and press ENTER to rearrange the fields into the
 order in which you would like them to be dumped. Change Width as required.
 Only fields with an Order value less than 999 will be dumped.
 Use the FILE primary command to write the table data to the export data set.
                                                                    Cumulative
 Order     Name      Field Heading                        Width     Width
 -----     --------  ----------------------------------   -------   ----------
 001       ZEDPTYPE  ZEDPTYPE                                   8           9
 002       ZEDPLRCL  ZEDPLRCL                                   8          18
 003       ZEDPRCFM  ZEDPRCFM                                   8          27
 004       ZEDPFLAG  ZEDPFLAG                                  24          52
 005       ZEDPBNDL  ZEDPBNDL                                   8          61
 006       ZEDPBNDR  ZEDPBNDR                                   8          70
 007       ZEDPTABC  ZEDPTABC                                   8          79
 008       ZEDPMASK  ZEDPMASK                                   8          88
 009       ZEDPTABS  ZEDPTABS                                   8          97
 ******************************* Bottom of data ********************************
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 133. Table Export Layout panel (ISRUTBP7)
The screen shows the current structure used to format the table data written to the export data set.
The list contains these fields:
Order
This input field allows you to enter a number which defines the sequence in which the table
variables for each row are placed in the export data set. For example, assigning an Order of 001 to
a table variable makes it the first to be written to each table data record in the export data set.
Note: Only table variables that have an Order value less than 999 are written to the export data
set.
Name
The name of the table variable.
Field Heading
This input field allows you to define a heading for each table variable written to the export data
set. It is initialized with the name of the associated table variable.
Width
This input field allows you to define the number of characters allocated to the column used to
print a table variable value. This field is initialized to the display length of the table variable value
on the table display screen.
Cumulative Width
This field shows the total number of characters required in the export data set record to
accommodate this variable and all the preceding table variables.
When you are happy you have the correct format defined, use the FILE primary command to write the
table data to the export data set.
These abbreviations can be used for the EXPORT command:
   EX
   EXP
   EXPO
   EXPOR
ISPF table utility (option 3.16)
224  z/OS: z/OS ISPF User's Guide Vol II

## Page 263

FILE
The FILE command causes the table data to be written to the export data set in the format defined
on the export layout panel.
The name of the export output data set can be specified as a parameter to the FILE command.
You can enter any fully qualified data set name by enclosing it in apostrophes. If you omit the
apostrophes, your TSO prefix or user ID (if no TSO prefix is defined in your TSO user profile) is
added to the beginning of the data set name. For example, if a user whose TSO prefix is LSACKV
issues the command FILE TAB1.DATA, the table data report is written to the export data set
LSACKV.TAB1.DATA.
If you do not specify an export data set name on the FILE command, a default name is generated
according to these rules:
• If no TSO prefix is defined in your TSO user profile: userid.tblname.TBLDUMP
• If your TSO prefix and user ID are the same: tsopref.tblname.TBLDUMP
• If your TSO prefix and user ID are not the same: tsopref.userid.tblname.TBLDUMP
tsopref is your TSO prefix. userid is your TSO user ID. tblname is the name of the table you are
processing.
Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is
included before the tblname qualifier.
A warning message might be displayed if the export data set already exists. You then have the
option of terminating the command to avoid overwriting the data set. If you don't want to receive
these warnings in future, clear the "Warn if export data set exists" check box on the Table Utility
Options panel.
When the FILE command has finished, the export data set is displayed. The "Display mode for
export data set" option on the Table Utility Options panel allows you to choose either the ISPF
Browse, View, or Edit functions to display the export data set.
These abbreviations can be used for the FILE command:
   FI
   FIL
FEXPORT
Fast EXPORT Command. The FEXPORT command writes the table data to the export output data set
without displaying the export layout panel.
The name of the export output data set can be specified as a parameter to the FEXPORT command.
You can enter any fully qualified data set name by enclosing it in apostrophes. If you omit the
apostrophes, your TSO prefix or user ID (if no TSO prefix is defined in your TSO user profile) is
added to the beginning of the data set name. For example, if a user whose TSO prefix is LSACKV
issues the command FEXPORT TAB1.DATA, the table data report is written to the export data set
LSACKV.TAB1.DATA.
If you do not specify an export data set name on the FEXPORT command, a default name is generated
according to these rules:
• If no TSO prefix is defined in your TSO user profile: userid.tblname.TBLDUMP
• If your TSO prefix and user ID are the same: tsopref.tblname.TBLDUMP
• If your TSO prefix and user ID are not the same: tsopref.userid.tblname.TBLDUMP
tsopref is your TSO prefix. userid is your TSO user ID. tblname is the name of the table you are
processing.
Note: If the ISPF configuration table field USE_ADDITIONAL_QUAL_FOR_PDF _DATA_SETS is set
to YES, an additional qualifier defined with the ISPF _TEMPORARY_DATA_SET_QUALIFIER field is
included before the tblname qualifier.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  225

## Page 264

If the export data set exists when the FEXPORT command is issued and you have selected "Warn if
export data set exists" on the Table Utility Options panel, a warning popup panel is displayed. You
then have the option of terminating the command to avoid overwriting the data set.
When the FEXPORT command has finished, the export data set is displayed. The "Display mode
for export data set" option on the Table Utility Options panel allows you to choose either the ISPF
Browse, View, or Edit functions to display the export data set.
These abbreviations can be used for the FEXPORT command:
   FE
   FEX
   FEXP
   FEXPO
   FEXPOR
FIND
Search for String in Table. The FIND command can be used to search for the occurrence of a character
string in a specified column in the table. If the string is found, the row in which it is found is positioned
at the top of the display.
The FIND command has these formats:
   FIND varname string
   FIND n string
where:
varname
The name of any of the table variables.
n
The ordinal number of any column displayed on the current screen.
string
The character string to be searched for. The search is not case sensitive.
These abbreviations can be used for the FIND command:
   F
   FI
   FIN
INSERT
Insert a Blank Row at the Top of the Table. Use the INSERT command to create a new blank row as the
first row in the table. This command allows you to create a row in an empty table.
LEFT
Scroll Left. The LEFT command (F10|F22) can be used to scroll the table display horizontally towards
the first table column.
The scroll amount values used for the LEFT command can be entered in the Shift field. You can also
enter one of these valid scroll amounts in the Command field:
PAGE
Causes the display to scroll left by the width of the screen.
MAX
Causes the display to scroll left so that the first column for the table is the leftmost displayed.
0 to 9999
Causes the display to scroll left the specified number of columns.
Note: Table variable values are displayed in scrollable fields. Therefore if the cursor is placed in a field
displaying a table variable value, the LEFT command operates on that field, not on the whole table
display.
ISPF table utility (option 3.16)
226  z/OS: z/OS ISPF User's Guide Vol II

## Page 265

RFIND
Repeat Last FIND Command. The RFIND command (F5|F17) is used to repeat the last FIND
command. It is most useful when assigned to a function key.
R can be used as an abbreviation for the RFIND command.
RIGHT
Scroll Right. The RIGHT command (F11|F23) can be used to scroll the table display horizontally
towards the last table column.
The scroll amount values used for the RIGHT command can be entered in the Shift field. You can also
enter one of these valid scroll amounts in the Command field:
PAGE
Causes the display to scroll right by the width of the screen.
MAX
Causes the display to scroll right so that the last column for the table is the rightmost displayed.
0 to 9999
Causes the display to scroll right the specified number of columns.
Note: Table variable values are displayed in scrollable fields. Therefore if the cursor is placed in a field
displaying a table variable value, the RIGHT command operates on that field, not on the whole table
display.
SAVE
Save Table Changes. The SAVE command causes the changes to the table data to be written to the
table output library. Normally the changes would be saved to the originating data set.
If you specified the table name and a Table DD on the ISPF Table Utility panel, and the "Always
save table in originating data set" check box on the Table Utility Options panel is not selected, ISPF
prompts you to specify the output data set. See “Table output data set selection” on page 230 for
more information.
SAV can be used as an abbreviation for the SAVE command.
SORT
Display Table Sort Definition. The Sort Specification panel is displayed when the SORT primary
command is entered on the Edit/Browse panel. This panel allows you to sort the table according
to the values of one or more table variables. 
   Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                       Table BLSGEDIT Sort Specification        Row 1 to 9 of 9
 Command ===> ________________________________________________ Scroll ===> CSR 
 Overtype the Order column and press ENTER to rearrange the table variables
 into the order in which you would like them to be sorted. Change Sequence to A
 (Ascending) or D (Descending) as required. Table BLSGEDIT will only be sorted
 using table variable with an Order less than 999.
 Order     Name           Sequence (A/D)
 -----     --------       --------------
 999       ZEDPTYPE             A
 999       ZEDPLRCL             A
 999       ZEDPRCFM             A
 999       ZEDPFLAG             A
 999       ZEDPBNDL             A
 999       ZEDPBNDR             A
 999       ZEDPTABC             A
 999       ZEDPMASK             A
 999       ZEDPTABS             A
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 134. Table Sort Specific ation  panel (ISRUTBP8)
The screen displays a list of the table variables and contains these fields:
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  227

## Page 266

Order
This input field allows you to enter a number which defines the sort priority for the associated
table variable. For example, assigning an Order of 001 makes the associated table variable the
primary sort key.
Note: The table will only be sorted on those variables that have an Order value less than 999.
Name
The name of the table variable.
Sequence (A/D)
This input field allows you to define whether to sort in ascending (A) or descending (D) sequence
for the associated table variable.
When you press Exit (F3) to return to the Edit/Browse panel it is sorted based on changes made on the
table sort display.
Note: If you are using Edit, the sort criteria entered on this screen are saved with the table.
These abbreviations can be used for the SORT command:
   SO
   SOR
STATS
Display Table Statistics. The table statistics display is invoked when the STATS primary command
is entered on the browse/edit table display. This screen shows the statistical information that ISPF
maintains for the table. 
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                         Statistics for Table BLG0CMDS                       │
 │ Command ===> __________________________________________________________     │
 │ The following information is returned by the TBSTATS service:               │
 │                                                                             │
 │ Created on . . . : 1997/01/14 at 13:26:23                                   │
 │ Last updated on  : 2004/08/27 at 09:37:02 by LSACKV                         │
 │                                                                             │
 │ Initial number of rows when created  . . . . . . : 101                      │
 │ Current number of rows . . . . . . . . . . . . . : 101                      │
 │ Number of existing rows which have been updated  : 2                        │
 │ Number of times table has been updated . . . . . : 7                        │
 │ Virtual storage size (bytes) . . . . . . . . . . : 8,072                    │
 │                                                                             │
 │ Last TABLE SERVICES command  . . . . . . . . . . : TBTOP                    │
 │ Return code from above service . . . . . . . . . : 0                        │
 │                                                                             │
 │ Status 1 : 1  - table exists in input library chain                         │
 │ Status 2 : 3  - table is open in WRITE mode in this logical screen          │
 │ Status 3 : 2  - table is not available for WRITE mode                       │
 │                                                                             │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F10=Actions    F12=Cancel                                    │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
Figure 135. Table statistics panel (ISRUTBP6)
The screen shows these fields:
Created on
The date and time the table was originally created.
Last updated on
The date and time the table was last modified.
by
User ID of the last user who modified the table.
Initial number of rows when created
Number of rows that were added during the session when the table was first created and then
closed.
ISPF table utility (option 3.16)
228  z/OS: z/OS ISPF User's Guide Vol II

## Page 267

Current number of rows
Number of rows currently in the table.
Number of existing rows which have been updated
Number of rows that have been modified in the table at least once. A row that is added to an
existing table is considered a modified row.
Number of times table has been updated
Number of editing sessions during which the table has been modified. Opening a table, then
making one or more updates, then closing and saving the table increments this count by one.
Virtual storage size (bytes)
Number of bytes of virtual storage required by the table.
Last TABLE SERVICES command
The name of the last table service called.
Return code from above service
The return code issued by the last table service called.
Status 1
The status of the table in the table input library chain.
Status 2
The status of the table in this logical screen.
Status 3
The availability of the table to be used in WRITE mode.
STRUCT
Display Table Structure. The table structure panel is invoked when the STRUCT primary command is
entered on the Edit/Browse panel. This panel shows the structure used to format the browse/edit
table display. You can change the data displayed on this screen to alter the format of the table display. 
   Options  Help
  ───────────────────────────────────────────────────────────────────────────────
                          Structure of Table BLSGEDIT           Row 1 to 7 of 9
 Command ===> ________________________________________________ Scroll ===> CSR 
                                  Rows scanned to produce
 Number of Rows . . : 17          structure  . . . . . . . . . . : 17
 Number of Keys . . : 0
 Number of Names  . : 9
 ────────────────────────────────────────────────────────────────────────────
 ZEDPTYPE ZEDPLRCL ZEDPRCFM ZEDPFLAG                 ZEDPBNDL ZEDPBNDR
 ────────────────────────────────────────────────────────────────────────────
 Column  Name      Type  Length  Display Area
 ------  --------  ----  ------  ------------------------------
 001     ZEDPTYPE  Name     8    <= start of display
 002     ZEDPLRCL  Name     8    <= displayed
 003     ZEDPRCFM  Name     8    <= displayed
 004     ZEDPFLAG  Name    24    <= displayed
 005     ZEDPBNDL  Name     8    <= displayed
 006     ZEDPBNDR  Name     8    <= end of display
 007     ZEDPTABC  Name     8
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 136. Table structure panel (ISRUTBP5)
The top area of this screen shows this information about the table:
• Number of Rows
• Number of Keys
• Number of Names
• Rows scanned to produce structure
Note: This value is controlled by the "Maximum rows searched to determine column width" option
on the Table Utility Options panel.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  229

## Page 268

The next area of the screen shows the current column headings for the table display.
The bottom area of the screen shows the current structure used to format the table display. This is a
list containing these fields:
Column
This input field shows a number representing the relative position of the associated table variable
in the table display. You can change the position of a variable in the table display by altering this
number.
Name
The name of the table variable.
Type
Shows a value of Key if the associated variable is defined as a key for the table. Otherwise shows a
value of Name.
Length
This input field shows the number of characters used to display the table variable value. The table
utility calculates this number by scanning the table rows and finding the largest length value for
each table variable.
Note: If you have specified a value for the "Maximum rows searched to determine column width"
option on the Table Utility Options panel, the table utility might not scan all the table rows and
therefore the calculated length value might not be large enough for all variable values.
Display Area
Identifies the table variables currently shown on the table display screen.
When you press Exit (F3) to return to the Edit/Browse panel it is reformatted based on changes made
on the table structure display.
These abbreviations can be used for the STRUCT command:
   STR
   STRU
   STRUC
UP
Scroll Up. The standard ISPF UP command (F7|F19) can be used to vertically scroll the table display
towards the top of the table.
ISPF supported scroll amount values used for the UP command can be entered in the Scroll field. You
can also enter a valid scroll amount in the Command field.
Table output data set selection
This panel is displayed when either the first SAVE command is issued or the EXIT command (F3) is issued,
and these conditions are all true:
• The table you have modified was specified on the Table Utility Entry panel
• you did not select "Always save table in originating data set" on the Table Utility Options panel
• you specified a Table DD rather than a Table Data Set on the Table Utility Entry panel
ISPF table utility (option 3.16)
230  z/OS: z/OS ISPF User's Guide Vol II

## Page 269

Help
 ───────────────────────────────────────────────────────────────────────────────
                        Table Output Data Set Selection        Row 1 to 7 of 2
 Command ===> ________________________________________________ Scroll ===> CSR 
 No table data set was originally specified, only a table DD. Since there was
 more than one table data set allocated to this DD, please select which data
 set should receive the updated table. All future SAVE requests will
 automatically use the selected table data set. Use END or CANCEL to return
 without saving the table.
 S  Table Data Set
 -  ----------------------------------------------
 _  PDFTDEV.LSACKV.TABLES
 _  PDFTOOL.COMMON.TABLES
 _  PDFTDEV.STG.TABLES
 _  PDFTDEV.INT.TABLES
 _  PDFTDEV.SVT.TABLES
 _  ISPFTEST.TABLES
 _  LSACKV.ISPF.ISPPROF
 _  MBURNS.ISPF.ISPPROF
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 137. Table Output Data Set Selection panel (ISRUTBP9)
This panel lists the data sets allocated to the table DD specified on the table utility entry panel. Enter an
S in the selection field for the data set in which you would like the updated table to be saved. If you press
Enter without selecting a data set, the table update is canceled.
Table utility options
To display the Table Utility Options panel, select Table Utility Options from the Options menu on the
action bar. This panel allows you to set options that control certain behaviors within the ISPF Table Utility.
These options on the first section of the panel affect the Edit and Browse functions:
Open table in SHARE mode
Select this option if the table you are to process is already open on another logical screen or if you
might need to share the table with another logical screen.
Use EDIT as default to process selected table
When you select this option, you will default to Edit mode if you do not specify either the Edit or
Browse functions on the table utility entry panel or the table selection panel. You will default to
browse mode when this option is not selected.
Always save table in originating data set
When this option is selected: If the table you are editing was specified on the table utility entry panel
but the Table data set was not specified, the table is automatically saved in the original data set.
When this option is cleared: When you first attempt to save the table, the table output data set
selection panel will be displayed. This panel allows you to choose within the table DD the data set
where the table will be saved.
Maximum rows searched to determine column width
To determine the width required to display each column field, the table utility scans the table rows
and checks the length of the table variable values. This option allows you to specify a number which
acts as the limit for the number of rows scanned in this process. If you leave this value blank, all rows
will be scanned.
Note: Because the table utility uses an efficient method to scan a table, you can leave this option
blank for all but extremely large tables.
Color used to display table key values
Use this option to specify the color (BLUE, RED, PINK, GREEN, TURQ, YELLOW, or WHITE) used to
display the values for the key variable in the table. The default is GREEN.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  231

## Page 270

Intensity used to display table key values
Use this option to specify the intensity (HIGH or LOW) used to display the values for the key variables
in the table. The default is HIGH.
These options on the second section of the panel affect the Import function:
Warn if table exists in the output library
When this option is selected, a warning message will be displayed if you try to import data into a
table that already exists in the specified output data set. You can then choose either to overwrite the
existing table or to cancel the import process. If you don't want to receive warning messages in this
situation, clear this option.
Use Edit to view the imported table
When this option is selected, the table utility uses the Edit function to display the table that was
created or updated by the Import function. If this option is not set, the Browse function will be used to
display the table.
These options on the final section of the panel affect the Export function:
Warn if export data set exists
When this option is selected, a warning message will be displayed if the data set you are exporting
table data into already exists. You can then choose either to overwrite the data set or to cancel the
export process. If you don't want to receive warning messages in this situation, clear this option.
Display mode for export data set
This option allows you to choose whether to use either the ISPF Browse, View, or Edit function to
display the export data set after the export process has completed.
Table export report options
To display the Table Export Reports Options panel, select Export Report Options from the Options menu
on the action bar. This panel allows you to set options which control the format of the report written to the
output data set by the table utility Export function.
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                        Table Export Report Options                          │
 │ Command ===> _____________________________________________________________  │
 │                                                                             │
 │                                                                             │
 │   Enter "/" to select option                                                │
 │      Set options to match IMPORT format report                              │
 │                                                                             │
 │   /  Generate headings                                                      │
 │   /  Underline headings                                                     │
 │                                                                             │
 │ Heading, column and page spacing:                                           │
 │                                                                             │
 │   Blank lines after heading  . . . . . 0   (0 - 9)                          │
 │                                                                             │
 │   Number of spaces between columns . .  1  (0 - 99)                         │
 │                                                                             │
 │   Number of lines per page . . . . . .  0  (0 - 99)                         │
 │                                            (0 if no paging is required)     │
 │                                                                             │
 │   Number of blank lines between pages   0  (0 - 99)                         │
 │                                            (ignored if lines/page = 0)      │
 │                                                                             │
 │ Enter END to save changes.                                                  │
 │ Enter CANCEL to cancel changes.                                             │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F12=Cancel                                                   │
 └─────────────────────────────────────────────────────────────────────────────┘
Figure 138. Table Export Report Options panel (ISRUTBO2)
The panel provides these options:
ISPF table utility (option 3.16)
232  z/OS: z/OS ISPF User's Guide Vol II

## Page 271

Set options for IMPORT format report
Select this option if you want to write the export report in the same format that is used by the table
utility Import function. Selecting this option causes other options on the panel to be set so as to
produce the export report in the appropriate format.
Generate headings
Select this option if you want the export report to have headings for the columns showing the table
variable values.
Underline headings (in export report)
Select this option if you want the column headings to be underlined. Column headings for key
variables are underlined with plus signs (++++++++). Column headings for non-key variables are
underlined with dashes (--------).
Blank lines after heading
This option allows you to specify the number of blank lines printed after the column headings. The
number must be between 0 and 9.
Number of spaces between columns
This option allows you to specify the number of spaces printed between the columns showing the
table variable values. The number must be between 0 and 99.
Number of lines per page
This option allows you to specify the maximum number of lines printed on each page of the report.
The number must be between 0 and 99. If you specify 0, no page breaks will be generated.
Number of blank lines between pages
This option allows you to specify the number of blank lines printed at the end of a page to separate it
from the following page. The number must be between 0 and 99. This option is ignored if you specify
0 for the "Number of lines per page" option.
Export data set attributes
To display the Export Data Set Attributes panel, select Export Data Set Attributes from the Options menu
on the action bar. This panel allows you to set various attributes for the output data set created by the
table utility Export function.
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                          Export Data Set Attributes                         │
 │  Command ===> ____________________________________________________________  │
 │                                                                             │
 │  Management class . . .                (Blank for default management class) │
 │  Storage class  . . . .                (Blank for default storage class)    │
 │   Volume serial . . . .                (Blank for system default volume) ** │
 │   Device type . . . . . SYSALLDA       (Generic unit or device address) **  │
 │  Data class . . . . . .                (Blank for default data class)       │
 │   Space units . . . . . TRACK          (BLKS, TRKS, or CYLS)                │
 │   Primary quantity  . . 5              (In above units)                     │
 │   Secondary quantity    5              (In above units)                     │
 │                                                                             │
 │  ( ** Only one of these fields may be specified)                            │
 │                                                                             │
 │  Enter END to save changes.                                                 │
 │  Enter CANCEL to cancel changes.                                            │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │   F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward    │
 │   F9=Swap       F12=Cancel                                                  │
 └─────────────────────────────────────────────────────────────────────────────┘
Figure 139. Table Export Data Set Attributes panel (ISRUTBO3)
The panel provides these options:
Management class
Specify the SMS management class for the data set. The management class is used to obtain data
management-related information for the data set, such as migration, backup, and retention criteria.
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  233

## Page 272

Storage class
Specify the SMS storage class for the data set. The storage class is used to obtain storage-related
information (volume serial) for the data set.
Volume serial
For a non-SMS data set, specify the volume serial of the direct-access volume you wish to contain the
data set.
Device type
For a non-SMS data set, specify the generic unit address for the direct access volume you wish to
contain the data set.
Data class
Specify the SMS data class for the data set. The data class is used to obtain data-related information
(space units, primary quantity, secondary quantity, directory block, record format, record length, and
data set name type) for the data set.
Space units
Specify the disk space units (TRACK, CYLINDER, or BLOCK).
Primary quantity
Enter a number for the primary allocation in space units.
Secondary quantity
Enter a number for the secondary allocation in space units.
Importing data into a table
The ISPF Table Utility supports an Import function, where data in a sequential data set is used to load
an ISPF table. The import function is invoked by entering option I (Import table data) on the ISPF Table
Utility entry panel.
When you use the Import function, enter these fields on the table utility entry panel:
Table Data Set
This identifies the data set where the table that will be created or updated by the Import function is
saved.
Table Name
The name of the table that will be created or updated by the Import function.
Import Data Set
The name of the data set containing the data used as input to the Import function.
The Import function requires the Import Data Set to contain a report in a specific format. This format is
generated by the ISPF table utility Export function and has these features:
• The variable name is used as a heading for each column showing the values for the table variables.
• The headings are underlined. Column headings for key variables are underlined with plus signs (++++++
++). Column headings for non-key variables are underlined with dashes (--------).
• There are no blank lines after headings.
• There is only 1 space between columns.
• There is no paging.
A warning message might be displayed if the table you specify already exists in the Table Data Set. You
then have the option of terminating the command to avoid overwriting the table. If you don't want to
receive these warnings in future, clear the "Warn if table exists in the output library" check box on the
Table Utility Options panel.
When the import process has finished, the table that was created or updated is displayed. The "Use Edit
to view the imported table" option on the Table Utility Options panel allows you to choose either the table
utility Edit or Browse function to display the imported table. The default is Browse.
ISPF table utility (option 3.16)
234  z/OS: z/OS ISPF User's Guide Vol II

## Page 273

Exporting data from a table
The ISPF Table Utility supports an Export function, where data in an ISPF table is used to write a
customizable report to a sequential file. The Export function is invoked by using either the EXPORT or
FEXPORT primary commands from the table display screen for the table utility Edit and Browse functions.
Note: The report does not show the values for extension variables defined for table rows.
The EXPORT primary command displays the Export Layout panel, where you can control the layout of the
table data in these ways:
• Exclude table variables from the report
• Change the order in which the table variables appear in the report
• Change the column headings
The Table Export Report Options panel allows you to change the format of the report. Selecting the
"Set options for IMPORT format report" option ensures the generated report is in a format that can be
processed by the table utility Import function. The Export Data Set Attributes panel allows you to define
the allocation attributes for the export data set.
Figure 140 on page 235 shows an example of the export report in a format suitable for the Import
function:
********************************* Top of Data **********************************
SUBSYS  SERVER  DESCRIPT                            BSECEXIT BCATOWN BJCLASS BJS
+++++++ ------- ----------------------------------- -------- ------- ------- ---
DB2D            DB2 Version 5                                        A
DB26            DB2 Version 6 - Subsystem 1                          A
DB62            DB2 Version 6 - Subsystem 2                          A
DB27            DB2 Version 7                                        A
DBT5            DB2 Version 5.1 for FM/DB2 FVT only                  A
DBT6            DB2 Version 6.1 for FM/DB2 FVT only                  A
DBT7            DB2 Version 6.1 for FM/DB2 FVT only                  A
******************************** Bottom of Data ********************************
Figure 140. Export report example
Processing tables that are currently open
Normally, ISPF does not allow a table to be opened and processed if that table is currently open.
However, if the table currently open has been opened with the SHARE option, a subsequent open of the
table is allowed if:
• The SHARE option is used, and
• The WRITE/NOWRITE option is the same as specified when the table was initially opened
The ISPF Table Utility provides support which allows you to process a table even when that table is
currently open.
If a table is currently open in SHARE mode, the ISPF Table Utility can be used to process that table
provided you select the "Open table in SHARE mode" option on the ISPF Table Utility panel and the Edit
(WRITE) or Browse (NOWRITE) option specified matches the WRITE/NOWRITE setting when the table
was originally opened.
If a table data set (rather than a table DD) is specified on the entry panel, the ISPF Table Utility also
allows you to process a table that is open but not in SHARE mode, or a table that is open in SHARE mode
but the WRITE/NOWRITE (edit/browse) setting does not match that of the open table. When this situation
is detected, one of the popup windows shown here is displayed allowing you to specify the way in which
to process the table:
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  235

## Page 274

┌───────────────────────────────────────────────────────────────────┐
 │ ISRUTBC2            Confirm Table Processing                      │
 │ Command ===>                                                      │
 │                                                                   │
 │ CAUTION:                                                          │
 │ The table TSTTABA is currently open to you or another user.       │
 │                                                                   │
 │ Instructions:                                                     │
 │                                                                   │
 │ Press ENTER key to process a temporary copy of the table in data  │
 │ set VANDYKE.TBUTIL.TABLES.                                        │
 │                                                                   │
 │ Press CANCEL or EXIT to cancel processing of the table.           │
 │                                                                   │
 │                                                                   │
 │                                                                   │
 │                                                                   │
 │                                                                   │
 │                                                                   │
 ⋘───────────────────────────────────────────────────────────────────┘
Figure 141. Panel displayed when the selected table is currently open but not in SHARE mode
The panel shown in Figure 141 on page 236 is displayed when the selected table from the table data set
(TSTTABA) is currently open but not in SHARE mode.
 ┌───────────────────────────────────────────────────────────────────┐
 │ ISRUTBC1            Confirm Table Processing                      │
 │ Command ===>                                                      │
 │                                                                   │
 │ CAUTION:                                                          │
 │ The table TSTTABA is currently open to you in SHARE/NOWRITE mode  │
 │                                                                   │
 │ Instructions:                                                     │
 │                                                                   │
 │ Press ENTER key to process a temporary copy of the table in data  │
 │ set VANDYKE.TBUTIL.TABLES.                                        │
 │                                                                   │
 │ Press EXIT key to process the currently open table                │
 │ in SHARE/NOWRITE (browse) mode.                                   │
 │                                                                   │
 │ Press CANCEL to cancel processing of the table.                   │
 │                                                                   │
 │                                                                   │
 │                                                                   │
 ⋘───────────────────────────────────────────────────────────────────┘
Figure 142. Panel displayed when the selected table is currently open in SHARE mode for NOWRITE
The panel shown in Figure 142 on page 236 (ISRUTBC1) is displayed when the selected table (TSTTABA)
from the table data set is currently open in SHARE mode for NOWRITE (not for update) and you either:
• Did not select the Open table in SHARE mode option on the entry or options panel, or
• Requested to edit (WRITE) the table
ISPF table utility (option 3.16)
236  z/OS: z/OS ISPF User's Guide Vol II

## Page 275

┌───────────────────────────────────────────────────────────────────┐
 │ ISRUTBC1            Confirm Table Processing                      │
 │ Command ===>                                                      │
 │                                                                   │
 │ CAUTION:                                                          │
 │ The table TSTTABA is currently open to you in SHARE/WRITE mode.   │
 │                                                                   │
 │ Instructions:                                                     │
 │                                                                   │
 │ Press ENTER key to process a temporary copy of the table in data  │
 │ set VANDYKE.TBUTIL.TABLES.                                        │
 │                                                                   │
 │ Press EXIT key to process the currently open table                │
 │ in SHARE/WRITE (edit) mode.                                       │
 │                                                                   │
 │ Press CANCEL to cancel processing of the table.                   │
 │                                                                   │
 │                                                                   │
 │                                                                   │
 ⋘───────────────────────────────────────────────────────────────────┘
Figure 143. Panel displayed when the selected table is currently open in SHARE mode for WRITE
The panel shown in Figure 143 on page 237 (ISRUTBC1) is displayed when the selected table (ISRPLIST)
is currently open in SHARE mode for WRITE (for update) and you either:
• Did not select the Open table in SHARE mode option on the entry or options panel, or
• Requested to browse (NOWRITE) the table
If you press Enter, the table utility:
• Creates a temporary partitioned data set.
• Copies the table from the table data set you specified. into a member in the temporary data set using a
generated member name.
• Opens the table using the generated name.
• Displays the table data.
If you press the Exit key (usually PF3) on panel ISRUTBC1, the table utility:
• Displays the data for the currently open table. If this table was originally opened for WRITE, the data is
displayed for edit, otherwise it is displayed for browse.
If you press Cancel (or Exit on panel ISRUTBC2):
• The table is not processed and you are returned to the entry or table selection panel.
If you edit a temporary copy of an open table, this panel is displayed when you exit the edit display:
ISRUTBPB                 Save Temporary Table
Command ===>
Specify the names of the data set and member where the temporary table
will be saved.
Partitioned Data Set Name and Member
   Name  . . . . . . . 'VANDYKE.TBUTIL.TABLES'                       
   Member  . . . . . . ISRPLIST
Enter "/" to select option
   Replace existing member
Instructions:
Press ENTER key to save the temporary table in the specified data set
and member.
Press EXIT or CANCEL to exit without saving the temporary table.
This panel allows you to save the updated table in a specified data set and member. The panel initially
shows the table data set and table (member) you requested to edit. The "Replace existing member"
ISPF table utility (option 3.16)
Chapter 5. Utilities (option 3)  237

## Page 276

option allows you to replace an existing member with the data from the table you have edited. If you
press Enter, the table utility writes the table data to the specified data set and member. If you press Exit
or Cancel, the data from the temporary table is not saved.
Line command table support
Figure 144 on page 238 shows Edit line command table LINECMD.
   Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                      ISPF EDIT Line Command Table  LINECMD     Row 1 to 5 of 5
 Command ===>                                                  Scroll ===> CSR 
                                                                Shift ===> PAGE
     User     MACRO    Program  Block    Multi    Dest
     Command           Macro    format   line     Used
     ----+--- ----+--- ----+--- ----+--- ----+--- ----+---                      
     CE       POSLINE  N        Y        Y        Y                             
     RV       POSLINE  N        Y        Y        Y                             
     LEF      POSLINE  N        Y        Y        Y                             
     RIT      POSLINE  N        Y        Y        Y                             
     XB       $XB      N        N        N        N      
 ******************************* Bottom of data ********************************
Figure 144. ISPF EDIT Line Command Table (LINECMD)
Figure 144 on page 238 defines five commands: CE, RV, LEF, RIT and XB. The first four commands are
processed by edit macro POSLINE and the last command is processed by $XB.
Each row in the table contains the following columns:
Table 16. ISPF EDIT Line Command Table Description
Column Description
User Command The line command value. Must not conflict with
ISPF editor line commands.
MACRO The name of the program, REXX exec, or CLIST
edit macro to be run when the specified Edit line
command is entered.
Program Macro Y - the macro is a program
N - the macro is a CLIST or REXX exec
Block format Y - the line command supports a block format
indicated by repeating the last character of the
command
N - block format is not supported
Multi line Y - the line command supports processing a range
of lines by providing a numeric suffix with the
command
N - processing a range is not supported
Dest Used Y - the line command requires a destination line
command
N - no destination command is required
Each row describes the characteristics of a user-written line command.
ISPF table utility (option 3.16)
238  z/OS: z/OS ISPF User's Guide Vol II

## Page 277

The following is an example of REXX Edit Macro POSLINE:
/* REXX implement CE, RV, LEF, and RIT line commands                */
/*                                                                  */
/* CE : Center text on a line                                       */
/* RV : Reverse text on a line                                      */
/* LEF: Move text all the way left                                  */
/* RIT: Move text all the way right                                 */
Address isredit                              /*  Start of macro     */
"MACRO (PARM) NOPROCESS"                     /*  Get line command   */
/* Parm contains the value exactly as entered in the line cmd area  */
/* If user enters a block or multi-line format it would be easier   */
/* to have the table entry handy                                    */
Address ispexec "VGET (ZLMACENT) SHARED"     /* Get the line command*/
Address ispexec "CONTROL ERRORS RETURN"      /*  Return ISPF errors */
If wordpos(zlmacent,"CE RV LEF RIT") = 0 Then/*  If not an expected */
    Do                                       /*   command           */
      zinfo=parm                             /*  Set up for message */
      Address ispexec "SETMSG MSG(ISRE041)"  /*    Invalid command  */
      Exit  8                /* let ISPF handle the error           */
    End                      /*                                     */
"PROCESS RANGE" zlmacent     /* Get range for command               */
If rc>0  Then                /* If an error occurred                */
  Do                         /*                                     */
    Address ispexec  "SETMSG MSG(ISRZ002)"   /* Set ISPF's message  */
    Exit  8                  /* Let ISPF handle the error           */
  End                        /*                                     */
parmin = parm                /* Actual value might be needed later  */
parm = zlmacent              /* just keeping the comments in line   */
"(START) = LINENUM .ZFRANGE" /* Get 1st line number in the range    */
"(STOP)  = LINENUM .ZLRANGE" /* Get last line number in the range   */
"(DW)  = DATA_WIDTH"         /* Get the width of the editable data  */
Do a = start to stop         /* Loop through the range of lines     */
  "(LINE) = LINE "a          /* Get old line value                  */
  SELECT                     /* process the command for this line   */
    When(parm = "CE")  Then line=center(strip(line),dw) /* Center   */
    When(parm = "RV")  Then line=reverse(line)          /* Reverse  */
    When(parm = "LEF") Then line=strip(line,"L")    /* Left justify */
    When(parm = "RIT") Then line=right(strip(line,"T"),dw) /* Right */
                             /*                            Justify  */
    Otherwise  Nop           /* Otherwise, no op (should not occur) */
  End                        /*                                     */
  "LINE "a" = (LINE)"        /* Set new line value                  */
End                          /* End of loop through lines           */
exit 0                       /* Return to ISPF                      */
z/OS UNIX directory list utility (option 3.17)
The z/OS UNIX Directory List Utility (option 3.17) supports processing of directories and files in a
z/OS UNIX directory structure. When you select this option, the z/OS UNIX Directory List Utility panel
(ISRUULP) is displayed.
The layout and options of the Directory List Utility are similar to those in the Data Set List Utility (ISPF
option 3.4). You can either display the directory list under the specified path name for further processing,
or print the directory list to the ISPF list data set.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  239

## Page 278

Menu  RefList  RefMode  Utilities  Options  File_Systems  Help 
────────────────────────────────────────────────────────────────────────────────
                        z/OS UNIX Directory List Utility
    blank Display directory list              P Print directory list            
                                                                                
    Pathname . . . /var                                                        +
                                                                                
                                                                                
 Enter "/" to select option                                                     
 /  Confirm File Delete                                                    
 /  Confirm Non-empty Directory Delete                                     
                                                                                
 When the directory list is displayed, enter either:                            
   "/" on the directory list line command field for the command prompt pop-up,  
   an ISPF line command, the name of a TSO command, CLIST, or REXX exec, or     
   "=" to execute the previous command.                                         
                                                                                
                    
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
 Option ===>                                                                  
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward   
  F9=Swap     F10=Actions  F12=Cancel 
Figure 145. z/OS UNIX Directory List Utility panel (ISRUULP)
Note: When the z/OS UNIX Directory List Utility panel is first displayed, a message is displayed showing
the time zone that is used to calculate the date and time values displayed in the directory list. The time
zone value is obtained from the z/OS UNIX TZ environment variable. If a value for the TZ environment
value is not found in the system-wide /etc/profile file or the user's .profile file, the utility calculates
displayed date and time values using the operating system GMT offset.
z/OS UNIX Directory List Utility panel action bar
The z/OS UNIX Directory List Utility Panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
RefList
For information about referral lists, refer to the Using Personal Data Set Lists and Library Lists topic in
the z/OS ISPF User's Guide Vol I.
RefMode
For information about referral list modes, refer to information about Personal List Modes in Using
Personal Data Set Lists and Library Lists topic in z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Options
The Options pull-down offers these choices:
 1
Directory List Options
Options that control the behavior of the directory list display.
 2
Directory List Column Arrangement
Settings that alter the order and size of the column fields that are displayed in the directory list.
z/OS UNIX directory list utility (option 3.17)
240  z/OS: z/OS ISPF User's Guide Vol II

## Page 279

3
Directory List Default Line Commands
Settings that define the default line commands for the different z/OS UNIX file types.
 4
Enable superuser mode(SU)
Select this option to switch to superuser mode.
File_Systems
The File_Systems pull-down offers these choices:
 1
Mount Table by File System...
Displays the z/OS UNIX mounted file systems, ordered by file system name. For more information,
see “z/OS UNIX Mounted File Systems” on page 286.
 2
Mount Table by Mount Point...
Displays the z/OS UNIX mounted file systems, ordered by mount point name. For more
information, see “z/OS UNIX Mounted File Systems” on page 286.
 3
Mount...
Provides the option to mount a file system. For more information, see “MOUNT command” on
page 299.
 4
New zFS...
Provides the option to create a new zSeries File System (zFS) data set. For more information, see
“Creating a new zFS” on page 301.
 5
zFS aggregates...
Displays the attached zFS aggregates. Provides options for displaying aggregate and file system
information and extending the size of a zFS aggregate. For more information, see “zFS aggregates”
on page 303.
Help
The Help pull-down provides general information about z/OS UNIX Directory List Utility topics as well
as information about displaying and printing a z/OS UNIX Directory List.
z/OS UNIX Directory List Utility panel fields
The z/OS UNIX Directory List Utility panel contains these fields:
Pathname
This is a scrollable field where you enter the path name of the directory you want to list or print. If you
leave this field blank, your home directory is used. If the field is blank and you do not have a home
directory, you are prompted to enter a path name.
Note: If you often enter long path names (greater than 56 characters), consider using the KEYLIST
utility to update the keylist for the panel and assign the ZEXPAND command to a function key.
The ZEXPAND command displays the scrollable input field in a scrollable dynamic area in a pop-up
window, making the task of entering a long pathname easier.
When you enter a z/OS UNIX file path name, a z/OS UNIX directory selection list is displayed.
When you enter a z/OS UNIX file path name containing glob characters and the entered value does not
match a z/OS UNIX directory, ISPF uses the C/C++ glob function to search the UNIX file system for
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  241

## Page 280

files and directories that match the mask. Unicode Conversion services are used to internally convert
the path name from the terminal codepage to codepage 1047 for use by the search function.
You can use these special characters at the beginning of the Pathname field to represent the path
name for a particular directory:
~
(Tilde) The path name for your home directory.
.
(Period) The path name for your current working directory.
..
(Double period) The path name of the parent directory of your current working directory.
Note: Within the z/OS UNIX Directory List Utility, you can also use these special characters in any field
where a z/OS UNIX file path name can be entered.
Glob characters and their meaning are:
?
Match any single character.
*
Match multiple characters.
[
Open a set of single characters.
]
Close the set of single characters. Each character in the set can match a single character at the
position specified.
Confirm File Delete
This option controls the display of the Confirm Delete panel. This panel is displayed when deleting
files or empty directories from the directory list display using the D line command. If this option
is selected, the Confirm Delete panel is displayed. If this option is not selected, the panel is not
displayed and the file or empty directory is deleted without any additional user interaction.
Confirm Non-empty Directory Delete
This option controls the display of the Confirm Non-empty Directory Delete panel. This panel is
displayed when using the directory list D line command to delete a directory that contains files and
subdirectories. If this option is selected, the Confirm Non-empty Directory Delete panel is displayed.
If this option is not selected, the panel is not displayed and the directory (including all contained files
and subdirectories) is deleted without any additional user interaction.
z/OS UNIX Directory List Utility panel options
See:
• “Blank—display directory list” on page 242
• “P—print directory list” on page 251
Blank—display directory list
To display a directory list for the specified path, leave the Option line blank and press Enter. If you leave
the Pathname field blank, your home directory will be used.
You can also specify the options Confirm File Delete and Confirm Non-empty Directory Deleteto control
the behavior of the D (delete file) line command in the directory list.
z/OS UNIX directory list utility (option 3.17)
242  z/OS: z/OS ISPF User's Guide Vol II

## Page 281

Menu  Utilities  View  Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                            z/OS UNIX Directory List          Row 1 to 13 of 25
 Command ===>                                                  Scroll ===> PAGE
 Pathname . : /SYSTEM/etc
 Command  Filename        Message          Type Permission Audit  Ext  Fmat
 -------------------------------------------------------------------------------
          .                                Dir  rwxr-xr-x  fff---
          ..                               Dir  rwxr-xr-x  fff---
          .nfsc                            File rw-r--r--  fff--- --s- ----
          ant.conf                         File rwxrwxrwx  fff--- --s- ----
          bpa                              Dir  rwxr-xr-x  fff---
          cmx                              Dir  rwxr-xr-x  fff---
          dce                              Dir  rwxr-xr-x  fff---
          dfs                              Dir  rwxr-xr-x  fff---
          inetd.conf                       File rwxrwxrwx  fff--- --s- ----
          inetd.pid                        File rw-r--r--  fff--- --s- ----
          ioepdcf                          Syml rwxrwxrwx  fff---
          ldap                             Dir  rwxr-xr-x  fff---
          licmgmt                          Dir  rwxr-xr-x  fff---
          log                              File rw-rw----  fff--- --s- ----
          pkiserv                          Dir  rwxr-xr-x  fff---
          profile                          File rwxr-xr-x  fff--- --s- ----
          security                         Dir  rwxr-xr-x  fff---
  F1=Help    F2=Split   F3=Exit    F4=Expand  F5=Rfind   F7=Up      F8=Down
  F9=Swap   F10=Left   F11=Right  F12=Cancel
Figure 146. z/OS UNIX Directory List panel (ISRUUDL0)
The information for each entry in the directory is displayed in column fields across the screen. The
number of columns displayed depends on the available screen width. Figure 146 on page 243 shows the
initial directory list display on a terminal with a screen width of 80 and a screen depth of 28.
The RIGHT primary command can be used to scroll the displayed column fields to the right. Figure 147 on
page 243 shows the directory list display when the RIGHT command is issued on the previous display:
   Menu  Utilities  View  Options  Help
 ───────────────────────────────────────────────────────────────────────────────
                            z/OS UNIX Directory List          Row 1 to 13 of 25
 Command ===>                                                  Scroll ===> PAGE
 Pathname . : /SYSTEM/etc
 Command  Filename        Message          Owner    Group    Links  Size
 -----------------------------------------------------------------------------
          .                                IBMUSER  OMVSGRP      14       8192
          ..                               IBMUSER  OMVSGRP       6       8192
          .nfsc                            IBMUSER  OMVSGRP       1          0
          ant.conf                         IBMUSER  OMVSGRP       1         29
          bpa                              IBMUSER  OMVSGRP       2       8192
          cmx                              IBMUSER  OMVSGRP       2       8192
          dce                              IBMUSER  OMVSGRP       9       8192
          dfs                              IBMUSER  OMVSGRP       8       8192
          inetd.conf                       IBMUSER  OMVSGRP       1       1215
          inetd.pid                        IBMUSER  OMVSGRP       1         10
          ioepdcf                          IBMUSER  OMVSGRP       1         22
          ldap                             IBMUSER  OMVSGRP       2       8192
          licmgmt                          IBMUSER  OMVSGRP       6       8192
          log                              IBMUSER  OMVSGRP       1          0
          pkiserv                          IBMUSER  OMVSGRP       2       8192
          profile                          IBMUSER  OMVSGRP       1      10665
          security                         IBMUSER  OMVSGRP       2       8192
  F1=Help    F2=Split   F3=Exit    F4=Expand  F5=Rfind   F7=Up      F8=Down
  F9=Swap   F10=Left   F11=Right  F12=Cancel
Figure 147. z/OS UNIX Directory List—scrolling right
Note: These two screens assume that the default column arrangement settings are used. You can change
the width of column fields and the order in which they are displayed, and remove selected columns from
the directory list display. See “z/OS UNIX Directory List Column Arrangement panel” on page 284.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  243

## Page 282

z/OS UNIX Directory List panel action bar
The z/OS UNIX Directory List panel action bar choices function as follows:
Menu
For more information, see the details about the Menu Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
Utilities
For more information, see the details about the Utilities Action Bar Choice in the ISPF User Interface
topic in the z/OS ISPF User's Guide Vol I.
View
The View pull-down offers this choice: 1. Sort
You can sort the list by any of these fields:
1. Filename
2. Message
3. File Type
4. Permissions
5. Permissions - Octal
6. Owner
7. Audit
8. Extended Attributes
9. Format
10. Group
11. Links
12. Size
13. Changed Date/Time
14. Modified Date/Time
15. Accessed Date/Time
16. Created Date/Time
17. Case-Insensitive
You can also specify the sort sequence (ascending or descending) or accept the default sequence for
the associated sort field. By default, character fields are sorted alphabetically and numeric fields are
sorted in descending order.
Options
The Options pull-down offers these choices:
 1
Directory List Options
Options that control the behaviour of the directory list display.
 2
Directory List Column Arrangement
Settings that alter the order and size of the directory list column fields and allow you to remove
columns from the display.
 3
Directory List Default Line Commands
Settings that define the default line commands for the different z/OS UNIX file types.
 4
Enable superuser mode(SU)
z/OS UNIX directory list utility (option 3.17)
244  z/OS: z/OS ISPF User's Guide Vol II

## Page 283

Select this option to switch to superuser mode.
 5
Refresh List
Refresh the display of the directory list.
 6
Save List
Save the directory list to a file.
 7
Reset
Reset the directory list.
Help
The Help pull-down provides general information about z/OS UNIX Directory List Utility topics,
including the format of the directory list and the available line commands and primary commands.
z/OS UNIX Directory List panel fields 
The fields listed here can appear on the directory list panel. Which fields are displayed depends on the
column arrangements settings and whether the display has been scrolled left or right.
Command
Field used to enter a line command, z/OS UNIX command, TSO command, CLIST, or REXX exec
against a directory list entry.
Filename
The name of the file or subdirectory.
Message
This field is initially blank. After you run one of the built-in line commands on a file or subdirectory, a
message is displayed showing the last function used on that file or subdirectory:
LineCommand
Message
AA
Modified
B
Browsed
CI
Replaced
CO
Copied
E
Edited
D
Deleted
FS
Information
I
Information
L
Listed
MF
Modified
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  245

## Page 284

MG
Modified
MM
Modified
MO
Modified
MX
Modified
N
Created
R
Renamed
UA
Modified
X
(Depends on whether a z/OS UNIX command or TSO command is executed)
If you enter a TSO command, CLIST, or REXX exec on the Command line, a default message appears in
the Message field. The message is in this format:
XXXXXXXX  RC=#
where:
XXXXXXXX
is the command entered
#
is the return code from the command
If you enter a z/OS UNIX command, the completion status is indicated by one of these messages
being displayed in the Message field:
Ended xxx
Command has ended with a return code of xxx
Terminated xxx
Command has terminated due to signal xxx
Stopped xxx
Command has stopped due to signal xxx
Timed out
The elapsed running time of the command exceeded the specified time limit. ISPF sent a SIGKILL
signal to terminate the process.
Type
The directory entry type. The possible values are:
Dir
Directory
File
Regular file
Char
Character special file
FIFO
FIFO (first-in-first-out) special file
Syml
Symbolic link
z/OS UNIX directory list utility (option 3.17)
246  z/OS: z/OS ISPF User's Guide Vol II

## Page 285

Extl
External symbolic link
Perm
The file or subdirectory permissions, in octal format. The permissions are displayed as three octal
(range 0-7) digits. The first digit defines the access permission for the file owner. The second digit
defines the access permission for any member of the file's group. The third digit defines the access
permission for anyone else. Table 17 on page 247 shows the values and associated permissions for
the octal digits: 
Table 17. Octal permission values
Value Permissions
0 None
1 Search (or execute)
2 Write
3 Write and search (or execute)
4 Read
5 Read and search (or execute)
6 Read and write
7 Read, write and search (or execute)
If there are extended access control list (ACL) entries defined for the file or subdirectory, the character
+ is displayed after the octal value.
Permissions
The file or subdirectory permissions, in symbolic format. There are three groups of three characters.
The first group describes owner permissions; the second describes group permissions; and the third
describes other (or "world") permissions. The characters that may appear in each group are:
r
Permission to read the file
w
Permission to write to the file
x
Permission to execute the file
These characters can appear in the execute (third) position of each group:
s
If in owner permissions group, the set-user-ID bit is on; if in group permissions section, the
set-group-ID bit is on.
S
Same as s except the execute bit is off.
t
The sticky bit is on.
T
Same as t except the execute bit is off.
Note: You can specify whether permissions are to be displayed in octal or symbolic format on the
z/OS UNIX Directory List Options panel.
Audit
Two groups of three characters describing the audit bit settings. The first three characters describe
the user-requested audit information. The last three characters describe the auditor-requested audit
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  247

## Page 286

information. Each group of three characters shows the read, write, and execute (search) audit options.
The possible values are:
s
Audit successful access attempts
f
Audit failed access attempts
a
Audit all access attempts
-
No audit
Ext
A group of four characters describing the extended attributes for a regular file. The possible values
are:
a
Program runs APF-authorized if linked AC=1
p
Program is considered program-controlled
s
Program is enabled to run in a shared address space
l
Program is loaded from the shared library region
-
Attribute not set
Fmat
File format for regular files. The possible values are:
bin
Binary data
nl
New line
cr
Carriage return
lf
Line feed
crlf
Carriage return followed by line feed
lfcr
Line feed followed by carriage return
crnl
Carriage return followed by new line
Owner
The user ID of the owner of the file or subdirectory.
Group
The group name of the owner of the file or subdirectory.
Links
For a file, the number of hard links to the file. For a subdirectory, the number of subdirectories it
contains.
Size
The file size, in bytes.
z/OS UNIX directory list utility (option 3.17)
248  z/OS: z/OS ISPF User's Guide Vol II

## Page 287

Modified
The date and time the file was last changed.
Changed
The date and time the status of the file was last changed.
Accessed
The date and time the data in the file was last accessed.
Created
The date and time the file was created.
Actions you can take from the Directory List panel
These topics describe actions you can take from the Directory List panel:
• “Line commands” on page 249
• “z/OS UNIX commands, TSO commands, CLISTs, and REXX EXECs” on page 249
• “Using the path name substitution character” on page 250
Line commands
Line commands can be entered in the Command field to the left of the directory list entries.
z/OS UNIX commands, TSO commands, CLISTs, and REXX EXECs
Besides the ISPF-supplied line commands, you can also enter z/OS UNIX Commands, TSO commands,
CLISTs, and REXX EXECs that use a path name as an operand. The line command field is a scrollable field
with a maximum length of 255 characters and a display length of 8 characters. If the command you want
to enter requires more space than is available in the display field, use the EXPAND function key (F4) to
display the entire 255-character line command input field in a pop-up window.
The line command prefix characters > and < are used to identify a command to be run in z/OS UNIX.
ISPF uses the spawn service (BPX1SPN) to create a new process and execute the command. The > prefix
character requests that the command be run by the z/OS UNIX login shell. The < prefix character requests
that the command be run directly.
Figure 148 on page 249 shows an example of using the c89 shell command to compile, link-edit, and
assemble the C program contained in the file /u/myhome/hello.c. The > character before the command
name indicates that it will be run in a login shell environment:
                            z/OS UNIX Directory List            Row 1 to 6 of 6
 Command ===>                                                  Scroll ===> CSR 
 Pathname . : /u/myhome
 Command  Filename        Message          Type Permission Audit  Ext  Fmat
 -------------------------------------------------------------------------------
          .                                Dir  rwxrwxrwx  fff---
          ..                               Dir  rwxrwxrwx  fff---
          bin                              Dir  rwxrwxrwx  fff---
 >c89_    hello.c                          File rwxrwxrwx  fff--- --s- ----
          prog1                            File rwxrwxrwx  fff--- --s- ----
          test1                            File rwxrwxrwx  fff--- --s- ----
 ******************************* Bottom of data ********************************
⋮
Figure 148. Example: specifying a z/OS UNIX command to run on a selected file 
Figure 149 on page 250 shows an example of running the program /u/myhome/hello.c directly in z/OS
UNIX. The < character indicates that the selected file is the name of a command that is to be run:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  249

## Page 288

z/OS UNIX Directory List            Row 1 to 6 of 6
 Command ===>                                                  Scroll ===> CSR 
 Pathname . : /u/myhome/bin
 Command  Filename        Message          Type Permission Audit  Ext  Fmat
 -------------------------------------------------------------------------------
          .                                Dir  rwxrwxrwx  fff---
          ..                               Dir  rwxrwxrwx  fff---
          bin                              Dir  rwxrwxrwx  fff---
 <_       hello.c                          File rwxrwxrwx  fff--- --s- ----
          prog1                            File rwxrwxrwx  fff--- --s- ----
          test1                            File rwxrwxrwx  fff--- --s- ----
 ******************************* Bottom of data ********************************
⋮
Figure 149. Example: running the selected file  directly
A line command that is not recognized as a z/OS UNIX Directory List line command, or is not prefixed
with < or >, is assumed to be a TSO command, CLIST, or REXX EXEC. These commands are passed to
TSO for execution using the ISPF SELECT CMD service. Variable names that start with an ampersand (&)
are evaluated by ISPF. If you want the underlying command processor to see the ampersand you must
specify two consecutive ampersands (&&).
Figure 150 on page 250 shows an example of running a REXX EXEC called LISTDATA against the file prog1
in directory /u/myhome. This is the same as entering this command on the Command line:
TSO LISTDATA '/u/myhome/prog1'
                            z/OS UNIX Directory List            Row 1 to 6 of 6
 Command ===>                                                  Scroll ===> CSR 
 Pathname . : /u/myhome
 Command  Filename        Message          Type Permission Audit  Ext  Fmat
 -------------------------------------------------------------------------------
          .                                Dir  rwxrwxrwx  fff---
          ..                               Dir  rwxrwxrwx  fff---
          bin                              Dir  rwxrwxrwx  fff---
          hello.c                          File rwxrwxrwx  fff--- --s- ----
 listdata prog1                            File rwxrwxrwx  fff--- --s- ----
          test1                            File rwxrwxrwx  fff--- --s- ----
 ******************************* Bottom of data ********************************
⋮
Figure 150. Example: specifying a REXX exec to run on a selected file 
Note: If the TSO command, CLIST, or REXX exec issues a return code greater than or equal to 8,
processing stops and an error message is displayed.
Using the path name substitution character
If a command, CLIST, or REXX exec requires the file or subdirectory path name in a position other than
the first operand or if other operands are needed, you can use the path name substitution character to
represent the path name. If no operands are specified after the command, ISPF uses the path name of
the file or subdirectory that is being acted on as the first operand of the command.
Note: For TSO commands, CLISTs, and REXX EXECs, the path name is enclosed in quotes.
The ISPF-defined default for the path name substitution character is the exclamation point (!) character.
You can change the value of this character using the z/OS UNIX Directory List Options panel.
For example, if you specify: CLIST1 FILE(!) DEBUG in the line command field for file test_data in
directory u/myhome the effect will be the same as if you had entered this primary command:
TSO CLIST1 FILE('/u/myhome/test_data') DEBUG
z/OS UNIX directory list utility (option 3.17)
250  z/OS: z/OS ISPF User's Guide Vol II

## Page 289

P—print directory list
Use option P to print a directory list. You must:
1. Enter the path name for the directory you want to list in the Pathname field. If you leave this field
blank, the path name for your home directory will be used.
2. Press Enter to print the directory list. The directory list is stored in the ISPF list data set.
Note: The format of the printed directory list is not affected by any changes you make using the z/OS
UNIX Directory List Column Arrangement panel.
z/OS UNIX directory list utility line commands
After you display a directory list by leaving the Option field blank, you can enter a line command to the
left of a directory entry. You can also enter TSO commands, CLIST names, or REXX exec names. The path
name substitution character can be used with TSO commands, CLISTs, and REXX EXECs to represent the
quoted path name for a file or subdirectory. For more information about using this symbol, see “Using the
path name substitution character” on page 250.
The line command field is a scrollable field with a maximum length of 255 characters and a display length
of 8 characters. If the command you want to enter requires more space than is available in the display
field, use the EXPAND function key (F4) to display the entire 255-character line command input field in a
pop-up window.
If you enter a slash (/) in the Command field, the Directory List Actions pop-up window is displayed. This
window allows you to select the line command you wish to invoke.
You can also enter line commands in block command format to execute the same line command for
several files at once. You mark the block by typing a "/ /" at the beginning of a block of rows and another
"/ /" at the end of the block of rows. You must type the line command either immediately after the / / on the
first row of the block, or immediately after the / / on the last row of the block. You can enter several blocks
of commands at the same time, but you cannot nest them. Single line commands are not allowed within
a block command. You can execute all line commands, including z/OS UNIX commands, TSO commands,
Clists and REXX execs as block commands.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  251

## Page 290

Menu  Utilities  View  Options  Help
 ─ ┌───────────────────────────────────────────────────────────────┐ ───────────
   │                    Directory List Actions                     │  to 5 of 5
 C │                                                               │  ===> PAGE
   │ File ----- /u/myhome/bin/ctest                                │
 P │                                                               │
   │ DIRLIST Action                                                │
 C │     1.  Edit                     13. File System              │   Fmat
 - │     2.  Edit - ASCII             14. Modify Mode Fields       │ -----------
   │     3.  View                     15. Modify Extended Attrs    │   ----
   │     4.  View - ASCII             16. Modify Owning User       │   ----
   │     5.  Browse                   17. Modify Owning Group      │ - ----
   │     6.  New                      18. Modify Format            │ - ----
   │     7.  Directory List           19. User Auditing            │ - ----
 * │     8.  Delete                   20. Auditor Auditing         │ ***********
   │     9.  Rename                   21. Execute command          │
   │     10. Copy Out                 22. Refadd                   │
   │     11. Copy In                  23. Manage ACLs              │
   │     12. Information                                           │
   │                                                               │
   │ Select a choice and press ENTER to process data set action.   │
   │  F1=Help        F2=Split       F3=Exit        F4=Expand       │
   │  F7=Backward    F8=Forward     F9=Swap       F10=Actions      │
   ⋘───────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 151. z/OS UNIX Directory List Actions pop-up window
AA—auditor auditing
The AA (auditor auditing) line command can be entered against any directory entry. This line command
causes the Modify z/OS UNIX File Auditor Audit Options panel to be displayed.
   Menu  Utilities  View  Options  Help
 ┌─────────────────────────────────────────────────────────────────────────────┐
 ─                Modify z/OS UNIX File Auditor Audit Options                  ─
 ─ Command ===>                                                                ─
 ─                                                                             ─
 ─ Pathname . : /u/myhome/prog1                                                ─
 ─ Type . . . : File                                                           ─
 ─                                                                             ─
 ─ Read 1  1. None     Write 1  1. None     Execute 1  1. None                 ─
 ─         2. Failure           2. Failure             2. Failure              ─
 ─         3. Success           3. Success             3. Success              ─
 ─         4. Both              4. Both                4. Both                 ─
 ─                                                                             ─
 ─                                                                             ─
 ─                                                                             ─
 ─                                                                             ─
 ─                                                                             ─
 ─  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward    ─
 ─  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                     ─
 ⋘─────────────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 152. Modify z/OS UNIX File Auditor Audit Options panel (ISRUULAA)
The Pathname field displays the path name of the selected file. The Type field display the file type for the
selected file.
The auditor auditing options for the file can be changed by a user defined with AUDITOR authority in
the security system. These options allow you to define the access attempts that will be audited by the
z/OS UNIX directory list utility (option 3.17)
252  z/OS: z/OS ISPF User's Guide Vol II

## Page 291

security system. You can specify auditing to occur for read, write, and search or execute attempts on the
file or directory.
The panel displays fields for specifying the Read, Write and Execute (or search) audit settings. For each
of these fields you enter one of the listed numbers corresponding to one of these results for the access
attempt:
None
No audit record is to be written for this type of access.
Failure
Write an audit record if this type of access fails.
Success
Write an audit record if this type of access is successful.
Both
Write an audit record for both failed and successful access attempts.
B—browse regular file
The B (browse) line command can be entered against a regular file or directory. The ISPF browse function
is invoked, allowing you to view the data in the file.
If you enter the B line command beside a directory a directory list is displayed allowing you to select a
regular file to browse.
A numeric record length can also be specified as an option with the B line command for a regular file.
This option allows you to browse fixed-length record files containing text or binary data without new line
delimiters.
C or CO—copy data out
The C or CO (copy out) line command can be entered against a regular file or directory.
Note: In the panel displayed by the CO line command, you can specify a "+" (plus) character as the first
character of a path name to represent the path name of the directory currently listed.
Copying from a regular file 
When the C or CO line command is entered against a regular file, the Copy From z/OS UNIX File panel is
displayed.
                          Copy From z/OS UNIX File
 Command ===>                                                             
 From z/OS UNIX file:
   Name . . . : /u/mburns/cargs1.c
 To z/OS UNIX file, data set, or member:
   Name . . . .                                                      +
   Permissions  700  (Octal)
 Options
   /  Confirm copy to existing target
      Update permissions for existing target file
      Binary copy
      Convert
 Conversion Table                                                          
  F1=Help     F2=Split    F3=Exit     F4=Expand   F5=Rfind    F7=Up
  F8=Down     F9=Swap    F10=Left    F11=Right   F12=Cancel
Figure 153. Copy From z/OS UNIX File panel (ISRUULCF)
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  253

## Page 292

This panel allows you to copy the data in a regular file to another z/OS UNIX file, a sequential data set, or
a member of a partitioned data set.
Note: When copying to a sequential data set or member of a partitioned data set, ISPF invokes the z/OS
UNIX OGET command to perform the copy operation.
The panel displays the path name of the file being copied.
These mandatory input fields are displayed on this panel:
Name
The destination where the data from the file will be copied. Any of these can be specified:
• The path name of a z/OS UNIX file.
• The name of a sequential data set.
• The names of an existing partitioned data set and member.
Permissions
When copying to a z/OS UNIX file, defines the permissions for that file. Enter as three octal (range
0-7) digits. The first digit defines the access permission for the file owner. The second digit defines the
access permission for any member of the file's group. The third digit defines the access permission for
anyone else. See Table 17 on page 247.
These optional input fields are available on this panel:
Confirm copy to existing target
When this option is selected and the target z/OS UNIX file, data set, or member exists, the Confirm
Copy panel displays a warning that the data in the target will be overwritten if the copy proceeds.
In this situation, proceeding with the copy will cause the data in the target to be overwritten. Since
this is an irrevocable process which may cause loss of valuable data, ISPF requires you to confirm
you really want the copy to proceed. If you have made a mistake, the copy operation can be canceled
using the CANCEL or EXIT commands.
Update permissions for existing target file
If this option is selected and the target of the copy is an existing z/OS UNIX file, the value specified in
the Permissions field will be used to update the permissions for this file.
Binary copy
When this option is selected it indicates the file being copied contains binary data. This causes the
copy to take place without any consideration for newline characters or the special characteristics of
DBCS data. If this option is not selected the file is assumed to contain TEXT data.
Note: This option is ignored when copying to another z/OS UNIX file.
Convert
This option specifies whether data conversion is required during the copy operation. Typically,
conversion is only required when the data contains square brackets. If no value is entered in
the Conversion Table field, the data being copied is converted using the default conversion table
(BPXFX000) in the standard library concatenation. By default, this would cause a conversion between
code pages IBM-037 and IBM-1047. Otherwise the value in the Conversion Table field identifies a
conversion table to be used for the copy operation.
Note: This option is ignored when copying to another z/OS UNIX file.
Conversion Table
These types of values can be specified in this field:
• data_set_name(member_name)
The partitioned data set and member containing the character conversion table.
• data_set_name
The partitioned data set that has the member BPXFX000 containing the character conversion table.
• (member_name)
z/OS UNIX directory list utility (option 3.17)
254  z/OS: z/OS ISPF User's Guide Vol II

## Page 293

The member containing the character conversion table. It is assumed to be in a data set in the
standard library concatenation. (The default data set is SYS1.LINKLIB.)
Note: This field is ignored if the Convert option is not selected or if copying to another z/OS UNIX
file.
For further information on the character conversion table refer to the description of the OGET
command in the z/OS UNIX System Services Command Reference.
Copying from a directory
When the C or CO line command is entered against a directory, the Copy From z/OS UNIX Directory panel
is displayed.
                       Copy From z/OS UNIX Directory
 Command ===>                                                             
 From z/OS UNIX directory:
   Name . . . : /u/mburns/jcldir
 To partitioned data set:
   Name . . . .                                               
 Options
      Replace like-named members
      Selection list...
   /  Include lowercase names
   /  Strip suffix                       (Suffix to strip)
      Binary copy
      Convert
 Conversion Table                                                          
  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward
  F8=Forward     F9=Swap       F10=Actions    F12=Cancel
Figure 154. Copy From z/OS UNIX Directory panel (ISRUULCD)
This panel allows you to copy the data from regular files in a directory to members in a partitioned data
set.
Note: When copying to a member of a partitioned data set, ISPF invokes the z/OS UNIX OGET command
to perform the copy operation.
For a file to be selected for copying, it must have a file name that conforms to the naming conventions for
a partitioned data set member. This panel also provides options that allow you to further control the files
selected for copying.
The panel displays the path name of the directory being copied. These mandatory input fields are
displayed on this panel:
Name
The name of an existing partitioned data set where the regular files in the directory will be copied. The
files are copied into members in the partitioned data set.
These optional input fields are available on this panel:
Replace like-named members
When this option is selected, if the file into which the data from a selected member is to be copied
already exists in the directory, the existing file will be overwritten with the data from the selected
member. If this option is not selected, the member will not be copied.
Selection list
If this option is selected, the z/OS UNIX Directory Copy Selection List panel is displayed. This panel
displays a list of the regular files that are eligible to be copied to the partitioned data set. The list
contains these fields:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  255

## Page 294

S
An input field where you can enter S to indicate the associated regular file is to be copied to the
partitioned data set.
Filename
The name of a regular file that can be copied to the partitioned data set. The name conforms to
the rules for a member name and fits the selection criteria specified on the Copy From z/OS UNIX
Directory panel.
Member
The name to be used for the member into which the data from the associated regular file will
be copied. Each member name is generated from the name of the source file. You can change
a generated member name to something other than the name assigned by ISPF. For example, if
ISPF generates the same member name for two files, you can change one of the member names
to make them both unique.
Message
This field displays a message indicating the result of copying the regular file to the member. The
possible values displayed are:
*COPIED
The data from the regular file was successfully copied to a new member in the partitioned data
set.
*REPL
The data from the regular file was copied to an existing member in the partitioned data set.
The data in the member was overwritten. This can only occur when the Replace like-named
members option is selected on the Copy From z/OS UNIX Directory panel.
*NO-REPL
The data from the regular file was not copied to the partitioned data set member because the
member already existed and the Replace like-named members option was not selected on the
Copy From z/OS UNIX Directory panel.
*FAILED RC=xx
The OGET command invoked to copy the data from the file to the member failed with return
code xx. The data was not copied.
When you press Enter on this panel, the selected files will be copied to the partitioned data set.
The Message field indicates the result of the copy operation for each file.
Include lowercase names
When this option is selected the file names for the regular files will be converted to uppercase before
being checked for a valid member name. If this option is not selected, regular files whose file name
contains lowercase characters will not be considered for copying to the partitioned data set.
Strip suffix
When this option is selected suffixes will be stripped from the file name at the first period (.) before
being checked for a valid member name. The accompanying input field allows you to specify a
particular suffix to be stripped (regular files with other suffixes will not be considered for copying). If
this option is not selected, any regular files whose file name includes suffixes will not be copied to the
partitioned data set.
Selecting this option can result in ISPF attempting to copy different files into the same member. For
example, if the Strip suffix option is selected and the directory being copied contains these files:
• pgm1.exe
• pgm1.o
• pgm1.C
the data for each of these files is written to member PGM1. If the Replace like-named members
option is also selected, member PGM1 will contain the data from file pgm1.C. If the Replace like-
named members option is not selected, member PGM1 will contain the data from file pgm1.exe.
z/OS UNIX directory list utility (option 3.17)
256  z/OS: z/OS ISPF User's Guide Vol II

## Page 295

Binary copy
When this option is selected it indicates the file being copied contains binary data. This causes the
copy to take place without any consideration for newline characters or the special characteristics of
DBCS data. If this option is not selected the file is assumed to contain TEXT data.
Convert
This option specifies whether data conversion is required during the copy operation. Typically,
conversion is only required when the data contains square brackets. If no value is entered in
the Conversion Table field, the data being copied is converted using the default conversion table
(BPXFX000) in the standard library concatenation. By default, this would cause a conversion between
code pages IBM-037 and IBM-1047. Otherwise the value in the Conversion Table field identifies a
conversion table to be used for the copy operation.
Conversion Table
These types of values can be specified in this field:
• data_set_name(member_name)
The partitioned data set and member containing the character conversion table.
• data_set_name
The partitioned data set that has the member BPXFX000 containing the character conversion table.
• (member_name)
The member containing the character conversion table. It is assumed to be in a data set in the
standard library concatenation. (The default data set is SYS1.LINKLIB.)
Note: This field is ignored if the Convert option is not selected or if copying to another z/OS UNIX
file.
For further information on the character conversion table refer to the description of the OGET
command in the z/OS UNIX System Services Command Reference.
CI—copy data in
The CI (copy in) line command can be entered against a regular file or directory.
Note: In the panel displayed by the CI line command, you can specify a "+" (plus) character as the first
character of a path name to represent the path name of the directory currently listed.
Copying into a regular file 
When the CI line command is entered against a regular file, the Replace z/OS UNIX File panel is displayed.
                           Replace z/OS UNIX File
 Command ===>                                                             
 Into z/OS UNIX file:
   Name . . . : /u/mburns/abcde
 From z/OS UNIX file, data set, or member:
   Name . . . .                                                      +
 Options
      Binary copy
      Convert
 Conversion Table                                                          
  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward
  F8=Forward     F9=Swap       F10=Actions    F12=Cancel
Figure 155. Replace z/OS UNIX File panel (ISRUULRF)
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  257

## Page 296

This panel allows you to copy into a regular file data from another z/OS UNIX file, a sequential data set, or
a member of a partitioned data set.
Note:
1. When copying from a sequential data set or member of a partitioned data set, ISPF invokes the z/OS
UNIX OPUT command to perform the copy operation.
2. this operation will cause existing data in the regular file to be overwritten.
The panel displays the path name of the file into which the data will be copied. These mandatory input
fields are displayed on this panel:
Name
The source of the data to be copied into the file. Any of these can be specified:
• The path name of a z/OS UNIX file
• The name of a sequential data set
• The names of an existing partitioned data set and member
These optional input fields are available on this panel:
Binary copy
When this option is selected it indicates the data set/member being copied contains binary data.
This causes the copy to take place without any consideration for newline characters or the special
characteristics of DBCS data. If this option is not selected the data set/member is assumed to contain
TEXT data.
Note: This option is ignored when copying to another z/OS UNIX file.
Convert
This option specifies whether data conversion is required during the copy operation. Typically,
conversion is only required when the data contains square brackets. If no value is entered in
the Conversion Table field, the data being copied is converted using the default conversion table
(BPXFX000) in the standard library concatenation. By default, this would cause a conversion between
code pages IBM-037 and IBM-1047. Otherwise the value in the Conversion Table field identifies a
conversion table to be used for the copy operation.
Note: This option is ignored when copying to another z/OS UNIX file.
Conversion Table
These types of values can be specified in this field:
• data_set_name(member_name)
The partitioned data set and member containing the character conversion table.
• data_set_name
The partitioned data set that has the member BPXFX000 containing the character conversion table.
• (member_name)
The member containing the character conversion table. It is assumed to be in a data set in the
standard library concatenation. (The default data set is SYS1.LINKLIB.)
Note: This field is ignored if the Convert option is not selected or if copying from another z/OS UNIX
file.
For further information on the character conversion table refer to the description of the OPUT
command in the z/OS UNIX System Services Command Reference.
Copying into a directory
When the CI line command is entered against a directory, the Copy Into z/OS UNIX Directory panel is
displayed.
z/OS UNIX directory list utility (option 3.17)
258  z/OS: z/OS ISPF User's Guide Vol II

## Page 297

Copy Into z/OS UNIX Directory
 Command ===>                                                             
                                                                More:     +
 Into z/OS UNIX directory:
   Name . . . : /u/mburns/abcdir1
 From partitioned data set:
   Name . . . . EXEC                                          
 Permissions    777
 Suffix . . . .                     
 Options
      Replace like-named files
      Update permissions for replaced files
   /  Selection list...
   /  Convert to lowercase
      Binary copy
      Convert
 Conversion Table                                                          
  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward
  F8=Forward     F9=Swap       F10=Actions    F12=Cancel
Figure 156. Copy Into z/OS UNIX Directory panel (ISRUULRD)
This panel allows you to copy the data from members of a partitioned data set into regular files in a
directory.
Note: ISPF invokes the z/OS UNIX OPUT command to perform the operation of copying data from a
member of a partitioned data set into a regular file.
The panel displays the path name of the directory into which the members of the partitioned data set will
be copied. These mandatory input fields are displayed on this panel:
Name
The name of an existing partitioned data set containing the members that will be copied as regular
files into the selected directory.
Permissions
Defines the permissions for new regular files created when copying a partitioned data set member
in the directory. When the option "Update permissions for replaced files" is selected, it also defines
new permissions applied to a file replaced during the copy operation. Enter as three octal (range 0-7)
digits. The first digit defines the access permission for the file owner. The second digit defines the
access permission for any member of the file's group. The third digit defines the access permission for
anyone else. See Table 17 on page 247.
These optional input fields are available on this panel:
Suffix
This field allows you to specify a value that will be added to the end of the member name to form the
file name of the regular file that is created or updated during the copy operation. The member name
and suffix are separated by a period (.). Any leading periods specified in the suffix are ignored.
Replace like-named files
When this option is selected, if the file into which the data from a selected member is to be copied
already exists in the directory, the contents of the existing file will be overwritten with the data from
the selected member. If this option is not selected, the copy of that member will not be performed.
Update permissions for replaced files
When this option is selected it causes existing files that are replaced by the copy operation to also
have their permissions changed to the value specified in the Permissions field.
Selection List
If this option is selected, the Copy Into z/OS UNIX Directory - Selection List panel is displayed. This
panel displays a list of the members in the partitioned data set that can be selected for copying into
the directory. The list contains these fields:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  259

## Page 298

S
An input field where you can enter S to indicate the associated member is to be copied into the
directory.
Member
The name of the partitioned data set member that can be copied into the directory.
Filename
The name to be used for the regular file into which the data from the associated member will be
copied. This is an input field, allowing you to change the file name to something other than the
name assigned by ISPF. The field is scrollable and is 1023 bytes long. Use the EXPAND function
key (F4) to display the entire field in a pop-up window.
Message
This field displays a message indicating the result of copying the member to the regular file. The
possible values displayed are:
*COPIED
The data from the member was successfully copied to a new regular file in the directory.
*REPL
The data from the member was copied to an existing regular file in the directory. The data
in the file was overwritten. This can only occur when the Replace like-named files option is
selected on the Copy Into z/OS UNIX Directory panel.
*NO-REPL
The data from the member was not copied to the file in the directory because the file already
existed and the Replace like-named files option was not selected on the Copy Into z/OS UNIX
Directory panel.
*FAILED RC=xx
The OPUT command invoked to copy the data from the member to the regular file failed with
return code xx. The data was not copied.
When you press Enter on this panel, the selected members will be copied to the directory. The
Message field indicates the result of the copy operation for each member.
Convert to lowercase
When this option is selected it causes the member name to be converted to lowercase before it is
used to generated the file name for the target regular file.
Binary copy
When this option is selected it indicates the members being copied contains binary data. This causes
the copy to take place without any consideration for newline characters or the special characteristics
of DBCS data. If this option is not selected the members are assumed to contain TEXT data.
Convert
This option specifies whether data conversion is required during the copy operation. Typically,
conversion is only required when the data contains square brackets. If no value is entered in
the Conversion Table field, the data being copied is converted using the default conversion table
(BPXFX000) in the standard library concatenation. By default, this would cause a conversion between
code pages IBM-037 and IBM-1047. Otherwise the value in the Conversion Table field identifies a
conversion table to be used for the copy operation.
Conversion Table
These types of values can be specified in this field:
• data_set_name(member_name)
The partitioned data set and member containing the character conversion table.
• data_set_name
The partitioned data set that has the member BPXFX000 containing the character conversion table.
• (member_name)
z/OS UNIX directory list utility (option 3.17)
260  z/OS: z/OS ISPF User's Guide Vol II

## Page 299

The member containing the character conversion table. It is assumed to be in a data set in the
standard library concatenation. (The default data set is SYS1.LINKLIB.)
Note: This field is ignored if the Convert option is not selected.
For further information on the character conversion table refer to the description of the OPUT
command in the z/OS UNIX System Services Command Reference.
D—delete a file
The D (delete file) line command can be entered against any directory entry. If entered against a file or an
empty directory and the Confirm File Delete option is selected, the Confirm Delete panel is displayed. This
panel allows the delete operation to be canceled if necessary using the CANCEL or EXIT commands. You
can prevent this panel being displayed for subsequent delete operations by selecting the "Set file delete
confirmation off" option.
If the deletion proceeds successfully, the file or directory is removed from the file system.
If the D line command is entered against a directory containing files and subdirectories and the
Confirm Non-empty Directory Delete option is selected, the Confirm Non-empty Directory Delete panel is
displayed. This panel allows the delete operation to be canceled if necessary using the CANCEL or EXIT
commands. You can prevent this panel being displayed for subsequent delete operations by selecting the
"Set non-empty directory delete confirmation off" option.
If the deletion proceeds successfully the directory, including all contained files and subdirectories, is
removed from the file system.
E—edit regular file
The E (edit) line command can be entered against a regular file or directory. The ISPF editor is invoked,
allowing you to change the data in the file.
If you enter the E line command beside a directory, a directory list is displayed allowing you to select a
regular file to edit.
A numeric record length can also be specified as an option with the E line command for a regular file. This
option allows you to set the record length when editing fixed-length text files. When specified, the file is
processed as variable length but loaded into the editor as fixed-length records and saved as fixed-length
records. This lets you convert a variable-length file to fixed length.
The Edit Entry panel can be displayed when the E line command is entered. This panel allows you specify
items including the initial macro, profile name, panel name, format, and mixed mode editing. These values
are stored in the profile and are used on subsequent edits. The Bypass z/OS UNIX File Edit Options
panel option on the z/OS UNIX Directory List Options panel can be selected to stop this panel being
displayed for subsequent file edit sessions.
EA—edit ASCII file
The EA (Edit - ASCII) line command can be entered against a regular file that contains data encoded in
ASCII and the file is not tagged with a CCSID of 819. The ISPF editor is invoked with the ASCII edit facility
which converts the ASCII data to the CCSID of the terminal, allowing you to read and change the ASCII
data in file. If the E line command is used and the file is tagged with a CCSID of 819, ISPF invokes the
ASCII edit facility.
EU—edit UTF-8 file
The EU (Edit - UTF-8) line command can be entered against a regular file that contains data encoded in
UTF-8 and the file is not tagged with a CCSID of 1208. The ISPF editor is invoked with the UTF-8 edit
facility which converts the UTF-8 data to the CCSID of the terminal, allowing you to read and change the
UTF-8 data in file. If the E line command is used and the file is tagged with a CCSID of 1208, ISPF invokes
the UTF-8 edit facility.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  261

## Page 300

FS—file system
The FS (file system) line command can be entered against any directory entry except a FIFO or character
special file. This line command causes the z/OS UNIX File System Attributes panel to be displayed.
                        z/OS UNIX File System Attributes
 Command ===>                                                                 
 Pathname : /u/myhome/prog1
 File system name . : OMVS.USERS.ISD1
 Mount point  . . . : /u
 Status . . . . . . : Available
 File system type . : ZFS
 Mount mode . . . . : R/W
 Device number  . . : 7
 Type number  . . . : 1
 DD name  . . . . . : SYS00012
 Ignore SETUID  . . : NO
 Bypass Security  . : NO
 Automove . . . . . : YES
 Owning system  . . : ISD1
 CCSID  . . . . . . :
 Text Convert . . . : NO
 Seclabel . . . . . :
 Block size . . . . : 4096
 Total blocks . . . : 2880000
 Available blocks . : 2178092
 Blocks in use  . . : 701411
 Data blocks read  . . . : 0
 Data blocks written . . : 0
 Directory blocks r/w  . : 0
 Mount parameters
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 157. z/OS UNIX File System Attributes panel (ISRUULFS)
This panel displays the attributes of the file system for the file in these fields:
File system name
The name of the data set for the file system.
Mount point
The name of the directory that is the mount point for this file system.
Status
One of these values describing the current state of the file system:
Available
The file system is mounted and available for use.
Not Active
The file system is not available for use.
Reset in progress
A reset unmount request is currently being processed.
Unmount drain in progress
The file system will be unmounted when it is no longer in use.
Unmount force in progress
The file system is being unconditionally unmounted.
Unmount immediate in progress
The file system is being unmounted, even though it may be in use.
z/OS UNIX directory list utility (option 3.17)
262  z/OS: z/OS ISPF User's Guide Vol II

## Page 301

Unmount in progress
The file system is being unmounted if it is not currently in use.
Pending unmount reset or force
An immediate unmount request failed.
Quiesced by (process ID)
The file system is quiesced, usually for backup.
Mount in progress
The file system is being mounted.
Recycling
The physical file system is in a recycle but a mount has not yet been done for this file system.
Recycling, Asynch Mounting
The physical file system is in a recycle and the file system is in an asynchronous mount.
Recycling, Not Active
The physical file system is in a recycle and the file system failed to successfully mount.
Unowned
The file system has become unowned.
In Recovery
Recovery processing is in progress.
Super Quiesced
The file system is in a super quiesced state.
File system type
The type of physical file system that manages this mounted file system.
Mount mode
Shows whether the file system is mounted read/write (R/W) or read-only(R/O).
Device number
The device number that uniquely identifies the mounted file system. This is a hexadecimal value.
Type number
A number set by the physical file system to indicate the type of this file system. The ZFS file system
sets this value to 1.
DD name
The MVS data definition name used by the physical file system to access the mounted file system.
Ignore SETUID
This value indicates whether the SETUID and SETGID mode bits on any executable in this file system
be ignored when the program is run.
Bypass Security
This value indicates whether security checks are not enforced for files in this file system.
Automove
This value indicates whether the system can automatically move the file system to another system
and remain local and unowned, or be unmounted.
Owning system
The name of the system that owns this file system.
CCSID
The coded character set identifier to be implicitly set for untagged files in the file system.
Text Convert
This value indicates whether untagged files are implicitly marked as containing pure text data that can
be converted.
Seclabel
This security label assigned to a file system that is mounted read-only. This security label applies to all
objects within the file system that do not have security labels assigned.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  263

## Page 302

Block size
The length, in bytes, of a data block for the physical file system.
Total blocks
The total number of 4096-byte blocks in this file system.
Available blocks
The number of 4096-byte blocks in this file system that are available for use.
Blocks in use
The number of 4096-byte blocks in this file system that are currently in use.
Data blocks read
The block I/O count for user data reads. This value is only available if SMF type 92 records are active.
Data blocks written
The block I/O count for user data writes. This value is only available if SMF type 92 records are active.
Directory blocks r/w
The block I/O count for directory I/Os. This value is only available if SMF type 92 records are active.
Mount parameters
The parameters specified with the mount command for this file system.
I—information
The I (information) line command can be entered against any directory entry.
Information display for non-link files 
When entered against any directory entry type apart from symbolic and external links, the z/OS UNIX File
Information panel is displayed. Figure 158 on page 264 shows an example.
   Modify
 ───────────────────────────────────────────────────────────────────────────────
                           z/OS UNIX File Information
 Command ===>                                                                 
                                                                    More:     +
 Pathname  . . : /SYSTEM/etc/profile
 General Data                                Mode Fields
  File Type . . : File                        Permissions . : 755
  File Size . . : 10665                       Set User ID . : NO
  Links . . . . : 1                           Set Group ID  : NO
  Inode . . . . : E0                          Sticky Bit  . : NO
  File Format . : ----
  Last Modified : 2003/07/10 03:21:51        Extended Attributes
  Last Changed  : 2003/07/10 03:22:40         Shared AS . . : YES
  Last Accessed : 2006/04/05 01:20:34         APF Auth  . . : NO
  Created . . . : 2003/07/10 03:18:18         Pgm Control . : NO
  CCSID . . . . :                             Shared Lib  . : NO
  Text Convert  : NO
                                             Audit
 Owner                                        Auditor . . . : ---
  File  . . . . : IBMUSER(0)                  User  . . . . : fff
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 158. z/OS UNIX File Information panel (ISRUULIN)
This panel displays information describing the attributes of a z/OS UNIX file. The Pathname field displays
the path name of the selected z/OS UNIX file.
The General Information section of the panel displays these fields:
File Type
The type of z/OS UNIX file. The possible values are:
Dir
Directory
z/OS UNIX directory list utility (option 3.17)
264  z/OS: z/OS ISPF User's Guide Vol II

## Page 303

File
Regular file
Char
Character special file
FIFO
FIFO (first-in-first-out) special file
Size
The file size, in bytes.
Links
For a file, the number of hard links to the file. For a directory, the number of subdirectories.
Inode
File identification number, unique within the file system.
File Format
File format for regular files. The possible values are:
----
Not specified
bin
Binary data
nl
New line
cr
Carriage return
lf
Line feed
crlf
Carriage return followed by line feed
lfcr
Line feed followed by carriage return
crnl
Carriage return followed by new line
Last Modified
The date and time the file was last changed.
Last Changed
The date and time the status of the file was last changed.
Last Accessed
The date and time the data in the file was last accessed.
Created
The date and time the file was created.
CCSID
The coded character set identifier assigned to the file for Enhanced ASCII support.
Text Convert
A value of YES indicates the file is enabled for Enhanced ASCII automatic conversion. NO indicates
the file is not enabled for automatic conversion.
The Owner section of the panel displays these fields:
File
The user ID and UID number of the owner of the file or directory.
Group
The group name and GID number of the owner of the file or directory.
The Mode Fields section of the panel displays these fields:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  265

## Page 304

Permissions
The file or directory permissions, in octal format. If there are extended access control list (ACL)
entries defined for the file or directory, + is displayed after the octal value.
Set User ID
A value of ON indicates the SETUID bit is on causing the effective user ID of the user process
executing a program to be set to that of the file owner when this file is run.
Set Group ID
A value of ON indicates the SETGID bit is on causing the effective group ID of the user process
executing a program to be set to that of the file owner when this file is run.
Sticky Bit
A value of ON indicates the sticky bit for the file or directory is set on. For files that are programs
this causes z/OS UNIX to search for the program in the user's STEPLIB, the link pack area, or the
link list concatenation. For a directory it means a user can only remove or rename a file or remove a
subdirectory if one of these conditions is true:
• The user owns the file or subdirectory
• The user owns the directory
• The user has superuser authority
The Extended Attributes section of the panel displays these fields:
Shared AS
A value of YES indicates that the program can run in a shared address space.
APF Auth
A value of YES indicates that the program can run APF authorized if it has been linked with AC=1.
Pgm Control
A value of YES indicates that the program can run as if it were from a program controlled library.
Shared Lib
A value of YES indicates that the program is loaded as a system shared library program.
The Audit section of the panel displays these fields:
Auditor
Shows the audit criteria for this file as defined by a user with auditor authority. The value shows three
characters describing the audit bit settings for read, write, and execute (search) access. The possible
values for each character are:
s
Audit successful access attempts
f
Audit failed access attempts
a
Audit all access attempts
-
No audit
User
Shows the audit criteria for this file as defined by the file owner or a superuser. See the field Auditor
for the possible values displayed.
The Device Data section of the panel displays these fields:
Device Number
A hexadecimal number that uniquely identifies the mounted file system for this file.
Major Device
For a character special file, this is a number that identifies the device type. The possible values are:
1
Primary pseudo-TTY device, which is tied to a secondary device by the minor number
z/OS UNIX directory list utility (option 3.17)
266  z/OS: z/OS ISPF User's Guide Vol II

## Page 305

2
Secondary pseudo-TTY device, which is tied to a primary device by the minor number
3
Controlling terminal TTY
4
Null file
5
File descriptor file, which is tied to a file descriptor by the minor number
6
UNIX domain socket name special file
9
System console file
Minor Device
A number that identifies a specific device of a given device type.
The Modify action bar choice provides these options:
Mode Fields
Displays the Modify z/OS UNIX File Mode Fields panel where you can update the mode fields for the
currently displayed file.
Extended Attributes
Displays the Modify z/OS UNIX File Extended Attributes panel where you can update the extended
attributes for the currently displayed file.
Information display for link files 
When the I line command is entered against a symbolic or external link file, the z/OS UNIX Symbolic Link
Information panel is displayed. Figure 159 on page 267 shows an example.
                      z/OS UNIX Symbolic Link Information
 Command ===>                                                                 
 Pathname  . . . : /SYSTEM/etc/ioepdcf
 General Data
  External Link : NO
  File Size . . : 22
  Links . . . . : 1
  Inode . . . . : 7
  Last Modified : 2002/11/20 19:30:53
  Last Changed  : 2002/11/20 19:30:53
  Last Accessed : 2002/11/20 19:30:53
  Created . . . : 2002/11/20 19:30:53
 Owner
  File  . . . . : IBMUSER(0)
  Group . . . . : OMVSGRP(1)
 Symbolic Link -
  ../etc/dfs/etc/ioepdcf
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 159. z/OS UNIX Symbolic Link Information panel (ISRUULIS)
This panel displays information describing the attributes of a z/OS UNIX symbolic or external file. The
Pathname field displays the path name of the selected symbolic or external link file.
The General Information section of the panel displays these fields:
External Link
A value of YES indicates the file is an external link to an object outside of the file system. A value of NO
indicates the file is a link to another file or a directory.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  267

## Page 306

Size
The file size, in bytes.
Links
The number of hard links to the file.
Inode
File identification number, unique within the file system.
Last Modified
The date and time the file was last changed.
Last Changed
The date and time the status of the file was last changed.
Last Accessed
The date and time the data in the file was last accessed.
Created
The date and time the file was created.
The Owner section of the panel displays these fields:
File
The user ID and UID number of the owner of the file or directory.
Group
The group name and GID number of the owner of the file or directory.
The Symbolic Link field is a scrollable field that displays the path name or external name to which this
symbolic link file refers.
L—directory list
The L (list directory) line command can be entered against a directory. This line command causes a new
z/OS UNIX Directory List panel to be displayed, showing the entries for the selected directory. This new
directory list display is nested so entering the END or EXIT command on this panel will return you to the
previous directory list. Entering the CANCEL command on a nested directory list display will return you to
the directory list utility entry panel.
MA—modify ACL
The MA (modify ACL) line command can be entered against any directory or file entry. This line command
causes the z/OS UNIX ACL list panel to be displayed.
z/OS UNIX directory list utility (option 3.17)
268  z/OS: z/OS ISPF User's Guide Vol II

## Page 307

z/OS UNIX ACL List                   Row 1 from 75
 Command ===>                                             Scroll ===> PAGE
 S    ID       Read  Write  eXecute  Name      Type
     108        R                    BILLSWA   USER
     607        R                    MBOTES    USER
     204        R                    SCLMU     GROUP
     991        R                    TGROUP1   GROUP
     992        R      W       X     TGROUP2   GROUP
     993        R      W             TGROUP3   GROUP
 ******************************* Bottom of data ********************************
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 160. z/OS UNIX ACL list panel (ISRUULMA)
Panel ISRUULMA may display with no entries indicating that no ACL entries have been created.
The list is sorted in Name order.
You can enter these commands on the command line:
A
Add a new ACL.
SA
Sort the ACL list alphabetically by Name.
SN
Sort the ACL list numerically on ID.
ST
Sort the ACL list alphabetically by Type.
If there are ACL entries displayed, these fields are shown:
S
Select field. These commands are valid:
A
To add further entries.
D
To delete the entry.
X
To list members of an OMVS group.
ID
UNIX ID (UID or GID) value.
Read, Write, eXecute
Read, write, and execute permissions for this ACL.
Name
The user or group name associated with the ID value.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  269

## Page 308

Type
Describes whether the entry is for a USER or GROUP.
MF—modify format
The MF (modify format) line command can be entered against any directory entry except a symbolic link
file. This line command causes the Modify z/OS UNIX File Format panel to be displayed.
   Menu  Utilities  View  Options  Help
 ┌─────────────────────────────────────────────────────────────────────────────┐
 e                        Modify z/OS UNIX File Format                         │
 │ Command ===>                                                                │
 │                                                                             │
 │ Pathname . : /u/myhome/prog1                                                │
 │ Type . . . : File                                                           │
 │                                                                             │
 │ Format . . . 1  1. NA       3. NL     5. LF     7. LFCR                     │
 │                 2. Binary   4. CR     6. CRLF   8. CRNL                     │
 │                                                                             │
 │ CCSID  . . .                                                                │
 │                                                                             │
 │ Enter "/" to select option                                                  │
 │    Automatic Conversion                                                     │
 │                                                                             │
 │                                                                             │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward    │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 161. Modify z/OS UNIX File Format panel (ISRUULMF)
The Pathname field displays the path name of the selected file. The Type field display the file type for the
selected file.
The format and tag information for the file can be changed by updating these input fields on the panel:
Format
Enter one of the listed numbers corresponding to one of these formats required for the file:
NA
No format specified.
Binary
Binary data.
NL
Text file; lines delimited by the newline character.
CR
Text file; lines delimited by the carriage-return character.
LF
Text file; lines delimited by the line-feed character.
CRLF
Text file; lines delimited by carriage-return and line-feed characters.
LFCR
Text file; lines delimited by line-feed and carriage-return characters.
CRNL
Text file; lines delimited by carriage-return and newline characters.
CCSID
Enter the numeric coded character set identifier (CCSID) associated with the file. The numeric value
must be between 0 and 65535. You can set this field to blanks or enter a value of 0 to indicate there is
no CCSID associated with the file.
z/OS UNIX directory list utility (option 3.17)
270  z/OS: z/OS ISPF User's Guide Vol II

## Page 309

Automatic Conversion
Select this option to identify the file as a candidate for automatic conversion provided by z/OS UNIX
Enhanced ASCII support.
Note:
1. A superuser or the owner can change the file format of a file.
2. A superuser, the owner, or a user with write permission can change the tag information (CCSID and
automatic conversion setting) for a file.
3. File tag information cannot be set for a z/OS UNIX directory. Therefore when processing a directory the
CCSID and Automatic Conversion fields are protected.
MG—modify group
The MG (modify group) line command can be entered against any directory entry except a symbolic link
file. This line command causes the Modify z/OS UNIX File Owning Group panel to be displayed.
   Menu  Utilities  View  Options  Help
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                     Modify z/OS UNIX File Owning Group                      │
 │ Command ===>                                                                │
 │                                                                             │
 │ Pathname . : /u/myhome/prog1                                                │
 │ Type . . . : File                                                           │
 │                                                                             │
 │ GID Number   108                                                            │
 │ Group ID . . SYSADMIN                                                       │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward    │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                           
Figure 162. Modify z/OS UNIX File Owning Group panel (ISRUULMG)
The Pathname field displays the path name of the selected file. The Type field display the file type for the
selected file.
The owning group for the file can be changed by a superuser or the owner by updating one of these input
fields on the panel:
GID Number
This field allows you to enter the GID of the new group. This must be a number, in the range 1 to
2147483647, and must be defined as a z/OS UNIX GID in your security data base.
Group ID
This field allows you to enter the group ID of the new group. The group ID must be defined as a z/OS
UNIX group in your security data base.
MM—modify mode fields
The MM (modify mode) line command can be entered against any file type apart from symbolic and
external link files. This line command causes the Modify z/OS UNIX File Mode Fields panel to be
displayed. This panel allows you to modify mode fields for the selected z/OS UNIX file.
The Pathname field displays the path name of the selected file. The Type field display the file type for the
selected file.
These optional input fields allow you to make modifications to the mode of the file:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  271

## Page 310

Permissions
This field allows you to change the permissions defined for the file. The current permissions for
the file are initially displayed. The permissions are displayed and entered as three octal (range 0-7)
digits. The first digit defines the access permission for the file owner. The second digit defines the
access permission for any member of the file's group. The third digit defines the access permission for
anyone else. See Table 17 on page 247.
Set UID bit
When this option is selected, the file mode SETUID bit is set on. When the option is not selected, the
SETUID bit is set off. If the SETUID bit is on, the effective user ID of the user process executing a
program will be set to that of the file owner when this file is run.
Set GID bit
When this option is selected, the file mode SETGID bit is set on. When the option is not selected, the
SETGID bit is set off. If the SETGID bit is on, the effective group ID of the user process executing a
program will be set to that of the file owner when this file is run.
Sticky bit
When this option is selected, the file mode sticky bit is set on. When this option is not selected, the
sticky bit is set off. If the sticky bit is on for a file that is a program, z/OS UNIX will search for the
program in the user's STEPLIB, the link pack area, or the link list concatenation. If the sticky bit is on
for a directory it means a user can only remove or rename a file or remove a subdirectory if one of
these conditions is true:
• The user owns the file or subdirectory
• The user owns the directory
• The user has superuser authority
MO—modify owner
The MO (modify owner) line command can be entered against any directory entry except a symbolic link
file. This line command causes the Modify z/OS UNIX File Owning User panel to be displayed.
   Menu  Utilities  View  Options  Help
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                     Modify z/OS UNIX File Owning User                       │
 │ Command ===>                                                                │
 │                                                                             │
 │ Pathname . : /u/myhome/prog1                                                │
 │ Type . . . : File                                                           │
 │                                                                             │
 │ UID Number   0                                                              │
 │ User ID  . . IBMUSER                                                        │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward    │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 163. Modify z/OS UNIX File Owning User panel (ISRUULMO)
The Pathname field displays the path name of the selected file. The Type field display the file type for the
selected file.
The owner of the file can be changed by a superuser by updating one of these input fields on the panel:
z/OS UNIX directory list utility (option 3.17)
272  z/OS: z/OS ISPF User's Guide Vol II

## Page 311

UID Number
This field allows you to enter the UID of the new owner. This must be a number, in the range 1 to
2147483647, and must be defined as a z/OS UNIX UID in your security data base.
User ID
This field allows you to enter the user ID of the new owner. The user ID must be defined in your
security data base and have the authority to use z/OS UNIX resources.
MX—modify extended attributes
The MX (Modify eXtended) line command can be entered against regular files in the directory list. This line
command causes the Modify z/OS UNIX File Extended Attributes panel to be displayed. This panel allows
you to modify the extended attributes for the selected z/OS UNIX regular file. These attributes only affect
files that are programs.
The Pathname field displays the path name of the selected file. The Type field display the file type for the
selected file.
These optional input fields allow you to modify the extended attributes:
Use Shared Address Space
When this option is selected, ISPF sets the extended attribute that makes the program eligible to run
in a shared address space.
APF Authorized
When this option is selected, ISPF sets the extended attribute that makes the program eligible to run
APF-authorized if it has been linked with AC=1.
Program Controlled
When this option is selected, ISPF sets the extended attribute that makes the program eligible to run
as if it were from a program controlled library.
Shared Library
When this option is selected, ISPF sets the extended attribute that causes the program to be loaded
from the system shared library region.
N—create a new directory entry
The N (new) line command can be entered against any directory entry. The Create New z/OS UNIX File
panel is displayed.
                         Create New z/OS UNIX File
 Command ===>                                                             
 Pathname . . . . /u/sclmtest                                          +
 Permissions  . .      (Octal)
 Link . . . . . .                                                      +
                                           Options
 File Type  . . .    1. Directory             Set sticky bit
                     2. Regular file          Copy...
                     3. FIFO                  Edit...
                     4. Symbolic Link
                     5. External Link
                     6. Hard Link
  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward
  F8=Forward     F9=Swap       F10=Actions    F12=Cancel
Figure 164. Create New z/OS UNIX File panel (ISRUULNW)
These mandatory input fields are displayed on this panel:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  273

## Page 312

Pathname
The path name for the z/OS UNIX file to be created. This field is initialized with the path name of the
file that the N line command was entered against. The field is scrollable with a length of 1023 bytes.
Use the EXPAND function key (F4) to display the entire field in a pop-up window.
Note: In the panel displayed by the N line command, you can specify a "+" (plus) character as the first
character of a path name to represent the path name of the directory currently listed.
Permissions
The permissions defined for the new file. Enter as three octal (range 0-7) digits. The first digit defines
the access permission for the file owner. The second digit defines the access permission for any
member of the file's group. The third digit defines the access permission for anyone else. See Table 17
on page 247.
Link
This field is only mandatory when creating a Symbolic Link, External Link, or Hard Link. When creating
a Symbolic Link or a Hard Link this field is used to define the path name of the existing file the link
refers to. When creating an External Link this field is used to define the external name the link refers
to. The field is scrollable with a length of 1023 bytes. Use the EXPAND function key (F4) to display the
entire field in a pop-up window.
File Type
This field is used to enter one of the listed numbers corresponding to the type of file you want to
create.
1. Directory
2. Regular file
3. FIFO
4. Symbolic Link
5. External Link
6. Hard Link
These optional fields can be selected on this panel:
Set sticky bit
When this option is selected it causes the sticky bit to be set on for the new file or subdirectory. When
the sticky bit is set for a directory, a user cannot remove or rename a file in the directory unless one or
more of these is true:
• The user owns the file
• The user owns the directory
• The user has superuser authority
If the sticky bit is set for a program file, when executing the program z/OS UNIX will search for the
program in the user's STEPLIB, the link pack area, or the link list concatenation.
Copy
When this option is selected and you are creating a new regular file, it causes the Replace z/OS UNIX
File panel to be displayed, allowing you to have the data from a z/OS UNIX file, data set, or partitioned
data set member copied into the new file. When selected and you are creating a new directory, it
causes the Copy Into z/OS UNIX Directory panel to be displayed, allowing you to have the data from
members in a partitioned data set copied into files in the new directory.
Edit
When this option is selected and you are creating a new regular file, it causes the edit function to be
invoked allowing you to create and modify data in the new file.
z/OS UNIX directory list utility (option 3.17)
274  z/OS: z/OS ISPF User's Guide Vol II

## Page 313

R—rename a file
The R (rename file) line command can be entered against any directory entry. This line command causes
the Rename z/OS UNIX File panel to be displayed. This panel displays the Pathname and Type of the file
being renamed. Use the New Pathname field to enter the new name for the file.
Note: In the panel displayed by the R line command, you can specify a "+" (plus) character as the first
character of a path name to represent the path name of the directory currently listed.
When you press Enter, ISPF attempts to rename the file.
Attention: If the New Pathname you specified corresponds to an existing file, the Confirm Rename
panel is displayed. In this situation, proceeding with the rename will cause the existing file with
the same name to be deleted.
RA—Add to Personal Data Set List
The RA (refadd) line command is used to add the pathname of the selected file or directory to a personal
data set list. When the RA line command is entered, the pop-up panel shown here is displayed, allowing
you to enter the name of the personal data set where the entry for the pathname is to be added.
   Menu  Utilities  View  Options  Help
 ─ ┌─────────────────────────────────────┐ ─────────────────────────────────────
   │     Personal Data Set List Add      │ tory List            Row 1 to 6 of 6
 C │                                     │                     Scroll ===> PAGE
   │ Enter a Personal List Name:         │
 P │                                     │
   │ List Name  . . .                    │
 C │                                     │ Type Permission Audit  Ext  Fmat
 - │                                     │ -------------------------------------
   │                                     │ Dir  rwxrwxrwx  fff---
   │                                     │ Dir  rwxrwxrwx  fff---
   │ Press ENTER to add data set.        │ Dir  rwxrwxrwx  fff---
 r │ Press CANCEL to cancel Refadd.      │ File rwxrwxrwx  fff--- --s- ----
   │                                     │ File rwxrwxrwx  fff--- --s- ----
   │  F1=Help           F2=Split         │ File rwxrwxrwx  fff--- --s- ----
 * │  F3=Exit           F7=Backward      │ data ********************************
   ⋘─────────────────────────────────────┘
  F1=Help    F2=Split   F3=Exit    F4=Expand  F5=Rfind   F7=Up      F8=Down
  F9=Swap   F10=Left   F11=Right  F12=Cancel
Figure 165. Personal Data Set List Add pop-up panel
S—invoke default line command
The S (invoke default) line command causes a default line command to be invoked against the entry. If an
S-replacement line command is specified by the environment that built the z/OS UNIX Directory List, that
line command is invoked. Otherwise, the default line command specified for the entry type on the z/OS
UNIX Directory List Default Line Commands panel is invoked against the entry.
UA—user auditing
The UA (user auditing) line command can be entered against any directory entry. This line command
causes the Modify z/OS UNIX File User Audit Options panel to be displayed.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  275

## Page 314

Menu  Utilities  View  Options  Help
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                  Modify z/OS UNIX File User Audit Options                   │
 │ Command ===>                                                                │
 │                                                                             │
 │ Pathname . : /u/myhome/prog1                                                │
 │ Type . . . : File                                                           │
 │                                                                             │
 │ Read 2  1. None     Write 2  1. None     Execute 2  1. None                 │
 │         2. Failure           2. Failure             2. Failure              │
 │         3. Success           3. Success             3. Success              │
 │         4. Both              4. Both                4. Both                 │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward    │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                     │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward
  F9=Swap     F10=Actions  F12=Cancel
Figure 166. Modify z/OS UNIX File User Audit Options panel (ISRUULUA)
The Pathname field displays the path name of the selected file. The Type field display the file type for the
selected file.
The user auditing options for the file can be changed by a superuser or the owner. These options allow you
to define the access attempts that are audited by the security system. You can specify auditing to occur
for read, write, and search or execute attempts on the file or directory.
The panel displays fields for specifying the Read, Write and Execute (or search) audit settings. For each
of these fields, you enter one of the listed numbers corresponding to one of these results for the access
attempt:
None
No audit record is to be written for this type of access.
Failure
Write an audit record if this type of access fails.
Success
Write an audit record if this type of access is successful.
Both
Write an audit record for both failed and successful access attempts.
V—view regular file
The V (view) line command can be entered against a regular file or directory. The ISPF editor is invoked,
allowing you to view the data in the file.
If you enter the V line command beside a directory, a directory list is displayed allowing you to select a
regular file to view.
The View Entry panel can be displayed when the V line command is entered. This panel allows you specify
items including the initial macro, profile name, panel name, format, and mixed mode editing. These values
are stored in the profile and are used on subsequent edits. The Bypass z/OS UNIX File Edit Options
panel option on the z/OS UNIX Directory List Options panel can be selected to stop this panel being
displayed for subsequent file edit sessions.
VA—view ASCII file
The VA (View - ASCII) line command can be entered against a regular file that contains data encoded in
ASCII and the file is not tagged with a CCSID of 819. The ISPF editor is invoked with the ASCII edit facility
z/OS UNIX directory list utility (option 3.17)
276  z/OS: z/OS ISPF User's Guide Vol II

## Page 315

which converts the ASCII data to the CCSID of the terminal, allowing you to view the ASCII data in file. If
the V line command is used and the file is tagged with a CCSID of 819, ISPF invokes the ASCII edit facility.
VU—view UTF8 file
The VU (View - UTF8) line command can be entered against a regular file that contains data encoded in
UTF8 and the file is not tagged with a CCSID of 1208. The ISPF editor is invoked with the UTF8 edit facility
which converts the UTF8 data to the CCSID of the terminal, allowing you to view the UTF8 data in file.
If the V line command is used and the file is tagged with a CCSID of 1208, ISPF invokes the UTF8 edit
facility.
X—execute command
The X (eXecute command) line command can be entered against regular files, directories, or symbolic
links to regular files or directories in the directory list. This line command causes the Execute Command
for z/OS UNIX File panel to be displayed.
This panel allows you to enter and execute a z/OS UNIX command, TSO command, CLIST, or REXX EXEC,
with the path name of the selected file being passed as a parameter.
The Pathname field displays the path name of the selected file.
These input fields allow you to specify the command and select the method by which it is run:
Command for file
Use this field to enter the z/OS UNIX command, TSO command, CLIST, or REXX exec to be run.
By default, ISPF appends the path name of the selected file to the end of the command you have
entered. If you need to have the path name in a position other than the end of the command, use
the path name substitution character to indicate where you want the path name to be placed. The
default pathname substitution character is ! (exclamation point). For more information about using
this symbol, see “Using the path name substitution character” on page 250.
The path name substitution character can also be changed using the Directory List Options panel (see
page “Path name substitution character” on page 283).
If the command is to run in z/OS UNIX by selecting a Run method of Direct or Login shell, then this
field can be left blank. This causes the selected file to be executed.
Run method
This field is mandatory. It allows you to select one of these methods for running the command:
Direct
Causes the command to be run in z/OS UNIX.
Login shell
Causes the command to be run under the login shell in z/OS UNIX.
TSO
Causes the command to be passed to TSO for execution.
z/OS UNIX command time limit
This field allows you to set a limit to the amount of time the command can run. This time limit is
entered as a number of seconds. If this limit is exceeded, ISPF sends a SIGKILL signal to the process
running the command to terminate execution. If you do not want a time limit set, leave the field blank
or enter a value of zero.
The time limit value can also be specified on the Directory List Options panel (see page “z/OS UNIX
command time limit” on page 283).
=—repeat previous line command
The = (repeat) line command repeats the line command that was most recently used. This command is
most helpful when the same TSO command, CLIST, or REXX exec is to be invoked for more than one entry
in the directory list.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  277

## Page 316

For example, if you just invoked a CLIST named TESTABC with file /u/mydir/data (by typing TESTABC to
the left of filename data in the list for directory /u/mydir) and now you want to invoke TESTABC with
file /u/mydir/data2, you can type = to the left of filename data2 instead of retyping TESTABC.
z/OS UNIX directory list utility primary commands
These topics describe the primary commands available when using the z/OS UNIX Directory List Utility:
• “EDIT command” on page 278
• “FIND and RFIND commands” on page 279
• “LEFT command” on page 279
• “LOCATE command” on page 279
• “REFRESH command” on page 280
• “RESET command” on page 280
• “RIGHT command” on page 280
• “SAVE command” on page 281
• “SORT command” on page 281
• “SU command” on page 282
• “z/OS UNIX commands” on page 282
Note: If you enter a "/" (forward slash) in the primary command field, ISPF displays a panel with an
extended primary command field allowing you to enter commands up to 255 characters in length.
EDIT command
The EDIT primary command is used to edit a file in the directory currently listed. Use this format:
EDIT filename
The command can be abbreviated as E, EA, or EU. If EA is used, the editor is invoked with the ASCII edit
feature. If EU is used, the editor is invoked with the UTF-8 edit feature.
For example, if the command shown here was entered while displaying the directory list for u/myhome it
would invoke Edit for the file with a path name of /u/myhome/prog1:
E prog1
ISPF calls the ISPF editor to edit the file. If the file specified on the EDIT command does not exist in the
directory, the ISPF editor is still called and can be used to create a new file in the directory.
FILTER command
The FILTER command is used to append to or replace the current path name filter. If the current display is
a personal list, then the filter can only be replaced.
FILTER string
APPEND
REPLACE
If the current filter is /u/harry/test* then
FILTER p
changes the filter to /u/harry/test*p.
The REFRESH command restores the entry value.
z/OS UNIX directory list utility (option 3.17)
278  z/OS: z/OS ISPF User's Guide Vol II

## Page 317

FIND and RFIND commands
The FIND primary command is used to find and display the next occurrence of a character string in the list
of file names. Use this format:
FIND string
NEXT
ALL
FIRST
LAST
PREV
The command can be abbreviated as F.
For example, this command would tell ISPF to find all occurrences of the character string dat1:
F dat1 ALL
For more information about the operands used with this command, see “FIND—find character strings” on
page 73.
ISPF automatically scrolls to bring the character string to the top of the directory list. To repeat the search
without re-entering the character string, use the RFIND command.
Note: The RFIND search starts from the second file on the current directory list screen. It is not cursor-
sensitive.
LEFT command
The LEFT primary command scrolls the columns displaying information for the directory list to the left.
These columns do not include the Filename and Message columns, which are fixed as the left-hand
columns of the Directory List display. Use this format:
LEFT
PAGE
MAX
n
where:
PAGE
Specifies to scroll left by the number of columns of data (not counting the fixed fields) that can be
displayed within the current screen width. This is the default. P can be used as an abbreviation.
MAX
Specifies to scroll left so that the first column of data is displayed in the leftmost position. M can be
used as an abbreviation.
n
Is a numeric value specifying the number of columns to be scrolled to the left.
Note: If you issue the LEFT command while the cursor is positioned in a scrollable field such as the
Filename field, ISPF will scroll the scrollable field and the directory list columns will not be scrolled to the
left.
LOCATE command
The LOCATE primary command scrolls the directory list based on the field on which the directory list is
sorted, as described under “SORT command” on page 281. Use this format:
LOCATE lparm
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  279

## Page 318

You can use the lparm operand with the LOCATE command for either of these situations:
• If the list is sorted by the Filename field, specify a file name.
• If the list is sorted by another field, specify a value for the field by which the list is sorted.
The command can be abbreviated as L.
For example, for a directory list sorted by type, you could enter:
L Syml
This command locates the first symbolic link file in the directory list. If the value is not found, the list is
displayed starting with the entry before which the specified value would have occurred.
REFRESH command
The REFRESH primary command updates the display of the directory list to whatever the list's current
state is. For example, after deleting several items on the list, REFRESH causes the list to be displayed
without the deleted items. ISPF rebuilds the directory list display by re-reading the entries for the
directory.
The command can be abbreviated as REF
REFRESH
REF
RESET command
The RESET primary command removes any pending line commands and messages from the directory list.
The command can be abbreviated as RES.
RESET
RES
RIGHT command
The RIGHT primary command scrolls the columns displaying information for the directory list to the right.
These columns do not include the Filename and Message columns, which are fixed as the left-hand
columns of the Directory List display. Use this format:
RIGHT
PAGE
MAX
n
where:
Page
Specifies to scroll right by the number of columns of data (not counting the fixed fields) that can be
displayed within the current screen width. This is the default. P can be used as an abbreviation.
Max
Specifies to scroll right so that the first column of data is displayed in the rightmost position. M can be
used as an abbreviation.
n
Is a numeric value specifying the number of columns to be scrolled to the right.
Note: If you issue the RIGHT command while the cursor is positioned in a scrollable field such as the
Filename field, ISPF will scroll the scrollable field and the directory list columns will not be scrolled to the
right.
z/OS UNIX directory list utility (option 3.17)
280  z/OS: z/OS ISPF User's Guide Vol II

## Page 319

SAVE command
The SAVE primary command writes the directory list to the ISPF list data set or to a sequential data set.
ISPF writes the directory list in its current sort order. Use this format:
SAVE
list-id
where list-id is an optional user-specified qualifier of the data set to which the directory list will be
written. ISPF names the data set pr efix .userid.list-id.DIRLIST where:
pr efix 
Your data set prefix, as specified in your TSO user profile. If you have no prefix set, or if your prefix is
the same as your user ID, the prefix is omitted and the data set name will be: userid.list-id.DIRLIST
userid
Your TSO user ID.
If the data set does not exist it is created. If the data set already exists and has compatible attributes it is
overwritten. If you omit the list-id operand, the list is written to the ISPF list data set.
This command would tell ISPF to write the list to a sequential data set named either
pr efix .userid.MY.DIRLIST or userid.MY.DIRLIST:
SAVE MY
If the sequential data set already exists, ISPF overwrites it; if not, ISPF creates it.
SORT command
The SORT primary command sorts the directory list by the specified field. Use this format:
SORT
field1
A
D
field2 A
D
where:
field1 
The major sort field. If omitted, Filename is assumed.
field2 
The minor sort field. If both operands are used, ISPF sorts the list by field1  first, then by field2  within
field1 .
A|D
Specifies the sort sequence for the associated sort field (A=ascending; D=descending). By default,
character fields are sorted alphabetically and numeric fields are sorted in descending order.
For example, to sort a directory list by file type and then in descending order by modification date and
time within each file type, use this command:
SORT TYPE MODIFIED
This table identifies the sort field names and associated sort sequence:
Table 18. Sort field  names and associated sort sequence
Field Sequence Description
FILENAME|FILE|NAME Ascending File name
MESSAGE|MES Ascending Command message
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  281

## Page 320

Table 18. Sort field  names and associated sort sequence (continued)
Field Sequence Description
TYPE Ascending File type
PERM Ascending Permissions
PERMO Descending Permissions (octal)
AUDIT|AUD Ascending Audit bit settings
EXTA|EXT Ascending Extended attributes
FORMAT|FMAT Ascending File format
OWNER|OWN Ascending File owner
GROUP|GRP Ascending Owner group
LINKS Descending File links
SIZE Descending File size
MODIFIED|MOD Descending Date/time file last changed
CHANGED|CHG Descending Date/time file status last changed
ACCESSED|ACC Descending Date/time file status last accessed
CREATED|CRE Descending Date/time file was created
CASELESS|CASE Ascending Case-Insensitive sort
SU command
The SU primary command allows you to switch to super-user mode (UID 0) or switch back to your initial
UID.
For more details, see “Switching UIDs with the SU primary command” on page 307.
z/OS UNIX commands
You can also enter z/OS UNIX commands in the primary command field on the directory list display panel
if the option Enter z/OS UNIX commands in Command field is selected on the z/OS UNIX Directory List
Options panel. These commands run under the login shell in z/OS UNIX.
If you enter / (forward slash) in the primary command field the z/OS UNIX Directory List Command Entry
panel is displayed. This panel provides a 255 character length command field for entering long z/OS
UNIX and TSO commands. The panel also has a list of point-and-shoot fields showing the last 10 z/OS
UNIX commands entered from the z/OS UNIX Directory List Utility. The point-and- shoot fields allow you
to retrieve and execute z/OS UNIX commands. The List action bar allows you to activate or deactivate
updates to the list. The Mode action bar allows you to specify that commands are just retrieved or
retrieved and executed from the list. There is also an option that allows you to delete entries from the list.
The Function action bar provides an option to compress null entries from the list.
z/OS UNIX directory list options panels
These topics describe the panels available through the Options pull-down menu:
• “z/OS UNIX Directory List Options panel” on page 283
• “z/OS UNIX Directory List Column Arrangement panel” on page 284
• “z/OS UNIX Directory List Default Line Commands panel” on page 284
z/OS UNIX directory list utility (option 3.17)
282  z/OS: z/OS ISPF User's Guide Vol II

## Page 321

z/OS UNIX Directory List Options panel
This panel allows you to set and save options that change the behaviour of z/OS UNIX Directory List Utility
functions. This panel contains these optional input fields:
Width of filename column
Use this field to define the width of the column used to display file names in the directory list. The
minimum value you can specify is 8. The maximum value is 110. If the value is larger than the screen
width minus 50, ISPF uses the screen width minus 50 for the width of the filename column.
Note: The panel field used for the Filename column is defined as scrollable.
Path name substitution character
This field defines the character that can be used to represent the full path name of a selected file.
This character shows the position of the file name when it is specified as an argument in a z/OS
UNIX command, TSO command, CLIST, or REXX exec. The substitution character can be used with
commands that are invoked either as a line command in the directory list or through the Execute
Command for z/OS UNIX File panel (see “X—execute command” on page 277). The default character
is ! (exclamation point).
z/OS UNIX command time limit
This field allows you to set a limit to the amount of elapsed time for a z/OS UNIX command run either
directly or under the login shell. z/OS UNIX commands can be invoked via the X line command (see
“X—execute command” on page 277) or by using the line command prefix characters < (direct) or >
(login shell) (see “z/OS UNIX commands, TSO commands, CLISTs, and REXX EXECs” on page 249). If
the time limit set is exceeded by a z/OS UNIX command, ISPF sends a SIGKILL signal to the process
running the command to terminate execution.
If you do not want to set a time limit, leave the field blank or enter a value of zero.
Output Mode
This field allows you to display the output from z/OS UNIX commands in View or Browse mode.
Confirm File Delete
This option controls the display of the Confirm Delete panel. This panel is displayed when you use the
directory list line command "D" to delete files or empty directories. When this option is selected, the
Confirm Delete panel is displayed. When this option is not selected, the panel is not displayed and the
file or empty directory is deleted without any additional user interaction.
Confirm Non-empty Directory Delete
This option controls the display of the Confirm Non-empty Directory Delete panel. This panel is
displayed when you use the directory list line command "D" to delete a directory that contains files
and subdirectories. When this option is selected, the Confirm Non-empty Directory Delete panel is
displayed. When this option is not selected, the panel is not displayed and the directory (including all
contained files and subdirectories) is deleted without any additional user interaction.
Bypass z/OS UNIX File Edit Options panel
When this option is selected, ISPF will not display the z/OS UNIX File Edit Options panel when the
directory list line command "E" is used to edit a regular file. When this option is not selected, this
panel, which allows you to specify an edit profile and initial edit macro, will be displayed before
editing a regular file.
Display permissions in octal format
When this option is selected, permissions for files in the directory list are displayed in octal format.
When this option is not selected, permissions are displayed in symbolic format.
Case-Insensitive sort
When selected, the files in the directory list are sorted without respect to case. Uppercase and
lowercase letters remain together, for example, aAbBcC.
When deselected, the files are sorted with all the lowercase letters at the start of the list, followed by
all the uppercase letters, for example, abcd..ABCD.
After you change your selection, to see the list sorted with the new option, refresh the list by entering
the REFRESH primary command on the z/OS Directory List panel.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  283

## Page 322

z/OS UNIX Directory List Column Arrangement panel
This panel allows you to alter the order and width of the columns displayed on the directory list panel. It
lists each column in the z/OS UNIX Directory List. These fields are displayed for each entry:
Restore default column arrangements
Selecting this option allows you to reset the Order and Width values used to format the directory list
display to their default values.
Order
This input field displays the current ordinal position for the column on the directory list display. You
can update the value in this field to alter the position of this column on the directory list display. For
example, to move the Owner field to be the second column displayed, type 02 over its current Order
number and press Enter. The list of Columns is rearranged to show the Owner field in the second
position. When you next display a directory list, the columns are shown in the new order:
 Pathname . : /
 Command  Filename        Message          Type Owner    Permission Audit  Ext
 -------------------------------------------------------------------------------
          bin                              Dir  IBMUSER  rwxr-xr-x  fff---
          dev                              Syml IBMUSER  rwxrwxrwx  fff---
          etc                              Syml IBMUSER  rwxrwxrwx  fff---
Column
This output field displays the heading for the column on the directory list display.
Width
This input field displays the current width for the field for the column on the directory list display. You
can update this value to increase or decrease the size of the field for the column. Setting the width
value to 0 (zero) means the column will not be displayed in the directory list.
Maximum
This output field displays the maximum value that can be entered in the Width field.
z/OS UNIX Directory List Default Line Commands panel
This panel allows you to set and save the default line commands for the different z/OS UNIX file types
displayed in a z/OS UNIX directory list. The default line command for a file type is invoked when the
cursor is positioned in the line command field for a file of that type, the Enter key is pressed but a
command is not entered in the field. This panel contains these input fields:
Directory
Use this field to define the default line command for directories. The valid values are:
• CO
• CI
• N
• L (default)
• I
• D
• R
• MM
• MO
• MG
• MF
• X
• UA
• AA
z/OS UNIX directory list utility (option 3.17)
284  z/OS: z/OS ISPF User's Guide Vol II

## Page 323

• FS
Regular file
Use this field to define the default line command for regular files. The valid values are:
• E
• EA
• V
• VA
• B (default)
• CO
• CI
• N
• I
• D
• R
• MM
• MX
• MO
• MG
• MF
• X
• UA
• AA
• FS
• RA
Character special
Use this field to define the default line command for character special files. The valid values are:
• N
• I (default)
• D
• R
• MM
• MO
• MG
• MF
• UA
• AA
FIFO
Use this field to define the default line command for FIFO files. The valid values are:
• N
• I (default)
• D
• R
• MM
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  285

## Page 324

• MO
• MG
• MF
• UA
• AA
Symbolic link
Use this field to define the default line command for symbolic link files. The valid values are:
• E
• EA
• V
• VA
• B
• CO
• CI
• N
• I (default)
• D
• R
• X
Use of scrollable fields for path names
Because path names can be up to 1023 characters in length, ISPF uses scrollable fields throughout the
z/OS UNIX Directory List Utility for the display and entry of path names.
For path name output fields, if a + (scroll indicator) is displayed to the right of the path name it indicates
that the path name is larger than the display field length. The RIGHT primary command can be used to
view more of the path name by scrolling the value right. Use the EXPAND function key (F4) to display the
entire path name field in a pop-up window.
For path name input fields the + scroll indicator is always displayed to the right of the path name,
indicating that you can enter a path name larger than the input field length. The RIGHT primary command
can be used to obtain more input space by scrolling the value right. Use the EXPAND function key (F4) to
display the entire path name input field in a pop-up window.
z/OS UNIX Mounted File Systems
When you select Mount Table by File System... from the File_Systems pull-down menu on the action bar
of the z/OS UNIX Directory List Utility Panel, ISPF displays the z/OS UNIX Mounted File Systems panel
(ISRUUMT0). The entries in the displayed list are ordered by file system name.
Note: The z/OS UNIX Mounted File Systems panel is initially displayed with all entries collapsed. Figure
167 on page 287 is an example of the panel with all list entries expanded.
z/OS UNIX directory list utility (option 3.17)
286  z/OS: z/OS ISPF User's Guide Vol II

## Page 325

Menu  Utilities  Options  Help              
 ───────────────────────────────────────────────────────────────────────────────
                         z/OS UNIX Mounted File Systems          Row 1 from 148 
                                                                                
    File System Name  Mount Point  Type Mode Owner    A/M Status     I/O Counts 
 -------------------------------------------------------------------------------
     -DB2.**                                                                    
      -DB2.V810.**                                                              
       -DB2.V810.OMVS                                                           
        -DB2.V810.OMV                                                           
          DB2.V810.OM /apc/db2810/ ZFS  R/O  ISA1     YES Available           0 
        -DB2.V810.OMV                                                           
          DB2.V810.OM /apc/db2810/ ZFS  R/O  ISA1     YES Available           0 
         DB2.V810.OMV /apc/db2810/ ZFS  R/O  ISA1     YES Available           0 
        -DB2.V810.OMV                                                           
          DB2.V810.OM /apc/db2810/ ZFS  R/O  ISA1     YES Available           0 
      -DB2.V910.**                                                              
       -DB2.V910.SDSN                                                           
         DB2.V910.SDS /apc/tdb2910 ZFS  R/O  ISA1     YES Available           0 
       -DB2.V910.SDSN                                                           
         DB2.V910.SDS /apc/tdb2910 ZFS  R/O  ISA1     YES Available           0 
       -DB2.V910.SDSN                                                           
         DB2.V910.SDS /apc/tdb2910 ZFS  R/O  ISA1     YES Available           0 
       -DB2.V910.SDSN                                                           
         DB2.V910.SDS /apc/tdb2910 ZFS  R/O  ISA1     YES Available           0 
     -FEK.**                                                                    
      -FEK.V850.**                                                              
       -FEK.V850.OMVS                                                           
                                                                                
 Command ===>                                                  Scroll ===> PAGE 
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward   
  F9=Swap     F10=Actions  F12=Cancel     
Figure 167. z/OS UNIX Mounted File Systems panel (ISRUUMT0), ordered by file  system name
When you select Mount Table by Mount Point... from the File_Systems pull-down menu on the action bar
of the z/OS UNIX Directory List Utility Panel, ISPF displays the z/OS UNIX Mounted File Systems panel
(ISRUUMT0). The entries in the displayed list are ordered by mount point name.
Note: The z/OS UNIX Mounted File Systems panel is initially displayed with all entries collapsed. Figure
168 on page 287 is an example of the panel with all list entries expanded.
  Menu  Utilities  Options  Help              
────────────────────────────────────────────────────────────────────────────────
                         z/OS UNIX Mounted File Systems           Row 1 from 75 
                                      
    Mount Point  File System Name  Type Mode Owner    A/M Status     I/O Counts 
 -------------------------------------------------------------------------------
    -/           SYS1.OMVS.$$SRCB. ZFS  R/O  ISA1     YES Available           0 
     -/apc       OMVS.APC.ZFS.ISA1 ZFS  R/W  ISA1     YES Available           0 
      -/apc/db28                                                                
       -/apc/db2                                                                
        -/apc/db                                                                
          /apc/d DB2.V810.OMVS.DB2 ZFS  R/O  ISA1     YES Available           0 
          /apc/d DB2.V810.OMVS.DB2 ZFS  R/O  ISA1     YES Available           0 
          /apc/d DB2.V810.OMVS.ZFS ZFS  R/O  ISA1     YES Available           0 
          /apc/d DB2.V810.OMVS.MSY ZFS  R/O  ISA1     YES Available           0 
       /apc/itim ITIMRACF.V5.ZFS   ZFS  R/O  ISA1     YES Available           0 
      -/apc/tdb2                                                                
       -/apc/tdb                                                                
        -/apc/td                                                                
          /apc/t DB2.V910.SDSNAZFS ZFS  R/O  ISA1     YES Available           0 
          /apc/t DB2.V910.SDSNJCC. ZFS  R/O  ISA1     YES Available           0 
          /apc/t DB2.V910.SDSNMQLS ZFS  R/O  ISA1     YES Available           0 
          /apc/t DB2.V910.SDSNWORF ZFS  R/O  ISA1     YES Available           0 
       /apc/tipt IPT4Z.V114.OMVS.H ZFS  R/O  ISA1     YES Available           0 
       /apc/trdz RD4Z.V710.OMVS.HF ZFS  R/O  ISA1     YES Available           0 
       /apc/trdz RD4Z.V750.OMVS.HF ZFS  R/O  ISA1     YES Available           0 
       /apc/trdz RD4Z.V760.OMVS.HF ZFS  R/O  ISA1     YES Available           0 
       /apc/trdz RD4Z.V801.OMVS.HF ZFS  R/O  ISA1     YES Available           0 
                                                                                
 Command ===>                                                  Scroll ===> PAGE 
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward   
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 168. z/OS UNIX Mounted File Systems panel (ISRUUMT0), ordered by mount point name
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  287

## Page 326

The z/OS UNIX Mounted File Systems panel, whether ordered by file system name or mount point name,
provides you with these options:
• Expand or contract sections of the list
• Modify the format of the list
• Find file systems or mount points in the list
• Display information for a file system
• Display directory list information for a file system
• Modify the attributes of a file system
• Reset the pending quiesce of a file system
• Mount or unmount a file system.
z/OS UNIX Mounted File Systems panel action bar
The z/OS UNIX Mounted File Systems panel action bar choices function as follows:
Menu
For more information, refer to the details about the Menu Action Bar Choice in the ISPF User Interface
topic in z/OS ISPF User's Guide Vol I.
Utilities
For more information, refer to the details about the Utilities Action Bar Choice in the ISPF User
Interface topic in z/OS ISPF User's Guide Vol I.
Options
The Options pull-down offers these choices:
 1
Mount Table List Options...
This option allows you to define the width of the leftmost column in the mounted file systems list.
For more information, see “Setting the mounted file systems list options” on page 289.
 2
Mount Table Column Arrangement...
This option allows you to alter the order and width of the columns displayed on the Mounted
File Systems panel. For more information, see “Setting the mounted file systems list column
arrangement” on page 289.
 3
Expand All Entries
Expands all entries in the mounted file systems list and all subentries. This function is also
provided by primary command XA. For more information on primary command XA, see “XA
command” on page 301.
 4
Mount...
Provides the option to mount a file system. This function is also provided by primary command
MOUNT. For more information on primary command MOUNT, see “MOUNT command” on page
299.
Help
The Help pull-down provides information about the z/OS UNIX Mounted File Systems primary
commands and line commands as well as information about the format of the mounted file systems
list.
z/OS UNIX directory list utility (option 3.17)
288  z/OS: z/OS ISPF User's Guide Vol II

## Page 327

Setting the mounted file systems list options
When you select Mount Table List Options... from the Options pull-down menu on the action bar, ISPF
displays the z/OS UNIX Mount Table List Options panel (ISRUMNO1).
  Menu  Utilities  Options  Help              
┌────────────────────────────────────────────────────────────────────────────┐
│                    z/OS UNIX Mount Table List Options                      │
│                                                                            │
│ Width of Mount Point column in Mount                                       │
│ Point List . . . . . . . . . . . . . . .  35                               │
│                                                                            │
│ Width of File System column in File                                        │
│ System List  . . . . . . . . . . . . . .  35                               │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│                                                                            │
│ Command ===>                                                               │
│  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward   │
│  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                    │
└────────────────────────────────────────────────────────────────────────────┘
                                                                        
                                                                        
                                                                        
 Command ===> MTBOPTS                                         Scroll ===> PAGE 
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward  F8=Forward 
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 169. z/OS UNIX Mount Table List Options panel (ISRUMNO1)
This panel allows you to define the width of the leftmost column in the mounted file systems list. This
column displays:
• Mount point names when the list is ordered by mount point name.
• File system names when the list is ordered by file system name
The following fields are available on this panel:
Width of Mount Point column in Mount Point List:
This field allows you to define the width of the column used to display mount point names in the
mount point list. The minimum value is 11 and the maximum value is 110. If the value is larger than
the screen width minus 50, ISPF uses the screen width minus 50 for the width of the mount point
name column.
Width of File System column in File System List:
This field allows you to define the width of the column used to display file system names in the file
system list. The minimum value is 16 and the maximum value is 110. If the value is larger than the
screen width minus 50, ISPF uses the screen width minus 50 for the width of the file system name
column.
Setting the mounted file systems list column arrangement
When you select Mount Table Column Arrangement... from the Options pull-down menu on the action bar,
ISPF displays the Mount Table Column Arrangement panel (ISRUMNO2).
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  289

## Page 328

Menu  Utilities  Options  Help              
┌─────────────────────────────────────────────────────────────────────────────┐
│                       Mount Table Column Arrangement        Row 1 to 7 of 7 │
│                                                                             │
│ Enter "/" to select option                                                  │
│    Restore default column arrangements                                      │
│                                                                             │
│ Order  Column            Width  Maximum                                     │
│  01    File System Name   025      70                                       │
│  02    Type               004       8                                       │
│  03    Mode               004       4                                       │
│  04    Owner              008       8                                       │
│  05    A/M                003       3                                       │
│  06    Status             040      40                                       │
│  07    I/O Counts         003      10                                       │
│ ***************************** Bottom of data ****************************** │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│ Command ===>                                             Scroll ===> PAGE   │
│  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward    │
│  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                     │
└─────────────────────────────────────────────────────────────────────────────┘
                                                                          
                                                                          
                                                                          
 Command ===>                                                 Scroll ===> PAGE 
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward  F8=Forward 
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 170. Mount Table Column Arrangement panel (ISRUMNO2)
This panel allows you to modify the order and width of the columns displayed on the z/OS UNIX Mounted
File Systems panel. You can also restore the settings to their default values.
The order and width settings are kept separately for the display ordered by file system name and the
display ordered by mount point name. The settings that are displayed and updated on the Mount Table
Column Arrangement panel are for the display that is active when the Mount Table Column Arrangement
panel is selected. Figure 170 on page 290 shows an example of the Mount Table Column Arrangement
panel when it is selected from the display ordered by mount point name.
The following input fields are available on this panel:
Restore default column arrangements
Indicates that the z/OS UNIX Mounted File Systems panel is displayed with the default column
arrangements. The default order and column lengths are:
Order Column Width
1 File system name 
or
Mount point name
35
25
2 Type 4
3 Mode 4
4 Owner 8
5 A/M 3
6 Status 10
7 I/O Counts 10
Order
The order for each of the columns on the panel.
z/OS UNIX directory list utility (option 3.17)
290  z/OS: z/OS ISPF User's Guide Vol II

## Page 329

Width
The length for each of the columns on the panel. The maximum length allowed for each column is also
displayed.
z/OS UNIX Mounted File Systems panel fields
The z/OS UNIX Mounted File Systems panel displays the following information about the file systems:
File System Name
The name of the data set for the file system.
Mount point
The name of the directory that is the mount point for the file system.
Type
The type of physical file system that manages the mounted file system.
Mode
The mount mode of the file system. Possible values are R/W (Read/Write) and R/O (Read only).
Owner
The name of the owning system of the file system.
A/M
Indicates whether the automove function is enabled for the file system.
Status
The status of the file system. For the possible status values and their explanations, see the FS (file
system) line command topic under “z/OS UNIX directory list utility line commands” on page 251.
I/O Counts
The sum of the block input/output counts for user data reads, user data writes, and directory inputs/
outputs. This value is only available if SMF type 92 records are active.
z/OS UNIX mounted file systems line commands
After you display the z/OS UNIX mounted file systems list, you can enter a line command to the left of a
list entry.
If you enter a slash (/) to the left of a list entry, the Mounted File Systems List Actions pop-up window is
displayed. This window allows you to select the line command you want to invoke.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  291

## Page 330

Menu  Utilities  Options  Help              
 ─ ┌─────────────────────────────────────────────────────────────────────┐ ─────
   │                 Mounted File Systems List Actions                   │  148 
   │                                                                     │ unts 
   │ Selected entry . . : DB2.V910.SDSNWORF.ZFS                          │ -----
 - │                                                                     │
   │ Action:                                                             │
   │    1. Display file system information                               │
   │    2. Display directory list information                            │
   │    3. Modify file system attributes                                 │
   │    4. Reset quiesce file system                                     │    0
   │    5. Unmount file system                                           │
   │    6. Expand or contract list entry                                 │    0
   │    7. Expand list entry and all subentries                          │    0
   │                                                                     │
   │ Select a choice and press ENTER to process action.                  │    0
   │                                                                     │
   │                                                                     │
   │                                                                     │    0
   │                                                                     │
   │  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward     │    0
   │  F9=Swap     F12=Cancel                                             │
   └─────────────────────────────────────────────────────────────────────┘    0
                                                                         
                                                                         
                                                                         
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 171. Mounted File Systems List Actions pop-up window
I — information
The I (information) line command displays the z/OS UNIX File System Attributes panel (ISRUULFS). This
panel, which displays file system information and attributes, is the same panel that is displayed by the FS
(file system) line command under z/OS UNIX Directory List Utility. For more information, see the FS--file
system topic under “z/OS UNIX directory list utility line commands” on page 251. The I command can be
entered only on lines that display both a file system name and a mount point name.
z/OS UNIX File System Attributes panel (ISRUULFS)
z/OS UNIX directory list utility (option 3.17)
292  z/OS: z/OS ISPF User's Guide Vol II

## Page 331

z/OS UNIX File System Attributes          
 Command ===>                                                                 
 Pathname : /apc/db2810/usr/lpp/db2ext_08_01_00                                 
                                                                                
 File system name . : DB2.V810.OMVS.DB2EXT.ZFS                                  
 Mount point  . . . : /apc/db2810/usr/lpp/db2ext_08_01_00                       
                                                                                
 Status . . . . . . : Available                                                 
 File system type . : ZFS                                                       
 Mount mode . . . . : R/O                                                       
                                                                                
 Device number  . . : C                                                         
 Type number  . . . : 1                                                         
 DD name  . . . . . : SYS00015                                                  
                                                                                
 Ignore SETUID  . . : NO                                                        
 Bypass Security  . : NO                                                        
 Automove . . . . . : YES                                                       
 Owning system  . . : ISA1                                                      
                                                                                
 CCSID  . . . . . . :                                                           
 Text Convert . . . : NO                                                        
 Seclabel . . . . . :                                                           
                                                                                
 Block size . . . . : 4096                                                      
 Total blocks . . . : 5220                                                      
 Available blocks . : 320                                                       
 Blocks in use  . . : 4869                                                      
                                                                                
 Data blocks read  . . . : 0                                                    
 Data blocks written . . : 0                                                    
 Directory blocks r/w  . : 0                                                    
                                                                                
 Mount parameters                                                               
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward   
  F9=Swap     F10=Actions  F12=Cancel                                          
L — directory list
The L (directory list) line command displays the z/OS UNIX Directory List panel (ISRUUDL0). This panel,
which displays directory list information for the file system, is the same panel that is displayed when
Enter is pressed on the z/OS UNIX Directory List Utility panel while the Option line is blank. For more
information, see the Blank—display directory list topic under “z/OS UNIX Directory List Utility panel
options” on page 242. The L command can be entered only on lines that display both a file system name
and a mount point name.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  293

## Page 332

Menu  Utilities  View  Options  Help  
 ────────────────────────────────────────────────────────────────────────────────
                            z/OS UNIX Directory List          Row 1 to 13 of 25
 Command ===>                                                  Scroll ===> PAGE
 Pathname . : /SYSTEM/etc
 Command  Filename        Message          Type Permission Audit  Ext  Fmat
 -------------------------------------------------------------------------------
          .                                Dir  rwxr-xr-x  fff---
          ..                               Dir  rwxr-xr-x  fff---
          .nfsc                            File rw-r--r--  fff--- --s- ----
          ant.conf                         File rwxrwxrwx  fff--- --s- ----
          bpa                              Dir  rwxr-xr-x  fff---
          cmx                              Dir  rwxr-xr-x  fff---
          dce                              Dir  rwxr-xr-x  fff---
          dfs                              Dir  rwxr-xr-x  fff---
          inetd.conf                       File rwxrwxrwx  fff--- --s- ----
          inetd.pid                        File rw-r--r--  fff--- --s- ----
          ioepdcf                          Syml rwxrwxrwx  fff---
          ldap                             Dir  rwxr-xr-x  fff---
          licmgmt                          Dir  rwxr-xr-x  fff---
          log                              File rw-rw----  fff--- --s- ----
          pkiserv                          Dir  rwxr-xr-x  fff---
          profile                          File rwxr-xr-x  fff--- --s- ----
          security                         Dir  rwxr-xr-x  fff---
  F1=Help    F2=Split   F3=Exit    F4=Expand  F5=Rfind   F7=Up      F8=Down
  F9=Swap   F10=Left   F11=Right  F12=Cancel
Figure 172. z/OS UNIX Directory List panel (ISRUUDL0)
M — modify attributes
The M (modify attributes) line command displays the Select the Attribute to Change panel (ISRUMATR),
allowing you to modify the file system attributes. The M command can be entered only on lines that
display both a file system name and a mount point name. This function is restricted to superusers.
   Menu  Utilities  Options  Help              
 ─ ┌───────────────────────────────────────────────────────────────────┐ ───────
   │                  Select the attribute to change                   │ om 148 
   │                                                                   │ 
   │ File system  . : DB2.V910.SDSNWORF.ZFS                            │ Counts
 - │                                                                   │ -------
   │ Select the attribute to change                                    │
   │    1. Change mount mode to R/W                                    │
   │    2. Change owning system from ISA1                              │
   │    3. Change automove attribute...                                │
   │    4. Remount samemode R/O                                        │      0
   │                                                                   │
   │ New owning system                                                 │      0
   │                                                                   │      0
   │                                                                   │
   │ Command ===>                                                      │      0
   │  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  │    
   │  F8=Forward   F9=Swap     F10=Actions  F12=Cancel                 │
   └───────────────────────────────────────────────────────────────────┘      0
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 173. Select the Attribute to Change panel (ISRUMATR)
The Select the Attribute to Change panel provides the following options:
 1
Change mount mode
You can change the mount mode between R/O (Read-only) and R/W (Read/Write).
z/OS UNIX directory list utility (option 3.17)
294  z/OS: z/OS ISPF User's Guide Vol II

## Page 333

2
Change owning system
In sysplex mode, the owning system can be changed. For this option, you must also specify the name
of the new owning system at the bottom of the selection menu.
 3
Change automove attribute
In sysplex mode, the automove attribute can be changed. If automove is set to no, the file system
becomes unavailable when the owning system is shutting down.
 4
Remount samemode
You can remount the file system without changing the mode. This can be used in an attempt to regain
use of a file system with I/O errors.
When you select the Change mount mode option, ISPF displays the Mode Change Confirmation panel
(ISRUCHGM).
   Menu  Utilities  Options  Help              
 ─ ┌───────────────────────────────────────────────────────────────────────┐ ───
   │                       Mode Change Confirmation                        │ 48 
   │                                                                       │ 
   │ CAUTION:                                                              │ ts
 - │ The selected file system is about to be remounted. The file system    │ ---
   │ is first unmounted and then mounted with a different mount mode.      │
   │                                                                       │
   │ File system name: DB2.V910.SDSNWORF.ZFS                               │
   │                                                                       │
   │                                                                       │  0
   │ To proceed with the remount, press the ENTER key. To cancel the       │
   │ remount, use the CANCEL or EXIT function key.                         │  0
   │                                                                       │  0
   │                                                                       │
   │                                                                       │  0
   │                                                                       │
   │ Command ===>                                                          │  
   │  F1=Help       F2=Split      F3=Exit       F4=Expand     F7=Backward  │  0
   │  F8=Forward    F9=Swap      F10=Actions   F12=Cancel                  │
   └───────────────────────────────────────────────────────────────────────┘  0
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 174. Mode Change Confirmation  panel (ISRUCHGM)
If you press the Enter key on the Mode Change Confirmation panel, the file system is first unmounted and
then remounted in the changed mode.
When you select the Change automove attribute option, ISPF displays the Set Automove Attribute panel
(ISRUSAMA).
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  295

## Page 334

Menu  Utilities  Options  Help              
 ─ ┌─────────────────────────────────────────────────────────────┐ ─────────────
   │                   Set Automove Attribute                    │ w 1 from 148
   │                                                             │
   │ Select the automove attribute                               │   I/O Counts
 - │ _  1. Yes                                                   │ -------------
   │    2. No                                                    │
   │    3. Unmount                                               │
   │    4. Include systems                                       │
   │    5. Exclude systems                                       │
   │                                                             │            0
   │                                                             │
   │ System names for Include or Exclude                         │            0
   │                                                             │            0
   │                                                             │
   │                                                             │            0
   │                                                             │
   │                                                             │
   │                                                             │            0
   │                                                             │
   │                                                             │            0
   │ Command ===>                                                │
   │  F1=Help        F2=Split       F3=Exit        F4=Expand     │            0
   │  F7=Backward    F8=Forward     F9=Swap       F10=Actions    │
 m └─────────────────────────────────────────────────────────────┘            0
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 175. Set Automove Attribute panel (ISRUSAMA)
The automove attribute specifies the action that is to be taken for the file system when the system is in
sysplex mode and the owning system fails.
The Set Automove Attribute panel provides the following options:
 1
Yes
Select this option to set automove to on. Recovery of the file system is performed when the current
owner fails. Use this option on mounts of file systems that are critical to operation across all the
systems in the sysplex. This is the default.
 2
No
Select this option to set automove to off. Attempts are not made to keep the file system active
when the current owner fails. The file system remains in the hierarchy for possible recovery when
the original owner reinitializes. Use this option on mounts for system-specific file systems to have
automatic recovery when the original owner rejoins the sysplex.
If this option is used, the file system becomes unowned when the owning system exits the sysplex.
The file system remains unowned until the last owning system restarts, or until the file system is
unmounted. The mount point for the file system is still in use because the file system still exists in the
file system hierarchy.
An unowned file system is a mounted file system that does not have an owner. It can be recovered or
unmounted because it still exists in the file system hierarchy.
 3
Unmount
Select this option to set automove to unmount. If the current owner fails, the file system becomes
inactive. The file system, as well as all the file systems mounted within it, is unmounted if the owner is
no longer active in the sysplex.
Use this option for system-specific file systems, such as those that would be mounted at /etc, /
dev, /tmp and /var.
z/OS UNIX directory list utility (option 3.17)
296  z/OS: z/OS ISPF User's Guide Vol II

## Page 335

4
Include Systems
Select this option to ensure that recovery of a file system transfers ownership only to a particular
system or set of systems in the sysplex. Recovery of the file system is performed in priority order only
by the list of systems specified in the include list. Specify the include list under System names for
Include or Exclude.
 5
Exclude Systems
Select this option to prevent recovery of a file system from transferring ownership to a particular
system, or set of systems, in the sysplex. When the current owner fails, recovery of the file system
is performed by a randomly selected owner outside the exclude list. Specify the exclude list under
System names for Include or Exclude.
R — release from quiesce
The R (release from quiesce) line command displays the Release From Quiesce Status panel (ISRURFQS).
The R command can be entered only on lines that display both a file system name and a mount point
name. This function is restricted to superusers.
   Menu  Utilities  Options  Help              
 ─ ┌───────────────────────────────────────────────────────────────────────┐ ───
   │                     Release From Quiesce Status                       │ 48 
   │                                                                       │ 
   │ CAUTION:                                                              │ ts
 - │ The selected file system is about to be released from the quiesce     │ ---
   │ status.                                                               │
   │                                                                       │
   │ A backup in progress may be invalidated.                              │
   │                                                                       │
   │ File system name: DB2.V910.SDSNWORF.ZFS                               │  0
   │                                                                       │
   │                                                                       │  0
   │ To proceed with the Release, press the ENTER key. To cancel the       │  0
   │ Release, use the CANCEL function key.                                 │
   │                                                                       │  0
   │                                                                       │
   │ Command ===>                                                          │  
   │  F1=Help       F2=Split      F3=Exit       F4=Expand     F7=Backward  │  0
   │  F8=Forward    F9=Swap      F10=Actions   F12=Cancel                  │
   └───────────────────────────────────────────────────────────────────────┘  0
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 176. Release From Quiesce Status panel (ISRURFQS)
If the release operation is successful, the file system returns to the available status. A file system is likely
to be in a quiesced status during a file system backup.
U — unmount
The U (unmount) line command displays the Unmount a z/OS UNIX File System panel (ISRUMNUM). The
U command can be entered only on lines that display both a file system name and a mount point. A
file system cannot be unmounted if other file systems are mounted on it. This function is restricted to
superusers.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  297

## Page 336

Menu  Utilities  Options  Help              
 ┌────────────────────────────────────────────────────────────────────────────┐
 │                      Unmount a z/OS UNIX File System                       │
 │                                                                            │
 │ CAUTION:                                                                   │
 │ The file system is about to be unmounted.                                  │
 │                                                                            │
 │ File System Name . : DB2.V910.SDSNWORF.ZFS                                 │
 │                                                                            │
 │ Unmount Type . . . . _  1. Normal                                          │
 │                         2. Immediate                                       │
 │                         3. Force                                           │
 │                                                                            │
 │                                                                            │
 │ Command ===>                                                               │
 │  F1=Help       F2=Split      F3=Exit       F4=Expand     F7=Backward       │
 │  F8=Forward    F9=Swap      F10=Actions   F12=Cancel                       │
 └────────────────────────────────────────────────────────────────────────────┘
 Command ===>                                                  Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 177. Unmount a z/OS UNIX File System panel (ISRUMNUM)
The Unmount a z/OS UNIX File System panel provides the following options:
1. Normal
Performs a normal unmount request. If the files in the named file system are not in use, the unmount
is performed. Otherwise, the request is rejected.
2. Immediate
Performs an unmount immediate request. The file system is unmounted immediately, forcing any
users of any files in the named file system to fail. All data changes that were made up to the time of the
request are saved. If there is a problem saving the data, the unmount request fails.
3. Force
Performs an unmount force request. The file system is unmounted immediately, forcing any users
of any files in the named file system to fail. All data changes that were made up to the time of the
request are saved. If there is a problem saving the data, the request continues and data might be
lost. Because data might be lost, the request is rejected unless an immediate unmount request was
previously attempted.
X — expand
If the mounted file systems list entry is not expanded and there are subentries available, the X (expand)
line command expands the entry by one level. If the mounted file systems list entry is expanded, the X
(expand) line command contracts the entry.
XA — expand all
If the mounted file systems list entry is not expanded and there are subentries available, the XA (expand
all) line command expands the entry and all subentries.
z/OS UNIX mounted file systems primary commands
These topics describe the primary commands available on the z/OS UNIX Mounted File Systems panel:
• “FIND and RFIND commands” on page 299
• “LEFT command” on page 299
• “MOUNT command” on page 299
• “RIGHT command” on page 301
z/OS UNIX directory list utility (option 3.17)
298  z/OS: z/OS ISPF User's Guide Vol II

## Page 337

• “XA command” on page 301
FIND and RFIND commands
The FIND primary command is used to find and display the next occurrence of a character string within
the mounted file systems list. The string can be found in either the mount point name or the file system
name. Use this format:
FIND string
NEXT
ALL
FIRST
LAST
PREV
The command can be abbreviated as F.
For example, this command finds all occurrences of the character string dat1:
 F dat1 ALL
For more information about the ALL, FIRST, NEXT, LAST, and PREV operands, see “FIND—find character
strings” on page 73.
ISPF automatically scrolls to bring the character string to the top of the list. To repeat the search without
re-entering the character string, use the RFIND command.
Note: The RFIND search starts from the second entry in the displayed list. It is not cursor-sensitive.
LEFT command
The LEFT primary command scrolls the columns displaying information for the mounted file systems to
the left. These columns do not include the leftmost column, which is fixed on the z/OS UNIX Mounted File
Systems display. Use this format:
LEFT
PAGE
MAX
n
where:
PAGE
Specifies to scroll left by the number of columns of data (not counting the leftmost column, which is
fixed) that can be displayed within the current screen width. This is the default. P can be used as an
abbreviation.
MAX
Specifies to scroll left so that the first scrollable column of data is displayed in the leftmost scrollable
position. M can be used as an abbreviation.
n
Specifies the number of columns to be scrolled to the left.
Note: If you issue the LEFT command while the cursor is positioned in a scrollable field, ISPF scrolls the
scrollable field and the mounted file systems list columns are not scrolled to the left.
MOUNT command
The MOUNT primary command displays the Mount z/OS UNIX File System panel (ISRUMNMT). This panel
can be used to logically mount a mountable file system. When a file system is mounted it is added to the
file system hierarchy. Use this format:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  299

## Page 338

MOUNT
The command can be abbreviated as MO, MOU, or MOUN.
Note: The MOUNT command is also available by using the Mount... choice on the Options pull-down menu
on the action bar.
                        Mount z/OS UNIX File System  
                                                                             
 Mount Point  . . . /u/testuser_                                         +
 File System Name                                                       
                                                                             
 File System Type             New Owner  . . . . .                 
 Owning System  . .           Character Set ID . .                
                                                                            
 Additional Mount Options                                                 
    Read-only file system           Set automove attribute...      
    Ignore SETUID and SETGID        Enable text conversion          
    Bypass security                                                      
                                                                               
 Mount Parameter                                                         +
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
 Command ===>                                                             
  F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward   
  F9=Swap     F10=Actions  F12=Cancel
Figure 178. Mount z/OS UNIX File System panel (ISRUMNMT)
This panel allows you to enter the information needed to mount a file system. These required input fields
are displayed on this panel:
Mount point
The name of the directory that is the mount point for the file system.
File System Name
The name of the data set for the file system.
File System Type
The type of physical file system that manages the mounted file system.
These optional input fields are available on this panel:
New Owner
The name of the system that is designated to own the file system if the current owning system fails.
Owning system
The name of the system that owns the file system.
Character Set ID
The coded character set identifier to be implicitly set for untagged files in the file system.
Read-only file system
Indicates that the file system is mounted in read-only (R/O) mode.
Ignore SETUID and SETGID
Indicates that the SETUID and SETGID mode bits are ignored on any executable file in the file system
when the program is run.
Bypass security
Indicates that the security checks are not enforced for files in the file system.
Set automove attribute...
Indicates that you want the Set Automove Attribute panel to be displayed when you press Enter.
Use this panel to specify the action that is to be taken for the file system when the system is in
z/OS UNIX directory list utility (option 3.17)
300  z/OS: z/OS ISPF User's Guide Vol II

## Page 339

sysplex mode and the owning system fails. For more information, see the section on the Set Automove
Attribute panel under “M — modify attributes” on page 294.
Enable text conversion
Indicates that untagged files are implicitly marked as containing pure text data that can be converted.
Mount parameters
The parameters that are specified with the mount command for the file system.
RIGHT command
The RIGHT primary command scrolls the columns displaying information for the mounted file systems to
the right. These columns do not include the leftmost column, which is fixed on the z/OS UNIX Mounted
File Systems display. Use this format:
RIGHT
PAGE
MAX
n
where:
PAGE
Specifies to scroll right by the number of columns of data (not counting the leftmost column, which is
fixed) that can be displayed within the current screen width. This is the default. P can be used as an
abbreviation.
MAX
Specifies to scroll right so that the last scrollable column of data is displayed in the rightmost position.
M can be used as an abbreviation.
n
Specifies the number of columns to be scrolled to the right.
Note: If you issue the RIGHT command while the cursor is positioned in a scrollable field, ISPF scrolls the
scrollable field and the mounted file systems list columns are not scrolled to the right.
XA command
The XA primary command expands all entries in the z/OS UNIX Mounted File Systems list and all
subentries. Use this format:
XA
Note: The XA primary command is also available by using the Expand All Entries choice on the Options
pull-down menu on the action bar.
Creating a new zFS
When you select New zFS from the File_Systems pull-down menu on the action bar of the z/OS UNIX
Directory List Utility Panel, ISPF displays the Create a zFS Aggregate and File System panel (ISRUUFS4).
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  301

## Page 340

Menu  RefList  RefMode  Utilities  Options  File_Systems  Help
 ┌────────────────────────────────────────────────────────────────────────────┐
 │                   Create a zFS Aggregate and File System                   │
 │                                                                            │
 │ Enter the fields as required then press Enter.                             │
 │                                                                            │
 │ Aggregate name  . . . . . _                                                │
 │ Owning user . . . . . . .             (Number or user name)                │
 │ Owning group  . . . . . .             (Number or group name)               │
 │ Permissions . . . . . . . 750  (3 digits, each 0-7)                        │
 │ Primary cylinders . . . .                                                  │
 │ Secondary cylinders . . .                                                  │
 │ Storage class . . . . . .                                                  │
 │ Management class  . . . .                                                  │
 │ Data class  . . . . . . .                                                  │
 │ Volume names  . . . . . .                                                  │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │ Command ===>                                                               │
 │  F1=Help       F2=Split      F3=Exit       F4=Expand     F7=Backward       │
 │  F8=Forward    F9=Swap      F10=Actions   F12=Cancel                       │
 └────────────────────────────────────────────────────────────────────────────┘
 Option ===> UDLFSZ                                             
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 179. Create a zFS Aggregate and File System panel (ISRUUFS4)
The Create a zFS Aggregate and File System panel allows you to allocate a data set for an aggregate,
format the aggregate, and create a file system in that aggregate. The file system defined in the aggregate
is the same name as the aggregate and data set. For detailed information on zFS aggregates, file systems,
and their attributes, refer to the zFS Administration book.
The following attributes can be specified for the create operation:
Aggregate name
Specify the fully qualified name for the new data set by enclosing it in apostrophes. If you omit
the apostrophes, your TSO prefix is left-appended to the data set name. If you omit the trailing
apostrophe, the apostrophe is assumed.
Owning user
Specify the UID or user id for the owner of the root directory for the file system that is created. If this
attribute is not specified, your UID is used.
Owning group
Specify the GID or group id for the owning group of the root directory for the file system that is
created. If this attribute is not specified, your GID is used.
Permissions
Specify the permissions in octal format. If this attribute is not specified, the value 750 is used.
Primary cylinders
Specify the number of cylinders to allocate for the primary extent for the data set. The aggregate is
formatted to fit within this space. A line command is available to increase the size of the aggregate to
expand into the secondary allocation extents. This field must be specified.
Secondary cylinders
Specify the secondary allocation for the data set. If this attribute is not specified, the value 0 is used.
Storage class
If the data set is to be SMS managed, specify the SMS storage class for the data set allocation. If this
attribute is not specified for an SMS managed data set, the default storage class is used.
Management class
If the data set is to be SMS managed, specify the SMS management class for the data set allocation. If
this attribute is not specified for an SMS managed data set, the default management class is used.
z/OS UNIX directory list utility (option 3.17)
302  z/OS: z/OS ISPF User's Guide Vol II

## Page 341

Data class
If the data set is to be SMS managed, specify the SMS data class for the data set allocation. If this
attribute is not specified for an SMS managed data set, the default data class is used.
Volume names
If the data set is not to be SMS managed, you must enter the volume names for the data set
allocation.
zFS aggregates
When you select zFS aggregates from the File_Systems pull-down menu on the action bar of the z/OS
UNIX Directory List Utility Panel, ISPF displays the Attached zFS Aggregates panel (ISRUUZ01). The name
of each attached zFS aggregate is displayed, along with the associated free space and total space values.
                         Attached zFS Aggregates               Row 1 to 4 of 4
                                                                               
Select an aggregate with a line command.                                       
A=Attributes  L=List file systems  E=Extend                                    
                                                                               
S Aggregate Name                                Free Space  Total Space        
_ FEK.V850.OMVS.ZFS                                  39399       136800        
_ FEK.V900.OMVS.ZFS                                  23209       168480        
_ ISPFTEST.ZFS.ISA1                                   6988         7200        
_ SYS1.OMVS.$$SRDG.ROOT                              23899      2415600        
******************************* Bottom of data ********************************
                                                                               
      
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
Command ===>                                             Scroll ===> PAGE      
 F1=Help      F2=Split     F3=Exit      F4=Expand    F7=Backward  F8=Forward   
 F9=Swap     F10=Actions  F12=Cancel       
Figure 180. Attached zFS Aggregates panel (ISRUUZ01)
Attached zFS aggregates line commands
After you display the attached zFS aggregates list, there are three line commands that you can use with
the displayed aggregates. Enter the line command in the S column to the left of the aggregate name.
A (Attributes)
Show the attributes of the aggregate.
E (Extend)
Extend the size of the aggregate.
L (List file systems)
List the file systems in the aggregate. You can perform actions on the file systems from this list.
A — Attributes
When the A (Attributes) line command is entered beside an aggregate name on the Attached zFS
Aggregates panel, the Aggregate Attributes panel (ISRUUZ11) is displayed.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  303

## Page 342

Attached zFS Aggregates              Row 1 to 4 of 4
 ┌────────────────────────────────────────────────────────────────────────────┐
 │                            Aggregate Attributes                            │
 │                                                                            │
 │ Aggregate name  . . . . : ISPFTEST.ZFS.ISA1                                │
 │ Attach mode . . . . . . : Read/write                                       │
 │ Monitored for full  . . : Disabled                                         │
 │ Auto-extend . . . . . . : Enabled                                          │
 │ Number of file systems  :          1                                       │
 │ Threshold . . . . . . . :          0                                       │
 │ Increment . . . . . . . :          0                                       │
 │ Number of fragments . . :       7200                                       │
 │ Fragment size . . . . . :       1024                                       │
 │ Block size  . . . . . . :       8192                                       │
 │ Blocks available  . . . :       7200                                       │
 │ Maximum fragments . . . :       6988                                       │
 │ Minimum fragments . . . :          0                                       │
 │                                                                            │
 │                                                                            │
 │ Command ===>                                                               │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward   │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                    │
 └────────────────────────────────────────────────────────────────────────────┘
 Command ===>                                             Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 181. Aggregate Attributes panel (ISRUUZ11)
The Aggregate Attributes panel displays the following information for the selected aggregate. For detailed
information on zFS aggregates and their attributes, refer to the zFS Administration book.
Aggregate name
The name of the aggregate.
Attach mode
The attach mode of the aggregate. Possible values are Read only and Read/write.
Monitored for full
Indicates whether aggregate full monitoring is enabled or disabled.
Auto-extend
Indicates whether auto-extend is enabled or disabled.
Number of file systems
The number of file systems in the aggregate.
Threshold
The threshold percentage value used for aggregate full monitoring.
Increment
The increment percentage value used for aggregate full monitoring.
Number of fragments
The number of fragments in the aggregate.
Fragment size
The size of the fragments in the aggregate.
Block size
The size of the blocks in the aggregate.
Blocks available
The number of blocks available in the aggregate.
Maximum fragments
The maximum number of fragments in the aggregate.
Minimum fragments
The minimum number of fragments in the aggregate.
z/OS UNIX directory list utility (option 3.17)
304  z/OS: z/OS ISPF User's Guide Vol II

## Page 343

E—Extend
The E (Extend) line command, entered beside an aggregate name on the Attached zFS Aggregates panel,
displays the Extend Aggregate panel (ISRUUZ07).
                          Attached zFS Aggregates              Row 1 to 4 of 4
 ┌────────────────────────────────────────────────────────────────────────────┐
 │                              Extend Aggregate                              │
 │                                                                            │
 │ Enter the New Aggregate Size in Kilobytes.                                 │
 │                                                                            │
 │ Aggregate  . . : ISPFTEST.ZFS.ISA1                                         │
 │ Current size . : 7200                                                      │
 │ New size . . . . _                                                         │
 │                                                                            │
 │ Command ===>                                                               │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward   │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                    │
 └────────────────────────────────────────────────────────────────────────────┘
 Command ===>                                             Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 182. Extend Aggregate panel (ISRUUZ07)
The Extend Aggregate panel displays the name of the selected aggregate and its current size, in kilobytes.
To extend the aggregate, enter the new size in kilobytes. zFS extends the aggregate to a size equal to or
greater than what you specify based on block boundaries. The data set for the aggregate must be defined
with sufficient space, secondary extents, or volumes to contain the increased allocation size.
Enter a new size of zero to extend the aggregate by one extent.
L— List file  systems
The L (List file systems) line command, entered beside an aggregate name on the Attached zFS
Aggregates panel, displays the File System List panel (ISRUUZ03). The name of each file system in the
selected aggregate is displayed, along with the associated space used and total space values.
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  305

## Page 344

Attached zFS Aggregates              Row 1 to 4 of 4
 ┌────────────────────────────────────────────────────────────────────────────┐
 │                            File System List                Row 1 to 1 of 1 │
 │                                                                            │
 │ Select a file system with a line command.                                  │
 │ A=Attributes                                                               │
 │                                                                            │
 │ S File System Name                              Space Used  Total Space    │
 │ _ ISPFTEST.ZFS.ISA1                                     59         7200    │
 │ ***************************** Bottom of data ***************************** │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │                                                                            │
 │ Command ===>                                             Scroll ===> PAGE  │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward   │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                    │
 └────────────────────────────────────────────────────────────────────────────┘
 Command ===>                                             Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 183. File System List panel (ISRUUZ03)
File system list line commands
The File System List panel provides one line command that you can use with the displayed file systems.
Enter the line command in the S column to the left of the file system name.
A (Attributes)
Shows the attributes of the file system.
A (Attributes)
The A (Attributes) line command, entered beside a file system name on the File System List panel,
displays the File System Attributes panel (ISRUUZ10).
                          Attached zFS Aggregates              Row 1 to 4 of 4
 ┌────────────────────────────────────────────────────────────────────────────┐
 │                           File System Attributes                           │
 │                                                                            │
 │ File system name  . . . : ISPFTEST.ZFS.ISA1                                │
 │ Mount status  . . . . . : Read/write                                       │
 │ Create time . . . . . . : 2013/03/12 14:46:17                              │
 │ Update time . . . . . . : 2013/05/23 09:48:06                              │
 │ Access time . . . . . . : 2013/05/23 09:48:06                              │
 │ Allocation limit  . . . : 4294967232                                       │
 │ Allocation used . . . . :         59                                       │
 │ Threshold . . . . . . . :          0                                       │
 │ Increment . . . . . . . :          0                                       │
 │                                                                            │
 │                                                                            │
 │ Command ===>                                                               │
 │  F1=Help        F2=Split       F3=Exit        F4=Expand      F7=Backward   │
 │  F8=Forward     F9=Swap       F10=Actions    F12=Cancel                    │
 └────────────────────────────────────────────────────────────────────────────┘
 Command ===>                                             Scroll ===> PAGE
  F1=Help      F2=Split     F3=Exit     F4=Expand    F7=Backward   F8=Forward
  F9=Swap     F10=Actions  F12=Cancel                                
Figure 184. File System Attributes panel (ISRUUZ10)
z/OS UNIX directory list utility (option 3.17)
306  z/OS: z/OS ISPF User's Guide Vol II

## Page 345

The File System Attributes panel displays the following information for the selected file system. For
detailed information on zFS file systems and their attributes, refer to the zFS Administration book.
File system name
The name of the file system.
Mount status
The mount status of the file system. Possible values are Read only, Read/Write, and Not mounted.
Create time
The date and time when the file system was created.
Update time
The date and time when the file system was last updated.
Access time
The date and time when the file system was last accessed.
Allocation limit
The allocation limit for the file system in kilobytes.
Allocation used
The amount of allocation used in kilobytes.
Threshold
The threshold percentage value used for file system full monitoring.
Increment
The increment percentage value used for file system full monitoring.
Switching to super-user (UID 0) mode and back
On the entry panel and the directory list display panel, you can switch to super-user mode (UID 0) or
switch back to your initial UID with either:
• The Options pull-down menu, or
• The SU primary command
Switching UIDs with the Options pull-down menu
The Options pull-down menu available on the entry panel and the directory list display panel provides an
option that lets you switch to super-user mode (UID 0) or switch back to your initial UID.
When you are operating under your UID, the Options pull-down menu displays this option:
3. Enable superuser mode(SU)
Note: If you select this option, and you have permission to the BPX.SUPERUSER facility class, you are
switched to UID 0 (super-user mode).
When you are operating in super-user mode, the Options pull-down menu displays this option:
3. Reset UID to nnn
If you select this option, you are switched back to your UID nnn.
Switching UIDs with the SU primary command
From the entry or directory list panels, you can switch to super-user mode (UID 0) or switch back to your
initial UID with the SU primary command. Use this format:
SU
UIDnum
where:
z/OS UNIX directory list utility (option 3.17)
Chapter 5. Utilities (option 3)  307

## Page 346

UIDnum
The UID to which you you want to switch.
Note: To switch to another UID, you must have permission to the BPX.DAEMON facility class (if defined).
If you do not specify a UID number, you are switched either to UID 0 (if you are currently operating under
your UID), or reset back to your UID (if you are currently operating in super-user mode).
If you specify a UID number, you must have read access to the SURROGAT class profile
BPX.SRV.uuuuuuuu (where uuuuuuuu is the MVS userid associated with the target UID).
z/OS UNIX directory list utility (option 3.17)
308  z/OS: z/OS ISPF User's Guide Vol II
