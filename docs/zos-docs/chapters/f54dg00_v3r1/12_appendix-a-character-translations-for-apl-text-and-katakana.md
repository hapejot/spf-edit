# Appendix A. Character translations for APL, TEXT, and Katakana

Source file: f54dg00_v3r1.md
Start page: 339
Page span: 339-342

## Page 339

Appendix A. Character translations for APL, TEXT,
and Katakana
This topic contains the character translation tables for APL, TEXT, and Katakana. This information does
not include Extended Code Page Support. See Chapter 10, “Extended code page support,” on page 299.
ISPF permits use of all keyboards for all models of 3270 and 3290 terminals, and text keyboards for 3278
and 3279 terminals. The 2-byte transmission codes for APL and text characters are translated by ISPF
into 1-byte codes for internal storage as shown in Figure 80 on page 312 and Figure 81 on page 313. ISPF
also permits use of 3277 and 3278 Japanese Katakana terminals. ISPF does not permit the use of 3277
and 3278 Katakana terminals and an APL terminal at the same time.
The character codes are documented in IBM 3270 hardware manuals. Many of the Katakana codes
overlay the lowercase EBCDIC codes. In a panel definition, it is assumed that lowercase EBCDIC
characters are to be displayed for these codes, unless the )BODY header statement includes the keyword
KANA. Example:
)BODY KANA
The keyword, KANA, is used on a )BODY header statement when Katakana characters are included within
the panel. Input and output fields and model line fields are not affected by use of the KANA keyword.
Rules for display of text fields are as follows:
• If the terminal type is Katakana, and
– The KANA keyword is present, text characters are left as is.
– The KANA keyword is not present, any lowercase text characters are translated to uppercase and
uppercase text characters are left as is.
• If the terminal type is not Katakana, and
– The KANA keyword is present, any lowercase text characters are treated as being nondisplayable and
are translated to a period. Any uppercase text characters are left as is.
– The KANA keyword is not present, lowercase and uppercase text characters are left as is.
See “How to define a message” on page 266 for a description of how the KANA keyword provides a similar
function for messages containing lowercase characters that must be displayed on a Katakana terminal.
Note: The KANA keyword is not needed for panels and messages that specify a CCSID for Extended Code
Page Support. See Chapter 10, “Extended code page support,” on page 299.
Character translations
© Copyright IBM Corp. 1980, 2025 311

## Page 340

Figure 80. Internal character representations for APL keyboards
Character translations
312  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 341

Figure 81. Internal character representations for text keyboards
 
Character translations
Appendix A. Character translations for APL, TEXT, and Katakana  313

## Page 342

Character translations
314  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
