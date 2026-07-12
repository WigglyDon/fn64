# Source Anchors

- Profile names, CLI spelling conversion, and exact Machine-owned copy layouts:
  `PifIpl2Profile::{from_cli_name,copy_layout}` in
  `rust/crates/fn64-core/src/pif_firmware.rs`, commit
  `0a0954874853bd46d0a2f15cb025d6893ba44f63`.
- Independent Machine firmware/profile storage and installation:
  `Machine::{install_pif_firmware,install_pif_ipl2_profile}` in
  `rust/crates/fn64-core/src/machine.rs`, repair commit
  `2ca21994bba489bc5ef645ee91547479fec8d070`.
- Materialization planner, missing-firmware pre-mutation result, atomic
  replacement, reset/restaging, stale-tail, and independent-Machine proof:
  `Machine::stage_cartridge_bootstrap` and embedded tests in
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`, repair commit
  `2ca21994bba489bc5ef645ee91547479fec8d070`.
- Per-byte SP IMEM source provenance:
  `SpImem::from_pif_ipl2_copy` and
  `SpImemByteProvenance::UserSuppliedPifFirmware` in
  `rust/crates/fn64-core/src/sp_imem.rs`, commit
  `0a0954874853bd46d0a2f15cb025d6893ba44f63`.
- Host argument parser and no-window observations:
  `parse_boot_probe_arguments` and `run_boot_probe_with_pif_firmware` in
  `rust/crates/fn64-inspection/src/boot_probe.rs`; literal path read in
  `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`; repair commit
  `2ca21994bba489bc5ef645ee91547479fec8d070`.
- Durable packet transport owner: `AGENTS.md`, section `Project packet
  transport`, commit `725ff7b2811f886d844858d9fa1fd96359505fb7`.
  This is a workflow rule only; it changes no emulator behavior. No second
  repository owner was created.
- Accepted source-mapping evidence:
  `ops/evidence/pif-ipl2-source-mapping-v1/SOURCE_MAPPING.tsv` and
  `ops/evidence/pif-ipl2-source-mapping-v1/MATERIALIZATION_VS_EXECUTION.md`,
  integrated dependency `2ee4b3c7a19b86da4653dd62d574dc46c64668dc`.
