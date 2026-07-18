# Validation

Status: candidate validation passed.

## Focused filters

All required filters selected at least one core test and passed:

| Filter | Selected tests |
| --- | ---: |
| `sp_imem` | 27 |
| `opaque` | 11 |
| `opaque_word` | 2 |
| `provenance` | 10 |
| `store_word` | 12 |
| `load_word` | 11 |
| `address` | 47 |
| `adel` | 15 |
| `ades` | 15 |
| `delay_slot` | 27 |
| `machine_step` | 12 |
| `bootstrap` | 42 |
| `cold_x105` | 9 |
| `generated_x105` | 1 |
| `init_cc` | 1 |
| `find_cc` | 1 |
| `source_knownness` | 1 |
| `pending_transfer` | 2 |
| `mi_init_transfer` | 1 |
| `mi_init_mode` | 5 |
| `first_responder` | 4 |
| `rdram_device_id` | 4 |
| `mi_version` | 8 |
| `ri_mode` | 7 |
| `ri_select` | 10 |
| `ri_config` | 7 |
| `ri_current_load` | 7 |

## Complete candidate validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `fn64-core`: 506 unit tests passed; 0 failed; 0 doctests.
- `fn64-inspection`: 16 library tests passed; 0 failed; 0 doctests.
- CLI integration: 11 tests passed; 0 failed.
- no-window machine probe: `no-window: ok`, `result: ok`.
- no-window step probe: 157 stable cases; `no-window: ok`, `result: ok`.
- complete Rust forward gate: `forward gate: ok`.
- context verifier: 15 checks, 0 errors; `result: ok`.
- candidate Context-SHA:
  `4f938e716f8ca5083badc153345309d89ca5eb196292941f8d52f1569c9e2a65`.
- fleet harness: all eight named checks passed.
- integration queue: `integration-queue: ok`.
- repository-local `rust/target`: absent.

Every Cargo invocation used the packet-owned target and temporary directories.
No frontend, SDL/window path, private PIF input, or commercial ROM was run or
read.

Independent clean-checkout validation, binary-safe patch reproduction,
canonical validation, integration, push, and artifact sealing depend on the
final committed candidate SHA and are operation-level proofs recorded in the
sealed delivery artifact rather than preclaimed repository truth.
