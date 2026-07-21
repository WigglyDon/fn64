# Validation

## Candidate tree

- focused generated subsystem proof: 1 passed, 527 filtered out;
- `cargo fmt --check`: passed;
- workspace all-target clippy with warnings denied: passed;
- complete `fn64-core`: 528 passed, 0 failed;
- inspection library: 16 passed, 0 failed;
- CLI integration: 11 passed, 0 failed;
- doc tests: 0 tests, passed;
- no-window machine probe: construction, reset, and no-window checks passed;
- no-window step probe: 174 stable cases passed;
- complete `rust/verify-forward`: `forward gate: ok`;
- candidate Context-SHA:
  `d34397b3c562c074383819cd56c7431b5cb260a1b8be78a15149ae2f0eebb313`;
- context and local-link verification: 15 checks, 0 errors, `result: ok`;
- fleet diagnostics: 52 checks passed, `result: ok`;
- integration queue: `integration-queue: ok`;
- `git diff --check`: passed.

The generated subsystem proof committed 214,734 public `Machine::step` calls
from the accepted start state and stopped before executing the cache-specific
word at `0xA4000400`. It proved two present 2 MiB modules, one absent-module
probe, deterministic calibration, final linear mapping, RI_REFRESH
`0x001E3634`, and guest-detected capacity `0x00400000`.

## Remaining seal gates

The exact committed candidate, detached clean-checkout, post-fast-forward
canonical, patch-reproduction, and archive checks are recorded here only after
their corresponding immutable SHAs exist.
