# Synthetic test coverage

All fixtures are generated in tests. No private path, metadata, hash, title,
instruction sequence, bootstrap dump, or ROM byte is embedded in test source.

| Semantic family | Test owner and name | Invariant proved |
| --- | --- | --- |
| Content byte-order normalization and exact staging | `machine_cartridge_bootstrap_normalizes_and_stages_all_source_layouts` | Big-endian, 16-bit byte-swapped, and 32-bit little-endian inputs normalize to identical owned bytes and exact source/destination boundaries without filename policy. |
| Atomic bootstrap rejection | `machine_cartridge_bootstrap_rejects_short_source_without_partial_mutation` | A short source rejects before CPU, RDRAM, SP DMEM, or provenance mutation. |
| Reset knownness and source | `machine_bootstrap_reset_state_lineage_stages_only_zero_and_general_pif_stack_pointer` | GPR zero is known architectural zero, GPR 29 has the exact general PIF reset value and source, and every unrelated PIF-produced GPR remains unknown. |
| Unknown operand rejection | `machine_bootstrap_unknown_gpr_source_rejection_has_no_partial_mutation` | `Machine::step` names the first unknown source and rejects before any GPR, HI, LO, COP0, RDRAM, SP DMEM, control-flow, Count, or knownness mutation. |
| Complete known `SpecialAdd` | `machine_bootstrap_known_special_add_commit_preserves_value_and_known_lineage` | Generated cartridge provenance, exact operands, read-before-write aliasing, zero-register discard, exact result, destination knownness, `pc / next_pc`, and one Count advance. |
| Fetch provenance boundaries | `machine_cartridge_bootstrap_source_provenance_covers_exact_boundaries` | First and last complete staged instructions map to exact cartridge offsets. |
| ROM-derived commit | `machine_cartridge_bootstrap_rom_derived_step_commits_cpu_effect` | Fetch, decode, identity, writeback, `pc / next_pc`, and Count commit through public `Machine::step`. |
| Reset and unavailable PIF state | `machine_cartridge_bootstrap_reset_clears_payload_and_provenance`; `machine_cartridge_bootstrap_instruction_inspection_keeps_pif_reset_unavailable` | Reset clears staged storage/provenance and the proprietary reset source remains explicitly unavailable. |
| Unsupported rollback | `machine_cartridge_bootstrap_unrepresented_frontier_preserves_control_flow` | Unrepresented `Lw` leaves `pc / next_pc` and Count unchanged. |
| Probe checkpoint/frontier | `boot_probe_reports_known_special_add_mutation_and_expected_load_frontier`; `boot_probe_unrepresented_first_instruction_stops_at_boot1_without_mutation` | BOOT-2 reports an exact known destination mutation and a known-base `Lw` frontier; staging alone remains BOOT-1. |
| Probe unknown state | `boot_probe_unknown_reset_state_rejection_is_distinct_and_uncommitted` | Unknown reset state is distinct from an unsupported instruction, attempted and committed counts differ, and rejected control flow and Count remain unchanged. |
| Probe budget and formatting | `boot_probe_fixed_budget_is_explicit_and_deterministic`; `boot_probe_argument_parser_owns_fixed_budget_policy` | Bounded policy, deterministic text, and explicit argument rejection. |
| Probe internal failure policy | `boot_probe_internal_machine_invariant_exit_policy_is_nonzero` | Internal invariant failure is not an expected frontier and exits nonzero. |
| CLI exit policy with generated local files | `boot_probe_cli_generated_local_fixture_reaches_expected_frontier_with_success_exit`; structural and argument failure tests | Expected machine frontier exits zero; structural and usage failures exit nonzero; no window, SDL, or audio path is involved. |
