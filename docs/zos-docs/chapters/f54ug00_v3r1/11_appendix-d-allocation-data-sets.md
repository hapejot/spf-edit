# Appendix D. Allocation data sets

Source file: f54ug00_v3r1.md
Start page: 185
Page span: 185-186

## Page 185

Appendix D. Allocation data sets
ISPF issues ALLOC commands based on the ISPF libraries, data set names, list IDs, options, and
additional input libraries you enter on the Foreground Assembler H and Foreground VS COBOL II Compile
panels. All allocations are done before Assembler H and the VS COBOL II compiler are called.
Because Assembler H and VS COBOL II do not provide a language prompter, ISPF allocates the required
data sets for you. Subsequent topics describe the data sets ISPF allocates when you use Assembler H or
the VS COBOL II compiler.
SYSIN data set
The SYSIN data set is the main input into Assembler H and VS COBOL II. It contains the ISPF libraries or
other partitioned or sequential data sets that you enter on the Foreground Assembler H and VS COBOL II
Compile panels. This data set is used to find the member that contains the program to be assembled or
compiled. For a PDS, the ALLOC command would be:
ALLOC FI(SYSIN) DA('proj.lib.type(mem)')
For a sequential data set, the ALLOC command would be:
ALLOC FI(SYSIN) DA('proj.lib.type')
In both commands, lib is the library in which the member or data set was found.
SYSLIB data set
The SYSLIB data set contains the ISPF library concatenation sequence used to resolve any copy
statements specified in your program. It contains the ISPF libraries or other partitioned or sequential
data sets and the additional input libraries you specify on the Foreground Assembler H and VS COBOL II
Compile panels. For example:
ALLOC FI(SYSLIB) DA('SYS1.MACLIB','proj.lib1.type',...,
       'proj.lib4.type','additional lib1','additional lib2')
SYSPRINT data set
The SYSPRINT data set contains the generated output listing. The entry in the List ID field determines the
destination of the output listing. If you enter a name in the List ID field, the output listing is stored in a
sequential data set:
ALLOC FI(SYSPRINT) DA('prefix.listid.LIST')
where listid is the name entered in the List ID field. However, if you leave the List ID field blank, ISPF
uses the name of the member being assembled or compiled instead of the list ID:
ALLOC FI(SYSPRINT) DA('prefix.member.LIST')
If you enter an asterisk (*) in the List ID field, ISPF displays the output listing at your terminal, using this
command:
ALLOC FI(SYSPRINT) DA(*)
See the information about list data sets in the Foreground (Option 4) topic in the z/OS ISPF User's Guide
Vol II for more information.
SYSIN data set
© Copyright IBM Corp. 1980, 2024 157

## Page 186

SYSTERM data set
The SYSTERM data set contains a summary of the information in the listing data set (SYSPRINT). It is
displayed at the terminal if the TERM option is used:
ALLOC FI(SYSTERM) DA(*)
SYSLIN data set
The SYSLIN data set must be preallocated before running Foreground (option 4) or Batch (option 5). The
SYSLIN data set contains the object module. This object module will be the input when you link-edit. For a
PDS, the ALLOC command would be:
ALLOC FI(SYSLIN) DA('proj.lib1.OBJ(mem)')
For a sequential data set, the ALLOC command would be:
ALLOC FI(SYSLIN) DA('proj.lib1.OBJ')
SYSPUNCH data set
The SYSPUNCH data set is the same as the SYSLIN data set. ISPF does not use this data set. The DUMMY
parameter on the ALLOC statement means it should not be used:
ALLOC FI(SYSPUNCH) DUMMY
SYSUT1 data set
The SYSUT1 data set is a temporary utility data set used during processing. It is deleted after it is used.
For Assembler H, the format is:
ALLOC FI(SYSUT1) UNIT(SYSDA) NEW DELETE
For VS COBOL II, the format is:
ALLOC FI(SYSUT1) UNIT(SYSDA) NEW DELETE SPACE(1,1) CYLINDER
SYSUT2 to SYSUT7 data sets
The SYSUT2, SYSUT3, …, SYSUT7 data sets are temporary utility data sets used by VS COBOL II only
during processing. They are deleted after they are used:
ALLOC FI(SYSUT2) UNIT(SYSDA) NEW DELETE SPACE(1,1) CYLINDER
ALLOC FI(SYSUT3) UNIT(SYSDA) NEW DELETE SPACE(1,1) CYLINDER
ALLOC FI(SYSUT4) UNIT(SYSDA) NEW DELETE SPACE(1,1) CYLINDER
ALLOC FI(SYSUT5) UNIT(SYSDA) NEW DELETE SPACE(1,1) CYLINDER
ALLOC FI(SYSUT6) UNIT(SYSDA) NEW DELETE SPACE(1,1) CYLINDER
ALLOC FI(SYSUT7) UNIT(SYSDA) NEW DELETE SPACE(1,1) CYLINDER
Note: SYSUT6 and SYSUT7 are required only if VS COBOL II Version 1, Release 3 is being used.
SYSTERM, SYSLIN, SYSPUNCH, and SYSUT1 Data Sets
158  z/OS: z/OS ISPF User's Guide Vol I
