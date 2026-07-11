# Source anchors

## Current fn64

- `LIVE_REPO_FACT` Immutable input and structural validation:
  `rust/crates/fn64-core/src/pif_firmware.rs`, `PifFirmware` and
  `PifFirmware::from_owned_bytes`.
- `LIVE_REPO_FACT` Machine owner/lifecycle:
  `rust/crates/fn64-core/src/machine.rs`, `Machine`,
  `Machine::install_pif_firmware`, `Machine::pif_firmware_state`, and
  `Machine::reset`.
- `LIVE_REPO_FACT` Bootstrap observation and all-Unknown SP IMEM replacement:
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`,
  `Machine::stage_cartridge_bootstrap`.
- `LIVE_REPO_FACT` Known-byte gate:
  `rust/crates/fn64-core/src/sp_imem.rs`, `SpImem::read_known_u32_be`.
- `LIVE_REPO_FACT` Host/parser/probe boundary:
  `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs::main` and
  `rust/crates/fn64-inspection/src/boot_probe.rs`,
  `parse_boot_probe_arguments`, `run_boot_probe_with_pif_firmware`, and
  `format_report`.
- `LIVE_REPO_FACT` CLI proof:
  `rust/crates/fn64-inspection/tests/boot_probe_cli.rs`.

## Structural and causal evidence

- `EXTERNAL TECHNICAL EVIDENCE` Official RCP header `rcp.h` documents the
  total 2 KiB PIF physical map, 1,984-byte Boot ROM, and 64-byte PIF RAM:
  <https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm>.
- `INFERENCE` The integrated provenance evidence identifies IPL1 copying an
  IPL2 source-symbol span into SP IMEM, while deliberately leaving its complete
  numeric source range unknown:
  `ops/evidence/sp-imem-bootstrap-provenance-v1/SP_IMEM_PRODUCER.md` and
  `BOOT_CHAIN_CAUSALITY.md`.

No external code or bytes were copied.
