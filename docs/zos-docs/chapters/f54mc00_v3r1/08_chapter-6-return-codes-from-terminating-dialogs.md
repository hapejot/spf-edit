# Chapter 6. Return codes from terminating dialogs

Source file: f54mc00_v3r1.md
Start page: 929
Page span: 929-930

## Page 929

Chapter 6. Return codes from terminating dialogs
This topic contains return codes from terminating dialogs. For more information, refer to z/OS ISPF Dialog
Developer's Guide and Reference.
Return Codes from Terminating Dialogs
The return code from ISPSTART for a successful dialog completion is either 0 or a value returned by the
executing dialog in the system variable ZISPFRC. ZISPFRC is a shared-pool input variable of length 8. The
dialog can set ZISPFRC to any value in the range of 0 to 16777215, except the values reserved for ISPF
use (900 through 999, and 9000 through 9100). This value must be left-justified and padded with blanks.
At termination, ISPF copies the value from ZISPFRC and passes it to the invoking application (or Terminal
Monitor Program) in register 15. If the value in ZISPFRC is not within the valid range or is otherwise not
valid, such as a value that is not numeric, ISPF issues an appropriate line message and passes a return
code of 908. If the dialog has not set ZISPFRC to a value, ISPF returns a value of 0.
Note:
1. CLIST procedures that invoke ISPSTART can check the CLIST variable LASTCC for the ISPF return
code. In REXX, check the variable rc after an ISPF function.
2. Even though ISPF restricts the return code value to the range 0 to 16777215, other products or
subsystems, such as JES when processing JCL condition codes, can be more restrictive on return code
values. See documentation for the affected product for more information.
3. ZISPFRC should not be confused with the normal dialog return code set by the function; it has no
effect on ISPF log/list termination processing.
ZISPFRC is intended to be used by applications that invoke a dialog dedicated to a single task or function.
However, it is valid to set ZISPFRC from a selection panel invoked by the ISPSTART command.
ISPF checks for the existence of ZISPFRC only at ISPF termination. If ZISPFRC is set by any dialog other
than the one invoked by the ISPSTART command, ISPF ignores the value.
Return Codes from Termination Dialogs
Error codes that ISPF can return in register 15 to an application are:
908
ZISPFRC value not valid.
920
ISPSTART command syntax not valid.
930
ISPSTART Program not found.
940
ISPSTART Command not found.
950
An ISPF session running on behalf of a z/OS client had to be abnormally terminated.
988
An error occurred initializing IKJSATTN.
990
An error occurred running in batch mode. If ZISPFRC has not been set previously, and ISPF
encounters a severe error that terminates the product, then 990 is set.
997
Uncorrectable TPUT error.
© Copyright IBM Corp. 1980, 2024 909

## Page 930

998
ISPF initialization error. A 998 error code can result from:
• Required ISPF data element library not preallocated
• Error opening ISPF data element library
• ISPF data element library has invalid data set characteristics
• Error loading literals module
• Recursive ISPF call
ISPF issues a line message that indicates which of these errors caused the 998 return code.
999
ISPF environment not valid. A 999 error code can result from:
• TSO/MVS environment not valid
• Unsupported screen size
ISPF issues a line message that indicates which of these errors caused the 999 return code.
When running in batch, ISPF can also return the following return codes:
9008
Abend termination.
9012
Attach error.
9014
Authorized command invocation error, or TSO CMD START exit routine rejected the command.
9016
Command not found, or was otherwise unable to execute, or an exit routine returned an invalid return
code.
9018
Invalid command: LOGOFF, ISPF, etc.
9020
TSO RTN IKJTBLS (called from CAU) abended.
910  z/OS: z/OS ISPF Messages and Codes
