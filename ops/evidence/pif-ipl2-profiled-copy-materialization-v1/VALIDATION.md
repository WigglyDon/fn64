# Validation

Exact source/process validation SHA:
`725ff7b2811f886d844858d9fa1fd96359505fb7`.

`RUNTIME_FACT` at that SHA:

| Command | Result |
| --- | --- |
| `cargo test -p fn64-core pif -- --nocapture` | 20 passed |
| `cargo test -p fn64-core cartridge_bootstrap -- --nocapture` | 16 passed |
| `cargo test -p fn64-core sp_imem -- --nocapture` | 13 passed |
| `cargo test -p fn64-core load_word -- --nocapture` | 7 passed |
| `cargo test -p fn64-core machine_step -- --nocapture` | 11 passed |
| `cargo test -p fn64-inspection boot_probe -- --nocapture` | 9 library and 9 CLI tests passed |
| `cargo test -p fn64-inspection --test boot_probe_cli -- --nocapture` | 9 passed |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --all-targets -- -D warnings` | pass |
| `./rust/verify-forward` | 392 core, 12 inspection, 9 CLI; both probes `result: ok`; `forward gate: ok` |

The evidence-seal commit necessarily follows this file. The exact final
candidate SHA, repeated final gate, Context-SHA, artifact identity, and clean
state belong to the external lane artifact and Worker final report, avoiding a
self-hash claim inside the commit that contains this evidence.
