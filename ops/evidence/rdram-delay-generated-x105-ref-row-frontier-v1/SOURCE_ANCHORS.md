# Source anchors

- Pinned public RCP definitions: decompals/N64-IPL commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `include/PR/rcp.h`.
- Pinned bounded sequence: the same commit, `src/ipl3.s`.
- SDK-header corroboration: `https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm`.

The header establishes RDRAM base `0x03F00000`, DELAY offset `0x08`, REF_ROW
offset `0x14`, global aperture offset `0x00080000`, and MI set-init `0x0100`.
The bounded source orders `MI_SET_INIT | 15`, rotated RDRAM_DELAY, then global
RDRAM_REF_ROW, and describes global writes as broadcast. No private input was
used.

