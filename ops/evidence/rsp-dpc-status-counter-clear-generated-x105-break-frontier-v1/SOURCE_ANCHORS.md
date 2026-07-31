# Source anchors

- Nintendo Ultra64 RSP Programmer's Guide:
  <https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf>
  identifies RSP COP0 control index 11 as DPC status, defines its write-one
  command bits, identifies four 24-bit DPC counters, and marks their power-up
  values undefined.
- Pinned RCP definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h>
  define `DPC_CLR_TMEM_CTR` as `0x0040`, `DPC_CLR_PIPE_CTR` as `0x0080`,
  `DPC_CLR_CMD_CTR` as `0x0100`, and `DPC_CLR_CLOCK_CTR` as `0x0200`.
- Pinned public x105 source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>
  provides the reached `0x240` DPC-status command followed by `break`.

No external source is copied into this evidence set. Source silence is not
used to invent reset-zero counter truth.
