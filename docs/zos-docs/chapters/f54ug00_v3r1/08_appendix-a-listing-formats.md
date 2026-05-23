# Appendix A. Listing formats

Source file: f54ug00_v3r1.md
Start page: 173
Page span: 173-180

## Page 173

Appendix A. Listing formats
This topic describes and displays the kinds of listings you can produce using ISPF. The sample listings
shown are for illustration purposes only. They are not intended to be exact replicas, because printouts of
ISPF listings vary according to the kind of printer you are using.
Source and index listings
If autolist mode is on, the ISPF editor automatically generates a source listing when you finish editing.
You can also get source listings and index listings by using these utility options:
Library (3.1)
P - Print member
Move/Copy (3.3)
CP - Copy and print
MP - Move and print
Data Set List (3.4)
P - Print data set list
PV - Print VTOC entries
Hardcopy (3.6)
PK - Print and keep data set
PD - Print and delete data set
Outlist (3.8)
P - Print job output
Source listings
Figure 33 on page 145 shows a sample source listing.
Figure 33. Sample Source Listing
Source and index listings
© Copyright IBM Corp. 1980, 2024 145

## Page 174

Information at the top of the page includes project, group, type, and member name, current version and
modification level, user ID, date and time that the listing was produced, and page number.
A column-positioning line is printed following the heading and preceding the actual data. The start column
is printed to the left of each line, indicating the position of the first character in each line that is not a
blank.
For ISPF library members with statistics, asterisks are either printed or not printed to the right of each line
according to the setting of the modification flag, as follows:
• If the modification flag (columns 79-80) in the line is 00, no asterisks are printed.
• If the modification flag is nonzero but differs from the current modification level of the member, a single
asterisk (*) is printed.
• If the modification flag is nonzero and has the same value as the current modification level of the
member, two asterisks (**) are printed.
The asterisks allow you to scan the listing quickly for lines that were added or changed since the version
was created (*) and for lines that were added or changed during the last update (**).
Index listings
ISPF provides index listings at your request through the X (Print index listing) option of the Library utility
(3.1), or the X or PX options of the Data Set List utility (3.4).
Index listings for source libraries
Figure 34 on page 146 shows a sample index listing for an ISPF library.
DATASET:     ISPF.TEMP.PANELS                                                                           DATE: 18/09/19
                                                                                                        TIME: 15:34   
                                                                                                        PAGE: 001     
GENERAL DATA:                     GENERAL DATA:                                      CURRENT ALLOCATION:              
  MANAGEMENT CLASS:  NOACT          RECORD FORMAT:                   FB                   56 BLOCKS                   
  STORAGE CLASS:     NORMAL         RECORD LENGTH:                   80                    8 EXTENTS                  
  DATA CLASS:        **None**       BLOCK SIZE:                  27,920               5 DIRECTORY BLOCKS              
  VOLUME SERIAL:       CPDLB0       1ST EXTENT SIZE:                 14                                               
  DEVICE TYPE:           3390       SECONDARY QUAN:                   5              CURRENT UTILIZATION:             
  ORGANIZATION:            PO                                                             52 BLOCKS                   
  DATA SET NAME TYPE:     PDS                                                              8 EXTENTS                  
  CREATION DATE:   2015/06/10                                                         2 DIRECTORY BLOCKS              
  EXPIRATION DATE: ***None***                                                         8 MEMBERS                       
  DATA SET ENCRYPTION:     NO                                                                                         
  EXTENDED ATTRIBUTE:                                                                                                 
  CREATE JOBNAME:                                                                                                     
  CREATE STEPNAME:                                                                                                    
 MEMBER           TTR   VERS.MOD    CREATION      DATE AND TIME         CURRENT      INITIAL     MODIFIED       USER  
  NAME           (HEX)    LEVEL       DATE        LAST MODIFIED        NO. LINES    NO. LINES    NO. LINES       ID   
ISRUADCS         001909   01.00   2018/04/23   2018/04/23  13:19:13           21           21            0     USER123
ISRUAIPS         001907   01.00   2018/04/23   2018/04/23  13:18:42           35           35            0     USER123
ISRUAISO         001903   01.00   2018/04/23   2018/04/23  12:54:14           33           33            0     USER123
ISRUAISX         001905   01.00   2018/04/23   2018/04/23  13:02:12          136          136            0     USER123
ISR01734         000102   01.13   2015/06/10   2016/09/29  15:58:42           32           27            0     USER123
ISR01735         000009   01.01   2015/06/10   2015/06/10  15:15:15           27           27            0     USER123
ISR01739         000E01   01.07   2017/12/07   2017/12/08  08:54:36          654          843            0     USER123
ISR01750         001801   01.11   2017/12/07   2017/12/08  12:03:05          871          843            0     USER123
  MAXIMUMS:               01.13   2018/04/23   2018/04/23  13:19:13          871          843            0            
  TOTALS:                                                                   1809         1965            0            
 END OF MEMBER LIST
Figure 34. Sample Index Listing - Managed Source Library
The sample index listing shown here is for a source library that is managed by the Storage Management
Subsystem.
Note: This index listing format is available only when DFSMSdfp is installed and available, and when
Storage Management Subsystem is active.
The heading information includes:
• Project, group (library), and type
• Date and time the listing was produced
• Page number.
Source and index listings
146  z/OS: z/OS ISPF User's Guide Vol I

## Page 175

This is followed by general information about the data set, including current space allocation and
utilization. The only differences between this index listing and one for a non-managed source library
are:
• Management, storage, and data classes are shown under the GENERAL DATA heading.
• The 1st extent size, secondary quantity, current allocation, and current utilization sizes can be shown in
bytes, kilobytes, or megabytes, in addition to tracks, blocks, or cylinders.
The 1st extent quantity, secondary quantity, current allocation, and current utilization sizes are shown
in tracks for data sets that are allocated in bytes, kilobytes, or megabytes on a non-managed volume.
Following this, the member name and statistics are printed for each member in the data set, arranged in
alphabetical order. For sequential data sets, the index listing contains only the general information.
Index listings for load libraries
An index listing for an OBJ library is similar to an index listing for a source library, except that no statistics
are maintained. A sample index listing for a LOAD library that is managed by the Storage Management
Subsystem is shown in Figure 35 on page 147.
Note: This index listing format is available only when DFSMSdfp is installed and available, and when
Storage Management Subsystem is active.
Here, the module attributes are printed to the right of each member name.
DATASET:     ISPF.TEMP.LOAD                                                                             DATE: 18/09/19
                                                                                                        TIME: 15:13   
                                                                                                        PAGE: 001     
GENERAL DATA:                     GENERAL DATA:                                      CURRENT ALLOCATION:              
  MANAGEMENT CLASS:  NOACT          RECORD FORMAT:                    U                  390 BLOCKS                   
  STORAGE CLASS:     NORMAL         RECORD LENGTH:                    0                    1 EXTENT                   
  DATA CLASS:        **None**       BLOCK SIZE:                  32,760              50 DIRECTORY BLOCKS              
  VOLUME SERIAL:       CPDLB0       1ST EXTENT SIZE:                390                                               
  DEVICE TYPE:           3390       SECONDARY QUAN:                 250              CURRENT UTILIZATION:             
  ORGANIZATION:            PO                                                             70 BLOCKS                   
  DATA SET NAME TYPE: LIBRARY                                                              1 EXTENT                   
  CREATION DATE:   2018/09/19                                                         2 DIRECTORY BLOCKS              
  EXPIRATION DATE: ***None***                                                        10 MEMBERS                       
  DATA SET ENCRYPTION:     NO                                                                                         
  EXTENDED ATTRIBUTE:                                                                                                 
  CREATE JOBNAME:                                                                                                     
  CREATE STEPNAME:                                                                                                    
 MEMBER          ALIAS                SIZE      SIZE      ENTRY     TTR    AUTH                       MODULE          
  NAME            OF         SSI      (HEX)     (DEC)     POINT    (HEX)   CODE  AMODE  RMODE  ---- ATTRIBUTES ----   
ISPCIP          ISPSUBS    82390274  001063A8  1,074,088 000A9D78  000107   00     31    ANY      RN RU               
ISPSUBS                    82390274  001063A8  1,074,088 00000000  000107   00     31    ANY      RN RU               
ISRPLEX                              00015860     88,160 000000D8  00420A   00     31    ANY      RN RU  
ISRSEPRM                             00013D48     81,224 000000D8  003F0B   00     31    ANY      RN RU  
ISRSFM                               00008FE8     36,840 000000D8  003E09   00     31    ANY      RN RU  
ISRSSM                               0000A640     42,560 000000D8  003C0C   00     31    ANY      RN RU  
ISRSUBS                    82560449  000F6200  1,008,128 00000000  002314   00     31    ANY      RN RU  
ISRUDA                     82560435  000541C0    344,512 000000D8  001A08   00     31    ANY      RN RU  
ISRUDL          ISRUDA     82560435  000541C0    344,512 0002E1F8  001A08   00     31    ANY      RN RU  
ISRUOLP                              00002AB8     10,936 00000000  00230A   00     31    ANY      RN RU  
  TOTALS:                            0028FDF0  2,686,448                                                 
 END OF MEMBER LIST                                                                                      
-----                                                                                                    
MODULE ATTRIBUTE CODES:                                                                                  
     NX          NOT EXECUTABLE                                                                          
     OL          DATA ONLY, NOT LOADABLE                                                                 
     OV          IN OVERLAY STRUCTURE                                                                    
     RF          REFRESHABLE                                                                             
     RN          REENTERABLE                                                                             
     RU          REUSABLE                                                                                
     TS          MODULE CONTAINS TEST INFORMATION                                                        
     SC          SCATTER LOAD 
Figure 35. Sample Index Listing - Managed Load Library
The only differences between this index listing and one for a non-managed LOAD library are:
• Management, storage, and data classes are shown under the GENERAL DATA heading.
• The 1st extent size, secondary quantity, current allocation, and current utilization sizes can be shown in
bytes, kilobytes, or megabytes, in addition to tracks, blocks, or cylinders.
As in the source library index listing, the 1st extent quantity, secondary quantity, current allocation,
and current utilization sizes are shown in tracks for data sets that are allocated in bytes, kilobytes, or
megabytes on a non-managed volume.
Source and index listings
Appendix A. Listing formats  147

## Page 176

ISPF log listings
Figure 36 on page 148 shows a sample ISPF log listing. The log contains a message for each significant
user action, such as saving edited data, moving members from one data set to another, or submitting a
batch job.
Figure 36. Sample ISPF Log Listing
Member list listings
This topic shows samples of member list listings created by the SAVE command. With this command, you
can create listings for both source and load libraries.
The sample listings show the format used when you do not specify a list ID.
Member list listings for source libraries
Figure 37 on page 149 shows a sample member list listing for a source library. These listings contain
the relative block address of each member, shown in hexadecimal format, and other characteristics when
available.
ISPF Log Listings
148  z/OS: z/OS ISPF User's Guide Vol I

## Page 177

Figure 37. Sample Member List Listing for a Source Library
Member list listings for load libraries
Figure 38 on page 149 shows a sample member list listing for a source library. These listings contain the
size of each load module, shown in hexadecimal format, and other characteristics when available.
Figure 38. Sample Member List Listing for a Load Library
Member List Listings
Appendix A. Listing formats  149

## Page 178

Formats for member list listings
Shown here is the format used by the SAVE command to create a member list listing for a source library.
The members of a source library have formatted records (RECFM≠U).
Table 23. Format of Source Library Member List Listing
Starting Column Length in Characters Description
4 8 Member name
19 6 Relative block address in hexadecimal format
25 2 Version number
28 2 Modification level
31 8 Creation date
40 8 Date last modified
49 5 Time last modified
55 5 Current number of lines
61 5 Initial number of lines
67 5 Number of modified lines
73 7 User ID
Shown here is the format used by the SAVE command to create a member list listing for a load library. The
members of a load library have unformatted records (RECFM=U).
Table 24. Format of Load Library Member List Listing
Starting Column Length in Characters Description
4 8 Member name
24 6 Load module size in hexadecimal format
33 6 Load module relative block address in hexadecimal
format
40 8 Alias
49 2 Authorization code
53 3 Addressing mode
56 3 Residency mode
61 18 Load module attributes
Data set list listings
The sample listing in Figure 39 on page 151 shows the format used when you do not specify a data set list
ID.
Data Set List Listings
150  z/OS: z/OS ISPF User's Guide Vol I

## Page 179

Figure 39. Sample Data Set List Listing
Format for data set list listings
Table 25 on page 151 shows the format of the data set list written by the SAVE command when a data set
list ID is specified.
Table 25. Format of Data Set List Listing
Starting Column Length in Characters Description
1 44 Data set name
46 7 Volume and volume indicator
53 4 Data set organization
58 5 Data set record format
64 5 Data set logical record length
70 5 Data set block size
76 6 Data set size in tracks
83 3 Percentage of used tracks or pages (PDSE)
87 3 Number of extents used
91 8 Device type
100 10 Creation date
111 10 Expiration date
122 10 Last reference date
Data Set List Listings
Appendix A. Listing formats  151

## Page 180

Data Set List Listings
152  z/OS: z/OS ISPF User's Guide Vol I
