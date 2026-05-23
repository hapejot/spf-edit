# Chapter 10. IBM products (option 9)

Source file: f54u200_v3r1.md
Start page: 437
Page span: 437-438

## Page 437

Chapter 10. IBM products (option 9)
Option 9 provides an interface to other IBM program development products. It displays a panel that lists
other IBM products that are supported as ISPF dialogs, as shown in Figure 240 on page 399.
   Menu  Utilities  Compilers  Options  Status  Help
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                Additional IBM Program Development Products                  │
 │ Option ===>                                                                 │
 │                                                                             │
 │ 3  INFOMAN   Tivoli Information Management for z/OS                         │
 │ 4  COBOL/SF  COBOL Structuring Facility                                     │
 │ 6  SDF II    Screen Definition Facility II - Editors and Utilities          │
 │ 7  SDF II-P  Screen Definition Facility II - Prototype                      │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │                                                                             │
 │  F1=Help        F2=Split       F3=Exit        F7=Backward    F8=Forward     │
 │  F9=Swap       F10=Actions    F12=Cancel                                    │
 ⋘─────────────────────────────────────────────────────────────────────────────┘
      Enter X to Terminate using Log/List defaults
  F1=Help      F2=Split     F3=Exit      F7=Backward  F8=Forward   F9=Swap
 F10=Actions  F12=Cancel
Figure 240. Additional IBM Program Development Products Panel (ISRDIIS)
When you select one of these products, ISPF tries to call it. However, the only way ISPF can determine
whether a product is installed and available is to check for the existence of a single product-related panel
in the panel library concatenation. No other check is made to ensure that the product is correctly installed
or that it is completely available to you.
If the product is not installed or is unavailable, ISPF displays an informational panel that describes the
product and shows how to obtain more information.
The names of the products on this panel are point-and-shoot fields. For more information on point-and-
shoot fields, see the ISPF User Interface topic in the z/OS ISPF User's Guide Vol I.
IBM products (option 9)
© Copyright IBM Corp. 1980, 2024 399

## Page 438

IBM products (option 9)
400  z/OS: z/OS ISPF User's Guide Vol II
