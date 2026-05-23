# Chapter 10. Extended code page support

Source file: f54dg00_v3r1.md
Start page: 327
Page span: 327-338

## Page 327

Chapter 10. Extended code page support
EXTENDED CODE PAGE support allows panels, messages, and variable application data to be displayed
correctly on terminals using any of the supported code pages. For example, a German panel can
be displayed on a French Country Extended Code Page (CECP) terminal, with all common characters
displayed correctly. Any characters in the panel that do not exist in the terminal code page are displayed
as periods (.).
ISPF supports the EXTENDED CODE PAGES listed in “Supported CCSIDs” on page 303. CCSID stands for
Coded Character Set IDentifier. The CCSID is a short identifier, representing a code page and character set
combination. An extended CCSID has the same code page as its base CCSID, but has a larger character
set.
Translating common characters
ISPF translates common characters from EXTENDED CODE PAGES to the code page of the terminal for
panel )BODY, )MODEL, and )AREA text, if the panel is tagged with a CCSID, and for the long and short
message text if the message member is tagged with a CCSID.
The TRANS service is provided to allow the application to translate variable application data from one
CCSID to another CCSID (see z/OS ISPF Services Guide).
In a panel tagged with a CCSID, all characters that are not )BODY, )MODEL, and )AREA text and all
characters in variable names within the )BODY, )MODEL, and )AREA text of a tagged panel and within the
message text of a tagged message member must be in the syntactic character set:
• A-Z
• a-z
• 0-9
• + < = > % & * " '
• ( ) , _ - . / : ; ?
Note: Lowercase a-z can be used for any CCSID supported by ISPF except the Japanese (Katakana)
Extended CCSID 930.
If an EXTENDED CODE PAGE is specified and the terminal code page and character set is one of those
recognized by ISPF, all displayable code points are available for display (no displayable code points are
invalidated by ISPF).
If an EXTENDED CODE PAGE is not indicated in a panel or message member, a base character set and
code page is assumed based on the terminal type specified in option 0 (see z/OS ISPF User's Guide Vol II).
Z variables
These Z variables are available for code page processing:
ZTERMCP
Terminal code page. Returned as a 4-digit decimal number (4 characters).
ZTERMCS
Terminal character set. Returned as a 4-digit decimal number (4 characters).
ZTERMCID
Terminal CCSID. Returned as a 5-digit decimal number (5 characters).
ZERRCSID
Contains the 5-digit decimal CCSID of a dialog error message, or blanks if the error message is not
tagged with a CCSID. Returned as a 5-digit decimal number (5 characters).
© Copyright IBM Corp. 1980, 2025 299

## Page 328

If an extended code page is specified for a panel or message and the terminal code page cannot be
determined, there is no transformation of characters.
Table 28 on page 300 illustrates when characters will be transformed for Extended Code Page support
and when they will not be transformed:
Table 28. Character transformation table
  Terminal Query
Reply CP/CS
Valid for ISPF
Terminal Query
Reply CP/CS
Not Returned
Terminal Query
Reply CP/CS
Invalid for ISPF
CCSID Tag Present Characters
transformed
Characters not
transformed
Characters not
transformed
No CCSID Tag Present Characters not
transformed
Characters not
transformed
Characters not
transformed
For DBCS languages, the beginning and ending inhibited character tables are enhanced to include
characters from the extended code pages for the text formatting of messages and panels.
Panels tagged with CCSID
Panels can be defined with a )CCSID section and the NUMBER(xxxxx) keyword where xxxxx is the CCSID
of the extended code page as defined by Character Data Representation Architecture. The )CCSID section
must be the first section in the panel. See “Defining the CCSID section” on page 175.
Messages tagged with CCSID
An ISPF message can be defined with .CCSID=xxxxx. See “Messages tagged with CCSID” on page 270.
GETMSG service
The GETMSG service can be called with a CCSID parameter. If the message is tagged with a CCSID, the
CCSID will be returned; otherwise, blanks will be returned.
TRANS service
Users can call the TRANS Service in ISPF to translate variable data specified by the user from one CCSID
to another CCSID. The to and from CCSIDs are also specified by the user in the TRANS call (see z/OS ISPF
Services Guide). For a list of the EXTENDED CODE PAGE translate tables provided by ISPF, see “Extended
code page translate tables provided by ISPF” on page 308.
ISPccsid translate load modules
The ISPccsid translate load modules provide ISPF with the information needed to translate data from
one CCSID to another. There is one ISPccsid translate load module for each of the supported CCSIDs.
The name (or alias for those ISPccsid modules provided by ISPF) of each CCSID translate load module
is made up of the 5-digit CCSID, prefixed with ISP. For example, load module ISP00111 supports
translation of the CCSID 00111. Each CCSID translate load module must contain two translate tables.
The required translate tables permit data to be translated between the respective CCSID and CCSID
00500. Additionally, each CCSID load module can contain up to 256 pairs of optional direct translate
tables. ISPF will use direct translate tables when available. Otherwise, ISPF translates through CCSID
00500. Translating through CCSID 00500 can result in valid characters being lost. This is due to CCSID
00500 not having all possible code points defined.
300  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 329

ISPccsid translate load module generation macro
An assembler macro that permits the user to generate customized ISPccsid translate load modules is
supplied with ISPF. The macro also allows the user to add direct translate tables to the ISPccsid translate
load modules ISPF supplies with the product.
Only the values for the hex digits X'40' through X'FE' are defined in a given translate table. These are the
only code points that vary from CCSID to CCSID.
The assembler macro is:
ISPCCSID  CCSID=nnnnn,TO=to-address,FROM=from-address
ISPCCSID macro
The initial ISPCCSID macro usage identifies the CCSID associated with the particular ISPccsid translate
load module and provides addresses of the to and from CCSID 00500 translate tables.
Subsequent usage of the ISPCCSID macro in a particular ISPccsid translate load module generation
identifies the CCSID and translate table addresses of optional direct to and from translate tables.
Description of parameters
nnnnn
Required parameter. The nnnnn value is a 5-digit decimal (5 characters) number that specifies
a CCSID number. The nnnnn value on the first or only ISPCCSID macro definition is the CCSID
associated with the ISPccsid translate load module. The nnnnn value on other than the first ISPCCSID
macro definition is the CCSID associated with direct to and from translate tables. Assembly errors will
occur if this parameter is not 5 digits.
to-address
Required parameter. On the first or only ISPCCSID macro definition, this parameter specifies the
address of the translate table that converts data from the CCSID associated with the respective
ISPccsid translate load module to CCSID 00500. On subsequent ISPCCSID macro definitions within
the same ISPccsid translate load module, it specifies the address of the translate table that converts
data from the CCSID associated with the respective ISPccsid translate load module to the CCSID
specified on this ISPCCSID macro definition.
from-address
Required parameter. On the first or only ISPCCSID macro definition, this parameter specifies the
address of the translate table that converts data from CCSID 00500 to the CCSID associated with
the respective ISPccsid translate load module. On subsequent ISPCCSID macro definitions within the
same ISPccsid translate load module, it specifies the address of the translate table that converts
data from the CCSID specified on this ISPCCSID macro definition to the CCSID associate with the
respective ISPccsid translate load module.
ISPccsid translate load module definition examples
Each ISPccsid translate load module must be compiled separately using Assembler H (or functional
equivalent). Figure 76 on page 301 shows an example of a basic translate model, and Figure 77 on page
302 shows an example of a translate model with two direct CCSID entries.
          ISPCCSID CCSID=00111,TO=TRTO500,FROM=TRFR500
*
*
TRTO500   DC    XL191'...            00111 TO 00500
TRFR500   DC    XL191'...            00111 FROM 00500 (00500 TO 00111)
          END
Figure 76. Basic ISP00111 translate module
Chapter 10. Extended code page support  301

## Page 330

ISPCCSID CCSID=00222,TO=TRTO500,FROM=TRFR500
          ISPCCSID CCSID=00333,TO=TRT00333,FROM=TRF00333
          ISPCCSID CCSID=00444,TO=TRT00444,FROM=TRF00444
*
*
TRTO500   DC    XL191'...            00222 TO 00500
TRFR500   DC    XL191'...            00222 FROM 00500 (00500 TO 00222)
*
*
TRT00333  DC    XL191'...            00222 TO 00333
Figure 77. ISP00222 translate module with two direct CCSID entries
KANA and NOKANA keywords
If a CCSID is specified, the KANA (panels and messages) and NOKANA (messages) keywords are ignored
by ISPF. Panels and messages that specify the Japanese (Katakana) Extended CCSID (CCSID=00930) are
handled as follows regardless of whether KANA or NOKANA (for messages) keywords are specified:
• If the terminal code page is the base Katakana code page, all characters in the panel )BODY, )MODEL,
or )AREA text or short and long message text, except lowercase English characters, are left as is.
Because the base Katakana code page does not support lowercase English characters, all lowercase
English characters are translated to uppercase English characters. All other parts of the panel or
message must be in the syntactic character set, excluding characters a-z.
• If the terminal code page is non-Katakana, all lowercase English characters in the )BODY, )MODEL,
or )AREA text or short and long message text in a panel or message that has been tagged with the
extended Katakana code page (CCSID=05026) are translated to the equivalent lowercase English
characters in the terminal code page for display. All Katakana characters are displayed as periods
(.). For example, the lowercase a, which is X'62' in the extended Katakana code page, is translated to
X'81' (lowercase a) in the U.S. English code page. The Katakana character which is X'81' is translated
to a period (X'4B') in the U.S. English code page. All other parts of the panel or message must be in the
syntactic character set, excluding characters a-z.
Character translation
Table 29 on page 302 illustrates the character translation from the extended Katakana code page and
from the extended Japanese (Latin) code page (if CCSID=00930 or CCSID=00939 is specified in a panel,
message, or in the TRANS service) to the U.S. English (CECP and base) code page, to the extended and
base Katakana, and to the Japanese (Latin) Extended code pages for code points X'81', X'62' and X'59'.
Table 29. Character translation from extended katakana code page
Destination Code Page Source
CCSID=00930
Translation Source
CCSID=00939
Translation
Base Katakana
(base code page)
X'81'
X'62'
X'81'
X'C1'
X'81'
X'59'
X'C1'
X'81'
Extended Katakana
(CCSID=00930)
X'81'
X'62'
X'81'
X'62'
X'81'
X'59'
X'62'
X'81'
U.S. English CECP
     and Non-CECP
Japanese (Latin)
     Non-Extended
X'81'
X'62'
X'4B'
X'81'
X'81'
X'59'
X'81'
X'4B'
Japanese (Latin) Extended
(CCSID=00939)
X'81'
X'62'
X'59'
X'81'
X'81'
X'59'
X'81'
X'59'
302  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 331

Code Points
Character Translation
X'81'
A Katakana character in the Katakana code pages and is lowercase a in the U.S. English (CECP and
base) and Japanese (Latin) (Extended and base) code pages.
X'62'
Lowercase a in the extended Katakana (CCSID=00930) code page, a Katakana character in the
extended Japanese (Latin) (CCSID=00939) code page, and is an unknown character in the U.S.
English, base Japanese (Latin), and base Katakana code pages.
X'59'
A Katakana character in the Japanese (Latin) Extended (CCSID=00939) code page, and an unknown
character in the other code pages.
X'C1'
Uppercase A and X'4B' is a period (.) in all of the previously mentioned code pages.
Supported CCSIDs
The CCSIDs listed in Table 30 on page 303 are supported for panels and messages that specify an
EXTENDED CODE PAGE and for the TRANS service.
Table 30. Extended CCSID1 Supported
CCSID Character Set Code Page Country/Language
00037 697 37 U.S.A.
Canada
Netherlands
Portugal
Brazil
Australia
New Zealand
00273 697 273 Austria
Germany
00277 697 277 Denmark
Norway
00278 697 278 Finland
Sweden
00280 697 280 Italy
00284 697 284 Spain
L.A. Spanish
00285 697 285 United Kingdom
00297 697 297 France
00420 235 420 Arabic
00424 941 424 Hebrew
00500 697 500 Switzerland
Belgium
00838 1176 838 Thailand
00870 959 870 Latin-2
00871 697 871 Iceland
Chapter 10. Extended code page support  303

## Page 332

Table 30. Extended CCSID1 Supported (continued)
CCSID Character Set Code Page Country/Language
00875 923 875 Greece
00880 960 880 Cyrillic
01025 1150 1025 Cyrillic
01026 1126 1026 Turkey
01047 697 1047 Latin1
01123 1326 1123 Ukraine
Table 31. Extended CCSID1 Supported (EURO)
CCSID Character Set Code Page Country/Language
00924 1353 0924 Latin9
01140 695 1140 U.S.A.
Canada
Netherlands
Portugal
Brazil
Australia
New Zealand
01141 695 1141 Austria
Germany
01142 695 1142 Denmark
Norway
01143 695 1143 Finland
Sweden
01144 695 1144 Italy
01145 695 1145 Spain
L.A. Spanish
01146 695 1146 United Kingdom
01147 695 1147 France
01148 695 1148 Switzerland
Belgium
01149 695 1149 Iceland
01153 1375 1153 Latin2
01154 1381 1154 Cyrillic
01155 1378 1155 Turkey
01158 1388 1158 Ukraine
01160 1395 1160 Thailand
04899 1356 0803 Hebrew
04971 1371 0875 Greece
12712 1357 0424 Hebrew
304  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 333

Table 31. Extended CCSID1 Supported (EURO) (continued)
CCSID Character Set Code Page Country/Language
16804 1461 0420 Arabic
The extended CCSIDs shown in Table 31 on page 304 and Table 32 on page 305 are supported for the
TRANS service, and also with the use of the CCSID keyword in panels and messages. These are the mixed
SBCS/DBCS CCSIDs for these languages.
Japanese (Katakana) and Simplified Chinese EXTENDED CODE PAGES are not supported on any terminal,
but these CCSIDs are supported by ISPF for the TRANS service and for tagging panels and messages.
Note: Although these CCSIDs represent both SBCS and DBCS character sets and code pages, only the
SBCS character set and code page are involved in the EXTENDED CODE PAGE support in ISPF.
Table 32. Extended SBCS and DBCS CCSIDs Supported
CCSID Character Set Code Page Country
00930 1172 290 Japanese (Katakana)
00939 1172 1027 Japanese (Latin)
00933 1173 833 Korean
00935 1174 836 Simplified Chinese
00937 1175 037 Traditional Chinese
01159 65535 1159 Traditional Chinese
01364 65535 0834 Korean
01371 65535 0835 Traditional Chinese
01388 65535 0837 Simplified Chinese
01390 65535 0300 Japanese
01399 65535 0300 Japanese
05123 65535 1027 Japanese
08482 65535 0290 Japanese
Base code pages for terminals
Translation to base character sets and code pages is supported for panels, messages, and the TRANS
service. See “Base CCSIDs” on page 307.
Direct translation between each base code page and its EXTENDED CODE PAGE is provided. Also, direct
translation between both base and extended Japanese (Katakana) and both base and extended Japanese
(Latin or English) is provided. All translation between the single-byte EXTENDED CODE PAGES for the
double-byte languages and the CECP code pages is through CCSID 00500.
Adding translate tables for extended code page support
You can add code pages to be used for messages and panels that specify code page and for the TRANS
service by creating these translate tables using the sample assembler module ISPEXCP as an example.
(ISPEXCP is provided in the SYS1.SAMPLIB library in the MVS environment.) The tables to translate
between the new code page and CCSID 00500 are needed to reduce the number of translate tables
necessary to translate characters between the new code page and any other supported (or added)
code page. For example, to translate characters from a panel with CCSID=xxxxx to a terminal with
Chapter 10. Extended code page support  305

## Page 334

CCSID=yyyyy, the characters in the panel are first translated to CCSID 00500 and then from CCSID 00500
to CCSID yyyyy for display on the terminal.
Note: The translate tables for the CCSIDs listed in Table 30 on page 303 and Table 32 on page 305 are
provided and included with ISPF. Also, see “Extended code page translate tables provided by ISPF” on
page 308.
Any translate tables that are added must be named ISPnnnnn, where nnnnn is the CCSID. The translate
tables should include code points X'40' through X'FE'.
• This example illustrates the translation to CCSID 00500 from CCSID xxxxx, where xxxxx is the CCSID for
the new code page. This CCSID must be different from any of the supported CCSIDs previously listed,
and should be a CCSID defined in the Character Data Representation Architecture. In Figure 78 on page
306, xxxxx is 00037. 
  Table      Hexadecimal Code               Position
  --------   ------------------------     -------------
  TO_500     DC X'4041424344454647'       (X'40' to X'47')
             DC X'4849B04B4C4D4EBB'       (X'48' to X'4F')
             DC X'5051525354555657'       (X'50' to X'57')
             DC X'58594F5B5C5D5EBA'       (X'58' to X'5F')
                       . . .
             DC X'78797A7B7C7D7E7F'       (X'78' to X'7F')
             DC X'8081828384858687'       (X'80' to X'87')
                       . . .
             DC X'E8E9EAEBECEDEEEF'       (X'E8' to X'EF')
             DC X'F0F1F2F3F4F5F6F7'       (X'F0' to X'F7')
             DC X'F8F9FAFBFCFDFE'         (X'F8' to X'FE')
 
Figure 78. Translation to CCSID 00500 from CCSID XXXXX
• Figure 79 on page 306 illustrates the translation to CCSID xxxxx from CCSID 00500, where xxxxx is
the CCSID for the new code page. This CCSID must be different from any of the supported CCSIDs
previously listed, and should be a CCSID defined in the Character Data Representation Architecture. In
this example, xxxxx is 00037. 
  Table      Hexadecimal Code               Position
  --------   ------------------------     -------------
  FROM_500   DC X'4041424344454647'       (X'40' to X'47')
             DC X'4849BA4B4C4D4E5A'       (X'48' to X'4F')
             DC X'5051525354555657'       (X'50' to X'57')
             DC X'5859BB5B5C5D5EB0'       (X'58' to X'5F')
                       . . .
             DC X'78797A7B7C7D7E7F'       (X'78' to X'7F')
             DC X'8081828384858687'       (X'80' to X'87')
                       . . .
             DC X'E8E9EAEBECEDEEEF'       (X'E8' to X'EF')
             DC X'F0F1F2F3F4F5F6F7'       (X'F0' to X'F7')
             DC X'F8F9FAFBFCFDFE'         (X'F8' to X'FE')
 
Figure 79. Translation to CCSID XXXXX from CCSID 00500
• Optionally, any number of pairs of to and from tables can be provided for direct translation from the new
CCSID to and from another CCSID.
The assembler macro, ISPCCSID, is supplied with ISPF to allow you to generate custom ISPxxxxx
translate load modules (where xxxxx is the new CCSID). Calls to this macro must also be coded for
the To_500 and From_500 tables and any to and from tables for direct translation. The load module must
either have the name ISPxxxxx (where xxxxx is the new CCSID) or an alias of ISPxxxxx. See “ISPccsid
translate load modules” on page 300, “ISPccsid translate load module generation macro” on page 301,
and “ISPCCSID macro” on page 301.
306  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 335

Note: New translate tables can still be added based on terminal type as described in z/OS ISPF Planning
and Customizing for untagged messages and panels.
Direct to and from translate tables can be added for direct translation (to prevent possible loss of
characters through CCSID 00500 for character sets other than 697). Additional direct translation tables
can also be added to the extended code page translate tables provided by ISPF. The direct translation
CCSID must be one of the CCSIDs supported by ISPF, or added by the user. If the CCSID of the terminal
is the same as the CCSID in any of the direct translation tables, those tables are used. Otherwise, the
To_500 and From_500 tables are used to translate through CCSID 00500.
Note: Both to and from translate tables must be provided for direct translation tables as well as CCSID
00500 tables, even though there may be no translation needed. For example, to translate from a base
CCSID to an extended CCSID for the same code page, all characters will translate to themselves.
Base CCSIDs
The CCSIDs for the BASE CODE PAGES supported by ISPF (that include mixed SBCS/DBCS CCSIDs for the
DBCS languages) are listed in Table 33 on page 307.
Table 33. Base CCSIDs Supported
CCSID Character Set Code Page Country/Language
00803 1147 424 Hebrew (Old)
00931 101 037 Japan (English)
04369 265 273 Germany and Austria
04371 273 275 Brazil
04373 281 277 Denmark and Norway
04374 285 278 Finland and Sweden
04376 293 280 Italy
04380 309 284 L.A. (Spanish Speaking)
04381 313 285 U.K. English
04393 1129 297 France
04934 938 838 Thailand
04966 959 870 Latin-2
04976 960 880 Cyrillic
05029 933 833 Korean
05031 936 836 Simplified Chinese
05033 101 037 Traditional Chinese
08229 101 037 U.S. English and Netherlands
08476 650 284 Spain
09122 332 290 Japan (Katakana)
41460 904 500 Switzerland
45556 908 500 Switzerland
Note: Although the CCSIDs for the DBCS languages (Japanese, Korean, and Chinese) represent both SBCS
and DBCS character sets and code pages, only the SBCS character set and code page are involved in the
EXTENDED CODE PAGE support in ISPF.
Chapter 10. Extended code page support  307

## Page 336

Extended code page translate tables provided by ISPF
The translate tables provided by ISPF that can be updated by the user are as follows:
• ISPSTC1 (CCSID=00037 / 01140 U.S.A., Canada, Netherlands, Portugal, Brazil, Australia, New Zealand)
• ISPSTC2 (CCSID=00273 / 01141 Austria and Germany)
• ISPSTC3 (CCSID=00277 / 01142 Denmark and Norway)
• ISPSTC4 (CCSID=00278 / 01143 Finland and Sweden)
• ISPSTC5 (CCSID=00280 / 01144 Italy)
• ISPSTC6 (CCSID=00284 / 01145 Spain and Spanish-Speaking)
• ISPSTC7 (CCSID=00285 / 01146 United Kingdom)
• ISPSTC8 (CCSID=00297 / 01147 France)
• ISPSTC9 (CCSID=00500 / 01148 Switzerland and Belgium)
• ISPSTC10 (CCSID=00939 Japan (Latin))
• ISPSTC11 (CCSID=00930 Japan (Katakana))
• ISPSTC12 (CCSID=00933 Korea)
• ISPSTC13 (CCSID=00935 Simplified Chinese)
• ISPSTC14 (CCSID=00937 Traditional Chinese)
• ISPSTC15 (CCSID=00870 Latin-2)
• ISPSTC16 (CCSID=00880 Cyrillic)
• ISPSTC17 (CCSID=01025 Cyrillic)
• ISPSTC18 (CCSID=00420 Arabic)
• ISPSTC19 (CCSID=00424 Hebrew)
• ISPSTC20 (CCSID=00838 Thai)
• ISPSTC21 (CCSID=00871 / 1149 Iceland)
• ISPSTC22 (CCSID=00875 Greek)
• ISPSTC23 (CCSID=01026 Turkish).
The source for the previous modules is provided in the SYS1.SAMPLIB library in the MVS environment.
Example of user-modifiable ISPF translate table
The module shown is for CCSID 00037 (ISPSTC1). The existing tables can be modified, or more pairs
of direct translation tables can be added. To add direct translation tables, add a new ISPCCSID macro
call for the new direct translate tables, and add the new tables. Rename the assembler program to
ISPTTCx(x), where x(x) is the last 1- or 2-digit number of the ISPSTCx(x) name. For example, ISPSTC1
should be renamed ISPTTC1, and ISPSTC14 renamed ISPTTC14.
*  THE FOLLOWING MACROS WILL GENERATE THE CCSID 00037 MODULE.
*
*
         ISPCCSID CCSID=00037,TO=TTC1T5H,FROM=TTC1F5H
         ISPCCSID CCSID=08229,TO=TTC1TB1,FROM=TTC1FB2
         ISPCCSID CCSID=04371,TO=TTC1TB2,FROM=TTC1FB2
*
*    TTC1T5H - CCSID 00037 TO CCSID 00500 Table
*
TTC1T5H  DS   0XL191
         DC X'4041424344454647'          (X'40' TO X'47')
         DC X'4849B04B4C4D4EBB'          (X'48' TO X'4F')
         DC X'5051525354555657'          (X'50' TO X'57')
         DC X'58594F5B5C5D5EBA'          (X'58' TO X'5F')
         DC X'6061626364656667'          (X'60' TO X'67')
         DC X'68696A6B6C6D6E6F'          (X'68' TO X'6F')
         DC X'7071727374757677'          (X'70' TO X'77')
         DC X'78797A7B7C7D7E7F'          (X'78' TO X'7F')
         DC X'8081828384858687'          (X'80' TO X'87')
308  z/OS: z/OS ISPF Dialog Developer's Guide and Reference

## Page 337

DC X'88898A8B8C8D8E8F'          (X'88' TO X'8F')
         DC X'9091929394959697'          (X'90' TO X'97')
         DC X'98999A9B9C9D9E9F'          (X'98' TO X'9F')
         DC X'A0A1A2A3A4A5A6A7'          (X'A0' TO X'A7')
         DC X'A8A9AAABACADAEAF'          (X'A8' TO X'AF')
         DC X'5FB1B2B3B4B5B6B7'          (X'B0' TO X'B7')
         DC X'B8B94A5ABCBDBEBF'          (X'B8' TO X'BF')
         DC X'C0C1C2C3C4C5C6C7'          (X'C0' TO X'C7')
         DC X'C8C9CACBCCCDCECF'          (X'C8' TO X'CF')
         DC X'D0D1D2D3D4D5D6D7'          (X'D0' TO X'D7')
         DC X'D8D9DADBDCDDDEDF'          (X'D8' TO X'DF')
         DC X'E0E1E2E3E4E5E6E7'          (X'E0' TO X'E7')
         DC X'E8E9EAEBECEDEEEF'          (X'E8' TO X'EF')
         DC X'F0F1F2F3F4F5F6F7'          (X'F0' TO X'F7')
         DC X'F8F9FAFBFCFDFE'            (X'F8' TO X'FE')
*
*    TTC1F5H - CCSID 00037 FROM CCSID 00500 Table
*
TTC1F5H  DS   0XL191
         DC X'4041424344454647'          (X'40' TO X'47')
         DC X'4849BA4B4C4D4E5A'          (X'48' TO X'4F')
         DC X'5051525354555657'          (X'50' TO X'57')
         DC X'5859BB5B5C5D5EB0'          (X'58' TO X'5F')
         DC X'6061626364656667'          (X'60' TO X'67')
         DC X'68696A6B6C6D6E6F'          (X'68' TO X'6F')
         DC X'7071727374757677'          (X'70' TO X'77')
         DC X'78797A7B7C7D7E7F'          (X'78' TO X'7F')
         DC X'8081828384858687'          (X'80' TO X'87')
         DC X'88898A8B8C8D8E8F'          (X'88' TO X'8F')
         DC X'9091929394959697'          (X'90' TO X'97')
         DC X'98999A9B9C9D9E9F'          (X'98' TO X'9F')
         DC X'A0A1A2A3A4A5A6A7'          (X'A0' TO X'A7')
         DC X'A8A9AAABACADAEAF'          (X'A8' TO X'AF')
         DC X'4AB1B2B3B4B5B6B7'          (X'B0' TO X'B7')
         DC X'B8B95F4FBCBDBEBF'          (X'B8' TO X'BF')
         DC X'C0C1C2C3C4C5C6C7'          (X'C0' TO X'C7')
         DC X'C8C9CACBCCCDCECF'          (X'C8' TO X'CF')
         DC X'D0D1D2D3D4D5D6D7'          (X'D0' TO X'D7')
         DC X'D8D9DADBDCDDDEDF'          (X'D8' TO X'DF')
         DC X'E0E1E2E3E4E5E6E7'          (X'E0' TO X'E7')
         DC X'E8E9EAEBECEDEEEF'          (X'E8' TO X'EF')
         DC X'F0F1F2F3F4F5F6F7'          (X'F0' TO X'F7')
         DC X'F8F9FAFBFCFDFE'            (X'F8' TO X'FE')
*
*    TTC1TB1 - CCSID 00037 TO CCSID 08229 Table
*
TTC1TB1  DS   0XL191
         DC X'404B4B4B4B4B4B4B'          (X'40' TO X'47')
         DC X'4B4B4A4B4C4D4E4F'          (X'48' TO X'4F')
         DC X'504B4B4B4B4B4B4B'          (X'50' TO X'57')
         DC X'4B4B5A5B5C5D5E5F'          (X'58' TO X'5F')
         DC X'60614B4B4B4B4B4B'          (X'60' TO X'67')
         DC X'4B4B6A6B6C6D6E6F'          (X'68' TO X'6F')
         DC X'4B4B4B4B4B4B4B4B'          (X'70' TO X'77')
         DC X'4B797A7B7C7D7E7F'          (X'78' TO X'7F')
         DC X'4B81828384858687'          (X'80' TO X'87')
         DC X'88894B4B4B4B4B4B'          (X'88' TO X'8F')
         DC X'4B91929394959697'          (X'90' TO X'97')
         DC X'98994B4B4B4B4B4B'          (X'98' TO X'9F')
         DC X'4BA1A2A3A4A5A6A7'          (X'A0' TO X'A7')
         DC X'A8A94B4B4B4B4B4B'          (X'A8' TO X'AF')
         DC X'4B4B4B4B4B4B4B4B'          (X'B0' TO X'B7')
         DC X'4B4B4B4B4B4B4B4B'          (X'B8' TO X'BF')
         DC X'C0C1C2C3C4C5C6C7'          (X'C0' TO X'C7')
         DC X'C8C94B4B4B4B4B4B'          (X'C8' TO X'CF')
         DC X'D0D1D2D3D4D5D6D7'          (X'D0' TO X'D7')
         DC X'D8D94B4B4B4B4B4B'          (X'D8' TO X'DF')
         DC X'E04BE2E3E4E5E6E7'          (X'E0' TO X'E7')
         DC X'E8E94B4B4B4B4B4B'          (X'E8' TO X'EF')
         DC X'F0F1F2F3F4F5F6F7'          (X'F0' TO X'F7')
         DC X'F8F94B4B4B4B4B'            (X'F8' TO X'FE')
*
*    TTC1FB1 - CCSID 00037 FROM CCSID 08229 Table
*
TTC1FB1  DS   0XL191
         DC X'4041424344454647'          (X'40' TO X'47')
         DC X'48494A4B4C4D4E4F'          (X'48' TO X'4F')
         DC X'5051525354555657'          (X'50' TO X'57')
         DC X'58595A5B5C5D5E5F'          (X'58' TO X'5F')
         DC X'6061626364656667'          (X'60' TO X'67')
         DC X'68696A6B6C6D6E6F'          (X'68' TO X'6F')
         DC X'7071727374757677'          (X'70' TO X'77')
Chapter 10. Extended code page support  309

## Page 338

DC X'78797A7B7C7D7E7F'          (X'78' TO X'7F')
         DC X'8081828384858687'          (X'80' TO X'87')
         DC X'88898A8B8C8D8E8F'          (X'88' TO X'8F')
         DC X'9091929394959697'          (X'90' TO X'97')
         DC X'98999A9B9C9D9E9F'          (X'98' TO X'9F')
         DC X'A0A1A2A3A4A5A6A7'          (X'A0' TO X'A7')
         DC X'A8A9AAABACADAEAF'          (X'A8' TO X'AF')
         DC X'B0B1B2B3B4B5B6B7'          (X'B0' TO X'B7')
         DC X'B8B9BABBBCBDBEBF'          (X'B8' TO X'BF')
         DC X'C0C1C2C3C4C5C6C7'          (X'C0' TO X'C7')
         DC X'C8C9CACBCCCDCECF'          (X'C8' TO X'CF')
         DC X'D0D1D2D3D4D5D6D7'          (X'D0' TO X'D7')
         DC X'D8D9DADBDCDDDEDF'          (X'D8' TO X'DF')
         DC X'E0E1E2E3E4E5E6E7'          (X'E0' TO X'E7')
         DC X'E8E9EAEBECEDEEEF'          (X'E8' TO X'EF')
         DC X'F0F1F2F3F4F5F6F7'          (X'F0' TO X'F7')
         DC X'F8F9FAFBFCFDFE'            (X'F8' TO X'FE')
*
*    TTC1TB2 - CCSID 00037 TO CCSID 04371 Table
*
TTC1TB2  DS   0XL191
         DC X'404B4B4B4B4B794B'          (X'40' TO X'47')
         DC X'4B4B4B4B4C4D4E4B'          (X'48' TO X'4F')
         DC X'50D04B4B4B4B4B4B'          (X'50' TO X'57')
         DC X'4B4B4F5A5C5D5E4B'          (X'58' TO X'5F')
         DC X'60614B4B4B4B7C4B'          (X'60' TO X'67')
         DC X'5B4B4B6B6C6D6E6F'          (X'68' TO X'6F')
         DC X'4B4A4B4B4B4B4B4B'          (X'70' TO X'77')
         DC X'4B4B7A4B4B7D7E7F'          (X'78' TO X'7F')
         DC X'4B81828384858687'          (X'80' TO X'87')
         DC X'88894B4B4B4B4B4B'          (X'88' TO X'8F')
         DC X'4B91929394959697'          (X'90' TO X'97')
         DC X'98994B4B4B4B4B4B'          (X'98' TO X'9F')
         DC X'4BA1A2A3A4A5A6A7'          (X'A0' TO X'A7')
         DC X'A8A94B4B4B4B4B4B'          (X'A8' TO X'AF')
         DC X'5F44B4BB4B4B4B4B'          (X'B0' TO X'B7')
         DC X'4B4B4B4B4B4B4B4B'          (X'B8' TO X'BF')
         DC X'4BC1C2C3C4C5C6C7'          (X'C0' TO X'C7')
         DC X'C8C94B4B4B4B4BC0'          (X'C8' TO X'CF')
         DC X'4BD1D2D3D4D5D6D7'          (X'D0' TO X'D7')
         DC X'D8D94B4B4B4B4B4B'          (X'D8' TO X'DF')
         DC X'E04BE2E3E4E5E6E7'          (X'E0' TO X'E7')
         DC X'E8E94B4B4B4B4B7B'          (X'E8' TO X'EF')
         DC X'F0F1F2F3F4F5F6F7'          (X'F0' TO X'F7')
         DC X'F8F94B4B4B4B4B'            (X'F8' TO X'FE')
*
*    TTC1FB2 - CCSID 00037 FROM CCSID 04371 Table
*
TTC1FB2  DS   0XL191
         DC X'4041424344454647'          (X'40' TO X'47')
         DC X'4849714B4C4D4E5A'          (X'48' TO X'4F')
         DC X'5051525354555657'          (X'50' TO X'57')
         DC X'58595B685C5D5EB0'          (X'58' TO X'5F')
         DC X'6061626364656667'          (X'60' TO X'67')
         DC X'6869486B6C6D6E6F'          (X'68' TO X'6F')
         DC X'7071727374757677'          (X'70' TO X'77')
         DC X'78467AEF667D7E7F'          (X'78' TO X'7F')
         DC X'8081828384858687'          (X'80' TO X'87')
         DC X'88898A8B8C8D8E8F'          (X'88' TO X'8F')
         DC X'9091929394959697'          (X'90' TO X'97')
         DC X'98999A9B9C9D9E9F'          (X'98' TO X'9F')
         DC X'A0A1A2A3A4A5A6A7'          (X'A0' TO X'A7')
         DC X'A8A9AAABACADAEAF'          (X'A8' TO X'AF')
         DC X'B0B1B2B3B4B5B6B7'          (X'B0' TO X'B7')
         DC X'B8B9BABBBCBDBEBF'          (X'B8' TO X'BF')
         DC X'CFC1C2C3C4C5C6C7'          (X'C0' TO X'C7')
         DC X'C8C9CACBCCCDCECF'          (X'C8' TO X'CF')
         DC X'51D1D2D3D4D5D6D7'          (X'D0' TO X'D7')
         DC X'D8D9DADBDCDDDEDF'          (X'D8' TO X'DF')
         DC X'E0E1E2E3E4E5E6E7'          (X'E0' TO X'E7')
         DC X'E8E9EAEBECEDEEEF'          (X'E8' TO X'EF')
         DC X'F0F1F2F3F4F5F6F7'          (X'F0' TO X'F7')
         DC X'F8F9FAFBFCFDFE'            (X'F8' TO X'FE')
         END
310  z/OS: z/OS ISPF Dialog Developer's Guide and Reference
