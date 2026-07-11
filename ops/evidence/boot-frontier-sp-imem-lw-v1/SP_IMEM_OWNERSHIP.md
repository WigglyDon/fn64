# SP IMEM ownership

- `LIVE_REPO_FACT` The sole storage owner is private
  `fn64_core::sp_imem::SpImem` in
  `rust/crates/fn64-core/src/sp_imem.rs`.
- `LIVE_REPO_FACT` Capacity is exactly `0x1000` bytes. The Nintendo 64
  Programming Manual describes the RSP instruction-memory bank as 4 KiB:
  <https://ultra64.ca/files/documentation/online-manuals/man-v5-2/allman52/pro-man/pro03/03-03.htm>.
- `LIVE_REPO_FACT` `Machine::from_cartridge` constructs one default SP IMEM;
  `Machine::reset` replaces it with one default SP IMEM; and
  `Machine::stage_cartridge_bootstrap` replaces it before publishing the new
  bootstrap state.
- `LIVE_REPO_FACT` Construction and reset create concrete zero backing bytes
  with `Unknown` provenance. They do not create known architectural data.
- `LIVE_REPO_FACT` Byte observation and test staging are `cfg(test)` and
  crate-private. Production exposes no public mutable SP IMEM and no public raw
  SP IMEM read.
- `LIVE_REPO_FACT` The storage enforces byte bounds and aligned four-byte word
  bounds. Word reads use N64 big-endian order.
- `LIVE_REPO_FACT` SP registers, status, DMA, RSP execution, mirroring, MMIO,
  device registries, and a generic bus remain absent.
