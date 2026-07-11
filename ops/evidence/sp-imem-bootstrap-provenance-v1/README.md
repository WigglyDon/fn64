# SP IMEM bootstrap provenance v1

- `USER_DECISION` This lane identifies the source-clear creation event for the
  SP IMEM bytes consumed by the authentic `Lw` at `0xA4000044` and implements
  behavior only if the complete consumed state is lawful to represent.
- `LIVE_REPO_FACT` Current construction, reset, and cartridge-bootstrap staging
  replace SP IMEM with concrete zero backing whose byte provenance is
  `Unknown`; production has no mutable SP IMEM staging surface.
- `INFERENCE` Pinned public source-level reconstruction and independent
  emulator corroboration establish that IPL1 copies IPL2 firmware code from
  PIF ROM into SP IMEM, IPL2 executes there, and IPL3 begins while that content
  remains resident.
- `INFERENCE` The selected x105-family IPL3 prelude consumes the contiguous SP
  IMEM prefix `[0x000, 0x020)`, rewrites that prefix, and then writes
  `[0x020, 0x02c)`. Offset zero alone is incomplete.
- `UNKNOWN` The exact firmware words are unavailable to fn64 without
  user-supplied proprietary firmware. They are not reset zero, cartridge data,
  an SP DMA result, or a general hardware constant.
- `WORKER_CLAIM` The honest implementation decision is
  `USER_SUPPLIED_FIRMWARE_REQUIRED_FUTURE_LANE`; Machine behavior remains
  unchanged and the lane result is PARTIAL.

No external code, firmware byte, private ROM byte, compatibility selector, or
private input identity is stored here. Exact private-input identity and command
logs belong only to the external Worker artifact.
