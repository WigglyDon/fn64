# Validation

Focused pre-commit results:

- `cargo test -p fn64-core mi_version`: 9 passed;
- `cargo test -p fn64-core generated_x105`: 1 passed;
- no-window `fn64_step_probe`: fixed identity, committed read, RCP 2.0 branch,
  and first-responder frontier cases pass; `no-window: ok`, `result: ok`.

Final candidate, clean-checkout, canonical, Context-SHA, probe-case counts,
fleet, queue, patch reproduction, and cleanup results are sealed in the
external Master artifact after validation. Every Cargo invocation uses
packet-owned `CARGO_TARGET_DIR` and `TMPDIR`; repository `rust/target`
remains absent.
