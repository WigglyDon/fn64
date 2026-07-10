# Source anchors

Candidate anchors at the commit containing this evidence:

- Host path and bounded stepping owner:
  `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`, `main`;
  `rust/crates/fn64-inspection/src/boot_probe.rs`,
  `parse_boot_probe_arguments` and `run_boot_probe`.
- Machine cartridge owner: `rust/crates/fn64-core/src/machine.rs`, `Machine`
  near line 1914 and `Machine::from_cartridge` immediately below it.
- Bootstrap creation point:
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`,
  `Machine::stage_cartridge_bootstrap` near line 189.
- Bootstrap state and source provenance:
  `MachineCartridgeBootstrapState` near line 34 and
  `MachineCpuInstructionSource` near line 144 in the same file.
- Read-only fetch/decode/identity/source inspection:
  `Machine::inspect_current_cpu_instruction` near line 249.
- Public execution entrance: `rust/crates/fn64-core/src/machine.rs`,
  `Machine::step` near line 1955; its existing producer/application spine
  remains the sole execution route.
- Probe formatting and frontier policy:
  `rust/crates/fn64-inspection/src/boot_probe.rs`, `format_report` near line 489.
- Core synthetic proofs:
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`, tests beginning
  near line 375.
- Inspection synthetic proofs:
  `rust/crates/fn64-inspection/src/boot_probe.rs`, tests beginning near line
  682, and `rust/crates/fn64-inspection/tests/boot_probe_cli.rs`.

Line numbers are orientation aids; symbols are the stable anchors.
