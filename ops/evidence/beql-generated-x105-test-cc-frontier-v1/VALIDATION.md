# Validation

Status: complete candidate validation passed.

## Focused filters

Every required filter selected at least one core proof and passed:

| Filter | Selected tests |
| --- | ---: |
| `beql` | 8 |
| `branch_likely` | 1 |
| `annul` | 3 |
| `branch` | 8 |
| `control_flow` | 21 |
| `delay_slot` | 28 |
| `exception` | 24 |
| `source_knownness` | 1 |
| `machine_step` | 12 |
| `generated_x105` | 1 |
| `find_cc` | 1 |
| `test_cc` | 1 |
| `write_cc` | 1 |
| `current_control` | 1 |
| `rdram_mode` | 1 |
| `opaque` | 10 |
| `opaque_word` | 2 |
| `sp_imem` | 28 |
| `bootstrap` | 42 |
| `mi_version` | 8 |
| `first_responder` | 4 |
| `rdram_device_id` | 4 |
| `ri_mode` | 7 |
| `ri_select` | 10 |
| `ri_config` | 7 |
| `ri_current_load` | 7 |

## Complete candidate validation

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `fn64-core`: 513 unit tests passed; 0 failed; 0 doctests.
- `fn64-inspection`: 16 library tests passed; 0 failed; 0 doctests.
- CLI integration: 11 tests passed; 0 failed.
- no-window machine probe: 2 stable checks; `no-window: ok`; `result: ok`.
- no-window step probe: 166 stable cases; `no-window: ok`; `result: ok`.
- complete Rust forward gate: `forward gate: ok`.
- context/local-link verifier: 15 checks, 0 errors; `result: ok`.
- candidate Context-SHA:
  `c835153f61126ffe02ccded30eed89f19192e6e569273f42f8a6de88959a3bf9`.
- fleet harness: 52 checks passed; `result: ok`.
- integration queue: `integration-queue: ok`.
- repository-local `rust/target`: absent.

Every Cargo invocation uses packet-owned `CARGO_TARGET_DIR` and `TMPDIR`. No
frontend, SDL/window path, private PIF input, or commercial ROM is run or read.

Independent clean-checkout validation, binary-safe patch reproduction,
canonical validation, integration, push, and artifact sealing depend on the
final committed candidate SHA and are operation-level proofs recorded in the
sealed delivery artifact rather than preclaimed repository truth.
