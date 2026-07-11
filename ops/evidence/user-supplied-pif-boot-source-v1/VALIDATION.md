# Validation

Candidate: `HEAD`, meaning the commit containing this evidence. The external
artifact records the immutable candidate SHA and complete command logs.

- `RUNTIME_FACT` Exact topology preflight, committed Context-SHA, clean start,
  and `lane-doctor: READY`: passed.
- `RUNTIME_FACT` `cargo test -p fn64-core pif_firmware -- --nocapture`: 9
  passed.
- `RUNTIME_FACT` `cargo test -p fn64-inspection boot_probe -- --nocapture`:
  boot-probe library 9 passed; CLI 8 passed.
- `RUNTIME_FACT` `cargo fmt --all -- --check`: passed.
- `RUNTIME_FACT` `cargo clippy --all-targets -- -D warnings`: passed.
- `RUNTIME_FACT` Focused SP IMEM: 12 passed; aligned `Lw`: 7 passed;
  `Machine::step`: 10 passed; cartridge-bootstrap: 8 passed.
- `RUNTIME_FACT` `PATH=/home/don/.cargo/bin:$PATH ./rust/verify-forward`:
  passed with 367 core tests, 12 inspection-library tests, 8 boot-probe CLI
  tests, `fn64_machine_probe` ending `result: ok`, `fn64_step_probe` ending
  `result: ok`, and `forward gate: ok`.
- `RUNTIME_FACT` Authorized private-ROM identity matched SHA-256
  `c916ab315fbe82a22169bff13d6b866e9fddc907461eb6b0a227b82acdf5b506`
  and size `33554432`.
- `RUNTIME_FACT` The no-PIF command attempted 2 steps, committed 1, remained at
  BOOT-2 with `pc=0xA4000044`, `next_pc=0xA4000048`, Count 1, and rejected
  `Lw` at SP IMEM offset zero before mutation.

No test is classified as authentic firmware validation. Compatibility claim:
none.
