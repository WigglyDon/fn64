# Source Anchors

Primary architecture evidence:

- Nintendo Ultra64 RSP Programmer's Guide: scalar register, local PC, scalar
  COP0 transfer, IMEM/DMEM, and concurrent CPU/RSP semantics.
- pinned N64-IPL `src/ipl3.s` at revision
  `928f59089c18a95cbffa59938a18fa6032c5d78c`: public x105 SP control and
  first RSP identity sequence.
- pinned N64-IPL `include/PR/rcp.h` at the same revision: SP register indices,
  status command bits, address mask, and semaphore contract.

Product anchors:

- `rust/crates/fn64-core/src/rsp.rs`: scalar availability, fetch/decode
  boundary, MFC0 planning, result provenance, and LQV frontier identity.
- `rust/crates/fn64-core/src/sp.rs`: singular SP control/current-PC owner,
  nested RSP state, run-start lineage, semaphore, and DRAM-address source.
- `rust/crates/fn64-core/src/machine.rs`: processor selection, RSP fetch,
  CPU-only cadence, atomic MFC0 application, and public generated composition.
- `rust/crates/fn64-core/src/sp_imem.rs`: word knownness and byte provenance.
- `rust/crates/fn64-inspection/src/bin/fn64_step_probe.rs`: public generated
  stable MFC0/LQV proof markers.

