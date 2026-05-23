# Appendix B. ISPTTDEF specify translate table set

Source file: f54dg00_v3r1.md
Start page: 343
Page span: 343-344

## Page 343

Appendix B. ISPTTDEF specify translate table set
ISPF provides a program, ISPTTDEF, for specifying the set of terminal translate tables to be used. This
program lets you specify private sets of translate tables.
Note: This program is not used for Extended Code Page Support translate tables. See Chapter 10,
“Extended code page support,” on page 299.
You can invoke ISPTTDEF from a selection panel, as a command, or from a dialog function. The format of
the ISPTTDEF program call is:
SELECT PGM(ISPTTDEF) PARM(xxx)
where xxx is the terminal type or the name of the load module containing translate tables.
Return codes from invoking ISPTTDEF are as follows:
0
Normal completion
4
Translate tables could not be loaded
Valid terminal types are those that can be specified using the ISPF Settings panel. If the name specified is
not a valid terminal type, ISPF attempts to load a module having that name.
Specify Translate Table Set
© Copyright IBM Corp. 1980, 2025 315

## Page 344

Specify Translate Table Set
316  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
