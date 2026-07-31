# Source anchors

- Nintendo Ultra64 RSP Programmer's Guide:
  <https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf>
  defines Break as halting the RSP, setting `SP_STATUS_BROKE`, conditionally
  signaling `MI_INTR_SP` when interrupt-on-break is set, and raising no
  architectural exception.
- Pinned RCP definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h>
  identify status bit 6 as interrupt-on-break and the separate CPU status
  command bits that clear and set it.
- Pinned public x105 source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>
  clears interrupt-on-break at run start, does not set it again in the bounded
  path, and ends the reached RSP sequence with zero-code `break`.

No external source is copied into this evidence set. The guide does not define
a post-Break PC value; that bounded functional decision is recorded separately.

