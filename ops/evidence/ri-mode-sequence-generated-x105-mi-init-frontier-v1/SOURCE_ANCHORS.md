# Source anchors

- Pinned public RCP definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h#L2609-L2637>
  and
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h#L3215-L3259>
  identify MI_INIT_MODE/MI_SET_INIT and RI_MODE's exact address and fields.
- Official SDK-header rendering:
  <https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm>
  independently repeats the RI_MODE R/W field definition.
- Pinned bounded x105 source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L2458-L2525>
  establishes both RI_MODE words, both loop relations, the second BNE's `Ori`
  delay slot, and following MI_INIT_MODE store.
- Pinned build owner:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/Makefile>
  gives X105 its own preprocessor selector rather than the 6101 selector, so
  the 32-iteration conditional is present.
- Current Rust owners: `ri.rs`, `machine.rs`,
  `machine/cartridge_bootstrap.rs`, and `fn64_step_probe.rs`.

No external source tree, authentic instruction word, assembly/disassembly
block, firmware byte, or cartridge bootcode byte is copied into this evidence.
