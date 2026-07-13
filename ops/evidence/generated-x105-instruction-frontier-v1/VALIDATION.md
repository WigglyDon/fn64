# Validation

Validated product/probe candidate: `80e1869b129098f01ef248e47377a43879b373a0`.

Home-backed generated paths:

- Cargo target:
  `/home/don/.cache/fn64/cargo-target-direct-generated-x105-frontier-v1`
- temporary fixtures:
  `/home/don/.cache/fn64/tmp-direct-generated-x105-frontier-v1`

Results:

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --all-targets -- -D warnings`: pass.
- focused `load_word`: 10 passed.
- focused `sp_dmem`: 16 passed.
- focused `machine_step`: 12 passed.
- focused `control_flow`: 21 passed.
- focused `cold_x105`: 9 passed.
- focused `cartridge_bootstrap`: 29 passed.
- focused `sp_imem`: 12 passed.
- boot-probe library filter: 13 passed.
- boot-probe CLI integration target: 11 passed.
- complete core suite: 408 passed.
- complete inspection suite: 16 passed.
- complete CLI suite: 11 passed.
- `fn64_machine_probe`: `result: ok`.
- `fn64_step_probe`: all stable markers present and `result: ok`.
- `rust/verify-forward`: `forward gate: ok`.

An initial CLI run using the operating system default temporary directory
failed before product execution with `Disk quota exceeded`. The exact same
tests passed after setting `TMPDIR` to the new pass-owned home-backed directory
above. The environment failure is not counted as product validation.

The generated probe reaches `pc=0xA4000050`, `next_pc=0xA4000054`, Count `4`,
then identifies aligned `Sw` as the next unrepresented frontier without
mutation. No private runtime or proprietary input was used.
