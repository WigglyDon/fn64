# Validation

Status: focused and complete candidate-worktree validation passed.

Focused `fn64-core` filter results:

- control_flow 22; jal 6; jalr 2; jump 1; link 5; provenance 9;
- bootstrap 41; pif 26; delay_slot 27; machine_step 12;
- generated_x105 1; init_cc 1; current_control 1; rdram_mode 1;
- first_responder 4; rdram_device_id 4; mi_version 8; address 46;
- store_word 12; load_word 11; ri_mode 7; ri_select 10; ri_config 7;
  ri_current_load 7.

Complete worktree results:

- `cargo fmt --all -- --check`: pass;
- `cargo clippy --workspace --all-targets -- -D warnings`: pass;
- core tests: 496 passed;
- inspection-library tests: 16 passed;
- CLI integration tests: 11 passed;
- no-window machine probe: 2 state cases, `result: ok`;
- no-window step probe: 155 cases, `result: ok`;
- `./rust/verify-forward`: `forward gate: ok`;
- context verifier: 15 checks, 0 errors;
- fleet tests: 52 passed;
- integration queue: ok.

No private PIF, proprietary BIOS, or commercial ROM input was used. Exact-SHA
clean-checkout, patch-reproduction, canonical, push, and archive results are
release records sealed outside the repository after the final commit exists.
