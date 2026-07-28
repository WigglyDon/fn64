# Validation

Candidate product proofs:

- 40 required focused filters selected nonzero tests and passed;
- full fn64-core: 627 passed, 0 failed;
- fn64-inspection all targets: 29 passed, 0 failed;
- CLI integration: 11 passed, 0 failed;
- public generated x105 scalar-control/semaphore/second-DMA/Vsub gate:
  1 passed, 0 failed;
- no-window `fn64_step_probe`: `no-window: ok`, `result: ok`.
- no-window `fn64_machine_probe`: `no-window: ok`, `result: ok`;
- stable step-probe cases: 206;
- formatting: clean;
- clippy all targets with warnings denied: clean;
- complete Rust gate: `forward gate: ok`;
- context/local links: 15 checks, 0 errors;
- fleet: 52 passed;
- integration queue: healthy;
- candidate Context-SHA:
  `32cc0ae3b55d925080de49594be12db6866bb8f9f455b18e624f5567374b14f0`.

The independent exact-SHA checkout and canonical-after-integration gates are
sealed in the release artifact because their exact committed SHAs do not yet
exist at repository-evidence authoring time.
