# Source Anchors

- `LIVE_REPO_FACT`: semantic profile names and exact Machine-owned copy layouts:
  `PifIpl2Profile::{name,copy_layout}` in
  `rust/crates/fn64-core/src/pif_firmware.rs`.
- `LIVE_REPO_FACT`: host-owned CLI token conversion and usage policy:
  `parse_pif_ipl2_profile`, `parse_boot_probe_arguments`, and
  `BootProbeArgumentError` in
  `rust/crates/fn64-inspection/src/boot_probe.rs`, repair commit
  `4eaa33d9fc59182d8e69a24edb39ee3be9ff8797`.
- `LIVE_REPO_FACT`: independent Machine firmware/profile storage and installation:
  `Machine::{install_pif_firmware,install_pif_ipl2_profile}` in
  `rust/crates/fn64-core/src/machine.rs`, repair commit
  `2ca21994bba489bc5ef645ee91547479fec8d070`.
- `LIVE_REPO_FACT`: materialization planner, missing-firmware pre-mutation result, atomic
  replacement, reset/restaging, stale-tail, and independent-Machine proof:
  `Machine::stage_cartridge_bootstrap` and embedded tests in
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`, repair commit
  `2ca21994bba489bc5ef645ee91547479fec8d070`.
- `LIVE_REPO_FACT`: per-byte SP IMEM source provenance:
  `SpImem::from_pif_ipl2_copy` and
  `SpImemByteProvenance::UserSuppliedPifFirmware` in
  `rust/crates/fn64-core/src/sp_imem.rs`, commit
  `0a0954874853bd46d0a2f15cb025d6893ba44f63`.
- `LIVE_REPO_FACT`: host literal-path read and no-window observations:
  `run_boot_probe_with_pif_firmware` in
  `rust/crates/fn64-inspection/src/boot_probe.rs`; literal path read in
  `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`; repair commit
  `2ca21994bba489bc5ef645ee91547479fec8d070`.
- `USER_DECISION` + `LIVE_REPO_FACT`: durable packet transport owner:
  `AGENTS.md`, section `Project packet
  transport`, commit `725ff7b2811f886d844858d9fa1fd96359505fb7`.
  This is a workflow rule only; it changes no emulator behavior. No second
  repository owner was created.
- `EXTERNAL_TECHNICAL_EVIDENCE`: accepted source-mapping basis:
  `ops/evidence/pif-ipl2-source-mapping-v1/SOURCE_MAPPING.tsv` and
  `ops/evidence/pif-ipl2-source-mapping-v1/MATERIALIZATION_VS_EXECUTION.md`,
  integrated dependency `2ee4b3c7a19b86da4653dd62d574dc46c64668dc`.
