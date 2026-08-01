# Source anchors

| Path | Symbol | Ownership purpose |
| --- | --- | --- |
| `rust/crates/fn64-core/src/pif_firmware.rs` | `PifFirmware` | Owns accepted immutable firmware bytes and explicit versus public-synthetic classification. |
| `rust/crates/fn64-core/src/pif_firmware.rs` | `MachinePifFirmwareState` | Represents absent or accepted material without inventing bytes. |
| `rust/crates/fn64-core/src/machine.rs` | `Machine::install_pif_firmware` | Accepts owned explicit bytes; no path enters Machine. |
| `rust/crates/fn64-core/src/machine.rs` | `Machine::install_public_synthetic_cold_x105_bootstrap` | Selects generated proof firmware only by an explicit synthetic call. |
| `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs` | `Machine::stage_cartridge_bootstrap` | Composes accepted firmware/profile/handoff inputs atomically. |
| `rust/crates/fn64-core/src/sp_imem.rs` | `MachineSpImemByteProvenance` | Owns copied-byte knownness and explicit-versus-synthetic provenance. |
| `rust/crates/fn64-inspection/src/bin/fn64_user_cartridge_probe.rs` | `read_explicit_pif_firmware` | Owns the optional host path read and fixed redacted failure identity. |
| `rust/crates/fn64-inspection/src/bin/fn64_user_cartridge_probe.rs` | `stage_explicit_pif_cold_x105_bootstrap` | Passes explicit bytes and represented handoff selectors to public core APIs; never falls back. |
| `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs` | `run` | Retains the existing optional explicit PIF host boundary with redacted read failure. |
| `rust/crates/fn64-inspection/tests/user_cartridge_probe_cli.rs` | generated CLI tests | Proves missing/explicit/malformed material, redaction, no search, and no fallback. |
