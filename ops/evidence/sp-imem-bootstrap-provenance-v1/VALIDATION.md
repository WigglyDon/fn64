# Validation

Candidate: `HEAD`, meaning the commit containing this evidence. The external
artifact records the immutable candidate SHA, resulting Context-SHA, exact
commands, exit statuses, and complete bounded logs.

Required validation set:

- `RUNTIME_FACT` topology preflight and lane-doctor: exact assigned topology,
  clean start, committed Context-SHA, `lane-doctor: READY`.
- `RUNTIME_FACT` source and hypothesis audit: H1 through H7 classified; no
  material contradiction silently discarded.
- `RUNTIME_FACT` source/copyright audit: no external code, firmware byte,
  private ROM byte, generated binary, or prohibited selector entered Git.
- `RUNTIME_FACT` context-neutral focused SP IMEM, `Lw`, bootstrap lifecycle,
  Machine-step, and boot-probe tests: passed.
- `RUNTIME_FACT` `cargo fmt --all -- --check`: passed.
- `RUNTIME_FACT` `cargo clippy --all-targets -- -D warnings`: passed.
- `RUNTIME_FACT` `PATH=/home/don/.cargo/bin:$PATH ./rust/verify-forward`:
  passed; both required probes ended `result: ok` and the verifier ended
  `forward gate: ok`.
- `RUNTIME_FACT` authorized private input identity matched the assigned digest
  and size. Its bootcode-family classification and bounded no-window trace are
  external only.
- `RUNTIME_FACT` bounded private trace: unchanged BOOT-2 frontier, one committed
  authentic `SpecialAdd`, followed by pre-mutation rejection of the represented
  `Lw` at unknown SP IMEM offset zero.
- `RUNTIME_FACT` post-validation hygiene: `rust/target` removed; no ROM-like
  file, private path, build output, or generated log staged.

Compatibility claim: none.
