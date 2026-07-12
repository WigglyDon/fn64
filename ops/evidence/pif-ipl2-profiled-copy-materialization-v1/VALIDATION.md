# Validation

Exact repaired product-source validation SHA:
`2ca21994bba489bc5ef645ee91547479fec8d070`.

`RUNTIME_FACT` at that SHA:

| Command | Result |
| --- | --- |
| `cargo test -p fn64-core pif -- --nocapture` | 25 passed |
| `cargo test -p fn64-core cartridge_bootstrap -- --nocapture` | 20 passed |
| `cargo test -p fn64-core sp_imem -- --nocapture` | 13 passed |
| `cargo test -p fn64-core load_word -- --nocapture` | 7 passed |
| `cargo test -p fn64-core machine_step -- --nocapture` | 11 passed |
| `cargo test -p fn64-inspection boot_probe -- --nocapture` | 10 filtered library and 10 CLI tests passed |
| `cargo test -p fn64-inspection --test boot_probe_cli -- --nocapture` | 10 passed |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --all-targets -- -D warnings` | pass |
| `./rust/verify-forward` | 396 core, 13 inspection, 10 CLI; both probes `result: ok`; `forward gate: ok` |

The evidence-repair commit necessarily follows this file. The exact final
candidate SHA, repeated final gate, Context-SHA, artifact identity, and clean
state belong to the external lane artifact and Worker final report, avoiding a
self-hash claim inside the commit that contains this evidence.
