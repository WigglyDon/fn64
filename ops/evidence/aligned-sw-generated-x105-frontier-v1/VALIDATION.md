# Validation

The product and inspection candidate through `a33939d` passed:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets -- -D warnings`;
- focused `store_word`, `sw`, `ades`, `sp_imem`, `load_word`, `machine_step`,
  `delay_slot`, `branch_delay_exception`, `control_flow`, `cold_x105`,
  `cartridge_bootstrap`, and `generated_x105` filters;
- all 421 core tests, 16 inspection-library tests, and 11 CLI tests;
- `fn64_machine_probe` and the expanded `fn64_step_probe`, both ending
  `result: ok`; and
- `./rust/verify-forward`, ending `forward gate: ok`.

Final combined context, clean-checkout, canonical, push, and output-cleanup
evidence is recorded in the direct Master artifact. No private runtime ran.
