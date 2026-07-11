# Validation

All commands used `PATH=/home/don/.cargo/bin:$PATH` where Cargo was involved.
Exact post-commit logs and candidate SHA are owned by the external artifact.

- `RUNTIME_FACT` Topology preflight: exact worker root, branch
  `worker/boot-frontier-sp-imem-lw-v1`, starting HEAD
  `5f77d2df6005fe34ebb20f4751c2980ff73c57f1`, clean worktree/index, expected
  Context-SHA, and `lane-doctor: READY`.
- `RUNTIME_FACT` `cargo test -p fn64-core load_word`: 7 passed.
- `RUNTIME_FACT` bootstrap unknown-source focused test: 1 passed.
- `RUNTIME_FACT` `cargo test -p fn64-core`: 358 passed.
- `RUNTIME_FACT` `cargo test -p fn64-inspection boot_probe`: boot-probe library
  7 passed and boot-probe CLI 3 passed.
- `RUNTIME_FACT` `cargo fmt --all -- --check`: passed.
- `RUNTIME_FACT` `cargo clippy --all-targets -- -D warnings`: passed.
- `RUNTIME_FACT` `./rust/verify-forward`: format passed, clippy passed, all Rust
  tests passed, `fn64_machine_probe` ended `result: ok`, `fn64_step_probe`
  retained all accepted cases and ended `result: ok`, and the verifier ended
  `forward gate: ok`.
- `RUNTIME_FACT` Exact private no-window command exited 0 and reported the
  expected fail-closed SP IMEM provenance frontier. The external artifact owns
  the digest, size, structural metadata, and complete bounded output.
- `LIVE_REPO_FACT` No test was removed. No Cargo metadata, documentation,
  fleet, queue, branch, worktree, canonical-main, runtime-host, SDL, window, or
  audio path changed.
