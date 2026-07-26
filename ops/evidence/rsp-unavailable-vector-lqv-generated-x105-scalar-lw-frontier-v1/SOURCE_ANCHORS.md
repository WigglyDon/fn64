# Source Anchors

Primary architecture evidence:

- Nintendo Ultra64 RSP Programmer's Guide:
  <https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf>.
  It defines 32 128-bit vector registers, architectural byte elements,
  DMEM-only vector loads, low-12-bit base addressing, the signed scaled LQV
  offset, aligned full-register LQV behavior, and vector-load interlocking.
- pinned public x105 source, N64-IPL `src/ipl3.s` revision
  `928f59089c18a95cbffa59938a18fa6032c5d78c`:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>.
  It directly establishes the public LQV word and following scalar-LW word.

Product anchors:

- `rust/crates/fn64-core/src/sp_dmem.rs`: singular backing, byte knowledge,
  and provenance ownership.
- `rust/crates/fn64-core/src/rsp.rs`: vector-slot state, LQV decode/planning,
  result provenance, and scalar-LW frontier.
- `rust/crates/fn64-core/src/sp.rs`: nested RSP ownership and singular current
  PC.
- `rust/crates/fn64-core/src/machine.rs`: one-processor step application and
  public generated composition.
- `rust/crates/fn64-inspection/src/bin/fn64_step_probe.rs`: stable no-window
  public-step markers.
