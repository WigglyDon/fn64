# Source anchors

- Pinned public RI definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h#L3215-L3259>
  identifies RI base `0x04700000`, RI_CURRENT_LOAD at `+0x08` as write-only,
  says any write updates current control, and places RI_SELECT at `+0x0C`.
- Pinned bounded source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L2430-L2468>
  establishes the preceding RI_CONFIG write and wait, r0 store to
  RI_CURRENT_LOAD, following `0x14` construction, and RI_SELECT store order.
- Current Rust owners: `ri.rs`, `machine.rs`,
  `machine/cartridge_bootstrap.rs`, `cpu/address.rs`, and
  `fn64_step_probe.rs`.

External code, instruction words, assembly, disassembly, firmware, cartridge
bytes, or source trees are not copied into this evidence.
