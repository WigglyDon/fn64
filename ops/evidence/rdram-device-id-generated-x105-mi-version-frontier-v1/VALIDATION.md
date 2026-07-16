# Validation

Implementation-stage proof:

- `cargo test -p fn64-core rdram_device_id`: 4 passed;
- `cargo run -p fn64-inspection --bin fn64_step_probe`: `no-window: ok`, `result: ok`.

Final candidate, clean-checkout, context/fleet, canonical, and push results are recorded after complete gates. Every Cargo invocation uses packet-owned target/TMPDIR paths; repository `rust/target` remains absent.
