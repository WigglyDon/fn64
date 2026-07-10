# Source anchors

Candidate anchors at the commit containing this evidence:

- Host path and bounded stepping owner:
  `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`, `main`;
  `rust/crates/fn64-inspection/src/boot_probe.rs`,
  `parse_boot_probe_arguments` and `run_boot_probe`.
- Machine cartridge owner: `rust/crates/fn64-core/src/machine.rs`, `Machine`
  near line 1965 and `Machine::from_cartridge` immediately below it.
- Bootstrap creation point:
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`,
  `Machine::stage_cartridge_bootstrap` near line 271.
- Bootstrap knownness and state:
  `MachineBootstrapGprSource` near line 38,
  `MachineCartridgeBootstrapState` near line 57,
  `Machine::require_known_bootstrap_gpr_sources` near line 341, and
  `Machine::record_known_bootstrap_gpr_destination` near line 369.
- Read-only fetch/decode/identity/source inspection:
  `Machine::inspect_current_cpu_instruction` near line 398.
- Public execution entrance: `rust/crates/fn64-core/src/machine.rs`,
  `Machine::step` near line 2006 and the existing classified action producer
  near line 2186; this remains the sole execution route.
- Probe formatting and frontier policy:
  `rust/crates/fn64-inspection/src/boot_probe.rs`, `run_boot_probe` near line
  241 and `format_report` near line 647.
- Core synthetic proofs:
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`, tests beginning
  near line 514; repair-specific knownness proofs are near lines 781-861.
- Inspection synthetic proofs:
  `rust/crates/fn64-inspection/src/boot_probe.rs`, tests beginning near line
  906, and `rust/crates/fn64-inspection/tests/boot_probe_cli.rs`.

Line numbers are orientation aids; symbols are the stable anchors.
