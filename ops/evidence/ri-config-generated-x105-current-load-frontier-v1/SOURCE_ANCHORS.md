# Source anchors

- Pinned public RI definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h#L3215-L3259>
  identifies RI base `0x04700000`, RI_CONFIG at `+0x04`, its defined fields,
  and RI_CURRENT_LOAD at `+0x08` as a distinct write transition.
- Pinned bounded source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L2397-L2458>
  establishes the generated operands, `0x40` purpose, x105 wait count 8,000,
  four-identity loop shape, and following RI_CURRENT_LOAD write.
- Current Rust owners: `ri.rs`, `machine.rs`,
  `machine/cartridge_bootstrap.rs`, `cpu/address.rs`, and
  `fn64_step_probe.rs`.

External code, instruction words, assembly, disassembly, firmware, cartridge
bytes, or source trees are not copied into this evidence.
