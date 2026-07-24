# Validation

Focused and complete candidate validation:

- redacted probe parser and basename tests: 2 passed;
- BEQL focused regression: 7 passed;
- generated x105 final-handoff regression: 1 passed;
- complete fn64-core suite: 594 passed.
- fn64-inspection library: 16 passed;
- CLI integration: 11 passed;
- no-window Machine probe: result `ok`;
- no-window step probe: 187 cases and result `ok`;
- formatting: clean;
- clippy, all workspace targets with warnings denied: clean;
- complete Rust forward gate: `forward gate: ok`;
- context verifier: 15 checks, zero errors;
- fleet verifier: 52 checks passed;
- integration queue: `ok`.

The detached exact-SHA, canonical, patch-reproduction, and artifact results are
recorded in the sealed packet report after their final commands complete. No
local user-ROM result is used as a standard CI dependency.
