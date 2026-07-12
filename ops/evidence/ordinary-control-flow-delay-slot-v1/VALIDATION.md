# Validation

Candidate identity: the commit containing this file. The external artifact and
authoritative Worker packet own the exact immutable candidate SHA; this
repository-contained file does not claim a self-referential commit digest.

Results recorded before final candidate sealing:

- `RUNTIME_FACT`: `cargo test -p fn64-core control_flow -- --nocapture`:
  21 passed.
- `RUNTIME_FACT`: `cargo test -p fn64-core branch_delay_exception --
  --nocapture`: 3 passed.
- `RUNTIME_FACT`: `cargo test -p fn64-core bootstrap -- --nocapture`: 13
  passed.
- `RUNTIME_FACT`: `cargo test -p fn64-core --lib`: 385 passed.
- `RUNTIME_FACT`: `cargo clippy --all-targets -- -D warnings`: passed.
- `RUNTIME_FACT`: `cargo run -p fn64-inspection --bin fn64_step_probe`:
  fourteen case markers, `no-window: ok`, and final `result: ok`.
- `RUNTIME_FACT`: `cargo test machine_step -- --nocapture`: 10 core tests
  passed.
- `RUNTIME_FACT`: `cargo test load_word -- --nocapture`: 7 core tests
  passed.
- `RUNTIME_FACT`: `cargo test pif_firmware -- --nocapture`: 9 core and 1
  inspection test passed.
- `RUNTIME_FACT`: `cargo test bootstrap -- --nocapture`: 13 core and 1
  inspection test passed.
- `RUNTIME_FACT`: `cargo test sp_imem -- --nocapture`: 12 core and 3
  inspection tests passed.
- `RUNTIME_FACT`: `PATH=/home/don/.cargo/bin:$PATH ./rust/verify-forward`:
  formatting, warnings-denied clippy, 385 core tests, 12 inspection tests, 8
  boot-probe CLI tests, both no-window probes, and final `forward gate: ok`.

These results cover the current source content. The external artifact records
the candidate-bound rerun and complete logs. No passing result implies
cartridge boot, timing, host runtime, graphics, audio, or game compatibility.
