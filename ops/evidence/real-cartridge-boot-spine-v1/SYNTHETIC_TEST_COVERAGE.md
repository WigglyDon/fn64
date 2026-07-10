# Synthetic test coverage

All fixtures are generated in tests. No private path, metadata, hash, title,
instruction sequence, bootstrap dump, or ROM byte is embedded in test source.

| Semantic family | Test owner and name | Invariant proved |
| --- | --- | --- |
| Content byte-order normalization and exact staging | `machine_cartridge_bootstrap_normalizes_and_stages_all_source_layouts` | Big-endian, 16-bit byte-swapped, and 32-bit little-endian inputs normalize to identical owned bytes and exact source/destination boundaries without filename policy. |
| Atomic bootstrap rejection | `machine_cartridge_bootstrap_rejects_short_source_without_partial_mutation` | A short source rejects before CPU, RDRAM, SP DMEM, or provenance mutation. |
| Fetch provenance boundaries | `machine_cartridge_bootstrap_source_provenance_covers_exact_boundaries` | First and last complete staged instructions map to exact cartridge offsets. |
| ROM-derived commit | `machine_cartridge_bootstrap_rom_derived_step_commits_cpu_effect` | Fetch, decode, identity, writeback, `pc / next_pc`, and Count commit through public `Machine::step`. |
| Reset and unavailable PIF state | `machine_cartridge_bootstrap_reset_clears_payload_and_provenance`; `machine_cartridge_bootstrap_instruction_inspection_keeps_pif_reset_unavailable` | Reset clears staged storage/provenance and the proprietary reset source remains explicitly unavailable. |
| Unsupported rollback | `machine_cartridge_bootstrap_unrepresented_frontier_preserves_control_flow` | Unrepresented `Lw` leaves `pc / next_pc` and Count unchanged. |
| Probe checkpoint/frontier | `boot_probe_reports_rom_derived_commit_and_expected_frontier`; `boot_probe_unrepresented_first_instruction_stops_at_boot1_without_mutation` | BOOT-2 requires an actual cartridge-derived commit; staging alone remains BOOT-1. |
| Probe budget and formatting | `boot_probe_fixed_budget_is_explicit_and_deterministic`; `boot_probe_argument_parser_owns_fixed_budget_policy` | Bounded policy, deterministic text, and explicit argument rejection. |
| CLI exit policy with generated local files | `boot_probe_cli_generated_local_fixture_reaches_expected_frontier_with_success_exit`; structural and argument failure tests | Expected machine frontier exits zero; structural and usage failures exit nonzero; no window, SDL, or audio path is involved. |
