# Machine ownership

- `LIVE_REPO_FACT` `Machine` privately owns `Option<PifFirmware>`.
- `LIVE_REPO_FACT` `PifFirmware` privately owns a boxed byte slice. No public
  byte accessor or mutable firmware surface exists.
- `LIVE_REPO_FACT` `Machine::from_cartridge` creates explicit absence.
- `LIVE_REPO_FACT` `Machine::install_pif_firmware` validates a complete local
  candidate before replacing the optional immutable owner.
- `LIVE_REPO_FACT` `Machine::pif_firmware_state` exposes only absent/accepted,
  classification, and byte count for the current inspection consumer.
- `LIVE_REPO_FACT` Accepted bytes are preserved byte-exactly. Mutating a
  caller-owned sibling buffer after transfer cannot alter Machine state.
- `LIVE_REPO_FACT` Reset preserves firmware like cartridge input while
  replacing CPU, RDRAM, SP DMEM, SP IMEM, reservations, and bootstrap state.
- `LIVE_REPO_FACT` Validation, classification, lifecycle, and bootstrap
  observation are core-owned. The host owns none of them.

No path, open handle, host error, borrowed buffer, global, or publicly mutable
byte vector enters `Machine`.
