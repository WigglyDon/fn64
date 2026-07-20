# Source Anchors

- Pinned RCP definitions: `N64-IPL` commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `include/PR/rcp.h`.
- Pinned generated-source relationship: the same commit, `src/ipl3.s`,
  `InitCCValue`, `FindCC`, `TestCCValue`, and `WriteCC`.
- SDK-header corroboration: Ultra64 online manual `header/rcp.htm`.
- Product owner: `rust/crates/fn64-core/src/rdram.rs`.
- CPU store planner/applicator and generated fixture:
  `rust/crates/fn64-core/src/machine.rs`.

The public definitions establish base `0x03F00000`, MODE offset `0x0C`, and
the named flag constants. The pinned source establishes nominal input zero,
manual mode, XOR-by-`0x3F`, bit scattering, and the exact generated request.
None establishes CPU readback, physical application, or response success.
