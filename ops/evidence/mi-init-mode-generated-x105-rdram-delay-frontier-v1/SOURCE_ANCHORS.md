# Source anchors

Pinned public register definition:

- decompals/N64-IPL commit `928f59089c18a95cbffa59938a18fa6032c5d78c`,
  `include/PR/rcp.h`;
- `MI_BASE_REG=0x04300000`, `MI_INIT_MODE_REG=MI_BASE_REG+0x00`;
- writes use bits 6:0 for initialization length, bit 7 to clear initialization
  mode, bit 8 to set initialization mode, bits 9/10 to clear/set EBUS test
  mode, bit 11 to clear DP interrupt, and bits 12/13 to clear/set RDRAM
  register mode;
- read state uses bits 6:0 for length and distinct state positions for
  initialization, EBUS-test, and RDRAM-register modes.

Pinned bounded sequence:

- the same commit, `src/ipl3.s`;
- the source constructs `MI_SET_INIT | 15`, writes MI_INIT_MODE, constructs
  the rotated RDRAM delay word, and next writes RDRAM_DELAY through the global
  configuration aperture.

SDK-header corroboration: the public Ultra64 `rcp.h` rendering identifies the
same MI command and state masks.

Only addresses, field definitions, operand relationships, bounded values, and
instruction order are used. No external source tree, instruction block,
firmware bytes, or cartridge bytes are copied.
