# Source Anchors

- Profile names, CLI tokens, and exact copy layouts:
  `rust/crates/fn64-core/src/pif_firmware.rs`, commit
  `0a0954874853bd46d0a2f15cb025d6893ba44f63`.
- Machine-owned install boundary, bootstrap creation event, atomic replacement,
  and lifecycle proof: `rust/crates/fn64-core/src/machine.rs` and
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`, commit
  `0a0954874853bd46d0a2f15cb025d6893ba44f63`.
- Per-byte SP IMEM source provenance:
  `rust/crates/fn64-core/src/sp_imem.rs`, commit
  `0a0954874853bd46d0a2f15cb025d6893ba44f63`.
- Explicit no-search CLI and no-window observations:
  `rust/crates/fn64-inspection/src/boot_probe.rs` and
  `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`, commit
  `641a9a26763f1904b7d74eb3d481be6448fb2c54`.
- Durable packet transport owner: `AGENTS.md`, section `Project packet
  transport`, commit `725ff7b2811f886d844858d9fa1fd96359505fb7`.
  This is a workflow rule only; it changes no emulator behavior. No second
  repository owner was created.
- Accepted source-mapping evidence:
  `ops/evidence/pif-ipl2-source-mapping-v1/SOURCE_MAPPING.tsv` and
  `ops/evidence/pif-ipl2-source-mapping-v1/MATERIALIZATION_VS_EXECUTION.md`,
  integrated dependency `2ee4b3c7a19b86da4653dd62d574dc46c64668dc`.
