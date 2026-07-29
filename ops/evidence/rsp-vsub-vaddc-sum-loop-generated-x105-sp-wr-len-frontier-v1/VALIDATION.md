# Validation

Pre-commit candidate validation:

- `cargo fmt --all -- --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- required focused nonzero filters: 35/35 pass
- fn64-core: 634 passed
- fn64-inspection library: 16 passed
- CLI integration: 2 passed
- inspection binary tests: 11 passed
- `fn64_machine_probe`: `no-window: ok`, `result: ok`
- `fn64_step_probe`: `no-window: ok`, `result: ok`
- `./rust/verify-forward`: `forward gate: ok`
- exact candidate public x105 gate: pass

Every Cargo invocation used the packet-owned target and TMPDIR beneath
`.fn64-codex/cache/rsp-vsub-vaddc-sum-loop-x105-sp-wr-len-frontier-v1`.
Exact committed-SHA clean-checkout and canonical-after-integration results are
release-artifact facts, not self-referential repository claims.
