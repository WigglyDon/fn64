# Source anchors

- Pinned public register definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h#L786-L814>
  identifies RI base `0x04700000`, RI_SELECT at `+0x0C` as R/W, RI_CONFIG at
  `+0x04`, and neighboring registers.
- Pinned bounded source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L106-L134>
  establishes the RI_SELECT read, cold/NMI branch relation, cold stack save,
  RI address prefix, and RI_CONFIG write order. A later source relation writes
  `0x14` to RI_SELECT.
- Official reset note:
  <https://ultra64.ca/files/documentation/online-manuals/man/developerNews/news-05.html>
  distinguishes power-cycle cold reset from NMI and states that NMI resets the
  CPU without resetting the RCP.
- Current Rust owners: `machine.rs`, `machine/cartridge_bootstrap.rs`,
  `cpu/address.rs`, `sp_imem.rs`, and `fn64_step_probe.rs`.

No external source tree, instruction word, assembly block, disassembly block,
firmware byte, or cartridge bootcode byte is copied into this evidence.
