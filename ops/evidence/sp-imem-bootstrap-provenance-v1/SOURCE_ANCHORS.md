# Source anchors

## Current fn64 source

- `LIVE_REPO_FACT` Machine construction:
  `rust/crates/fn64-core/src/machine.rs`, `Machine::from_cartridge`.
- `LIVE_REPO_FACT` Machine reset:
  `rust/crates/fn64-core/src/machine.rs`, `Machine::reset`.
- `LIVE_REPO_FACT` Public execution entrance:
  `rust/crates/fn64-core/src/machine.rs`, `Machine::step`.
- `LIVE_REPO_FACT` Aligned load planning and application:
  `rust/crates/fn64-core/src/machine.rs`,
  `Machine::produce_load_word_step_action` and
  `Machine::apply_load_word_step_action`.
- `LIVE_REPO_FACT` Narrow SP IMEM routing:
  `rust/crates/fn64-core/src/machine.rs`,
  `classify_cpu_data_word_target` and
  `translate_cpu_physical_sp_imem_data_word_address`.
- `LIVE_REPO_FACT` Bootstrap creation/lifecycle and GPR lineage:
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`,
  `Machine::stage_cartridge_bootstrap`,
  `Machine::require_known_bootstrap_gpr_sources`, and
  `Machine::record_known_bootstrap_gpr_destination`.
- `LIVE_REPO_FACT` SP DMEM storage and private range write:
  `rust/crates/fn64-core/src/sp_dmem.rs`, `SpDmem` and
  `SpDmem::write_bytes`.
- `LIVE_REPO_FACT` SP IMEM construction, provenance, and read path:
  `rust/crates/fn64-core/src/sp_imem.rs`, `SpImemByteProvenance`,
  `SpImem::read_known_u32_be`, and `SpImem::default`.
- `LIVE_REPO_FACT` Probe stepping and reporting:
  `rust/crates/fn64-inspection/src/boot_probe.rs`, `run_boot_probe`,
  `format_load_word_rejection_frontier`, and `format_report`; host file-read
  owner is `src/bin/fn64_boot_probe.rs::main`.

## Pinned public source

- `WORKER_CLAIM` N64-IPL revision
  `928f59089c18a95cbffa59938a18fa6032c5d78c`:
  `README.md` IPL1/IPL2/IPL3 sections;
  `src/pifrom.s:98-123` (`ipl2_copyloop`, `ipl2_rom`),
  `src/pifrom.s:219-247` (`ipl3_copyloop`),
  `src/pifrom.s:589-668` (`VerifyAndRunIPL3`, `ipl2_rom_end`), and
  `src/ipl3.s:72-105` (`IPL3_X105`, `ipl3_entry`).
- `WORKER_CLAIM` Mupen64Plus revision
  `9eb6a7cbefe663c0a7c527afc705f5dea5197d7c`:
  `src/device/pif/bootrom_hle.c:48` and `:133-149`.
- `WORKER_CLAIM` n64docs revision
  `3719566877985e7749cadff3a8d4490644d80d06`:
  physical memory map and Boot Process / Simulating the PIF ROM sections.
- `WORKER_CLAIM` Nintendo 64 Programming Manual v5.2:
  Chapter 3, section 3.3, Reality Signal Processor.
- `WORKER_CLAIM` n64checksum revision
  `b99bb8a721d442120eb8b61fee90c48f529c355c`:
  `README.md`, Different checksum types.
- `WORKER_CLAIM` CEN64 revision
  `e0641c8452a3ae8edcd2bf4e46794bb4eaafc076`:
  `README.md`, Usage.

Line numbers are pinned-revision anchors. No external source text or executable
bytes are reproduced here.
