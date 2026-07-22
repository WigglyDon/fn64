# Validation

Focused product proofs reached:

- complete `fn64-core` suite: 550 passed, 0 failed;
- `fn64-inspection` library: 16 passed, 0 failed;
- CLI integration: 11 passed, 0 failed;
- exact generated final-handoff composition: 1 passed, 0 failed;
- PI-focused filter: 9 passed, 0 failed;
- D-cache-focused filter: 1 passed, 0 failed;
- BGEZAL-focused filter: 1 passed, 0 failed;
- no-window machine probe: `construct: ok`, `reset: ok`, `no-window: ok`,
  `result: ok`;
- stable no-window step probe: 183 cases, `no-window: ok`, `result: ok`;
- formatting: clean;
- clippy, all workspace targets with warnings denied: clean after one narrow
  derivable-`Default` and test type-alias correction;
- complete Rust forward gate: `forward gate: ok`;
- context/local-link verification: 15 checks, 0 errors, `result: ok`;
- fleet verification: 52 passed, `result: ok`;
- integration queue: `integration-queue: ok`;
- candidate Context-SHA:
  `559c15faf390d422467dcbd48b4156fe03fadbfb0bd0a512ecefc2411698cfd0`.

The exact generated run commits 7,225,461 post-frontier steps, including
262,144 checksum iterations and 2,048 SP teardown stores, then reaches
`0x80001000` without executing its first instruction.

The first exact-name generated filter attempt selected zero tests because its
report shorthand did not match the Rust suffix; the corrected exact filter
selected and passed one test. No product state changed in that retry.

Detached exact-SHA, canonical post-fast-forward, and artifact verification are
post-commit gates and belong to the sealed external product report rather than
a self-referential committed evidence SHA.
