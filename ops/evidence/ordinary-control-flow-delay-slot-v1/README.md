# Ordinary control flow and delay slot v1

- `USER_DECISION`: this lane represents `BEQ`, `BNE`, `J`, `JAL`, `JR`, and
  `JALR` through public `Machine::step` with one explicit delay slot.
- `LIVE_REPO_FACT`: identity-specific planning and Machine-owned application
  live in `rust/crates/fn64-core/src/machine.rs`.
- `LIVE_REPO_FACT`: CPU control-flow state owns one optional delay-slot context
  containing the owning branch-or-jump PC.
- `LIVE_REPO_FACT`: COP0 remains the only owner of EPC, Cause.BD, exception
  code, BadVAddr, EXL, and exception-vector entry.
- `RUNTIME_FACT`: generated tests cover target/link formulas, aliases, taken
  and untaken cadence, all-six inner-control-flow rejection, and arithmetic,
  fetch-AdEL, and data-AdEL delay-slot exceptions.
- `RUNTIME_FACT`: the extended `fn64_step_probe` uses generated words and
  public `Machine::step`; it opens no window and ends with `result: ok`.
- `UNKNOWN`: game compatibility, cartridge boot beyond accepted BOOT-2,
  timing, interrupts, ERET, branch-likely, and unassigned control flow remain
  unearned.

Private ROM used: no. Private PIF used: no. Compatibility claim: none.
