# Source anchors

- Pinned public RCP header: decompals/N64-IPL commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `include/PR/rcp.h`.
- Pinned bounded x105 sequence: the same commit, `src/ipl3.s`.
- SDK-header corroboration:
  `https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm`.

The headers establish RDRAM base `0x03F00000`, DEVICE_ID offset `0x04`,
REF_ROW offset `0x14`, and global aperture offset `0x00080000`. The bounded
source places the zero REF_ROW store immediately after RDRAM_DELAY and before
the `0x02000000 << 6` DEVICE_ID construction. No public source inspected here
defines REF_ROW fields or refresh-engine completion semantics.
