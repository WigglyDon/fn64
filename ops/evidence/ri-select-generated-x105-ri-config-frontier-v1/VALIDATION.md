# Validation

Pre-reconciliation focused proof:

- `cargo test ri_select -- --nocapture`: pass;
- extended generated-x105 RI_CONFIG-frontier test: pass;
- direct `fn64_step_probe`: pass, final marker `result: ok`.

All commands use the pass-owned home-backed Cargo target and fixture root.
Format, clippy, all focused filters, the complete Rust forward gate,
clean-checkout proof, context/fleet/queue/link checks, canonical validation, and
final SHAs are recorded after reconciliation.
