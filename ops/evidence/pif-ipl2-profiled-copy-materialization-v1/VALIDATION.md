# Validation

Exact Repair 2 product-source validation SHA:
`4eaa33d9fc59182d8e69a24edb39ee3be9ff8797`.

`RUNTIME_FACT` at that SHA:

Cargo commands ran from `rust/` with
`PATH=/home/don/.cargo/bin:$PATH` and
`TMPDIR=/home/don/fn64-worktrees/pif-ipl2-profiled-copy-materialization-v1`.
The forward gate ran from the repository root with the same environment.

| Command | Result |
| --- | --- |
| `cargo test -p fn64-core pif -- --nocapture` | 25 passed |
| `cargo test -p fn64-core cartridge_bootstrap -- --nocapture` | 20 passed |
| `cargo test -p fn64-core sp_imem -- --nocapture` | 13 passed |
| `cargo test -p fn64-core load_word -- --nocapture` | 7 passed |
| `cargo test -p fn64-core machine_step -- --nocapture` | 11 passed |
| `cargo test -p fn64-inspection boot_probe -- --nocapture` | 11 library and 10 CLI tests passed |
| `cargo test -p fn64-inspection --test boot_probe_cli -- --nocapture` | 10 passed |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --all-targets -- -D warnings` | pass |
| `./rust/verify-forward` | 396 core, 14 inspection, 10 CLI; both probes `result: ok`; `forward gate: ok` |

`RUNTIME_FACT`: the core CLI-boundary audit found no `from_cli_name`,
`cli_name`, `--pif-profile`, `ntsc-pinned`, `pal-pinned`, or `mpal-pinned` in
`rust/crates/fn64-core`. The inspection ownership audit found the exact option
surface and three spellings in `fn64-inspection`, with no duplicated copy range
or host-owned SP IMEM mutation.

The evidence-repair commit necessarily follows this file. The exact final
candidate SHA, repeated final gate, Context-SHA, artifact identity, and clean
state belong to the external lane artifact and Worker final report, avoiding a
self-hash claim inside the commit that contains this evidence.

`WORKER_CLAIM`: the final candidate and artifact remain subject to independent
Supervisor and Master verification; this file does not convert that acceptance
claim into a runtime fact.
