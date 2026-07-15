# Source anchors

- Pinned public RI definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h#L3215-L3259>
  identifies RI base `0x04700000`, RI_MODE at `+0x00`, RI_CONFIG at `+0x04`,
  RI_CURRENT_LOAD at `+0x08`, and R/W RI_SELECT at `+0x0C`; its duplicated
  RI_SELECT `[2:0]` description is ambiguous.
- Pinned bounded source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L2458-L2468>
  establishes RI_CURRENT_LOAD then `Ori` of `0x10 | 4`, the `0x14` RI_SELECT
  write described as enabling TX/RX select, and the immediately following zero
  store to RI_MODE.
- Official SDK-header rendering:
  <https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm>
  independently repeats the same ambiguous RI_SELECT field wording.
- Nintendo N64 system patent inspected for stronger primary context:
  <https://patents.google.com/patent/US6331146B1/en>
  describes RDRAM transmit/receive clocks but supplies no RI_SELECT register
  field map and therefore grants no additional product authority.
- Current Rust owners: `ri.rs`, `machine.rs`,
  `machine/cartridge_bootstrap.rs`, and `fn64_step_probe.rs`.

No external source tree, authentic instruction word, assembly/disassembly
block, firmware byte, or cartridge bootcode byte is copied into this evidence.
