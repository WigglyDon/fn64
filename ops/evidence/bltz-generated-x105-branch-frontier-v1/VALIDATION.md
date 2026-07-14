# Validation

Required final validation is recorded in the Master artifact, not as mutable
terminal-wall output here. The accepted proof set is:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets -- -D warnings`;
- nonzero focused filters for BLTZ, REGIMM, control flow, branches, delay slots,
  branch-delay exceptions, Machine step, SP-IMEM stores/AdES, cold x105, and
  generated x105 composition;
- direct `fn64_step_probe`, ending `result: ok`;
- `./rust/verify-forward`, ending `forward gate: ok`;
- context, fleet, queue, link, and clean-checkout verification.

No private ROM, private PIF, C++, CMake, SDL, window, audio, or external
emulator runtime is part of validation.
