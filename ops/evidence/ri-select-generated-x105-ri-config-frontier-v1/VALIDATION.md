# Validation

Candidate-tree validation used only the pass-owned home-backed Cargo target and
fixture root. The dirty-tree resulting Context-SHA before the reconciliation
commit was
`ee5540e2be89003a83ded029facf9e22e97fd9fcc93997e144d4d4205cbdf2e5`.

- `cargo fmt --all -- --check`: pass;
- `cargo clippy --all-targets -- -D warnings`: pass;
- all nonzero requested filters (`ri`, `ri_select`, `load_word`, `address`,
  `machine_step`, `control_flow`, `branch`, `delay_slot`, `store_word`,
  `sp_imem`, `cartridge_bootstrap`, `cold_x105`, `generated_x105`): pass;
- complete tests: 434 core, 16 inspection-library, and 11 CLI tests pass;
- `fn64_machine_probe` and sixty-two-case `fn64_step_probe`: `result: ok`;
- `./rust/verify-forward`: `forward gate: ok`;
- context/local-link verifier: 15 checks, 0 errors;
- fleet diagnostics: 52 passed; integration queue: empty.

The final committed candidate, clean checkout, canonical main, exact committed
Context-SHA values, output cleanup, and push proof are recorded externally. A
repository file cannot contain the hash of the commit that contains itself.
