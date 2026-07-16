# Validation

Candidate proof at correction commit `0b681a1`:

- focused core filters: `rdram` 102; `rdram_device_id` / `device_id` 4 each;
  `rdram_ref_row` / `ref_row` 4 each; `rdram_delay` 6;
  `mi_init_transfer` 1; `mi_init_mode` 5; `ri_mode` 7; `ri_select` 10;
  `ri_config` 7; `ri_current_load` 7;
- execution filters: `load_word` 11; `store_word` 12; `address` 46;
  `adel` 13; `ades` 14; `delay_slot` 24; `machine_step` 12;
  `bootstrap` 38; `cold_x105` 9; `generated_x105` 1; `mi_version` 1;
- `cargo fmt --all -- --check`: passed;
- `cargo clippy --all-targets -- -D warnings`: passed;
- complete core: 482 passed;
- inspection library: 16 passed;
- CLI integration: 11 passed;
- `fn64_machine_probe`: `no-window: ok`, `result: ok`;
- `fn64_step_probe`: 150 stable cases, including generated DEVICE_ID commit,
  MI_VERSION setup, and MI_VERSION rejection; `no-window: ok`, `result: ok`;
- `rust/verify-forward`: `forward gate: ok`;
- Context-SHA: `f12c598b2eb67086ee3e3bc1d57c56e247eee503651dac77f3cb12018ef74f9c`;
- context verification: 15 checks, 0 errors;
- fleet verification: 52 passed;
- integration queue: `ok`.

Every Cargo invocation used packet-owned target/TMPDIR paths. Clean-checkout,
canonical replay, push, and final cleanup results are sealed in the external
Master artifact so this committed evidence does not require a post-integration
rewrite. Repository `rust/target` remains absent.
