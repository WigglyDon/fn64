# Source anchors

- Pinned public RCP header:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h>
  establishes MI base `0x04300000`, MI_VERSION offset `0x04`, and the four
  byte-field positions.
- Pinned public x105 source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>
  reads MI_VERSION, compares with `0x01010101`, and selects the source-labelled
  RCP 2.0 path on inequality.
- SDK header rendering:
  <https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm>
  corroborates address and field layout.
- Direct-hardware test source at pinned commit:
  <https://github.com/meauxdal/N64-Revision-Test/blob/8a8f919c228601a0764a63b74bc0519c820b646d/main.c>
  reads `0xA4300004`.
- Direct-hardware result record at the same commit:
  <https://github.com/meauxdal/N64-Revision-Test/blob/8a8f919c228601a0764a63b74bc0519c820b646d/readme.md>
  records the accepted standard-retail word `0x02020102`.

Generated words are taken from the repository's public synthetic x105 fixture,
not from private PIF or ROM inputs.
